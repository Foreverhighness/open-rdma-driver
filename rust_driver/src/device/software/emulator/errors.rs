use super::queues::errors::ParseDescriptorError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("descriptor parse failed")]
    ParseDescriptor(#[from] ParseDescriptorError),

    #[error("net error")]
    Network(#[from] super::net::Error),
}
