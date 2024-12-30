//! Direct Memory Access, I only concern DMA in emulator

use super::address::DmaAddress;

pub trait Client {
    // type Ref;
    // type RefMut
    // type Ptr;
    // Currently use only one ptr type for simplicity

    fn with_dma_addr<T>(&self, addr: DmaAddress) -> impl PointerMut<Output = T>;
}

pub trait PointerMut: Clone + Copy {
    type Output;

    /// Reads the value from `self` without moving it. This leaves the
    /// memory in `self` unchanged.
    ///
    /// See [`core::ptr::read`] for safety concerns and examples.
    unsafe fn read(self) -> Self::Output;

    /// Overwrites a memory location with the given value without reading or
    /// dropping the old value.
    ///
    /// See [`core::ptr::write`] for safety concerns and examples.
    unsafe fn write(self, val: Self::Output);

    /// Copies `count * size_of<T>` bytes from `src` to `self`. The source
    /// and destination may *not* overlap.
    ///
    /// NOTE: this has the *opposite* argument order of [`core::ptr::copy_nonoverlapping`].
    ///
    /// See [`core::ptr::copy_nonoverlapping`] for safety concerns and examples.
    unsafe fn copy_from_nonoverlapping(self, src: *const Self::Output, count: usize);

    /// Copies `count * size_of<T>` bytes from `self` to `dest`. The source
    /// and destination may *not* overlap.
    ///
    /// NOTE: this has the *same* argument order as [`core::ptr::copy_nonoverlapping`].
    ///
    /// See [`core::ptr::copy_nonoverlapping`] for safety concerns and examples.
    unsafe fn copy_to_nonoverlapping(self, dest: *mut Self::Output, count: usize);

    unsafe fn add(self, count: u64) -> Self;

    // unsafe fn write_bytes(self, data: &[u8]);

    // unsafe fn read_bytes(self, len: usize) -> Vec<u8>;
}

#[expect(unused, reason = "for zero copy usage, may use later")]
pub trait PointerMutExt: PointerMut {
    unsafe fn with<F, T>(self, f: F) -> T
    where
        F: FnOnce(&Self::Output) -> T;

    unsafe fn with_mut<F, T>(self, f: F) -> T
    where
        F: FnOnce(&mut Self::Output) -> T;
}
