//! Emulator for blue rdma device

pub mod device_api;
pub mod emulator;
pub mod simulator;

mod address;
mod csr;
mod device_inner;
mod dma;
mod errors;
mod memory_region;
mod mr_table;
mod net;
mod queue_pair;
mod queues;
mod types;

pub use device_inner::DeviceInner;

pub type Result<T = ()> = core::result::Result<T, errors::Error>;

pub type Simulator = DeviceInner<simulator::UdpAgent, simulator::DmaClient>;
pub type Emulator = DeviceInner<emulator::NetAgent, emulator::DmaClient>;

mod third_party;
