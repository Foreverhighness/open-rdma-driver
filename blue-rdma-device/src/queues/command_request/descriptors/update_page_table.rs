use core::fmt;

use super::Opcode;
use crate::address::DmaAddress;
use crate::dma::{Client, PointerMut};
use crate::net::Agent;
use crate::queues::command_request::common::{CommonHeader, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE, Header, Unknown};
use crate::queues::complete_queue::CompleteQueue;
use crate::queues::descriptor::HandleDescriptor;
use crate::third_party::queues::command_request::descriptor::CmdQueueReqDescUpdatePGT;
use crate::{DeviceInner, Result};

#[repr(C, align(32))]
pub struct UpdatePageTable(CmdQueueReqDescUpdatePGT<[u8; DESCRIPTOR_SIZE]>);
const _: () = assert!(size_of::<UpdatePageTable>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<UpdatePageTable>() == DESCRIPTOR_ALIGN);

impl UpdatePageTable {
    const OPCODE: Opcode = Opcode::UpdatePageTable;
}

impl<UA: Agent, DC: Client> HandleDescriptor<UpdatePageTable> for DeviceInner<UA, DC> {
    type Context = ();
    type Output = ();

    fn handle(&self, request: &UpdatePageTable, (): &mut ()) -> Result<Self::Output> {
        log::debug!("handle {request:?}");

        let dma_addr = request.dma_addr();
        let ptr = self.dma_client.with_dma_addr(dma_addr);

        let len = usize::try_from(request.dma_read_length() / 8).unwrap();
        let mut entries = Vec::with_capacity(len);

        let entries_uninit = entries.spare_capacity_mut();

        unsafe { ptr.copy_to_nonoverlapping(entries_uninit.as_mut_ptr(), len) };
        // SAFETY: entries are init
        unsafe { entries.set_len(len) };

        let page_table = self.page_table.pin();
        let offset = request.start_index();
        assert!(page_table.insert(offset, entries).is_none());

        let response = CommonHeader::new(UpdatePageTable::OPCODE, true, request.header().user_data());
        unsafe { self.command_response_queue().push(response) };

        Ok(())
    }
}

impl UpdatePageTable {
    pub fn dma_addr(&self) -> DmaAddress {
        self.0.get_dma_addr().into()
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
            .field("dma_addr", &self.dma_addr())
            .field("start_index", &self.start_index())
            .field("dma_read_len", &self.dma_read_length())
            .finish()
    }
}

impl AsRef<Unknown> for UpdatePageTable {
    fn as_ref(&self) -> &Unknown {
        // SAFETY: const sound because we transmute two types with the same layout
        unsafe { core::mem::transmute(self) }
    }
}

impl AsRef<UpdatePageTable> for Unknown {
    fn as_ref(&self) -> &UpdatePageTable {
        assert_eq!(self.header().opcode().unwrap(), UpdatePageTable::OPCODE);

        // SAFETY: const sound because we transmute two types with the same layout
        unsafe { core::mem::transmute(self) }
    }
}
