use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Document not found: {0}")]
    DocumentNotFound(u64),

    #[error("No active document")]
    NoActiveDocument,

    #[error("Invalid position: {0}")]
    InvalidPosition(usize),

    #[error("Parse error: {0}")]
    ParseError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
