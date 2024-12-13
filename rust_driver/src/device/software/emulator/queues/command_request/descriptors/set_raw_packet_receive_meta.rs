use core::fmt;

use super::Opcode;
use crate::device::layout::CmdQueueReqDescSetRawPacketReceiveMeta;
use crate::device::software::emulator::address::DmaAddress;
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::command_request::common::{
    CommonHeader, Header, Unknown, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE,
};
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
use crate::device::software::emulator::types::MemoryRegionKey;
use crate::device::software::emulator::{Emulator, Result};

#[repr(C, align(32))]
pub struct SetRawPacketReceiveMeta(CmdQueueReqDescSetRawPacketReceiveMeta<[u8; DESCRIPTOR_SIZE]>);
const _: () = assert!(size_of::<SetRawPacketReceiveMeta>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<SetRawPacketReceiveMeta>() == DESCRIPTOR_ALIGN);

impl SetRawPacketReceiveMeta {
    const OPCODE: Opcode = Opcode::SetRawPacketReceiveMeta;
}

impl<UA: Agent> HandleDescriptor<SetRawPacketReceiveMeta> for Emulator<UA> {
    type Output = ();

    #[expect(unreachable_code, reason = "testing")]
    fn handle(&self, request: &SetRawPacketReceiveMeta) -> Result<Self::Output> {
        log::debug!("handle {request:?}");

        todo!();
        let response = CommonHeader::new(SetRawPacketReceiveMeta::OPCODE, true, request.header().user_data());
        unsafe { self.command_response_queue().push(response) };

        Ok(())
    }
}

impl SetRawPacketReceiveMeta {
    pub fn write_base_addr(&self) -> DmaAddress {
        self.0.get_write_base_addr().into()
    }

    pub fn write_mr_key(&self) -> MemoryRegionKey {
        MemoryRegionKey::new(self.0.get_write_mr_key().try_into().unwrap())
    }
}

impl fmt::Debug for SetRawPacketReceiveMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandRequestSetRawPacketReceiveMeta")
            .field("header", self.header())
            .field("write_base_addr", &self.write_base_addr())
            .field("write_mr_key", &self.write_mr_key())
            .finish()
    }
}

impl AsRef<Unknown> for SetRawPacketReceiveMeta {
    fn as_ref(&self) -> &Unknown {
        // SAFETY: const sound because we transmute two types with the same layout
        unsafe { core::mem::transmute(self) }
    }
}

impl AsRef<SetRawPacketReceiveMeta> for Unknown {
    fn as_ref(&self) -> &SetRawPacketReceiveMeta {
        assert_eq!(self.header().opcode().unwrap(), SetRawPacketReceiveMeta::OPCODE);

        // SAFETY: const sound because we transmute two types with the same layout
        unsafe { core::mem::transmute(self) }
    }
}
