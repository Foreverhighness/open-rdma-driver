//! Base Transport Header and RDMA Extended Transport Header

use crate::device::software::emulator::queues::meta_report::common::{
    BaseTransportHeader, MessageSequenceNumberAndCanAutoAck, PsnAndReqStatus, RdmaExtendedTransportHeader,
    DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE,
};

#[repr(C, align(32))]
struct BthReth {
    psn_and_req_status: PsnAndReqStatus,
    bth: BaseTransportHeader,
    reth: RdmaExtendedTransportHeader,
    men_and_can_auto_ack: MessageSequenceNumberAndCanAutoAck,
}
type Descriptor = BthReth;
const _: () = assert!(size_of::<Descriptor>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Descriptor>() == DESCRIPTOR_ALIGN);
