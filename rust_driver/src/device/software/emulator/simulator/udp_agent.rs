//! Transmit/Receive network packet using simulator's network

use core::net::{IpAddr, Ipv4Addr};

use eui48::MacAddress;
use log::trace;
use smoltcp::phy::ChecksumCapabilities;
use smoltcp::wire::{
    EthernetAddress, EthernetFrame, EthernetProtocol, EthernetRepr, IpProtocol, IpRepr, Ipv4Packet, Ipv6Packet,
    UdpPacket, UdpRepr,
};

use super::super::net::{Result, UdpAgent, RDMA_PROT};
use super::rpc::{RpcAgent, RpcNetIfcRxTxPayload};

/// UdpAgent by using RPC call to communicate with peers
pub struct Agent<R: RpcAgent> {
    client_id: u64,
    mac: MacAddress,
    ip: IpAddr,

    rpc: R,
}

fn ip2mac(ip: IpAddr) -> MacAddress {
    if ip == IpAddr::V4(Ipv4Addr::new(192, 168, 0, 2)) {
        MacAddress::new([0xAA, 0xAB, 0xAC, 0xAD, 0xAE, 0xFE])
    } else if ip == IpAddr::V4(Ipv4Addr::new(192, 168, 0, 3)) {
        MacAddress::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF])
    } else {
        unimplemented!()
    }
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

    // TODO(fh): return `Vec<RpcNetIfcRxTxPayload>` for more meaningful testing?
    // So I can convert between `Vec<RpcNetIfcRxTxPayload>` and `payload: Vec<u8>`
    // Currently I convert between `eth_frame: Vec<u8>` and `payload: Vec<u8>`

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
            let payload = &request.data.0;

            if request.byte_en == [u8::MAX; 8] {
                // Potential zero copy, but keep it simple for now.
                buffer.extend_from_slice(payload);
            } else {
                // TODO(fh): use byte_en to select valid bits.
                buffer.extend_from_slice(payload);
            }

            let last_fragment = request.is_last == 1;
            if last_fragment {
                return buffer;
            }
        }
    }

    /// Transmit a single ethernet frame buffer to NIC
    fn transmit_ethernet_frame_buffer(&self, buffer: &[u8]) -> usize {
        let mut buf = buffer;
        let mut transmitted_bytes = 0;
        while !buf.is_empty() {
            let (mut request, len) = RpcNetIfcRxTxPayload::new_request(buf);
            transmitted_bytes += len;

            unsafe {
                self.rpc.c_netIfcPutTxData(self.client_id, &raw mut request);
            }
            buf = &buf[len..];
        }

        transmitted_bytes
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
    fn construct_frame(&self, dst_addr: IpAddr, payload: &[u8]) -> EthernetFrame<Vec<u8>> {
        const HOP_LIMIT: u8 = 64;

        let udp_repr = UdpRepr {
            src_port: RDMA_PROT,
            dst_port: RDMA_PROT,
        };

        let ip_repr = IpRepr::new(
            self.ip.into(),
            dst_addr.into(),
            IpProtocol::Udp,
            udp_repr.header_len() + payload.len(),
            HOP_LIMIT,
        );

        let ethertype = match ip_repr {
            IpRepr::Ipv4(_) => EthernetProtocol::Ipv4,
            IpRepr::Ipv6(_) => EthernetProtocol::Ipv6,
        };
        let ethernet_repr = EthernetRepr {
            src_addr: EthernetAddress::from_bytes(self.mac.as_bytes()),
            dst_addr: EthernetAddress::from_bytes(ip2mac(dst_addr).as_bytes()),
            ethertype,
        };

        let mut frame = EthernetFrame::new_checked(vec![0; ethernet_repr.buffer_len() + ip_repr.buffer_len()]).unwrap();
        ethernet_repr.emit(&mut frame);
        let buffer = frame.payload_mut();
        assert_eq!(buffer.len(), ip_repr.buffer_len());

        let buffer = match ip_repr {
            IpRepr::Ipv4(repr) => {
                let mut packet = Ipv4Packet::new_checked(buffer).unwrap();
                repr.emit(&mut packet, &ChecksumCapabilities::default());
                packet.set_ident(1);
                packet.clear_flags();
                packet.fill_checksum();

                let range = packet.header_len() as usize..packet.total_len() as usize;
                let buffer = packet.into_inner();
                &mut buffer[range]
            }
            IpRepr::Ipv6(repr) => {
                let mut packet = Ipv6Packet::new_checked(buffer).unwrap();
                repr.emit(&mut packet);

                let range = packet.header_len()..packet.total_len();
                let buffer = packet.into_inner();
                &mut buffer[range]
            }
        };

        ip_repr.emit(buffer, &ChecksumCapabilities::default());
        let buffer = &mut frame.payload_mut()[ip_repr.header_len()..ip_repr.buffer_len()];

        let mut datagram = UdpPacket::new_unchecked(buffer);
        udp_repr.emit(
            &mut datagram,
            &self.ip.into(),
            &dst_addr.into(),
            payload.len(),
            |p| p.copy_from_slice(&payload),
            &ChecksumCapabilities::ignored(),
        );

        frame
    }
}

