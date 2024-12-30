//! Emulator in simulator network
use std::sync::Arc;

use super::super::Simulator;
use super::dma_client::DmaClient;
use super::rpc::RpcClient;
use super::udp_agent::UdpAgent;
use crate::memory_region::Table;

impl Simulator {
    #[must_use]
    pub fn new_simulator(client_id: u64) -> Arc<Self> {
        let dma_client = DmaClient::new(client_id, RpcClient);

        let mr_table = Table::new();

        let dev = Arc::new(Self::new(dma_client, mr_table));

        dev.start_work_queue();
        dev.start_net(move |para| UdpAgent::new(client_id, para.mac, para.ip.into(), RpcClient));

        dev
    }
}
