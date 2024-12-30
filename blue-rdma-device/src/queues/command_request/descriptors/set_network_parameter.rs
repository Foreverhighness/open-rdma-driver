use core::fmt;
use core::net::Ipv4Addr;

use eui48::MacAddress;

use super::Opcode;
use crate::device_inner::NetParameter;
use crate::dma::Client;
use crate::net::Agent;
use crate::queues::command_request::common::{CommonHeader, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE, Header, Unknown};
use crate::queues::complete_queue::CompleteQueue;
use crate::queues::descriptor::HandleDescriptor;
use crate::third_party::queues::command_request::descriptor::CmdQueueReqDescSetNetworkParam;
use crate::{DeviceInner, Result};

#[repr(C, align(32))]
pub struct SetNetworkParameter(CmdQueueReqDescSetNetworkParam<[u8; DESCRIPTOR_SIZE]>);
const _: () = assert!(size_of::<SetNetworkParameter>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<SetNetworkParameter>() == DESCRIPTOR_ALIGN);

impl SetNetworkParameter {
    const OPCODE: Opcode = Opcode::SetNetworkParam;
}

impl<UA: Agent, DC: Client> HandleDescriptor<SetNetworkParameter> for DeviceInner<UA, DC> {
    type Context = ();
    type Output = ();

    fn handle(&self, request: &SetNetworkParameter, _: &mut ()) -> Result<Self::Output> {
        log::debug!("handle {request:?}");

        let net_parameter = NetParameter::new(request.ip(), request.gateway(), request.subnet_mask(), request.mac());
        self.net_parameter
            .get()
            .unwrap()
            .send(net_parameter)
            .expect("network not started?");

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
        MacAddress::new(self.0.get_eth_mac_addr().to_be_bytes()[2..8].try_into().unwrap())
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
        // SAFETY: const sound because we transmute two types with the same layout
        unsafe { core::mem::transmute(self) }
    }
}

impl AsRef<SetNetworkParameter> for Unknown {
    fn as_ref(&self) -> &SetNetworkParameter {
        assert_eq!(self.header().opcode().unwrap(), SetNetworkParameter::OPCODE);

        // SAFETY: const sound because we transmute two types with the same layout
        unsafe { core::mem::transmute(self) }
    }
}
