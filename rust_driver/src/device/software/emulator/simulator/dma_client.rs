//! Direct Memory Access Client

use core::marker::PhantomData;
use core::mem::MaybeUninit;

use super::config::WORD_WIDTH;
use super::rpc;
use crate::device::software::emulator::dma::{self, DmaAddress, PointerMut};

#[derive(Debug)]
pub struct DmaClient<R: rpc::Client = rpc::RpcClient> {
    client_id: u64,

    rpc: R,
}

impl<R: rpc::Client> DmaClient<R> {
    pub const fn new(client_id: u64, rpc: R) -> Self {
        Self { client_id, rpc }
    }

    fn read_at_64(&self, addr: u64) -> [u8; 64] {
        let mut result = [0u8; 64];
        unsafe {
            self.rpc.c_readBRAM(
                (&raw mut result).cast(),
                self.client_id,
                addr / WORD_WIDTH as u64,
                WORD_WIDTH,
            );
        }
        result
    }

    fn write_at_most_64(&self, addr: u64, data: &[u8]) -> usize {
        debug_assert!(!data.is_empty());

        let len = data.len().min(64);
        let byte_en = if len == 64 {
            [u8::MAX; 8]
        } else {
            let byte_en: u64 = (1 << len) - 1;
            byte_en.to_ne_bytes()
        };

        unsafe {
            self.rpc.c_writeBRAM(
                self.client_id,
                addr,
                data.as_ptr().cast::<u32>().cast_mut(),
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

        while n_read < size {
            let n_bytes = (size - n_read).min(64);

            let buf = self.read_at_64(addr.checked_add(u64::try_from(n_read).unwrap()).unwrap());

            // #![feature(maybe_uninit_write_slice)] <https://github.com/rust-lang/rust/issues/79995>
            for i in 0..n_bytes {
                let _result = slice[n_read].write(buf[i]);
                debug_assert_eq!(*_result, buf[i]);

                n_read += 1;
            }
        }

        log::debug!("DMA: read  {size} bytes: {slice:02X?}");

        debug_assert_eq!(n_read, size);
        unsafe { ret.assume_init() }
    }

    unsafe fn write<T>(&self, addr: u64, val: T) {
        let size = size_of::<T>();

        // Safety: Byte slice
        let slice = unsafe { core::slice::from_raw_parts::<u8>((&raw const val).cast(), size) };

        let mut data = slice;
        while !data.is_empty() {
            let n_written = self.write_at_most_64(addr, data);
            data = &data[n_written..];
        }

        log::debug!("DMA: write {size} bytes: {slice:02X?}");
        // Assert read back same
        {
            let read_back = unsafe { self.read::<T>(addr) };
            let read_back_slice = unsafe { core::slice::from_raw_parts::<u8>((&raw const read_back).cast(), size) };
            debug_assert_eq!(read_back_slice, slice);
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
