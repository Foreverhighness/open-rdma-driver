use crate::device::software::emulator::dma::PointerMut;

pub(crate) trait CompleteQueue {
    type Descriptor;

    fn addr(&self) -> u64;
    fn head(&self) -> u32;
    fn tail(&self) -> u32;

    /// use [`core::ops::Index`] instead?
    /// return ptr here, because access pointer is considered unsafe
    fn index<T>(&self, index: u32) -> impl PointerMut<Output = T>;
    fn advance(&self);

    // SAFETY: caller should grantee queue is initialized
    unsafe fn push<T>(&self, val: T) {
        const { assert!(size_of::<T>() <= size_of::<Self::Descriptor>()) };

        let head = self.head();
        let tail = self.tail();
        assert!(tail <= head, "assertion failed: {tail} <= {head}");

        let ptr = self.index(head);
        // SAFETY: caller uphold
        unsafe { ptr.write(val) };

        // push item
        self.advance();
    }
}
