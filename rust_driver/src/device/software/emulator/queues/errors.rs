use super::command_request;

#[derive(Debug, thiserror::Error)]
pub enum ParseDescriptorError {
    #[error("command request descriptor parse failed {0:?}")]
    CommandRequest(Box<command_request::common::Unknown>),

    #[error("command request get unknown opcode {0}")]
    CommandRequestUnknownOpcode(u8),

    #[error("send descriptor parse failed {0:?}")]
    Send(Box<[u8; 32]>),

    #[error("send get unknown opcode {0}")]
    SendUnknownOpcode(u8),

    #[error("Invalid queue pair type {0}")]
    InvalidQueuePairType(u8),

    #[error("Invalid packet MTU kind {0}")]
    InvalidPacketMTUKind(u8),
}
