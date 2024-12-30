mod common;
mod read;
mod read_response;
mod write;
mod write_with_immediate;

pub(super) type Opcode = crate::third_party::queues::send::ToCardWorkRbDescOpcode;

pub(super) type ReadBuilder = read::Builder;
pub(super) type ReadResponseBuilder = read_response::Builder;
pub(super) type WriteBuilder = write::Builder;
pub(super) type WriteWithImmediateBuilder = write_with_immediate::Builder;
