//! Emulator for blue rdma device

pub mod device_api;
pub mod simulator;

mod address;
mod csr;
mod dma;
mod errors;
mod impls;
mod memory_region;
mod mr_table;
mod net;
mod queue_pair;
mod queues;

pub use impls::*;

pub type Result<T> = core::result::Result<T, errors::Error>;
