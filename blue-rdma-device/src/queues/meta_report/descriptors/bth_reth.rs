//! Base Transport Header and RDMA Extended Transport Header

use super::common::{
    BaseTransportHeader, MessageSequenceNumberAndCanAutoAck, PsnAndReqStatus, RdmaExtendedTransportHeader,
};
use super::{DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE};
use crate::types::{MessageSequenceNumber, PacketSequenceNumber};

#[derive(Debug)]
#[repr(C, align(32))]
pub struct BthReth {
    psn_and_req_status: PsnAndReqStatus,
    bth: BaseTransportHeader,
    reth: RdmaExtendedTransportHeader,
    msn_and_can_auto_ack: MessageSequenceNumberAndCanAutoAck,
}

#[expect(unused, reason = "for consistency")]
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
        let psn_and_req_status = PsnAndReqStatus::new()
            .with_expected_psn(expect_psn)
            .with_req_status(req_status);

        let msn_and_can_auto_ack = MessageSequenceNumberAndCanAutoAck::new()
            .with_message_sequence_number(msn as u32)
            .with_can_auto_ack(can_auto_ack);

        Self {
            psn_and_req_status,
            bth,
            reth,
            msn_and_can_auto_ack,
        }
    }

    #[expect(unused, reason = "may use later")]
    pub const fn from_ne_bytes(bytes: [u8; DESCRIPTOR_SIZE]) -> Self {
        unsafe { core::mem::transmute(bytes) }
    }
}

// impl fmt::Debug for BthReth {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("MetaReportBthReth")
//             .field("psn", &self.psn_and_req_status)
//             .field("req_status", &self.psn_and_req_status)
//             .field("bth", &self.bth)
//             .field("reth", &self.reth)
//             .field("msn", &self.msn_and_can_auto_ack)
//             .field("can_auto_ack", &self.msn_and_can_auto_ack)
//             .finish()
//     }
// }
