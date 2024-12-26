use core::fmt;
use core::net::IpAddr;

use smoltcp::phy::ChecksumCapabilities;
use smoltcp::wire::{IpProtocol, IpRepr, Ipv4Packet, Ipv6Packet, UdpPacket, UdpRepr};

use crate::device::software::emulator::net::{self, RDMA_PORT};

pub struct NetAgent {
    tun: tun::Device,
    tun_ip: IpAddr,

    ip: IpAddr,
}

impl fmt::Debug for NetAgent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NetAgent")
            .field("ip", &self.ip)
            .field("tun", &self.tun_ip)
            .finish_non_exhaustive()
    }
}

impl NetAgent {
    pub fn new(ip: IpAddr, netmask: IpAddr, tun_ip: IpAddr) -> Self {
        log::info!("new tun {tun_ip} -> {ip}");
        let mut config = tun::configure();
        let config = config.address(tun_ip).netmask(netmask).destination(ip).up();
        let tun = tun::create(&config).unwrap();

        Self { tun, tun_ip, ip }
    }

    fn parse_packet_and_extract_payload<'b>(&self, buffer: &'b [u8]) -> Result<(&'b [u8], IpAddr), net::Error> {
        // May use `etherparse` crate instead of `smoltcp::wire`
        let (src_ip, dst_ip, datagram) = match self.ip {
            IpAddr::V4(_) => {
                let packet = Ipv4Packet::new_checked(buffer)?;
                if !packet.verify_checksum() {
                    return Err(net::Error::Crc);
                }

                (
                    packet.src_addr().into_address(),
                    packet.dst_addr().into_address(),
                    packet.payload(),
                )
            }
            IpAddr::V6(_) => {
                let packet = Ipv6Packet::new_checked(buffer)?;
                (
                    packet.src_addr().into_address(),
                    packet.dst_addr().into_address(),
                    packet.payload(),
                )
            }
        };
        assert_eq!(self.ip, IpAddr::from(dst_ip));

        let udp_datagram = UdpPacket::new_checked(datagram)?;
        let payload = udp_datagram.payload();

        Ok((payload, src_ip.into()))
    }

    /// construct ethernet frame from UDP payload
    fn construct_frame(&self, dst_addr: IpAddr, payload: &[u8]) -> Vec<u8> {
        const HOP_LIMIT: u8 = 64;

        let udp_repr = UdpRepr {
            src_port: RDMA_PORT,
            dst_port: RDMA_PORT,
        };

        let ip_repr = IpRepr::new(
            self.ip.into(),
            dst_addr.into(),
            IpProtocol::Udp,
            udp_repr.header_len() + payload.len(),
            HOP_LIMIT,
        );

        let mut packet = vec![0; ip_repr.buffer_len()];
        let buffer = packet.as_mut_slice();

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

        // ip_repr.emit(&mut *buffer, &ChecksumCapabilities::default());
        // let buffer = &mut buffer[ip_repr.header_len()..ip_repr.buffer_len()];

        let mut datagram = UdpPacket::new_unchecked(buffer);
        udp_repr.emit(
            &mut datagram,
            &self.ip.into(),
            &dst_addr.into(),
            payload.len(),
            |p| p.copy_from_slice(&payload),
            &ChecksumCapabilities::ignored(),
        );

        packet
    }
}

impl net::Agent for NetAgent {
    fn send_to(&self, buf: &[u8], addr: IpAddr) -> net::Result<usize> {
        let buffer = self.construct_frame(addr, buf);
        let len = self.tun.send(&buffer)?;

        // FIXME(fh): len is not send packet len
        Ok(len)
    }

    fn recv_from(&self, buf: &mut [u8]) -> net::Result<(usize, IpAddr)> {
        let mut buffer = vec![0u8; 8192];
        loop {
            let len = self.tun.recv(&mut buffer)?;
            log::trace!("tun recv {:?}", &buffer[..len]);

            let (payload, origin) = match self.parse_packet_and_extract_payload(&buffer[..len]) {
                Ok(res) => res,
                Err(net::Error::Crc) => continue,
                _ => todo!(),
            };
            let len = buf.len().min(payload.len());
            buf[..len].copy_from_slice(&payload[..len]);

            return Ok((len, origin));
        }
    }
}

#[cfg(test)]
mod tests {
    use core::net::{Ipv4Addr, SocketAddr};
    use std::net::UdpSocket;

    use net::{Agent, RDMA_PORT};

    use super::*;

    const SENDER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)), RDMA_PORT);
    const SENDER_TUN_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 233)), RDMA_PORT);
    const RECEIVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 0, 1, 1)), RDMA_PORT);
    const RECEIVER_TUN_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 0, 1, 233)), RDMA_PORT);
    const NETMASK: IpAddr = IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0));

    #[test]
    fn test_recv_from() {
        let _sender = NetAgent::new(SENDER_ADDR.ip(), NETMASK, SENDER_TUN_ADDR.ip());
        let receiver = NetAgent::new(RECEIVER_ADDR.ip(), NETMASK, RECEIVER_TUN_ADDR.ip());

        let socket = UdpSocket::bind(SENDER_TUN_ADDR).unwrap();
        let expected: [u8; 32] = core::array::from_fn(|i| i as u8);
        socket.send_to(&expected, RECEIVER_ADDR).unwrap();

        let mut buf = [0u8; 64];
        let (len, src) = receiver.recv_from(&mut buf).unwrap();

        assert_eq!(expected, &buf[..len]);
        assert_eq!(SENDER_TUN_ADDR.ip(), src);
    }

    #[test]
    fn test_sent_to() {
        let sender = NetAgent::new(SENDER_ADDR.ip(), NETMASK, SENDER_TUN_ADDR.ip());
        let receiver = NetAgent::new(RECEIVER_ADDR.ip(), NETMASK, RECEIVER_TUN_ADDR.ip());

        let expected: [u8; 32] = core::array::from_fn(|i| i as u8);
        let _len = sender.send_to(&expected, RECEIVER_ADDR.ip()).unwrap();

        let mut buf = [0u8; 64];
        let (len, src) = receiver.recv_from(&mut buf).unwrap();

        assert_eq!(expected, &buf[..len]);
        assert_eq!(src, SENDER_ADDR.ip());
    }
}
