use core::fmt;

use crate::device::software::emulator::queues::send::common::{
    ScatterGatherElement, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE,
};

#[repr(C, align(32))]
pub(crate) struct VariableLengthSge {
    sge2: ScatterGatherElement,
    sge1: ScatterGatherElement,
}
type Descriptor = VariableLengthSge;
const _: () = assert!(size_of::<Descriptor>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Descriptor>() == DESCRIPTOR_ALIGN);

impl fmt::Debug for VariableLengthSge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VariableLengthSGE")
            .field("sge1", &self.sge1)
            .field("sge2", &self.sge2)
            .finish()
    }
}

impl Descriptor {
    pub fn from_bytes(raw: [u8; DESCRIPTOR_SIZE]) -> Self {
        let descriptor = unsafe { core::mem::transmute::<_, Self>(raw) };
        assert!((&raw const descriptor).is_aligned());
        descriptor
    }
}
