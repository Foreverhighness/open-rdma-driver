//! Base Transport Header and ACK Extended Transport Header

use super::common::{AckExtendedTransportHeader, PsnAndReqStatus};
use super::{BaseTransportHeader, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE};

#[derive(Debug)]
#[repr(C, align(32))]
pub(crate) struct BthAeth {
    req_status: PsnAndReqStatus,
    bth: BaseTransportHeader,
    aeth: AckExtendedTransportHeader,
    _reserved: [bool; 12],
}
type Descriptor = BthAeth;
const _: () = assert!(size_of::<Descriptor>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Descriptor>() == DESCRIPTOR_ALIGN);

impl BthAeth {
    // TODO(fh): replace args with reference?
    pub const fn from_ne_bytes(bytes: [u8; DESCRIPTOR_SIZE]) -> Self {
        unsafe { core::mem::transmute(bytes) }
    }
}
