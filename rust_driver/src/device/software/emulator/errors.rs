use super::queues::errors::ParseDescriptorError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("descriptor parse failed: {0}")]
    ParseDescriptor(#[from] ParseDescriptorError),

    #[error("net error: {0}")]
    Network(#[from] super::net::Error),

    #[error("memory region error: {0}")]
    MemoryRegion(#[from] super::mr_table::Error),
}
