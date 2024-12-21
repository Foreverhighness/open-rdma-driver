//! Emulator in simulator network
use std::sync::Arc;

use super::super::Emulator;
use super::dma_client::DmaClient;
use super::rpc::RpcClient;
use super::udp_agent::UdpAgent;
use crate::device::software::emulator::memory_region::Table;

impl Emulator<UdpAgent<RpcClient>> {
    pub fn new_testing(client_id: u64) -> Arc<Self> {
        let dma_client = DmaClient::new(client_id, RpcClient);

        let mr_table = Table::new();

        let dev = Arc::new(Self::new(dma_client, mr_table));

        dev.start_work_queue();
        dev.start_net(move |para| UdpAgent::new(client_id, para.mac, para.ip.into(), RpcClient));

        dev
    }
}
