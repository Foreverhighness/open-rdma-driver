use core::fmt;

use super::Opcode;
use crate::device::layout::CmdQueueReqDescUpdateMrTable;
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::command_request::common::{
    Header, Unknown, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE,
};
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
use crate::device::software::emulator::{Emulator, Result};

#[repr(C, align(32))]
pub struct UpdateMemoryRegionTable(CmdQueueReqDescUpdateMrTable<[u8; DESCRIPTOR_SIZE]>);
const _: () = assert!(size_of::<UpdateMemoryRegionTable>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<UpdateMemoryRegionTable>() == DESCRIPTOR_ALIGN);

const OPCODE: Opcode = Opcode::UpdateMrTable;

impl<UA: Agent> HandleDescriptor<UpdateMemoryRegionTable> for Emulator<UA> {
    type Output = ();

    fn handle(&self, request: &UpdateMemoryRegionTable) -> Result<Self::Output> {
        todo!()
    }
}

impl UpdateMemoryRegionTable {
    pub fn mr_base_va(&self) -> u64 {
        self.0.get_mr_base_va()
    }

    pub fn mr_len(&self) -> u32 {
        self.0.get_mr_length().try_into().unwrap()
    }

    pub fn mr_key(&self) -> u32 {
        self.0.get_mr_key().try_into().unwrap()
    }

    pub fn pd_handler(&self) -> u32 {
        self.0.get_pd_handler().try_into().unwrap()
    }

    pub fn access_flag(&self) -> u8 {
        self.0.get_acc_flags().try_into().unwrap()
    }

    pub fn page_table_offset(&self) -> u32 {
        self.0.get_pgt_offset().try_into().unwrap()
    }
}

impl fmt::Debug for UpdateMemoryRegionTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandRequestUpdateMemoryRegionTable")
            .field("header", self.header())
            .field("mr_base_va", &format_args!("{:#018X}", self.mr_base_va()))
            .field("mr_len", &self.mr_len())
            .field("mr_key", &self.mr_key())
            .field("pd_handler", &self.pd_handler())
            .field("access_flag", &format_args!("{:#010b}", self.access_flag()))
            .field("page_table_offset", &self.page_table_offset())
            .finish()
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
