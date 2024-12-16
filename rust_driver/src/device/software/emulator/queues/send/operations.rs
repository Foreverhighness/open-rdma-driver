mod common;
mod read;
mod read_response;
mod write;
mod write_with_immediate;

pub(super) type Opcode = crate::device::types::ToCardWorkRbDescOpcode;

pub(super) type WriteBuilder = write::Builder;
