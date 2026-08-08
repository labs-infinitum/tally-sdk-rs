//! Error types returned by this crate.

use thiserror::Error;

/// Errors produced while talking to Tally or validating inputs.
#[derive(Debug, Error)]
pub enum TallyError {
    /// Request or model validation failed before contacting Tally.
    #[error("Validation error: {0}")]
    Validation(String),

    /// Tally returned a non-success HTTP status or transport body error.
    #[error("HTTP error: {0}")]
    Http(String),

    /// Could not connect to the Tally HTTP endpoint (after retries).
    #[error("Connection error: {0}")]
    Connection(String),

    /// XML encode/decode or envelope construction failed.
    #[error("XML error: {0}")]
    Xml(String),

    /// Catch-all for unexpected internal failures.
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

/// Convenient result alias used throughout the SDK.
pub type Result<T> = std::result::Result<T, TallyError>;
