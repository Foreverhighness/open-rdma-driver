use core::fmt;

use crate::device::layout::CmdQueueReqDescSetRawPacketReceiveMeta;
use crate::device::software::emulator::queue::command_request::common::{Unknown, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE};

#[repr(C, align(32))]
pub struct SetRawPacketReceiveMeta(CmdQueueReqDescSetRawPacketReceiveMeta<[u8; DESCRIPTOR_SIZE]>);
const _: () = assert!(size_of::<SetRawPacketReceiveMeta>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<SetRawPacketReceiveMeta>() == DESCRIPTOR_ALIGN);

impl fmt::Debug for SetRawPacketReceiveMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl AsRef<Unknown> for SetRawPacketReceiveMeta {
    fn as_ref(&self) -> &Unknown {
        unsafe { core::mem::transmute(self) }
    }
}

impl AsRef<SetRawPacketReceiveMeta> for Unknown {
    fn as_ref(&self) -> &SetRawPacketReceiveMeta {
        unsafe { core::mem::transmute(self) }
    }
}
