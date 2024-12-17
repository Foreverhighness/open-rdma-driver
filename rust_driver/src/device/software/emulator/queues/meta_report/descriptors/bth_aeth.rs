//! Base Transport Header and ACK Extended Transport Header

use super::common::{AckExtendedTransportHeader, PsnAndReqStatus};
use super::{BaseTransportHeader, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE};

#[repr(C, align(32))]
struct BthAeth {
    req_status: PsnAndReqStatus,
    bth: BaseTransportHeader,
    aeth: AckExtendedTransportHeader,
    _reserved: [bool; 12],
}
type Descriptor = BthAeth;
const _: () = assert!(size_of::<Descriptor>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Descriptor>() == DESCRIPTOR_ALIGN);
