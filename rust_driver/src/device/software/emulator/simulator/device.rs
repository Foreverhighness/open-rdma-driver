//! Emulator in simulator network
use core::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

use eui48::MacAddress;

use super::super::Emulator;
use super::config::{MEMORY_SIZE, WORD_WIDTH};
use super::rpc::{Client, RpcClient};
use super::udp_agent::UdpAgent;

impl Emulator<UdpAgent<RpcClient>> {
    pub fn new_testing(client_id: u64) -> Arc<Self> {
        let mac = MacAddress::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 3));
        let rpc = RpcClient;
        let udp_agent = UdpAgent::new(client_id, mac, ip, rpc);
        let dev = Arc::new(Self::new(udp_agent));

        dev.start_net();

        dev
    }
}
