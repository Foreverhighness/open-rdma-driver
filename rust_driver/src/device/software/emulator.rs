//! Emulator for blue rdma device

pub mod device_api;
pub mod simulator;

mod csr;
mod dma;
mod errors;
mod impls;
mod net;
mod queue;

pub use impls::*;

pub type Result<T> = core::result::Result<T, errors::Error>;
