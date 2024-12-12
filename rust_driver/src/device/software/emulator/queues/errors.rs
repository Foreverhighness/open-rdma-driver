use super::command_request;

#[derive(Debug, thiserror::Error)]
pub enum ParseDescriptorError {
    #[error("command request descriptor parse failed {0:?}")]
    CommandRequest(Box<command_request::common::Unknown>),

    #[error("command request get unknown opcode {0}")]
    CommandRequestUnknownOpcode(u8),

    #[error("send descriptor parse failed {0:?}")]
    Send(Box<command_request::common::Unknown>),
}
