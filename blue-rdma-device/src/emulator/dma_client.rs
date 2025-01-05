use crate::address::DmaAddress;
use crate::dma;

#[derive(Debug)]
pub struct DmaClient;

impl dma::Client for DmaClient {
    fn with_dma_addr<T>(&self, addr: DmaAddress) -> impl dma::PointerMut<Output = T> {
        Ptr(addr.0 as usize as *mut T)
    }
}

#[derive(Debug)]
pub struct Ptr<T>(pub *mut T);

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Ptr<T> {}

impl<T> dma::PointerMut for Ptr<T> {
    type Output = T;

    unsafe fn read(self) -> Self::Output {
        unsafe { self.0.read() }
    }

    unsafe fn write(self, val: Self::Output) {
        unsafe { self.0.write(val) }
    }

    unsafe fn copy_from_nonoverlapping(self, src: *const Self::Output, count: usize) {
        unsafe { self.0.copy_from_nonoverlapping(src, count) }
    }

    unsafe fn copy_to_nonoverlapping(self, dest: *mut T, count: usize) {
        unsafe { self.0.copy_to_nonoverlapping(dest, count) }
    }

    unsafe fn add(self, count: u64) -> Self {
        Self(unsafe { self.0.add(count as _) })
    }
}
