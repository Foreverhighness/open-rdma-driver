mod bth;
mod bth_aeth;
mod bth_reth;
mod common;
mod imm_dt;
mod secondary_reth;

pub use bth_aeth::BthAeth;
pub use bth_reth::BthReth;
pub use common::{AckExtendedTransportHeader, BaseTransportHeader, RdmaExtendedTransportHeader};
pub use imm_dt::ImmDt;
pub use secondary_reth::SecondaryReth;

pub(super) const DESCRIPTOR_SIZE: usize = 32; // 256 bits
#[expect(unused, reason = "only defined")]
pub(super) const DESCRIPTOR_ALIGN: usize = 32; // 256 bits
