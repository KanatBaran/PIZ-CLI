use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("file not found: {0}")]
    FileNotFound(String),

    #[error("please specify either the '--add' or '--size' parameter")]
    MissingSizeFlag,

    #[error("invalid size format: {0}")]
    InvalidSizeFormat(String),

    #[error("could not parse number: {0}")]
    InvalidNumber(String),

    #[error("invalid size unit: {0}")]
    InvalidSizeUnit(String),

    #[error("target size ({target}) cannot be smaller than the current file size ({current})")]
    TargetSizeTooSmall { target: u64, current: u64 },

    #[error("could not open source file: {0}")]
    OpenSource(#[source] std::io::Error),

    #[error("could not create output file: {0}")]
    CreateOutput(#[source] std::io::Error),

    #[error("could not copy file: {0}")]
    CopyFile(#[source] std::io::Error),

    #[error("could not expand file: {0}")]
    ExpandFile(#[source] std::io::Error),

    #[error("could not flush write buffer: {0}")]
    Flush(#[source] std::io::Error),

    #[error("could not read file metadata: {0}")]
    Metadata(#[source] std::io::Error),
}
