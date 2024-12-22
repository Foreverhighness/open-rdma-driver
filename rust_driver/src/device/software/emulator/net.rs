mod agent;
mod message;
mod util;

pub(super) use agent::Agent;

pub type Result<T> = core::result::Result<T, Error>;

// Should the RDMA port always be fixed? I think making it configurable might be a better idea.
/// Assume UDP port is always 4791.
pub const RDMA_PROT: u16 = 4791;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Ethernet frame is malformed")]
    MalformedFrame(#[from] smoltcp::wire::Error),

    #[error("invalid rdma packet")]
    InvalidPacket,
}
