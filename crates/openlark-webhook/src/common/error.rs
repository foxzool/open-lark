//! Error types for webhook module

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebhookError {
    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Missing required field: {0}")]
    MissingField(String),
}

pub type Result<T> = std::result::Result<T, WebhookError>;
