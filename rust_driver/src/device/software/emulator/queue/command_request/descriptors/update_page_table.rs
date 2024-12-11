use core::fmt;

use crate::device::layout::CmdQueueReqDescUpdatePGT;
use crate::device::software::emulator::device_api::RawDevice;
use crate::device::software::emulator::queue::command_request::common::{Unknown, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE};
use crate::device::software::emulator::Result;

#[repr(C, align(32))]
pub struct UpdatePageTable(CmdQueueReqDescUpdatePGT<[u8; DESCRIPTOR_SIZE]>);
const _: () = assert!(size_of::<UpdatePageTable>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<UpdatePageTable>() == DESCRIPTOR_ALIGN);

impl UpdatePageTable {
    // better naming?
    fn execute<Dev: RawDevice>(&self, dev: &Dev) -> Result<()> {
        todo!()
    }
}

impl fmt::Debug for UpdatePageTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl AsRef<Unknown> for UpdatePageTable {
    fn as_ref(&self) -> &Unknown {
        unsafe { core::mem::transmute(self) }
    }
}

impl AsRef<UpdatePageTable> for Unknown {
    fn as_ref(&self) -> &UpdatePageTable {
        unsafe { core::mem::transmute(self) }
    }
}
