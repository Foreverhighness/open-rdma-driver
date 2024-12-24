use core::fmt;

use super::Opcode;
use crate::device::layout::CmdQueueReqDescUpdateMrTable;
use crate::device::software::emulator::address::VirtualAddress;
use crate::device::software::emulator::dma::Client;
use crate::device::software::emulator::memory_region::Context;
use crate::device::software::emulator::mr_table::MemoryRegionTable;
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::command_request::common::{
    CommonHeader, Header, Unknown, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE,
};
use crate::device::software::emulator::queues::complete_queue::CompleteQueue;
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
use crate::device::software::emulator::types::{MemoryAccessFlag, MemoryRegionKey};
use crate::device::software::emulator::{DeviceInner, Result};

#[repr(C, align(32))]
pub struct UpdateMemoryRegionTable(CmdQueueReqDescUpdateMrTable<[u8; DESCRIPTOR_SIZE]>);
const _: () = assert!(size_of::<UpdateMemoryRegionTable>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<UpdateMemoryRegionTable>() == DESCRIPTOR_ALIGN);

impl UpdateMemoryRegionTable {
    const OPCODE: Opcode = Opcode::UpdateMrTable;
}

impl<UA: Agent, DC: Client> HandleDescriptor<UpdateMemoryRegionTable> for DeviceInner<UA, DC> {
    type Context = ();
    type Output = ();

    fn handle(&self, request: &UpdateMemoryRegionTable, _: &mut ()) -> Result<Self::Output> {
        log::debug!("handle {request:?}");

        let mr_context = Context::from_req(request);
        self.memory_region_table().update(mr_context)?;

        let response = CommonHeader::new(UpdateMemoryRegionTable::OPCODE, true, request.header().user_data());
        unsafe { self.command_response_queue().push(response) };

        Ok(())
    }
}

impl Context {
    pub(crate) fn from_req(req: &UpdateMemoryRegionTable) -> Self {
        Self::new(
            req.mr_base_va(),
            req.mr_len(),
            req.mr_key(),
            req.pd_handler(),
            req.access_flag(),
            req.page_table_offset(),
        )
    }
}

impl UpdateMemoryRegionTable {
    pub fn mr_base_va(&self) -> VirtualAddress {
        self.0.get_mr_base_va().into()
    }

    pub fn mr_len(&self) -> u32 {
        self.0.get_mr_length().try_into().unwrap()
    }

    pub fn mr_key(&self) -> MemoryRegionKey {
        MemoryRegionKey::new(self.0.get_mr_key().try_into().unwrap())
    }

    pub fn pd_handler(&self) -> u32 {
        self.0.get_pd_handler().try_into().unwrap()
    }

    pub fn access_flag(&self) -> MemoryAccessFlag {
        MemoryAccessFlag::from_bits(self.0.get_acc_flags().try_into().unwrap()).unwrap()
    }

    pub fn page_table_offset(&self) -> u32 {
        self.0.get_pgt_offset().try_into().unwrap()
    }
}

impl fmt::Debug for UpdateMemoryRegionTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandRequestUpdateMemoryRegionTable")
            .field("header", self.header())
            .field("mr_base_va", &self.mr_base_va())
            .field("mr_len", &format_args!("{:#08X}", self.mr_len()))
            .field("mr_key", &self.mr_key())
            .field("pd_handler", &self.pd_handler())
            .field("access_flag", &self.access_flag())
            .field("page_table_offset", &self.page_table_offset())
            .finish()
    }
}

impl AsRef<Unknown> for UpdateMemoryRegionTable {
    fn as_ref(&self) -> &Unknown {
        // SAFETY: const sound because we transmute two types with the same layout
        unsafe { core::mem::transmute(self) }
    }
}

impl AsRef<UpdateMemoryRegionTable> for Unknown {
    fn as_ref(&self) -> &UpdateMemoryRegionTable {
        assert_eq!(self.header().opcode().unwrap(), UpdateMemoryRegionTable::OPCODE);

        // SAFETY: const sound because we transmute two types with the same layout
        unsafe { core::mem::transmute(self) }
    }
}
