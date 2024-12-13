use super::super::Result;
use crate::device::software::emulator::device_api::RawDevice;

/// Descriptor marker trait, not used for simplicity
pub(super) trait Descriptor {}

/// Can handle descriptor
pub(super) trait HandleDescriptor<Desc>: RawDevice {
    // Seems like Output is always `()`, may remove it in future
    type Output;

    fn handle(&self, request: &Desc) -> Result<Self::Output>;
}
