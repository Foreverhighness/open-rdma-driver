//! Emulator in simulator network
use core::net::IpAddr;

use eui48::MacAddress;

use super::super::Emulator;
use super::config::{MEMORY_SIZE, WORD_WIDTH};
use super::rpc::{Client, RpcClient};
use super::udp_agent::UdpAgent;

impl Emulator<UdpAgent<RpcClient>> {
    pub fn new_testing(mac: MacAddress, ip: IpAddr) -> Self {
        let rpc = RpcClient;
        let client_id = unsafe { rpc.c_createBRAM(WORD_WIDTH, MEMORY_SIZE) };
        let udp_agent = UdpAgent::new(client_id, mac, ip, rpc);
        Self::new(udp_agent)
    }
}
