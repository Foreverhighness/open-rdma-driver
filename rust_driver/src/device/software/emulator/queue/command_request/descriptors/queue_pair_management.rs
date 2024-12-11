use core::fmt;

use crate::device::layout::CmdQueueReqDescQpManagementSeg0;
use crate::device::software::emulator::queue::command_request::common::{Unknown, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE};

#[repr(C, align(32))]
pub struct QueuePairManagement(CmdQueueReqDescQpManagementSeg0<[u8; DESCRIPTOR_SIZE]>);
const _: () = assert!(size_of::<QueuePairManagement>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<QueuePairManagement>() == DESCRIPTOR_ALIGN);

impl fmt::Debug for QueuePairManagement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl AsRef<Unknown> for QueuePairManagement {
    fn as_ref(&self) -> &Unknown {
        unsafe { core::mem::transmute(self) }
    }
}

impl AsRef<QueuePairManagement> for Unknown {
    fn as_ref(&self) -> &QueuePairManagement {
        unsafe { core::mem::transmute(self) }
    }
}
