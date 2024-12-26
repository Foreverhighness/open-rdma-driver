//! Base Transport Header

use super::common::{BaseTransportHeader, MessageSequenceNumberAndCanAutoAck, PsnAndReqStatus};
use super::{DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE};

#[repr(C, align(32))]
struct Bth {
    psn_and_req_status: PsnAndReqStatus,
    bth: BaseTransportHeader,
    msn: MessageSequenceNumberAndCanAutoAck,
    _reserved: core::mem::MaybeUninit<[u8; 12]>,
    can_auto_ack: MessageSequenceNumberAndCanAutoAck,
}
type Descriptor = Bth;
const _: () = assert!(size_of::<Descriptor>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Descriptor>() == DESCRIPTOR_ALIGN);
