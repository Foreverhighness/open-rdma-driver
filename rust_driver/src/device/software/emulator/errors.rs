#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("descriptor parse failed")]
    DescriptorParse(
        // TODO(fh): Add field
    ),
}
