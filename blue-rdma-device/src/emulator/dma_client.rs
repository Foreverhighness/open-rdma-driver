use core::marker::PhantomData;

use crate::address::DmaAddress;
use crate::dma;

#[derive(Debug)]
pub struct DmaClient;

impl dma::Client for DmaClient {
    fn with_dma_addr<T>(&self, addr: DmaAddress) -> impl dma::PointerMut<'_, Output = T> {
        Ptr {
            inner: core::ptr::null_mut::<T>().with_addr(addr.0.try_into().unwrap()),
            _provenance: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct Ptr<'prov, T> {
    pub inner: *mut T,

    _provenance: PhantomData<fn(&'prov ()) -> &'prov ()>,
}

impl<T> Clone for Ptr<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Ptr<'_, T> {}

impl<'prov, T> dma::PointerMut<'prov> for Ptr<'prov, T> {
    type Output = T;

    unsafe fn read(self) -> Self::Output {
        unsafe { self.inner.read() }
    }

    unsafe fn write(self, val: Self::Output) {
        unsafe { self.inner.write(val) }
    }

    unsafe fn copy_from_nonoverlapping(self, src: *const Self::Output, count: usize) {
        unsafe { self.inner.copy_from_nonoverlapping(src, count) }
    }

    unsafe fn copy_to_nonoverlapping(self, dest: *mut T, count: usize) {
        unsafe { self.inner.copy_to_nonoverlapping(dest, count) }
    }

    unsafe fn add(self, count: u64) -> Self {
        Ptr {
            inner: unsafe { self.inner.add(count as _) },
            ..self
        }
    }
}
