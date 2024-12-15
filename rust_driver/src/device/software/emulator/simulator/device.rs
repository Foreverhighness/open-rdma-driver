//! Emulator in simulator network
use core::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

use eui48::MacAddress;

use super::super::Emulator;
use super::dma_client::DmaClient;
use super::rpc::RpcClient;
use super::udp_agent::UdpAgent;
use crate::device::software::emulator::memory_region::Table;

impl Emulator<UdpAgent<RpcClient>> {
    pub fn new_testing(client_id: u64) -> Arc<Self> {
        let mac = MacAddress::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 3));

        let udp_agent = UdpAgent::new(client_id, mac, ip, RpcClient);
        let dma_client = DmaClient::new(client_id, RpcClient);

        let mr_table = Table::new();

        let dev = Arc::new(Self::new(udp_agent, dma_client, mr_table));

        dev.start_work_queue();
        dev.start_net();

        dev
    }
}
