use core::net::IpAddr;
use std::sync::Arc;

use super::{DmaClient, NetAgent};
use crate::Emulator;
use crate::memory_region::Table;

impl Emulator {
    pub fn new_emulator(tun_ip: IpAddr) -> Arc<Self> {
        let dma_client = DmaClient;
        let mr_table = Table::new();

        let dev = Arc::new(Self::new(dma_client, mr_table));

        dev.start_work_queue();
        dev.start_net(move |para| NetAgent::new(para.ip.into(), para.subnet_mask.into(), tun_ip));

        dev
    }
}
