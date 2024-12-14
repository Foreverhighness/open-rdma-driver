//! Base Transport Header and ACK Extended Transport Header

use crate::device::software::emulator::queues::meta_report::common::{
    AckExtendedTransportHeader, BaseTransportHeader, PsnAndReqStatus, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE,
};

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
