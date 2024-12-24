use core::fmt;

use super::Opcode;
use crate::device::layout::CmdQueueReqDescUpdatePGT;
use crate::device::software::emulator::address::DmaAddress;
use crate::device::software::emulator::dma::{Client, PointerMut};
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::command_request::common::{
    CommonHeader, Header, Unknown, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE,
};
use crate::device::software::emulator::queues::complete_queue::CompleteQueue;
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
use crate::device::software::emulator::{DeviceInner, Result};

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

    fn handle(&self, request: &UpdatePageTable, _: &mut ()) -> Result<Self::Output> {
        log::debug!("handle {request:?}");

        let dma_addr = request.dma_addr();
        let mut ptr = self.dma_client.with_dma_addr::<u64>(dma_addr);

        let len = request.dma_read_length() / 8;
        let mut entries = Vec::with_capacity(len.try_into().unwrap());

        for _ in 0..len {
            let dma_addr = unsafe { ptr.read() }.into();

            log::trace!("insert pa: {dma_addr:?} into page table");

            entries.push(dma_addr);

            ptr = unsafe { ptr.add(1) };
        }

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
