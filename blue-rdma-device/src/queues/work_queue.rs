use crate::dma::PointerMut;

pub trait WorkQueue {
    type Descriptor;

    fn addr(&self) -> u64;
    fn head(&self) -> u32;
    fn tail(&self) -> u32;

    /// use [`core::ops::Index`] instead?
    /// return ptr here, because access pointer is considered unsafe
    fn index(&self, index: u32) -> impl PointerMut<Output = Self::Descriptor>;
    fn advance(&self);

    // SAFETY: caller should grantee queue is initialized
    unsafe fn pop(&self) -> Option<Self::Descriptor> {
        let head = self.head();
        let tail = self.tail();
        if head == tail {
            return None;
        }
        assert!(tail < head, "assertion failed: {tail} < {head}");

        let ptr = self.index(tail);
        // SAFETY: caller uphold
        let raw = unsafe { ptr.read() };

        // pop item
        self.advance();

        Some(raw)
    }
}
