use core::fmt;

use super::Opcode;
use crate::device::layout::CmdQueueReqDescUpdatePGT;
use crate::device::software::emulator::device_api::csr::{RegisterOperation, RegistersQueue, RegistersQueueAddress};
use crate::device::software::emulator::device_api::{ControlStatusRegisters, RawDevice};
use crate::device::software::emulator::dma::{Client, PointerMut};
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queue::command_request::common::{
    CommonHeader, Header, Unknown, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE,
};
use crate::device::software::emulator::queue::descriptor::HandleDescriptor;
use crate::device::software::emulator::{Emulator, Result};

#[repr(C, align(32))]
pub struct UpdatePageTable(CmdQueueReqDescUpdatePGT<[u8; DESCRIPTOR_SIZE]>);
const _: () = assert!(size_of::<UpdatePageTable>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<UpdatePageTable>() == DESCRIPTOR_ALIGN);

const OPCODE: Opcode = Opcode::UpdatePageTable;

impl<UA: Agent> HandleDescriptor<UpdatePageTable> for Emulator<UA> {
    type Output = ();

    fn handle(&self, request: &UpdatePageTable) -> Result<Self::Output> {
        log::trace!("handle {request:?}");

        let response = CommonHeader::new(OPCODE, true);

        let csrs = self.csrs();
        let cmd_response_csrs = csrs.cmd_response();
        let base_addr = cmd_response_csrs.addr().read();
        let head_reg = cmd_response_csrs.head();
        let head = head_reg.read();

        let addr = base_addr
            .checked_add(u64::from(head) * u64::try_from(DESCRIPTOR_SIZE).unwrap())
            .unwrap()
            .into();
        let ptr = self.dma_client.new_ptr_mut::<CommonHeader>(addr);
        // Safety: src and dst is valid
        unsafe {
            ptr.write(response);
        }

        head_reg.write(head + 1);

        Ok(())
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
