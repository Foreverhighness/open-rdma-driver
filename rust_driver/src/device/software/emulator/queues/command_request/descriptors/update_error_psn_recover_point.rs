use core::fmt;

use super::Opcode;
use crate::device::layout::CmdQueueReqDescUpdateErrRecoverPoint;
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::command_request::common::{
    CommonHeader, Header, Unknown, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE,
};
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
use crate::device::software::emulator::{Emulator, Result};

#[repr(C, align(32))]
pub struct UpdateErrorPacketSequenceNumberRecoverPoint(CmdQueueReqDescUpdateErrRecoverPoint<[u8; DESCRIPTOR_SIZE]>);
const _: () = assert!(size_of::<UpdateErrorPacketSequenceNumberRecoverPoint>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<UpdateErrorPacketSequenceNumberRecoverPoint>() == DESCRIPTOR_ALIGN);

impl UpdateErrorPacketSequenceNumberRecoverPoint {
    const OPCODE: Opcode = Opcode::UpdateErrorPsnRecoverPoint;
}

impl<UA: Agent> HandleDescriptor<UpdateErrorPacketSequenceNumberRecoverPoint> for Emulator<UA> {
    type Output = ();

    fn handle(&self, request: &UpdateErrorPacketSequenceNumberRecoverPoint) -> Result<Self::Output> {
        log::debug!("handle {request:?}");

        let response = CommonHeader::new(
            UpdateErrorPacketSequenceNumberRecoverPoint::OPCODE,
            true,
            request.header().user_data(),
        );
        unsafe { self.command_response_queue().push(response) };

        Ok(())
    }
}

impl UpdateErrorPacketSequenceNumberRecoverPoint {
    pub fn packet_sequence_number(&self) -> u32 {
        self.0.get_psn().try_into().unwrap()
    }

    pub fn queue_pair_number(&self) -> u32 {
        self.0.get_qpn().try_into().unwrap()
    }
}

impl fmt::Debug for UpdateErrorPacketSequenceNumberRecoverPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandRequestUpdateErrorPacketSequenceNumberRecoverPoint")
            .field("header", self.header())
            .field("queue_pair_number", &self.queue_pair_number())
            .field("packet_sequence_number", &self.packet_sequence_number())
            .finish()
    }
}

impl AsRef<Unknown> for UpdateErrorPacketSequenceNumberRecoverPoint {
    fn as_ref(&self) -> &Unknown {
        unsafe { core::mem::transmute(self) }
    }
}

impl AsRef<UpdateErrorPacketSequenceNumberRecoverPoint> for Unknown {
    fn as_ref(&self) -> &UpdateErrorPacketSequenceNumberRecoverPoint {
        assert_eq!(
            self.header().opcode().unwrap(),
            UpdateErrorPacketSequenceNumberRecoverPoint::OPCODE
        );
        unsafe { core::mem::transmute(self) }
    }
}
