use thiserror::Error;

/// 飞书开放平台API错误类型
///
/// 包含所有可能的API调用错误，提供详细的错误信息和处理建议。
/// 支持错误分类、重试判断和用户友好的错误消息。
///
/// # 错误类型分类
///
/// - **网络错误**: RequestError, IOErr, UrlParseError
/// - **数据错误**: DeserializeError, DataError
/// - **参数错误**: IllegalParamError, BadRequest
/// - **API错误**: ApiError, APIError
/// - **认证错误**: MissingAccessToken
///
/// # 错误处理示例
///
/// ```rust
/// use open_lark::core::error::LarkAPIError;
///
/// fn handle_api_error(error: LarkAPIError) {
///     match error {
///         LarkAPIError::MissingAccessToken => {
///             println!("请检查应用凭据配置");
///         }
///         LarkAPIError::ApiError { code, message, .. } if code == 403 => {
///             println!("权限不足: {}", message);
///         }
///         err if err.is_retryable() => {
///             println!("网络错误，可以重试: {}", err.user_friendly_message());
///         }
///         _ => {
///             println!("操作失败: {}", error.user_friendly_message());
///         }
///     }
/// }
/// ```
///
/// # 最佳实践
///
/// - 使用 `is_retryable()` 判断是否可以重试
/// - 使用 `user_friendly_message()` 获取用户友好的错误提示
/// - 使用 `is_permission_error()` 检查权限相关错误
#[derive(Error, Debug)]
pub enum LarkAPIError {
    /// 输入输出错误
    ///
    /// 通常由文件操作、网络IO等底层操作失败引起。
    #[error("IO error: {0}")]
    IOErr(String),

    /// 非法参数错误
    ///
    /// 当传入的参数不符合API要求时抛出，如无效的ID格式、超出范围的值等。
    #[error("Invalid parameter: {0}")]
    IllegalParamError(String),

    /// JSON反序列化错误
    ///
    /// 当API响应的JSON格式无法解析为预期的数据结构时发生。
    #[error("JSON deserialization error: {0}")]
    DeserializeError(String),

    /// HTTP请求失败
    ///
    /// 网络请求层面的错误，如连接超时、DNS解析失败等。通常可以重试。
    #[error("HTTP request failed: {0}")]
    RequestError(String),

    /// URL解析错误
    ///
    /// 当构建的API请求URL格式不正确时发生。
    #[error("URL parse error: {0}")]
    UrlParseError(String),

    /// 增强的API错误
    ///
    /// 包含错误码、消息和请求ID的完整错误信息，便于调试和问题追踪。
    #[error("API error: {message} (code: {code}, request_id: {request_id:?})")]
    ApiError {
        /// API错误码
        code: i32,
        /// 错误消息
        message: String,
        /// 请求ID，用于问题追踪
        request_id: Option<String>,
    },

    /// 缺少访问令牌
    ///
    /// 当API调用需要认证但未提供有效的访问令牌时发生。
    #[error("Missing access token")]
    MissingAccessToken,

    /// 错误的请求
    ///
    /// 请求格式或内容不符合API规范。
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// 数据处理错误
    ///
    /// 数据验证、转换或处理过程中发生的错误。
    #[error("Data error: {0}")]
    DataError(String),

    /// 标准API响应错误
    ///
    /// 飞书开放平台返回的标准错误响应，包含完整的错误信息。
    #[error("API error: {msg} (code: {code})")]
    APIError {
        /// API错误码
        code: i32,
        /// 错误消息
        msg: String,
        /// 详细错误信息
        error: Option<String>,
    },
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
            Self::DataError(msg) => Self::DataError(msg.clone()),
            Self::APIError { code, msg, error } => Self::APIError {
                code: *code,
                msg: msg.clone(),
                error: error.clone(),
            },
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
///
/// 用于对错误进行分级，帮助确定错误处理策略和用户提示方式。
///
/// # 使用场景
///
/// - **Info**: 信息性消息，通常不需要特殊处理
/// - **Warning**: 警告信息，可能影响功能但不阻断操作
/// - **Error**: 错误信息，导致操作失败但系统可恢复
/// - **Critical**: 严重错误，可能导致系统不稳定
///
/// # 示例
///
/// ```rust
/// use open_lark::core::error::ErrorSeverity;
///
/// fn log_error(severity: ErrorSeverity, message: &str) {
///     match severity {
///         ErrorSeverity::Info => println!("ℹ️  {}", message),
///         ErrorSeverity::Warning => println!("⚠️  {}", message),
///         ErrorSeverity::Error => eprintln!("❌ {}", message),
///         ErrorSeverity::Critical => eprintln!("🚨 CRITICAL: {}", message),
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorSeverity {
    /// 信息级别 - 一般性提示信息
    Info,
    /// 警告级别 - 可能的问题但不影响核心功能
    Warning,
    /// 错误级别 - 操作失败但系统可恢复
    Error,
    /// 严重错误级别 - 可能影响系统稳定性
    Critical,
}

impl LarkAPIError {
    /// 创建包含上下文信息的API错误
    ///
    /// # 参数
    /// - `code`: 错误码
    /// - `message`: 错误消息
    /// - `request_id`: 请求ID，用于问题追踪
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::core::error::LarkAPIError;
    ///
    /// let error = LarkAPIError::api_error(
    ///     403,
    ///     "权限不足",
    ///     Some("req_123456".to_string())
    /// );
    /// ```
    pub fn api_error<M: Into<String>>(code: i32, message: M, request_id: Option<String>) -> Self {
        Self::ApiError {
            code,
            message: message.into(),
            request_id,
        }
    }

