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
