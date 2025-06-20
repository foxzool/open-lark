use thiserror::Error;

#[derive(Error, Debug)]
pub enum LarkAPIError {
    #[error("IO error: {0}")]
    IOErr(#[from] std::io::Error),

    #[error("Invalid parameter: {0}")]
    IllegalParamError(String),

    #[error("JSON deserialization error: {0}")]
    DeserializeError(#[from] serde_json::Error),

    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),

    /// Enhanced API error with additional context
    #[error("API error: {message} (code: {code}, request_id: {request_id:?})")]
    ApiError {
        code: i32,
        message: String,
        request_id: Option<String>,
    },
}

impl LarkAPIError {
    /// Create an API error with additional context
    pub fn api_error<M: Into<String>>(code: i32, message: M, request_id: Option<String>) -> Self {
        Self::ApiError {
            code,
            message: message.into(),
            request_id,
        }
    }

    /// Create an illegal parameter error
    pub fn illegal_param<T: Into<String>>(message: T) -> Self {
        Self::IllegalParamError(message.into())
    }
}
