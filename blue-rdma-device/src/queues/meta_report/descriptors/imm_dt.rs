//! Immediate Extended Transport Header

use super::common::ImmediateExtendedTransportHeader;
use super::{DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE};

#[derive(Debug)]
#[repr(C, align(32))]
pub struct ImmDt {
    imm_dt: ImmediateExtendedTransportHeader,
    _reserved: core::mem::MaybeUninit<[u8; 28]>,
}

#[expect(unused, reason = "for consistency")]
type Descriptor = ImmDt;
const _: () = assert!(size_of::<Descriptor>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Descriptor>() == DESCRIPTOR_ALIGN);

impl ImmDt {
    pub fn new(data: u32) -> Self {
        Self {
            imm_dt: ImmediateExtendedTransportHeader::new(data),
            _reserved: core::mem::MaybeUninit::uninit(),
        }
    }
}
