//! Modules for communicate within simulator network.

pub mod csr_proxy;
pub mod device;
pub mod rpc;

mod config;
mod dma_client;
mod udp_agent;

pub(crate) use dma_client::DmaClient;
pub(crate) use udp_agent::UdpAgent;
