use core::fmt;

use crate::device::layout::CmdQueueReqDescUpdateMrTable;
use crate::device::software::emulator::queue::command_request::common::{Unknown, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE};

#[repr(C, align(32))]
pub struct UpdateMemoryRegionTable(CmdQueueReqDescUpdateMrTable<[u8; DESCRIPTOR_SIZE]>);
const _: () = assert!(size_of::<UpdateMemoryRegionTable>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<UpdateMemoryRegionTable>() == DESCRIPTOR_ALIGN);

impl fmt::Debug for UpdateMemoryRegionTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl AsRef<Unknown> for UpdateMemoryRegionTable {
    fn as_ref(&self) -> &Unknown {
        unsafe { core::mem::transmute(self) }
    }
}

impl AsRef<UpdateMemoryRegionTable> for Unknown {
    fn as_ref(&self) -> &UpdateMemoryRegionTable {
        unsafe { core::mem::transmute(self) }
    }
}
