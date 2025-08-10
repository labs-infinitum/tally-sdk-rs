use thiserror::Error;

#[derive(Debug, Error)]
pub enum TallyError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("XML error: {0}")]
    Xml(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

pub type Result<T> = std::result::Result<T, TallyError>;
