//! Direct Memory Access Client

use core::marker::PhantomData;
use core::mem::MaybeUninit;

use super::config::WORD_WIDTH;
use super::rpc;
use crate::device::software::emulator::dma::{self, DmaAddress, PointerMut};

const BYTES_PER_WORD: u32 = WORD_WIDTH / 8;

#[derive(Debug)]
pub struct DmaClient<R: rpc::Client = rpc::RpcClient> {
    client_id: u64,

    rpc: R,
}

impl<R: rpc::Client> DmaClient<R> {
    pub const fn new(client_id: u64, rpc: R) -> Self {
        Self { client_id, rpc }
    }

    fn read_at_64<'b>(&self, addr: u64, buf: &'b mut [u8]) -> &'b [u8] {
        assert!(buf.len() >= 64);

        let buf = &mut buf[..64];
        let start = usize::try_from(addr & u64::from(BYTES_PER_WORD - 1)).unwrap();
        unsafe {
            self.rpc.c_readBRAM(
                buf.as_mut_ptr().cast(),
                self.client_id,
                addr / BYTES_PER_WORD as u64,
                WORD_WIDTH,
            );
        }
        println!("read over @{addr} start@{start} {buf:?}");
        &buf[start..]
    }

    fn write_at_most_64(&self, addr: u64, data: &[u8]) -> usize {
        let mut buf = [0u8; 64];

        let start = usize::try_from(addr & u64::from(BYTES_PER_WORD - 1)).unwrap();
        let len = data
            .len()
            .min(usize::try_from(BYTES_PER_WORD).unwrap().checked_sub(start).unwrap());

        buf[start..start + len].copy_from_slice(&data[..len]);

        let byte_en = if len == 64 {
            [u8::MAX; 8]
        } else {
            let byte_en: u64 = (1 << len) - 1;
            let byte_en = byte_en.checked_shl(start.try_into().unwrap()).unwrap();

            byte_en.to_ne_bytes()
        };

        unsafe {
            self.rpc.c_writeBRAM(
                self.client_id,
                addr / u64::from(BYTES_PER_WORD),
                buf.as_mut_ptr().cast::<u32>(),
                byte_en.as_ptr().cast::<u32>().cast_mut(),
                WORD_WIDTH,
            );
        }
        len
    }

    unsafe fn read<T>(&self, addr: u64) -> T {
        let mut ret = MaybeUninit::uninit();
        let size = size_of::<T>();

        // let mut slice = ret.as_bytes_mut();
        // Safety: #![feature(maybe_uninit_as_bytes) <https://github.com/rust-lang/rust/issues/93092>
        let slice = unsafe { core::slice::from_raw_parts_mut::<MaybeUninit<u8>>((&raw mut ret).cast(), size) };
        let mut n_read = 0;

        let mut buf = [0u8; 64];
        while n_read < size {
            let result = self.read_at_64(addr.checked_add(u64::try_from(n_read).unwrap()).unwrap(), &mut buf);
            let len = result.len().min(size - n_read);

            // #![feature(maybe_uninit_write_slice)] <https://github.com/rust-lang/rust/issues/79995>
            for &val in &result[..len] {
                let &mut _result = slice[n_read].write(val);
                debug_assert_eq!(_result, val);

                n_read += 1;
            }
        }

        log::debug!("DMA: read  {size} bytes: {slice:?}");

        debug_assert_eq!(n_read, size);
        unsafe { ret.assume_init() }
    }

    unsafe fn write<T>(&self, addr: u64, val: T) {
        let size = size_of::<T>();

        // Safety: Byte slice
        let slice = unsafe { core::slice::from_raw_parts::<u8>((&raw const val).cast(), size) };

        let mut address = addr;
        let mut data = slice;
        while !data.is_empty() {
            let n_written = self.write_at_most_64(address, data);

            data = &data[n_written..];
            address = address.checked_add(u64::try_from(n_written).unwrap()).unwrap();
        }

        log::debug!("DMA: write {size} bytes: {slice:?}");
        // Assert read back same
        {
            let read_back = unsafe { self.read::<T>(addr) };
            let read_back_slice = unsafe { core::slice::from_raw_parts::<u8>((&raw const read_back).cast(), size) };
            // debug_assert_eq!(read_back_slice, slice);
        }
    }
}

#[derive(Debug)]
pub struct Ptr<'p, T, R: rpc::Client> {
    addr: DmaAddress,
    client: &'p DmaClient<R>,

    _ptr: PhantomData<*mut T>,
}

