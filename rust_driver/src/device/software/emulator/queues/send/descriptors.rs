mod common;
mod seg0;
mod seg1;
mod variable_len_sge;

pub(super) use common::ScatterGatherElement;
pub(super) use seg0::Seg0;
pub(super) use seg1::Seg1;
pub(super) use variable_len_sge::VariableLengthSge;

pub(super) const DESCRIPTOR_SIZE: usize = 32; // 256 bits
pub(super) const DESCRIPTOR_ALIGN: usize = 32; // 256 bits