    /// 创建非法参数错误
    ///
    /// # 参数
    /// - `message`: 错误详细信息
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::core::error::LarkAPIError;
    ///
    /// let error = LarkAPIError::illegal_param("用户ID格式不正确");
    /// ```
    pub fn illegal_param<T: Into<String>>(message: T) -> Self {
        Self::IllegalParamError(message.into())
    }

    /// 检查是否为权限相关错误
    ///
    /// 用于判断错误是否由权限不足引起，便于进行相应的错误处理。
    ///
    /// # 返回值
    /// - `true`: 权限相关错误
    /// - `false`: 其他类型错误
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

    /// 检查错误是否可以重试
    ///
    /// 判断当前错误是否为临时性错误，可以通过重试解决。
    /// 通常网络超时、连接失败等错误可以重试。
    ///
    /// # 返回值
    /// - `true`: 可以重试的错误
    /// - `false`: 不可重试的错误（如参数错误、权限错误）
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::core::error::LarkAPIError;
    ///
    /// let error = LarkAPIError::RequestError("连接超时".to_string());
    /// if error.is_retryable() {
    ///     println!("可以重试该请求");
    /// }
    /// ```
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
    ///
    /// 将技术性的错误信息转换为用户容易理解的提示信息。
    /// 包含错误原因和可能的解决建议。
    ///
    /// # 返回值
    /// 经过本地化和优化的错误消息字符串
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::core::error::LarkAPIError;
    ///
    /// let error = LarkAPIError::MissingAccessToken;
    /// println!("错误提示: {}", error.user_friendly_message());
    /// // 输出: "缺少访问令牌，请检查认证配置"
    /// ```
    pub fn user_friendly_message(&self) -> String {
        match self {
            Self::ApiError { code, message, .. } => {
                if let Some(error_code) = crate::core::error_codes::LarkErrorCode::from_code(*code)
                {
                    error_code.detailed_description().to_string()
                } else {
                    format!("API调用失败: {message} (错误码: {code})")
                }
            }
            Self::MissingAccessToken => "缺少访问令牌，请检查认证配置".to_string(),
            Self::IllegalParamError(msg) => format!("参数错误: {msg}"),
            Self::RequestError(req_err) => {
                if req_err.contains("timeout") || req_err.contains("timed out") {
                    "请求超时，请检查网络连接".to_string()
                } else if req_err.contains("connect") || req_err.contains("connection") {
                    "连接失败，请检查网络设置".to_string()
                } else {
                    format!("网络请求失败: {req_err}")
                }
            }
            _ => self.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error as IOError, ErrorKind};

    #[test]
    fn test_lark_api_error_creation() {
        let error = LarkAPIError::IOErr("test error".to_string());
        assert!(matches!(error, LarkAPIError::IOErr(_)));
    }

    #[test]
    fn test_error_display() {
        let io_error = LarkAPIError::IOErr("file not found".to_string());
        assert_eq!(io_error.to_string(), "IO error: file not found");

        let param_error = LarkAPIError::IllegalParamError("invalid user_id".to_string());
        assert_eq!(
            param_error.to_string(),
            "Invalid parameter: invalid user_id"
        );

        let deserialize_error = LarkAPIError::DeserializeError("invalid json".to_string());
        assert_eq!(
            deserialize_error.to_string(),
            "JSON deserialization error: invalid json"
        );

        let request_error = LarkAPIError::RequestError("timeout".to_string());
        assert_eq!(request_error.to_string(), "HTTP request failed: timeout");

        let url_error = LarkAPIError::UrlParseError("malformed url".to_string());
        assert_eq!(url_error.to_string(), "URL parse error: malformed url");

        let missing_token = LarkAPIError::MissingAccessToken;
        assert_eq!(missing_token.to_string(), "Missing access token");

        let bad_request = LarkAPIError::BadRequest("malformed data".to_string());
        assert_eq!(bad_request.to_string(), "Bad request: malformed data");

        let data_error = LarkAPIError::DataError("validation failed".to_string());
        assert_eq!(data_error.to_string(), "Data error: validation failed");
    }

    #[test]
    fn test_api_error_with_context() {
        let api_error = LarkAPIError::ApiError {
            code: 403,
            message: "Permission denied".to_string(),
            request_id: Some("req_123".to_string()),
        };

        let display = api_error.to_string();
        assert!(display.contains("403"));
        assert!(display.contains("Permission denied"));
        assert!(display.contains("req_123"));
    }

    #[test]
    fn test_api_error_without_request_id() {
        let api_error = LarkAPIError::ApiError {
            code: 404,
            message: "Not found".to_string(),
            request_id: None,
        };

        let display = api_error.to_string();
        assert!(display.contains("404"));
        assert!(display.contains("Not found"));
    }

    #[test]
    fn test_standard_api_error() {
        let api_error = LarkAPIError::APIError {
            code: 500,
            msg: "Internal server error".to_string(),
            error: Some("Database connection failed".to_string()),
        };

        let display = api_error.to_string();
        assert!(display.contains("500"));
        assert!(display.contains("Internal server error"));
    }

    #[test]
    fn test_error_clone() {
        let original = LarkAPIError::ApiError {
            code: 403,
            message: "Forbidden".to_string(),
            request_id: Some("req_456".to_string()),
        };

        let cloned = original.clone();

        match (&original, &cloned) {
            (
                LarkAPIError::ApiError {
                    code: c1,
                    message: m1,
                    request_id: r1,
                },
                LarkAPIError::ApiError {
                    code: c2,
                    message: m2,
                    request_id: r2,
                },
            ) => {
                assert_eq!(c1, c2);
                assert_eq!(m1, m2);
                assert_eq!(r1, r2);
            }
            _ => panic!("Clone did not preserve variant"),
        }
    }

    #[test]
    fn test_from_std_io_error() {
        let io_error = IOError::new(ErrorKind::NotFound, "file not found");
        let lark_error: LarkAPIError = io_error.into();

        match lark_error {
            LarkAPIError::IOErr(msg) => assert!(msg.contains("file not found")),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_from_serde_json_error() {
        let json_result: Result<serde_json::Value, _> = serde_json::from_str("invalid json");
        let json_error = json_result.unwrap_err();
        let lark_error: LarkAPIError = json_error.into();

        match lark_error {
            LarkAPIError::DeserializeError(msg) => assert!(!msg.is_empty()),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_from_reqwest_error() {
        // Create a reqwest error (this is a bit tricky, so we'll test the conversion logic)
        let error = LarkAPIError::RequestError("connection refused".to_string());
        assert!(matches!(error, LarkAPIError::RequestError(_)));
    }

    #[test]
    fn test_from_url_parse_error() {
        let url_result = url::Url::parse("not a url");
        let url_error = url_result.unwrap_err();
        let lark_error: LarkAPIError = url_error.into();

        match lark_error {
            LarkAPIError::UrlParseError(msg) => assert!(!msg.is_empty()),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_error_severity_values() {
        assert_eq!(ErrorSeverity::Info, ErrorSeverity::Info);
        assert_ne!(ErrorSeverity::Info, ErrorSeverity::Warning);
        assert_ne!(ErrorSeverity::Warning, ErrorSeverity::Error);
        assert_ne!(ErrorSeverity::Error, ErrorSeverity::Critical);
    }

    #[test]
    fn test_error_severity_debug() {
        let info = ErrorSeverity::Info;
        let debug_string = format!("{:?}", info);
        assert_eq!(debug_string, "Info");
    }

    #[test]
    fn test_error_severity_clone() {
        let original = ErrorSeverity::Critical;
        let cloned = original;
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_api_error_constructor() {
        let error = LarkAPIError::api_error(404, "Resource not found", Some("req_789".to_string()));

        match error {
            LarkAPIError::ApiError {
                code,
                message,
                request_id,
            } => {
                assert_eq!(code, 404);
                assert_eq!(message, "Resource not found");
                assert_eq!(request_id, Some("req_789".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_illegal_param_constructor() {
        let error = LarkAPIError::illegal_param("Invalid user ID format");

        match error {
            LarkAPIError::IllegalParamError(msg) => {
                assert_eq!(msg, "Invalid user ID format");
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_is_permission_error() {
        let permission_error = LarkAPIError::ApiError {
            code: 403,
            message: "Forbidden".to_string(),
            request_id: None,
        };
        assert!(permission_error.is_permission_error());

        let not_permission_error = LarkAPIError::ApiError {
            code: 404,
            message: "Not found".to_string(),
            request_id: None,
        };
        assert!(!not_permission_error.is_permission_error());

        let other_error = LarkAPIError::MissingAccessToken;
        assert!(!other_error.is_permission_error());
    }

    #[test]
    fn test_is_retryable() {
        // Test retryable request errors
        let timeout_error = LarkAPIError::RequestError("connection timeout".to_string());
        assert!(timeout_error.is_retryable());

        let connect_error = LarkAPIError::RequestError("connection refused".to_string());
        assert!(connect_error.is_retryable());

        let timed_out_error = LarkAPIError::RequestError("request timed out".to_string());
        assert!(timed_out_error.is_retryable());

        // Test non-retryable errors
        let param_error = LarkAPIError::IllegalParamError("bad param".to_string());
        assert!(!param_error.is_retryable());

        let missing_token = LarkAPIError::MissingAccessToken;
        assert!(!missing_token.is_retryable());

        let other_request_error = LarkAPIError::RequestError("bad request".to_string());
        assert!(!other_request_error.is_retryable());
    }

    #[test]
    fn test_user_friendly_message() {
        // Test missing access token
        let missing_token = LarkAPIError::MissingAccessToken;
        assert_eq!(
            missing_token.user_friendly_message(),
            "缺少访问令牌，请检查认证配置"
        );

        // Test parameter error
        let param_error = LarkAPIError::IllegalParamError("invalid format".to_string());
        assert_eq!(
            param_error.user_friendly_message(),
            "参数错误: invalid format"
        );

        // Test timeout
        let timeout_error = LarkAPIError::RequestError("connection timeout".to_string());
        assert_eq!(
            timeout_error.user_friendly_message(),
            "请求超时，请检查网络连接"
        );

        // Test connection error
        let connect_error = LarkAPIError::RequestError("connection failed".to_string());
        assert_eq!(
            connect_error.user_friendly_message(),
            "连接失败，请检查网络设置"
        );

        // Test generic request error
        let generic_error = LarkAPIError::RequestError("unknown error".to_string());
        assert_eq!(
            generic_error.user_friendly_message(),
            "网络请求失败: unknown error"
        );

        // Test other error types
        let io_error = LarkAPIError::IOErr("file error".to_string());
        assert!(io_error.user_friendly_message().contains("file error"));
    }

    #[test]
    fn test_api_error_user_friendly_message() {
        let api_error = LarkAPIError::ApiError {
            code: 123456, // Unknown code
            message: "Unknown error".to_string(),
            request_id: Some("req_test".to_string()),
        };

        let friendly_msg = api_error.user_friendly_message();
        assert!(friendly_msg.contains("Unknown error"));
        assert!(friendly_msg.contains("123456"));
    }

    #[test]
    fn test_all_error_variants_clone() {
        let errors = vec![
            LarkAPIError::IOErr("io".to_string()),
            LarkAPIError::IllegalParamError("param".to_string()),
            LarkAPIError::DeserializeError("json".to_string()),
            LarkAPIError::RequestError("request".to_string()),
            LarkAPIError::UrlParseError("url".to_string()),
            LarkAPIError::ApiError {
                code: 400,
                message: "test".to_string(),
                request_id: Some("req".to_string()),
            },
            LarkAPIError::MissingAccessToken,
            LarkAPIError::BadRequest("bad".to_string()),
            LarkAPIError::DataError("data".to_string()),
            LarkAPIError::APIError {
                code: 500,
                msg: "error".to_string(),
                error: Some("detail".to_string()),
            },
        ];

        for error in errors {
            let cloned = error.clone();
            // Test that clone preserves the variant and data
            assert_eq!(error.to_string(), cloned.to_string());
        }
    }

    #[test]
    fn test_debug_trait() {
        let error = LarkAPIError::ApiError {
            code: 403,
            message: "Forbidden".to_string(),
            request_id: Some("req_debug".to_string()),
        };

        let debug_string = format!("{:?}", error);
        assert!(debug_string.contains("ApiError"));
        assert!(debug_string.contains("403"));
        assert!(debug_string.contains("Forbidden"));
    }

    #[test]
    fn test_api_error_with_string_conversion() {
        let error = LarkAPIError::api_error(500, String::from("Server error"), None);

        match error {
            LarkAPIError::ApiError { message, .. } => {
                assert_eq!(message, "Server error");
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_illegal_param_with_string_conversion() {
        let error = LarkAPIError::illegal_param(String::from("Bad parameter"));

        match error {
            LarkAPIError::IllegalParamError(msg) => {
                assert_eq!(msg, "Bad parameter");
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_error_severity_hash() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(ErrorSeverity::Info, "info");
        map.insert(ErrorSeverity::Warning, "warning");
        map.insert(ErrorSeverity::Error, "error");
        map.insert(ErrorSeverity::Critical, "critical");

        assert_eq!(map.get(&ErrorSeverity::Info), Some(&"info"));
        assert_eq!(map.get(&ErrorSeverity::Critical), Some(&"critical"));
    }
}
