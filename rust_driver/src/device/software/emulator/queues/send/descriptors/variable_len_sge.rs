use crate::device::software::emulator::queues::send::common::{
    ScatterGatherElement, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE,
};

#[repr(C, align(32))]
pub(crate) struct VariableLengthSGE {
    sge2: ScatterGatherElement,
    sge1: ScatterGatherElement,
}
type Descriptor = VariableLengthSGE;
const _: () = assert!(size_of::<Descriptor>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Descriptor>() == DESCRIPTOR_ALIGN);
