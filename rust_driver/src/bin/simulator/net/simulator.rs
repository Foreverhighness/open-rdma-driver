//! Transmit/Receive network packet using simulator's network

use super::{Result, UdpAgent};

/// UdpAgent by using RPC call to communicate with peers
pub struct Agent {
    buffer: Vec<u8>,
}

impl Agent {
    /// Create a UDP agent
    pub fn new() -> Self {
        todo!()
    }
}

impl UdpAgent for Agent {
    fn send_to(&self, buf: &[u8], addr: core::net::IpAddr) -> Result<usize> {
        todo!()
    }

    fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, core::net::IpAddr)> {
        todo!()
    }
}
