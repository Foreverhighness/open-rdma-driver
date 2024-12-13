use core::fmt;
use core::net::Ipv4Addr;

use eui48::MacAddress;

use super::Opcode;
use crate::device::layout::CmdQueueReqDescSetNetworkParam;
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::command_request::common::{
    CommonHeader, Header, Unknown, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE,
};
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
use crate::device::software::emulator::{Emulator, Result};

#[repr(C, align(32))]
pub struct SetNetworkParameter(CmdQueueReqDescSetNetworkParam<[u8; DESCRIPTOR_SIZE]>);
const _: () = assert!(size_of::<SetNetworkParameter>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<SetNetworkParameter>() == DESCRIPTOR_ALIGN);

impl SetNetworkParameter {
    const OPCODE: Opcode = Opcode::SetNetworkParam;
}

impl<UA: Agent> HandleDescriptor<SetNetworkParameter> for Emulator<UA> {
    type Output = ();

    fn handle(&self, request: &SetNetworkParameter) -> Result<Self::Output> {
        log::debug!("handle {request:?}");

        let response = CommonHeader::new(SetNetworkParameter::OPCODE, true, request.header().user_data());
        unsafe { self.command_response_queue().push(response) };

        Ok(())
    }
}

impl SetNetworkParameter {
    pub fn gateway(&self) -> Ipv4Addr {
        Ipv4Addr::from_bits(self.0.get_ip_gateway().try_into().unwrap())
    }

    pub fn subnet_mask(&self) -> Ipv4Addr {
        Ipv4Addr::from_bits(self.0.get_ip_netmask().try_into().unwrap())
    }

    pub fn ip(&self) -> Ipv4Addr {
        Ipv4Addr::from_bits(self.0.get_ip_addr().try_into().unwrap())
    }

    pub fn mac(&self) -> MacAddress {
        MacAddress::new(self.0.get_eth_mac_addr().to_ne_bytes()[..6].try_into().unwrap())
    }
}

impl fmt::Debug for SetNetworkParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandRequestSetNetworkParameter")
            .field("header", self.header())
            .field("gateway", &self.gateway())
            .field("subnet_mask", &self.subnet_mask())
            .field("ip_addr", &self.ip())
            .field("mac", &self.mac())
            .finish()
    }
}

impl AsRef<Unknown> for SetNetworkParameter {
    fn as_ref(&self) -> &Unknown {
        unsafe { core::mem::transmute(self) }
    }
}

impl AsRef<SetNetworkParameter> for Unknown {
    fn as_ref(&self) -> &SetNetworkParameter {
        assert_eq!(self.header().opcode().unwrap(), SetNetworkParameter::OPCODE);
        unsafe { core::mem::transmute(self) }
    }
}
