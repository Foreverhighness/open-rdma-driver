use core::fmt;

use crate::device::layout::CmdQueueReqDescUpdateErrRecoverPoint;
use crate::device::software::emulator::queues::command_request::common::{Unknown, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE};

#[repr(C, align(32))]
pub struct UpdateErrorPacketSequenceNumberRecoverPoint(CmdQueueReqDescUpdateErrRecoverPoint<[u8; DESCRIPTOR_SIZE]>);
const _: () = assert!(size_of::<UpdateErrorPacketSequenceNumberRecoverPoint>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<UpdateErrorPacketSequenceNumberRecoverPoint>() == DESCRIPTOR_ALIGN);

impl fmt::Debug for UpdateErrorPacketSequenceNumberRecoverPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl AsRef<Unknown> for UpdateErrorPacketSequenceNumberRecoverPoint {
    fn as_ref(&self) -> &Unknown {
        unsafe { core::mem::transmute(self) }
    }
}

impl AsRef<UpdateErrorPacketSequenceNumberRecoverPoint> for Unknown {
    fn as_ref(&self) -> &UpdateErrorPacketSequenceNumberRecoverPoint {
        unsafe { core::mem::transmute(self) }
    }
}
