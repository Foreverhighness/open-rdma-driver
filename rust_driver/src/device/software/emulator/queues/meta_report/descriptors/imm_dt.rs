//! Immediate Extended Transport Header

use crate::device::software::emulator::queues::meta_report::common::{
    ImmediateExtendedTransportHeader, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE,
};

#[repr(C, align(32))]
struct ImmDt {
    imm_dt: ImmediateExtendedTransportHeader,
    _reserved: [bool; 28],
}
type Descriptor = ImmDt;
const _: () = assert!(size_of::<Descriptor>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Descriptor>() == DESCRIPTOR_ALIGN);
