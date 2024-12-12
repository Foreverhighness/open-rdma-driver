use core::fmt;

use super::Opcode;
use crate::device::layout::CmdQueueReqDescQpManagementSeg0;
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::command_request::common::{
    Header, Unknown, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE,
};
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
use crate::device::software::emulator::{Emulator, Result};

#[repr(C, align(32))]
pub struct QueuePairManagement(CmdQueueReqDescQpManagementSeg0<[u8; DESCRIPTOR_SIZE]>);
const _: () = assert!(size_of::<QueuePairManagement>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<QueuePairManagement>() == DESCRIPTOR_ALIGN);

impl QueuePairManagement {
    const OPCODE: Opcode = Opcode::QpManagement;
}

impl<UA: Agent> HandleDescriptor<QueuePairManagement> for Emulator<UA> {
    type Output = ();

    fn handle(&self, request: &QueuePairManagement) -> Result<Self::Output> {
        todo!()
    }
}

type QueuePairType = crate::types::QpType;
type MemoryAccessFlag = crate::types::MemAccessTypeFlag;
type PacketMtuKind = crate::types::Pmtu;

impl QueuePairManagement {
    pub fn valid(&self) -> bool {
        self.0.get_is_valid().try_into().unwrap()
    }

    pub fn error(&self) -> bool {
        self.0.get_is_error().try_into().unwrap()
    }

    pub fn queue_pair_number(&self) -> u32 {
        self.0.get_qpn().try_into().unwrap()
    }

    pub fn protect_domain_handler(&self) -> u32 {
        self.0.get_pd_handler().try_into().unwrap()
    }

    pub fn queue_pair_type(&self) -> QueuePairType {
        u8::try_from(self.0.get_qp_type()).unwrap().try_into().unwrap()
    }

    pub fn remote_queue_access_flag(&self) -> MemoryAccessFlag {
        MemoryAccessFlag::from_bits(self.0.get_rq_access_flags().try_into().unwrap()).unwrap()
    }

    pub fn packet_mtu_kind(&self) -> PacketMtuKind {
        u8::try_from(self.0.get_pmtu()).unwrap().try_into().unwrap()
    }

    pub fn peer_queue_pair_number(&self) -> u32 {
        self.0.get_peer_qpn().try_into().unwrap()
    }
}

impl fmt::Debug for QueuePairManagement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandRequestQueuePairManagement")
            .field("header", self.header())
            .field("valid", &self.valid())
            .field("error", &self.error())
            .field("queue_pair_number", &self.queue_pair_number())
            .field("protect_domain_handler", &self.protect_domain_handler())
            .field("queue_pair_type", &self.queue_pair_type())
            .field("remote_queue_access_flag", &self.remote_queue_access_flag())
            .field("packet_mtu_kind", &self.packet_mtu_kind())
            .field("peer_queue_pair_number", &self.peer_queue_pair_number())
            .finish()
    }
}

impl AsRef<Unknown> for QueuePairManagement {
    fn as_ref(&self) -> &Unknown {
        unsafe { core::mem::transmute(self) }
    }
}

impl AsRef<QueuePairManagement> for Unknown {
    fn as_ref(&self) -> &QueuePairManagement {
        assert_eq!(self.header().opcode().unwrap(), QueuePairManagement::OPCODE);
        unsafe { core::mem::transmute(self) }
    }
}
