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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_error_display_http() {
        let err = WebhookError::Http("connection refused".to_string());
        assert_eq!(err.to_string(), "HTTP error: connection refused");
    }

    #[test]
    fn test_webhook_error_display_serialization() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        let err = WebhookError::Serialization(json_err);
        assert!(err.to_string().starts_with("Serialization error:"));
    }

    #[test]
    fn test_webhook_error_display_invalid_signature() {
        let err = WebhookError::InvalidSignature;
        assert_eq!(err.to_string(), "Invalid signature");
    }

    #[test]
    fn test_webhook_error_display_missing_field() {
        let err = WebhookError::MissingField("webhook_url".to_string());
        assert_eq!(err.to_string(), "Missing required field: webhook_url");
    }

    #[test]
    fn test_webhook_error_debug() {
        let err = WebhookError::InvalidSignature;
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("InvalidSignature"));
    }

    #[test]
    fn test_result_type() {
        let ok_result: Result<i32> = Ok(42);
        assert_eq!(ok_result.unwrap(), 42);

        let err_result: Result<i32> = Err(WebhookError::InvalidSignature);
        assert!(err_result.is_err());
    }
}