impl<'p, T, R: rpc::Client> Ptr<'p, T, R> {
    const fn new<'cli>(addr: DmaAddress, client: &'cli DmaClient<R>) -> Self
    where
        'cli: 'p,
    {
        Ptr {
            addr,
            client,
            _ptr: PhantomData,
        }
    }
}

impl<T, R: rpc::Client> Clone for Ptr<'_, T, R> {
    fn clone(&self) -> Self {
        Ptr::new(self.addr, self.client)
    }
}
impl<T, R: rpc::Client> Copy for Ptr<'_, T, R> {}

impl<T, R: rpc::Client> PointerMut for Ptr<'_, T, R> {
    type Output = T;

    unsafe fn read(self) -> T {
        unsafe { self.client.read(self.addr.into()) }
    }

    unsafe fn write(self, val: T) {
        unsafe { self.client.write(self.addr.into(), val) }
    }
}

impl<R: rpc::Client> dma::Client for DmaClient<R> {
    type PtrMut<'a, T>
        = Ptr<'a, T, R>
    where
        Self: 'a;

    fn new_ptr_mut<T>(&self, addr: DmaAddress) -> Self::PtrMut<'_, T> {
        Ptr::new(addr, self)
    }
}

#[cfg(test)]
mod tests {
    use core::cell::RefCell;

    use rand::rngs::StdRng;
    use rand::Rng;

    use super::*;

    struct MockRpc {
        memory: RefCell<Vec<u8>>,
    }
    impl MockRpc {
        fn new(size: usize) -> Self {
            MockRpc {
                memory: RefCell::new(vec![0; size]),
            }
        }
    }

    impl rpc::Client for MockRpc {
        unsafe fn c_readBRAM(&self, result: *mut u32, _client_id: u64, shrunk_addr: u64, word_width: u32) {
            let memory = self.memory.borrow();
            let bytes_per_word = word_width / 8;

            let addr = usize::try_from(shrunk_addr * u64::from(bytes_per_word)).unwrap();
            let size = usize::try_from(bytes_per_word).unwrap();
            assert_eq!(size, 64);

            let result = unsafe { core::slice::from_raw_parts_mut(result.cast::<u8>(), size) };

            result.copy_from_slice(&memory[addr..addr + size]);

            println!("really read  at {addr} {size} bytes: {result:?}");
        }

        unsafe fn c_writeBRAM(
            &self,
            _client_id: u64,
            shrunk_addr: u64,
            data: *mut u32,
            byte_en: *mut u32,
            word_width: u32,
        ) {
            let mut memory = self.memory.borrow_mut();
            let bytes_per_word = word_width / 8;

            let addr = usize::try_from(shrunk_addr * u64::from(bytes_per_word)).unwrap();

            println!("byte_en {:?}", unsafe { core::mem::transmute::<_, &[u8; 8]>(byte_en) });
            let byte_en = unsafe { byte_en.cast::<u64>().read() };
            assert_ne!(byte_en, 0);

            let start = usize::try_from(byte_en.trailing_zeros()).unwrap();
            let end = usize::try_from(64 - byte_en.leading_zeros()).unwrap();
            let len = end - start;

            let data = unsafe { core::slice::from_raw_parts(data.cast::<u8>().cast_const().add(start), len) };
            println!("really write at {addr} {len} bytes: {data:?}");

            println!("old: {:?}", &memory[..96]);
            memory[addr..addr + len].copy_from_slice(data);
            println!("new: {:?}", &memory[..96]);
        }
    }

    macro_rules! case {
        ($cli:ident, $addr:literal, $val:expr, $typ:ty) => {
            let val: $typ = $val;
            unsafe { $cli.write::<$typ>($addr, val) };
            assert_eq!(unsafe { $cli.read::<$typ>($addr) }, val);
        };
    }

    #[test]
    fn test_client() {
        use rand::SeedableRng;
        let rpc = MockRpc::new(1024);
        let client = DmaClient::new(0, rpc);

        let mut rng = StdRng::seed_from_u64(0);

        case!(client, 0, [1u8, 2, 3, 4], [u8; 4]);
        case!(client, 0, core::array::from_fn::<u8, 66, _>(|_| rng.gen()), [u8; 66]);
        // case!(client, 20, core::array::from_fn::<u8, 20, _>(|_| rng.gen()), [u8; 20]);
        case!(
            client,
            0,
            core::array::from_fn::<u8, 96, _>(|i| i.try_into().unwrap()),
            [u8; 96]
        );
        case!(
            client,
            32,
            core::array::from_fn::<u8, 96, _>(|i| i.try_into().unwrap()),
            [u8; 96]
        );
        case!(
            client,
            64,
            core::array::from_fn::<u8, 96, _>(|i| i.try_into().unwrap()),
            [u8; 96]
        );
    }
}
