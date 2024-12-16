use super::super::Result;

/// Descriptor marker trait, not used for simplicity
pub(super) trait Descriptor {}

/// Can handle descriptor
pub(super) trait HandleDescriptor<Desc> {
    // Seems like Output is always `()`, may remove it in future
    type Output;
    type Context;

    fn handle(&self, request: &Desc, cx: &mut Self::Context) -> Result<Self::Output>;
}
