//! `Python Server` RPC call wrapper

mod client;
pub(crate) mod mock_client;

pub use client::{BarIoInfo, Client, RpcClient, RpcNetIfcRxTxPayload};
