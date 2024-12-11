use core::fmt;

use crate::device::layout::CmdQueueReqDescUpdatePGT;
use crate::device::software::emulator::device_api::RawDevice;
use crate::device::software::emulator::queue::command_request::common::{
    Header, Unknown, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE,
};
use crate::device::software::emulator::queue::request::Request;
use crate::device::software::emulator::Result;

#[repr(C, align(32))]
pub struct UpdatePageTable(CmdQueueReqDescUpdatePGT<[u8; DESCRIPTOR_SIZE]>);
const _: () = assert!(size_of::<UpdatePageTable>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<UpdatePageTable>() == DESCRIPTOR_ALIGN);

impl Request for UpdatePageTable {
    type Response = ();

    fn handle<Dev: RawDevice>(&self, dev: &Dev) -> Result<Self::Response> {
        todo!()
    }
}

impl UpdatePageTable {
    pub fn dma_addr(&self) -> u64 {
        self.0.get_dma_addr()
    }

    pub fn start_index(&self) -> u32 {
        self.0.get_start_index().try_into().unwrap()
    }

    pub fn dma_read_length(&self) -> u32 {
        self.0.get_dma_read_length().try_into().unwrap()
    }
}

impl fmt::Debug for UpdatePageTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandRequestUpdatePageTable")
            .field("header", self.header())
            .field("dma_addr", &format_args!("{:#018X}", self.dma_addr()))
            .field("start_index", &self.start_index())
            .field("dma_read_len", &self.dma_read_length())
            .finish()
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
