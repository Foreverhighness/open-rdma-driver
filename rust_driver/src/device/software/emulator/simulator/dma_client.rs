//! Direct Memory Access Client

use core::marker::PhantomData;
use core::mem::MaybeUninit;
use std::thread::JoinHandle;

use super::config::WORD_WIDTH;
use super::rpc;
use crate::device::software::emulator::address::DmaAddress;
use crate::device::software::emulator::dma;

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

        let data = &buf[start..];

        log::trace!(
            "read  at {addr:#018X}, start at {start:02}, buf: {buf:02X?}, data: {data:02X?}",
            addr = addr & !(u64::from(BYTES_PER_WORD) - 1),
            data = data
        );

        data
    }

    fn write_at_most_64(&self, addr: u64, data: &[u8], write_group: &mut Vec<JoinHandle<()>>) -> usize {
        let mut buf = [0u8; 64];

        let start = usize::try_from(addr & u64::from(BYTES_PER_WORD - 1)).unwrap();
        let len = data
            .len()
            .min(usize::try_from(BYTES_PER_WORD).unwrap().checked_sub(start).unwrap());

        buf[start..start + len].copy_from_slice(&data[..len]);

        log::trace!(
            "write at {addr:#018X}, start at {start:02}, buf: {buf:02X?}, data: {data:02X?}",
            addr = addr & !(u64::from(BYTES_PER_WORD) - 1),
            data = &data[..len]
        );

        let byte_en = if len == 64 {
            [u8::MAX; 8]
        } else {
            let byte_en: u64 = (1 << len) - 1;
            let byte_en = byte_en.checked_shl(start.try_into().unwrap()).unwrap();

            byte_en.to_ne_bytes()
        };

        let rpc = self.rpc.clone();
        let client_id = self.client_id;
        let handler = std::thread::spawn(move || unsafe {
            rpc.c_writeBRAM(
                client_id,
                addr / u64::from(BYTES_PER_WORD),
                buf.as_mut_ptr().cast::<u32>(),
                byte_en.as_ptr().cast::<u32>().cast_mut(),
                WORD_WIDTH,
            );
        });
        write_group.push(handler);

        len
    }

    /// Reads the value from `src` without moving it. This leaves the
    /// memory in `src` unchanged.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if any of the following conditions are violated:
    ///
    /// * `src` must be [valid] for reads.
    ///
    /// * `src` must be properly aligned. Use [`read_unaligned`] if this is not the case.
    ///
    /// * `src` must point to a properly initialized value of type `T`.
    ///
    /// Note that even if `T` has size `0`, the pointer must be properly aligned.
    unsafe fn read<T>(&self, src: u64) -> T {
        assert_eq!(src % u64::try_from(align_of::<T>()).unwrap(), 0);

        let mut ret = MaybeUninit::uninit();
        let size = size_of::<T>();

        // let mut slice = ret.as_bytes_mut();
        // Safety: #![feature(maybe_uninit_as_bytes) <https://github.com/rust-lang/rust/issues/93092>
        let slice = unsafe { core::slice::from_raw_parts_mut::<MaybeUninit<u8>>((&raw mut ret).cast(), size) };
        let mut n_read = 0;

        let mut buf = [0u8; 64];
        while n_read < size {
            let result = self.read_at_64(src.checked_add(u64::try_from(n_read).unwrap()).unwrap(), &mut buf);
            let len = result.len().min(size - n_read);

            // #![feature(maybe_uninit_write_slice)] <https://github.com/rust-lang/rust/issues/79995>
            for &val in &result[..len] {
                let &mut _result = slice[n_read].write(val);
                debug_assert_eq!(_result, val);

                n_read += 1;
            }
        }

        log::debug!("DMA: read @ {src:#018X} {size:02} bytes: {:02X?}", unsafe {
            core::mem::transmute::<_, &mut [u8]>(slice)
        });

        debug_assert_eq!(n_read, size);

        // Safety: Each byte has been written.
        unsafe { ret.assume_init() }
    }

    /// Overwrites a memory location with the given value without reading or
    /// dropping the old value.
    ///
    /// `write` does not drop the contents of `dst`. This is safe, but it could leak
    /// allocations or resources, so care should be taken not to overwrite an object
    /// that should be dropped.
    ///
    /// Additionally, it does not drop `src`. Semantically, `src` is moved into the
    /// location pointed to by `dst`.
    ///
    /// This is appropriate for initializing uninitialized memory, or overwriting
    /// memory that has previously been [`read`] from.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if any of the following conditions are violated:
    ///
    /// * `dst` must be [valid] for writes.
    ///
    /// * `dst` must be properly aligned. Use [`write_unaligned`] if this is not the case.
    ///
    /// Note that even if `T` has size `0`, the pointer must be properly aligned.
    unsafe fn write<T>(&self, dst: u64, src: T) {
        let size = size_of::<T>();

        // Safety: Byte slice
        let slice = unsafe { core::slice::from_raw_parts::<u8>((&raw const src).cast(), size) };

        unsafe { self.write_bytes(dst, slice) };

        log::debug!("DMA: write @ {dst:#018X} {size:02} bytes: {slice:?}");

        // Assert read back same, for debugging purpose
        // {
        //     let read_back = unsafe { self.read::<T>(dst) };
        //     let read_back_slice = unsafe { core::slice::from_raw_parts::<u8>((&raw const read_back).cast(), size) };
        //     debug_assert_eq!(read_back_slice, slice);
        // }
    }

    unsafe fn write_bytes(&self, mut addr: u64, mut data: &[u8]) {
        let mut write_group = Vec::with_capacity(data.len() / 64 + 2);

        while !data.is_empty() {
            let n_written = self.write_at_most_64(addr, data, &mut write_group);

            data = &data[n_written..];
            addr = addr.checked_add(u64::try_from(n_written).unwrap()).unwrap();
        }

        // TODO(fh): replace with `std::thread::scope`?
        for handler in write_group {
            handler.join().unwrap();
        }
    }

    /// Copies `count * size_of::<T>()` bytes from `src` to `dst`. The source
    /// and destination must *not* overlap.
    ///
    /// For regions of memory which might overlap, use [`core::ptr::copy`] instead.
    ///
    /// `copy_nonoverlapping` is semantically equivalent to C's [`memcpy`], but
    /// with the argument order swapped.
    ///
    /// The copy is "untyped" in the sense that data may be uninitialized or otherwise violate the
    /// requirements of `T`. The initialization state is preserved exactly.
    ///
    /// [`memcpy`]: https://en.cppreference.com/w/c/string/byte/memcpy
    ///
    /// # Safety
    ///
    /// Behavior is undefined if any of the following conditions are violated:
    ///
    /// * `src` must be [valid] for reads of `count * size_of::<T>()` bytes.
    ///
    /// * `dst` must be [valid] for writes of `count * size_of::<T>()` bytes.
    ///
    /// * Both `src` and `dst` must be properly aligned.
    ///
    /// * The region of memory beginning at `src` with a size of `count * size_of::<T>()` bytes must *not* overlap with
    ///   the region of memory beginning at `dst` with the same size.
    ///
    /// Like [`read`], `copy_nonoverlapping` creates a bitwise copy of `T`, regardless of
    /// whether `T` is [`Copy`]. If `T` is not [`Copy`], using *both* the values
    /// in the region beginning at `*src` and the region beginning at `*dst` can
    /// [violate memory safety][read-ownership].
    ///
    /// Note that even if the effectively copied size (`count * size_of::<T>()`) is
    /// `0`, the pointers must be properly aligned.
    ///
    /// [`read`]: core::ptr::read
    /// [read-ownership]: core::ptr::read#ownership-of-the-returned-value
    /// [valid]: core::ptr#safety
    unsafe fn copy_nonoverlapping<T>(&self, mut src: *const T, dst: u64, count: usize) {
        let size = size_of::<T>();

        for _ in 0..count {
            // Safety: Byte slice
            let slice = unsafe { core::slice::from_raw_parts(src.cast(), size) };
            src = unsafe { src.add(1) };

            unsafe { self.write_bytes(dst, slice) };
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

impl<T, R: rpc::Client> dma::PointerMut for Ptr<'_, T, R> {
    type Output = T;

    unsafe fn read(self) -> T {
        unsafe { self.client.read(self.addr.into()) }
    }

    unsafe fn write(self, val: T) {
        unsafe { self.client.write(self.addr.into(), val) }
    }

    // Turn into more generic version? but it is hard to implement
    // unsafe fn copy_nonoverlapping(self, src: impl PointerConst<Output = T>, count: usize) {}

    unsafe fn copy_nonoverlapping(self, src: *const T, count: usize) {
        unsafe { self.client.copy_nonoverlapping(src, self.addr.into(), count) }
    }

    unsafe fn add(mut self, count: u64) -> Self {
        self.addr = self
            .addr
            .0
            .checked_add(count.checked_mul(u64::try_from(size_of::<T>()).unwrap()).unwrap())
            .unwrap()
            .into();
        self
    }

    unsafe fn write_bytes(self, data: &[u8]) {
        unsafe { self.client.write_bytes(self.addr.into(), data) }
    }
}

impl<R: rpc::Client> dma::Client for DmaClient<R> {
    fn with_dma_addr<T>(&self, addr: DmaAddress) -> impl dma::PointerMut<Output = T> {
        Ptr::new(addr, self)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use rand::rngs::StdRng;
    use rand::Rng;

    use super::*;

    #[derive(Clone)]
    struct MockRpc {
        memory: Arc<Mutex<Vec<u8>>>,
    }
    impl MockRpc {
        fn new(size: usize) -> Self {
            MockRpc {
                memory: Arc::new(Mutex::new(vec![0; size])),
            }
        }
    }

    impl rpc::Client for MockRpc {
        unsafe fn c_readBRAM(&self, result: *mut u32, _client_id: u64, shrunk_addr: u64, word_width: u32) {
            let memory = self.memory.lock().unwrap();
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
            let mut memory = self.memory.lock().unwrap();
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

            println!("old: {:?}", &memory[32..64]);
            memory[addr + start..addr + end].copy_from_slice(data);
            println!("new: {:?}", &memory[32..64]);
        }
    }

    macro_rules! case {
        ($cli:ident, $addr:literal, $val:expr, $typ:ty) => {
            let val: $typ = $val;
            unsafe { $cli.write::<$typ>($addr, val) };
            assert_eq!(unsafe { $cli.read::<$typ>($addr) }, val);
            unsafe { $cli.copy_nonoverlapping(&raw const val, $addr, 1) };
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
        case!(client, 20, core::array::from_fn::<u8, 20, _>(|_| rng.gen()), [u8; 20]);
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
        let values = unsafe { client.read::<[u8; 64 + 96 - 1]>(1) };

        for (expect, &val) in (1..32).zip(&values[0..31]) {
            assert_eq!(expect, val);
        }
        for (expect, &val) in (0..32).zip(&values[31..63]) {
            assert_eq!(expect, val);
        }
        for (expect, &val) in (0..96).zip(&values[63..63 + 96]) {
            assert_eq!(expect, val);
        }
    }
}
