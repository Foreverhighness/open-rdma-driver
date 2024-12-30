use core::fmt;

use super::Opcode;
use crate::dma::Client;
use crate::net::Agent;
use crate::queue_pair::Context;
use crate::queues::command_request::common::{CommonHeader, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE, Header, Unknown};
use crate::queues::complete_queue::CompleteQueue;
use crate::queues::descriptor::HandleDescriptor;
use crate::queues::errors::ParseDescriptorError;
use crate::third_party::queues::command_request::descriptor::CmdQueueReqDescQpManagementSeg0;
use crate::types::{MemoryAccessFlag, PathMtuKind, ProtectDomainHandler, QueuePairNumber, QueuePairType};
use crate::{DeviceInner, Result};

#[repr(C, align(32))]
pub struct QueuePairManagement(CmdQueueReqDescQpManagementSeg0<[u8; DESCRIPTOR_SIZE]>);
const _: () = assert!(size_of::<QueuePairManagement>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<QueuePairManagement>() == DESCRIPTOR_ALIGN);

impl QueuePairManagement {
    const OPCODE: Opcode = Opcode::QpManagement;
}

impl<UA: Agent, DC: Client> HandleDescriptor<QueuePairManagement> for DeviceInner<UA, DC> {
    type Context = ();
    type Output = ();

    fn handle(&self, request: &QueuePairManagement, _: &mut ()) -> Result<Self::Output> {
        log::debug!("handle {request:?}");

        let qpn = request.queue_pair_number();
        let qp_context = Context::from_req(request)?;

        let success = if request.valid() {
            // create
            let _ = self.queue_pair_table().insert(qp_context);
            true
        } else {
            // delete
            self.queue_pair_table().remove(qpn)
        };

        let response = CommonHeader::new(QueuePairManagement::OPCODE, success, request.header().user_data());
        unsafe { self.command_response_queue().push(response) };

        Ok(())
    }
}

impl Context {
    fn from_req(req: &QueuePairManagement) -> Result<Self> {
        Ok(Self::new(
            req.queue_pair_number(),
            req.peer_queue_pair_number(),
            req.protect_domain_handler(),
            req.queue_pair_type()?,
            req.remote_queue_access_flag(),
            req.path_mtu_kind()?,
        ))
    }
}

impl QueuePairManagement {
    pub fn valid(&self) -> bool {
        self.0.get_is_valid()
    }

    pub fn error(&self) -> bool {
        self.0.get_is_error()
    }

    pub fn queue_pair_number(&self) -> QueuePairNumber {
        self.0.get_qpn().try_into().unwrap()
    }

    pub fn protect_domain_handler(&self) -> ProtectDomainHandler {
        self.0.get_pd_handler().try_into().unwrap()
    }

    pub fn queue_pair_type(&self) -> Result<QueuePairType> {
        let queue_pair_type = u8::try_from(self.0.get_qp_type()).unwrap();
        let queue_pair_type = queue_pair_type
            .try_into()
            .map_err(|_| ParseDescriptorError::InvalidQueuePairType(queue_pair_type))?;

        Ok(queue_pair_type)
    }

    pub fn remote_queue_access_flag(&self) -> MemoryAccessFlag {
        MemoryAccessFlag::from_bits(self.0.get_rq_access_flags().try_into().unwrap()).unwrap()
    }

    pub fn path_mtu_kind(&self) -> Result<PathMtuKind> {
        let path_mtu_kind = u8::try_from(self.0.get_pmtu()).unwrap();
        let path_mtu_kind = path_mtu_kind
            .try_into()
            .map_err(|_| ParseDescriptorError::InvalidPathMTUKind(path_mtu_kind))?;

        Ok(path_mtu_kind)
    }

    pub fn peer_queue_pair_number(&self) -> QueuePairNumber {
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
            .field("queue_pair_type", &self.queue_pair_type().map_err(|_| fmt::Error))
            .field("remote_queue_access_flag", &self.remote_queue_access_flag())
            .field("path_mtu_kind", &self.path_mtu_kind().map_err(|_| fmt::Error))
            .field("peer_queue_pair_number", &self.peer_queue_pair_number())
            .finish()
    }
}

impl AsRef<Unknown> for QueuePairManagement {
    fn as_ref(&self) -> &Unknown {
        // SAFETY: const sound because we transmute two types with the same layout
        unsafe { core::mem::transmute(self) }
    }
}

impl AsRef<QueuePairManagement> for Unknown {
    fn as_ref(&self) -> &QueuePairManagement {
        assert_eq!(self.header().opcode().unwrap(), QueuePairManagement::OPCODE);

        // SAFETY: const sound because we transmute two types with the same layout
        unsafe { core::mem::transmute(self) }
    }
}
