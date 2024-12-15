use super::queues::errors::ParseDescriptorError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("descriptor parse failed: {0}")]
    ParseDescriptor(#[from] ParseDescriptorError),

    #[error("net error: {0}")]
    Network(#[from] super::net::Error),
}
