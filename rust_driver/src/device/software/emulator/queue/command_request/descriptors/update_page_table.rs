use core::fmt;

use crate::device::layout::CmdQueueReqDescUpdatePGT;
use crate::device::software::emulator::device_api::RawDevice;
use crate::device::software::emulator::Result;

#[repr(transparent)]
pub struct UpdatePageTable(CmdQueueReqDescUpdatePGT<[u8; 32]>);

const _: () = assert!(size_of::<UpdatePageTable>() == 256 / 8);

impl fmt::Debug for UpdatePageTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dma_addr = self.0.get_dma_addr();
        let start_index = self.0.get_start_index();
        let dma_read_len = self.0.get_dma_read_length();
        write!(f, "UpdatePageTable: ")
    }
}

impl UpdatePageTable {
    // better naming?
    fn execute<Dev: RawDevice>(&self, dev: &Dev) -> Result<()> {
        todo!()
    }
}
