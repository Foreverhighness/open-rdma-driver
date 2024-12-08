//! Transmit/Receive network packet using simulator's network

use core::mem::transmute;
use core::net::IpAddr;

use eui48::MacAddress;
use log::trace;
use smoltcp::phy::ChecksumCapabilities;
use smoltcp::wire::{EthernetFrame, EthernetProtocol, Ipv4Packet, Ipv6Packet, UdpPacket, UdpRepr};

use super::{Result, UdpAgent, RDMA_PROT};
use crate::rpc::agent::RpcAgent;
use crate::rpc::RpcNetIfcRxTxPayload;

/// UdpAgent by using RPC call to communicate with peers
pub struct Agent<R: RpcAgent> {
    client_id: u64,
    mac: MacAddress,
    ip: IpAddr,

    rpc: R,
}

impl<R: RpcAgent> Agent<R> {
    /// Create a UDP agent
    pub const fn new(client_id: u64, mac: MacAddress, ip: IpAddr, rpc: R) -> Self {
        Self {
            client_id,
            mac,
            ip,
            rpc,
        }
    }

    /// Receive a single ethernet frame buffer from NIC
    fn receive_ethernet_frame_buffer(&self) -> Vec<u8> {
        let mut request = RpcNetIfcRxTxPayload::new();
        let mut buffer = Vec::new();

        loop {
            unsafe {
                self.rpc.c_netIfcGetRxData(&raw mut request, self.client_id, 0);
            }
            let invalid_fragment = request.is_valid == 0;
            if invalid_fragment {
                continue;
            }
            trace!("Get NIC fragment: {request:?}");
            let payload = unsafe { transmute::<_, [u8; 64]>(request.data) };

            if request.byte_en == [u8::MAX; 8] {
                // Potential zero copy, but keep it simple for now.
                buffer.extend_from_slice(&payload);
            } else {
                // TODO(fh): use byte_en to select valid bits.
                buffer.extend_from_slice(&payload);
            }

            let last_fragment = request.is_last == 1;
            if last_fragment {
                return buffer;
            }
        }
    }

    /// Transmit a single ethernet frame buffer to NIC
    fn transmit_ethernet_frame_buffer(&self, buffer: &[u8]) {
        let mut buf = buffer;
        while !buf.is_empty() {
            let (request, next_buf) = RpcNetIfcRxTxPayload::request(buf);
            buf = next_buf;
        }
    }

    /// Parse frame and extract UDP payload and source ip address
    fn parse_frame_and_extract_payload<'b>(&self, buffer: &'b [u8]) -> Result<(&'b [u8], IpAddr)> {
        // May use `etherparse` crate instead of `smoltcp::wire`
        let eth_frame = EthernetFrame::new_checked(buffer)?;
        assert_eq!(eth_frame.dst_addr().as_bytes(), self.mac.as_bytes());

        let (src_ip, dst_ip, datagram) = match eth_frame.ethertype() {
            EthernetProtocol::Ipv4 => {
                let packet = Ipv4Packet::new_checked(eth_frame.payload())?;
                (
                    packet.src_addr().into_address(),
                    packet.dst_addr().into_address(),
                    packet.payload(),
                )
            }
            EthernetProtocol::Ipv6 => {
                let packet = Ipv6Packet::new_checked(eth_frame.payload())?;
                (
                    packet.src_addr().into_address(),
                    packet.dst_addr().into_address(),
                    packet.payload(),
                )
            }
            _ => unimplemented!(),
        };
        assert_eq!(IpAddr::from(dst_ip), self.ip);

        let udp_datagram = UdpPacket::new_checked(datagram)?;
        let payload = udp_datagram.payload();

        Ok((payload, src_ip.into()))
    }

    /// construct ethernet frame from UDP payload
    fn construct_frame<'p>(&self, dst_addr: IpAddr, payload: &'p [u8]) -> EthernetFrame<&'p [u8]> {
        let repr = UdpRepr {
            src_port: RDMA_PROT,
            dst_port: RDMA_PROT,
        };
        let mut buffer = vec![0; repr.header_len() + payload.len()];
        let mut datagram = UdpPacket::new_unchecked(&mut buffer);
        repr.emit(
            &mut datagram,
            &self.ip.into(),
            &dst_addr.into(),
            payload.len(),
            |p| p.copy_from_slice(&payload),
            &ChecksumCapabilities::default(),
        );

        todo!()
    }
}

impl<R: RpcAgent> UdpAgent for Agent<R> {
    fn send_to(&self, buf: &[u8], addr: IpAddr) -> Result<usize> {
        todo!()
    }

    fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, IpAddr)> {
        let buffer = self.receive_ethernet_frame_buffer();

        let (payload, origin) = self.parse_frame_and_extract_payload(&buffer)?;
        let len = buf.len().min(payload.len());
        buf[..len].copy_from_slice(&payload[..len]);

        Ok((len, origin))
    }
}
