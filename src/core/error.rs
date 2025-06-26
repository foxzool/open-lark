use thiserror::Error;

#[derive(Error, Debug)]
pub enum LarkAPIError {
    #[error("IO error: {0}")]
    IOErr(String),

    #[error("Invalid parameter: {0}")]
    IllegalParamError(String),

    #[error("JSON deserialization error: {0}")]
    DeserializeError(String),

    #[error("HTTP request failed: {0}")]
    RequestError(String),

    #[error("URL parse error: {0}")]
    UrlParseError(String),

    /// Enhanced API error with additional context
    #[error("API error: {message} (code: {code}, request_id: {request_id:?})")]
    ApiError {
        code: i32,
        message: String,
        request_id: Option<String>,
    },

    #[error("Missing access token")]
    MissingAccessToken,

    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl Clone for LarkAPIError {
    fn clone(&self) -> Self {
        match self {
            Self::IOErr(msg) => Self::IOErr(msg.clone()),
            Self::IllegalParamError(msg) => Self::IllegalParamError(msg.clone()),
            Self::DeserializeError(msg) => Self::DeserializeError(msg.clone()),
            Self::RequestError(msg) => Self::RequestError(msg.clone()),
            Self::UrlParseError(msg) => Self::UrlParseError(msg.clone()),
            Self::ApiError {
                code,
                message,
                request_id,
            } => Self::ApiError {
                code: *code,
                message: message.clone(),
                request_id: request_id.clone(),
            },
            Self::MissingAccessToken => Self::MissingAccessToken,
            Self::BadRequest(msg) => Self::BadRequest(msg.clone()),
        }
    }
}

impl From<std::io::Error> for LarkAPIError {
    fn from(err: std::io::Error) -> Self {
        Self::IOErr(err.to_string())
    }
}

impl From<serde_json::Error> for LarkAPIError {
    fn from(err: serde_json::Error) -> Self {
        Self::DeserializeError(err.to_string())
    }
}

impl From<reqwest::Error> for LarkAPIError {
    fn from(err: reqwest::Error) -> Self {
        Self::RequestError(err.to_string())
    }
}

impl From<url::ParseError> for LarkAPIError {
    fn from(err: url::ParseError) -> Self {
        Self::UrlParseError(err.to_string())
    }
}

/// 错误严重程度
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// 信息级别
    Info,
    /// 警告级别
    Warning,
    /// 错误级别
    Error,
    /// 严重错误级别
    Critical,
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

    /// 检查是否为权限相关错误
    pub fn is_permission_error(&self) -> bool {
        match self {
            Self::ApiError { code, .. } => {
                *code == 403
                    || matches!(
                        crate::core::error_codes::LarkErrorCode::from_code(*code),
                        Some(crate::core::error_codes::LarkErrorCode::Forbidden)
                    )
            }
            _ => false,
        }
    }

    /// 检查是否可以重试
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::ApiError { code, .. } => {
                if let Some(error_code) = crate::core::error_codes::LarkErrorCode::from_code(*code)
                {
                    error_code.is_retryable()
                } else {
                    false
                }
            }
            Self::RequestError(req_err) => {
                req_err.contains("timeout")
                    || req_err.contains("timed out")
                    || req_err.contains("connect")
                    || req_err.contains("connection")
            }
            _ => false,
        }
    }

    /// 获取用户友好的错误消息
    pub fn user_friendly_message(&self) -> String {
        match self {
            Self::ApiError { code, message, .. } => {
                if let Some(error_code) = crate::core::error_codes::LarkErrorCode::from_code(*code)
                {
                    error_code.detailed_description().to_string()
                } else {
                    format!("API调用失败: {} (错误码: {})", message, code)
                }
            }
            Self::MissingAccessToken => "缺少访问令牌，请检查认证配置".to_string(),
            Self::IllegalParamError(msg) => format!("参数错误: {}", msg),
            Self::RequestError(req_err) => {
                if req_err.contains("timeout") || req_err.contains("timed out") {
                    "请求超时，请检查网络连接".to_string()
                } else if req_err.contains("connect") || req_err.contains("connection") {
                    "连接失败，请检查网络设置".to_string()
                } else {
                    format!("网络请求失败: {}", req_err)
                }
            }
            _ => self.to_string(),
        }
    }
}
