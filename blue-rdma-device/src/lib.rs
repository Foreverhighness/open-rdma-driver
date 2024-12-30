//! Emulator for blue rdma device

pub mod device_api;
// TODO(fh): move emulator to driver layer?
// pub mod emulator;
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
// pub type Emulator = device_inner::DeviceInner<emulator::NetAgent, emulator::DmaClient>;

#[allow(clippy::all, dead_code, unused)]
mod third_party;
