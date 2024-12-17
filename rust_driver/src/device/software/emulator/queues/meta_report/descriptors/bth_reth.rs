//! Base Transport Header and RDMA Extended Transport Header

use super::common::{
    BaseTransportHeader, MessageSequenceNumberAndCanAutoAck, PsnAndReqStatus, RdmaExtendedTransportHeader,
};
use super::{DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE};
use crate::device::software::emulator::types::{MessageSequenceNumber, PacketSequenceNumber};

#[repr(C, align(32))]
pub struct BthReth {
    pub psn_and_req_status: PsnAndReqStatus,
    pub bth: BaseTransportHeader,
    pub reth: RdmaExtendedTransportHeader,
    pub msn_and_can_auto_ack: MessageSequenceNumberAndCanAutoAck,
}
type Descriptor = BthReth;
const _: () = assert!(size_of::<Descriptor>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Descriptor>() == DESCRIPTOR_ALIGN);

impl BthReth {
    pub const fn new(
        expect_psn: PacketSequenceNumber,
        req_status: u8,
        bth: BaseTransportHeader,
        reth: RdmaExtendedTransportHeader,
        msn: MessageSequenceNumber,
        can_auto_ack: bool,
    ) -> Self {
        Self {
            psn_and_req_status: PsnAndReqStatus::new()
                .with_expected_psn(expect_psn)
                .with_req_status(req_status),
            bth,
            reth,
            msn_and_can_auto_ack: MessageSequenceNumberAndCanAutoAck::new()
                .with_message_sequence_number(msn as u32)
                .with_can_auto_ack(can_auto_ack),
        }
    }
}