impl<R: RpcAgent> UdpAgent for Agent<R> {
    fn send_to(&self, buf: &[u8], addr: IpAddr) -> Result<usize> {
        // TODO(fh): return errors

        let buffer = self.construct_frame(addr, buf).into_inner();
        let transmitted_bytes = self.transmit_ethernet_frame_buffer(&buffer);
        Ok(transmitted_bytes)
    }

    fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, IpAddr)> {
        let buffer = self.receive_ethernet_frame_buffer();

        let (payload, origin) = self.parse_frame_and_extract_payload(&buffer)?;
        let len = buf.len().min(payload.len());
        buf[..len].copy_from_slice(&payload[..len]);

        Ok((len, origin))
    }
}

#[cfg(test)]
mod tests {
    use core::net::Ipv4Addr;

    use smoltcp::wire::Ipv4Repr;

    use super::super::rpc::mock_agent::MockAgent;
    use super::*;

    const SENDER: Agent<MockAgent> = Agent::new(
        0,
        MacAddress::new([0xAA, 0xAB, 0xAC, 0xAD, 0xAE, 0xFE]),
        IpAddr::V4(Ipv4Addr::new(192, 168, 0, 2)),
        MockAgent::new(),
    );

    const RECEIVER: Agent<MockAgent> = Agent::new(
        0,
        MacAddress::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]),
        IpAddr::V4(Ipv4Addr::new(192, 168, 0, 3)),
        MockAgent::new(),
    );

    fn cmp_ethernet_frame(lhs: &[u8], rhs: &[u8]) {
        let lhs = EthernetFrame::new_checked(lhs).unwrap();
        let rhs = EthernetFrame::new_checked(rhs).unwrap();

        assert_eq!(EthernetRepr::parse(&lhs).unwrap(), EthernetRepr::parse(&rhs).unwrap());

        let lhs = Ipv4Packet::new_checked(lhs.payload()).unwrap();
        let rhs = Ipv4Packet::new_checked(rhs.payload()).unwrap();
        let checksum_caps = &ChecksumCapabilities::default();

        assert_eq!(
            Ipv4Repr::parse(&lhs, checksum_caps).unwrap(),
            Ipv4Repr::parse(&rhs, checksum_caps).unwrap(),
        );
        assert_eq!(lhs.checksum(), rhs.checksum());
        assert_eq!(lhs.payload(), rhs.payload());

        let ip_repr = Ipv4Repr::parse(&lhs, checksum_caps).unwrap();
        let src_addr = &ip_repr.src_addr.into_address();
        let dst_addr = &ip_repr.dst_addr.into_address();

        let lhs = UdpPacket::new_checked(lhs.payload()).unwrap();
        let rhs = UdpPacket::new_checked(rhs.payload()).unwrap();

        assert_eq!(
            UdpRepr::parse(&lhs, src_addr, dst_addr, checksum_caps),
            UdpRepr::parse(&rhs, src_addr, dst_addr, checksum_caps)
        );
        assert_eq!(lhs.checksum(), rhs.checksum());
        assert_eq!(lhs.payload(), rhs.payload());
    }

    #[test]
    fn test_recv_from() {
        let udp_agent = RECEIVER;
        let mut buf = vec![0; 8192];

        for frame in 0..=1 {
            let filename = &format!("ethernet-frame-{frame}.bin");
            let frame = std::fs::read(filename).unwrap();
            let (expected_payload, expected_origin) = udp_agent.parse_frame_and_extract_payload(&frame).unwrap();

            let (len, origin) = udp_agent.recv_from(&mut buf).unwrap();

            assert_eq!(origin, expected_origin);
            assert_eq!(len, expected_payload.len());
            assert_eq!(&buf[..len], expected_payload);
        }
    }

    #[test]
    fn test_construct_frame() {
        let udp_agent = SENDER;
        let dst_addr = RECEIVER.ip;

        for frame in 0..=1 {
            let filename = &format!("ethernet-frame-{frame}.bin");
            let buffer = std::fs::read(filename).unwrap();

            let (expected_payload, origin) = RECEIVER.parse_frame_and_extract_payload(&buffer).unwrap();
            assert_eq!(udp_agent.ip, origin);

            let frame = udp_agent.construct_frame(dst_addr, expected_payload);
            let frame = frame.into_inner();

            cmp_ethernet_frame(&frame, &buffer);
        }
    }

    #[test]
    fn test_transmit_ethernet_frame_buffer() {
        let udp_agent = SENDER;
        let dst_addr = RECEIVER.ip;

        for i in 0..=1 {
            let filename = &format!("ethernet-frame-{i}.bin");
            let buffer = std::fs::read(filename).unwrap();

            let (expected_payload, origin) = RECEIVER.parse_frame_and_extract_payload(&buffer).unwrap();
            assert_eq!(udp_agent.ip, origin);

            let frame = udp_agent.construct_frame(dst_addr, expected_payload);
            let frame = frame.into_inner();

            cmp_ethernet_frame(&frame, &buffer);

            let mut buf = frame.as_slice();
            let mut fragment = 0;
            while !buf.is_empty() {
                let filename = &format!("fragment-{i}-{fragment}.bin");
                fragment += 1;
                let json = std::fs::read(filename).unwrap();
                let expected = serde_json::from_slice(&json).unwrap();

                let (request, len) = RpcNetIfcRxTxPayload::new_request(buf);
                assert_eq!(request, expected);
                buf = &buf[len..];
            }
        }
    }
}
