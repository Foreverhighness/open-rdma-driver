//! Immediate Extended Transport Header

use super::common::ImmediateExtendedTransportHeader;
use super::{DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE};

#[repr(C, align(32))]
struct ImmDt {
    imm_dt: ImmediateExtendedTransportHeader,
    _reserved: core::mem::MaybeUninit<[u8; 28]>,
}
type Descriptor = ImmDt;
const _: () = assert!(size_of::<Descriptor>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Descriptor>() == DESCRIPTOR_ALIGN);
