mod bth;
mod bth_aeth;
mod bth_reth;
mod common;
mod imm_dt;
mod secondary_reth;

pub(crate) use bth_reth::BthReth;
pub(crate) use common::{BaseTransportHeader, RdmaExtendedTransportHeader};

pub(super) const DESCRIPTOR_SIZE: usize = 32; // 256 bits
pub(super) const DESCRIPTOR_ALIGN: usize = 32; // 256 bits
