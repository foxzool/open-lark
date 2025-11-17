//! OpenLark 统一错误处理系统

use std::collections::HashMap;
use std::fmt;

use openlark_core::SDKResult;
use serde::{Serialize, Deserialize};

/// OpenLark 统一错误类型
///
/// 统一客户端中所有错误的标准表示。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnifiedError {
    /// 配置错误
    ConfigurationError(String),

    /// 服务未找到
    ServiceNotFound(String),

    /// 服务已存在
    ServiceAlreadyExists(String),

    /// 服务不可用
    ServiceNotAvailable(String),

    /// 依赖未找到
    DependencyNotFound(String),

    /// 服务有依赖者
    ServiceHasDependents {
        service: String,
        dependents: Vec<String>,
    },

    /// 循环依赖
    CircularDependency(String),

    /// 网络错误
    NetworkError {
        message: String,
        code: Option<i32>,
        retry_count: u32,
    },

    /// API错误
    APIError {
        code: i32,
        message: String,
        request_id: Option<String>,
        details: HashMap<String, String>,
    },

    /// 认证错误
    AuthenticationError(String),

    /// 授权错误
    AuthorizationError(String),

    /// 令牌已过期
    TokenExpired,

    /// 令牌无效
    TokenInvalid,

    /// 请求超时
    Timeout {
        operation: String,
        timeout_ms: u64,
    },

    /// 请求限流
    RateLimit {
        limit: u32,
        window_ms: u64,
        retry_after: Option<u64>,
    },

    /// 数据解析错误
    ParseError {
        field: String,
        value: String,
        expected_type: String,
    },

    /// 验证错误
    ValidationError {
        field: String,
        message: String,
        value: Option<String>,
    },

    /// 内部错误
    InternalError(String),

    /// 不支持的操作
    UnsupportedOperation {
        operation: String,
        reason: Option<String>,
    },

    /// 功能未实现
    NotImplemented(String),

    /// 资源未找到
    ResourceNotFound {
        resource_type: String,
        resource_id: String,
    },

    /// 资源冲突
    ResourceConflict {
        resource_type: String,
        resource_id: String,
        conflict_type: String,
    },

    /// 配额超限
    QuotaExceeded {
        quota_type: String,
        current_usage: u64,
        limit: u64,
    },

    /// 中间件错误
    MiddlewareError {
        middleware: String,
        stage: String,
        message: String,
    },

    /// 包装其他错误
    WrappedError {
        error_type: String,
        message: String,
        source: Option<Box<UnifiedError>>,
    },
}

impl UnifiedError {
    /// 检查错误是否可重试
    pub fn is_retryable(&self) -> bool {
        match self {
            UnifiedError::NetworkError { .. } => true,
            UnifiedError::Timeout { .. } => true,
            UnifiedError::RateLimit { retry_after: Some(_), .. } => true,
            UnifiedError::ServiceNotAvailable(_) => true,
            UnifiedError::APIError { code, .. } => {
                // 某些API错误码可重试
                matches!(code, 429 | 500 | 502 | 503 | 504 | 507 | 509)
            }
            _ => false,
        }
    }

    /// 获取建议的重试延迟（毫秒）
    pub fn suggested_retry_delay(&self) -> Option<u64> {
        match self {
            UnifiedError::RateLimit { retry_after, .. } => *retry_after,
            UnifiedError::Timeout { timeout_ms, .. } => Some(*timeout_ms * 2),
            UnifiedError::NetworkError { retry_count, .. } => {
                // 指数退避：1s, 2s, 4s, 8s, 16s...
                Some(1000 * 2u64.pow(retry_count.min(5)))
            }
            UnifiedError::APIError { code, .. } => {
                match code {
                    429 => Some(60_000), // 1分钟
                    500 | 502 | 503 | 504 => Some(30_000), // 30秒
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// 检查错误是否需要重新认证
    pub fn requires_reauthentication(&self) -> bool {
        match self {
            UnifiedError::TokenExpired | UnifiedError::TokenInvalid => true,
            UnifiedError::AuthenticationError(_) => true,
            UnifiedError::APIError { code, .. } => {
                matches!(code, 401 | 403)
            }
            _ => false,
        }
    }

    /// 获取用户友好的错误消息
    pub fn user_friendly_message(&self) -> String {
        match self {
            UnifiedError::ConfigurationError(msg) => {
                format!("配置错误: {}", msg)
            }
            UnifiedError::ServiceNotFound(service) => {
                format!("服务未找到: {}", service)
            }
            UnifiedError::ServiceNotAvailable(service) => {
                format!("服务暂时不可用: {}", service)
            }
            UnifiedError::NetworkError { message, .. } => {
                format!("网络连接失败: {}", message)
            }
            UnifiedError::AuthenticationError(msg) => {
                format!("认证失败: {}", msg)
            }
            UnifiedError::TokenExpired => {
                "登录已过期，请重新登录".to_string()
            }
            UnifiedError::Timeout { operation, .. } => {
                format!("操作超时: {}", operation)
            }
            UnifiedError::RateLimit { limit, window_ms, .. } => {
                format!("请求过于频繁，限制：{} 次/{}ms", limit, window_ms)
            }
            UnifiedError::ParseError { field, expected_type, .. } => {
                format!("数据格式错误：{} 字段应为 {}", field, expected_type)
            }
            UnifiedError::ValidationError { field, message, .. } => {
                format!("数据验证失败：{} - {}", field, message)
            }
            UnifiedError::ResourceNotFound { resource_type, resource_id } => {
                format!("{} 未找到: {}", resource_type, resource_id)
            }
            UnifiedError::QuotaExceeded { quota_type, limit, .. } => {
                format!("{} 配额已超限，限制：{}", quota_type, limit)
            }
            UnifiedError::InternalError(msg) => {
                format!("内部错误: {}", msg)
            }
            UnifiedError::UnsupportedOperation { operation, .. } => {
                format!("不支持的操作: {}", operation)
            }
            UnifiedError::NotImplemented(feature) => {
                format!("功能尚未实现: {}", feature)
            }
            _ => {
                "未知错误，请联系技术支持".to_string()
            }
        }
    }

    /// 获取错误代码
    pub fn error_code(&self) -> &'static str {
        match self {
            UnifiedError::ConfigurationError(_) => "CONFIG_ERROR",
            UnifiedError::ServiceNotFound(_) => "SERVICE_NOT_FOUND",
            UnifiedError::ServiceNotAvailable(_) => "SERVICE_NOT_AVAILABLE",
            UnifiedError::NetworkError { .. } => "NETWORK_ERROR",
            UnifiedError::APIError { .. } => "API_ERROR",
            UnifiedError::AuthenticationError(_) => "AUTHENTICATION_ERROR",
            UnifiedError::TokenExpired => "TOKEN_EXPIRED",
            UnifiedError::TokenInvalid => "TOKEN_INVALID",
            UnifiedError::Timeout { .. } => "TIMEOUT",
            UnifiedError::RateLimit { .. } => "RATE_LIMIT",
            UnifiedError::ParseError { .. } => "PARSE_ERROR",
            UnifiedError::ValidationError { .. } => "VALIDATION_ERROR",
            UnifiedError::InternalError(_) => "INTERNAL_ERROR",
            UnifiedError::UnsupportedOperation { .. } => "UNSUPPORTED_OPERATION",
            UnifiedError::NotImplemented(_) => "NOT_IMPLEMENTED",
            UnifiedError::ResourceNotFound { .. } => "RESOURCE_NOT_FOUND",
            UnifiedError::ResourceConflict { .. } => "RESOURCE_CONFLICT",
            UnifiedError::QuotaExceeded { .. } => "QUOTA_EXCEEDED",
            UnifiedError::MiddlewareError { .. } => "MIDDLEWARE_ERROR",
            UnifiedError::WrappedError { .. } => "WRAPPED_ERROR",
            _ => "UNKNOWN_ERROR",
        }
    }

    /// 获取错误详情
    pub fn details(&self) -> HashMap<String, String> {
        let mut details = HashMap::new();

        match self {
            UnifiedError::APIError { code, request_id, details: api_details, .. } => {
                details.insert("code".to_string(), code.to_string());
                if let Some(req_id) = request_id {
                    details.insert("request_id".to_string(), req_id.clone());
                }
                for (k, v) in api_details {
                    details.insert(k.clone(), v.clone());
                }
            }
            UnifiedError::NetworkError { code, retry_count, message, .. } => {
                if let Some(c) = code {
                    details.insert("code".to_string(), c.to_string());
                }
                details.insert("retry_count".to_string(), retry_count.to_string());
                details.insert("message".to_string(), message.clone());
            }
            UnifiedError::RateLimit { limit, window_ms, retry_after } => {
                details.insert("limit".to_string(), limit.to_string());
                details.insert("window_ms".to_string(), window_ms.to_string());
                if let Some(ra) = retry_after {
                    details.insert("retry_after_ms".to_string(), ra.to_string());
                }
            }
            _ => {}
        }

        details
    }
}

impl fmt::Display for UnifiedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.user_friendly_message())
    }
}

impl std::error::Error for UnifiedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            UnifiedError::WrappedError { source, .. } => {
                source.as_ref().map(|e| e as &dyn std::error::Error)
            }
            _ => None,
        }
    }
}

/// 从其他错误类型转换
impl From<openlark_core::error::LarkAPIError> for UnifiedError {
    fn from(core_error: openlark_core::error::LarkAPIError) -> Self {
        match core_error {
            openlark_core::error::LarkAPIError::APIError { code, message, request_id, .. } => {
                UnifiedError::APIError {
                    code,
                    message,
                    request_id,
                    details: HashMap::new(),
                }
            }
            openlark_core::error::LarkAPIError::NetworkError(err) => {
                UnifiedError::NetworkError {
                    message: err.to_string(),
                    code: None,
                    retry_count: 0,
                }
            }
            openlark_core::error::LarkAPIError::AuthenticationError(msg) => {
                UnifiedError::AuthenticationError(msg)
            }
            openlark_core::error::LarkAPIError::TokenExpired => {
                UnifiedError::TokenExpired
            }
            openlark_core::error::LarkAPIError::ConfigurationError(msg) => {
                UnifiedError::ConfigurationError(msg)
            }
            openlark_core::error::LarkAPIError::ValidationError(msg) => {
                UnifiedError::ValidationError {
                    field: "unknown".to_string(),
                    message: msg,
                    value: None,
                }
            }
            _ => {
                UnifiedError::InternalError(format!("Core error: {:?}", core_error))
            }
        }
    }
}

/// 从reqwest错误转换
impl From<reqwest::Error> for UnifiedError {
    fn from(reqwest_error: reqwest::Error) -> Self {
        if reqwest_error.is_timeout() {
            UnifiedError::Timeout {
                operation: "HTTP request".to_string(),
                timeout_ms: 30000, // 默认超时
            }
        } else if reqwest_error.is_connect() {
            UnifiedError::NetworkError {
                message: "连接失败".to_string(),
                code: None,
                retry_count: 0,
            }
        } else if reqwest_error.is_request() {
            UnifiedError::NetworkError {
                message: format!("请求错误: {}", reqwest_error),
                code: None,
                retry_count: 0,
            }
        } else {
            UnifiedError::NetworkError {
                message: reqwest_error.to_string(),
                code: None,
                retry_count: 0,
            }
        }
    }
}

/// 从serde_json错误转换
impl From<serde_json::Error> for UnifiedError {
    fn from(json_error: serde_json::Error) -> Self {
        UnifiedError::ParseError {
            field: "json".to_string(),
            value: json_error.to_string(),
            expected_type: "valid JSON".to_string(),
        }
    }
}

/// 从IO错误转换
impl From<std::io::Error> for UnifiedError {
    fn from(io_error: std::io::Error) -> Self {
        match io_error.kind() {
            std::io::ErrorKind::Timeout => {
                UnifiedError::Timeout {
                    operation: "IO operation".to_string(),
                    timeout_ms: 0,
                }
            }
            std::io::ErrorKind::ConnectionRefused => {
                UnifiedError::NetworkError {
                    message: "连接被拒绝".to_string(),
                    code: None,
                    retry_count: 0,
                }
            }
            _ => {
                UnifiedError::InternalError(format!("IO错误: {}", io_error))
            }
        }
    }
}

/// 统一结果类型
pub type UnifiedResult<T> = Result<T, UnifiedError>;

/// 将SDKResult转换为UnifiedResult
pub fn convert_sdk_result<T>(result: SDKResult<T>) -> UnifiedResult<T> {
    result.map_err(|e| e.into())
}

/// 错误上下文
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    /// 错误发生时的操作
    pub operation: String,
    /// 服务名称
    pub service: Option<String>,
    /// 请求ID
    pub request_id: Option<String>,
    /// 用户ID
    pub user_id: Option<String>,
    /// 时间戳
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// 额外上下文信息
    pub extra: HashMap<String, String>,
}

impl ErrorContext {
    /// 创建新的错误上下文
    pub fn new(operation: &str) -> Self {
        Self {
            operation: operation.to_string(),
            service: None,
            request_id: None,
            user_id: None,
            timestamp: chrono::Utc::now(),
            extra: HashMap::new(),
        }
    }

    /// 设置服务名称
    pub fn with_service(mut self, service: impl Into<String>) -> Self {
        self.service = Some(service.into());
        self
    }

    /// 设置请求ID
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    /// 设置用户ID
    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// 添加额外信息
    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }
}

/// 带上下文的错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualError {
    /// 基础错误
    pub error: UnifiedError,
    /// 错误上下文
    pub context: ErrorContext,
}

impl ContextualError {
    /// 创建带上下文的错误
    pub fn new(error: UnifiedError, context: ErrorContext) -> Self {
        Self { error, context }
    }

    /// 创建带基础上下文的错误
    pub fn with_context(error: UnifiedError, operation: &str) -> Self {
        Self::new(error, ErrorContext::new(operation))
    }
}

impl fmt::Display for ContextualError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.context.operation, self.error)
    }
}

impl std::error::Error for ContextualError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

/// 错误报告器
pub struct ErrorReporter {
    enabled: bool,
    endpoint: Option<String>,
}

impl ErrorReporter {
    /// 创建新的错误报告器
    pub fn new() -> Self {
        Self {
            enabled: false,
            endpoint: None,
        }
    }

    /// 启用错误报告
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// 设置报告端点
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    /// 报告错误
    pub async fn report(&self, error: &ContextualError) {
        if !self.enabled {
            return;
        }

        if let Some(endpoint) = &self.endpoint {
            // 这里应该实现错误上报逻辑
            // 例如发送到监控系统或日志服务
            tracing::error!("Error reported to {}: {}", endpoint, error);
        }
    }
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_retryable() {
        let network_error = UnifiedError::NetworkError {
            message: "Connection failed".to_string(),
            code: None,
            retry_count: 1,
        };
        assert!(network_error.is_retryable());

        let auth_error = UnifiedError::AuthenticationError("Invalid credentials".to_string());
        assert!(!auth_error.is_retryable());
    }

    #[test]
    fn test_retry_delay() {
        let rate_limit_error = UnifiedError::RateLimit {
            limit: 100,
            window_ms: 60000,
            retry_after: Some(5000),
        };
        assert_eq!(rate_limit_error.suggested_retry_delay(), Some(5000));
    }

    #[test]
    fn test_reauthentication_required() {
        let token_expired = UnifiedError::TokenExpired;
        assert!(token_expired.requires_reauthentication());

        let network_error = UnifiedError::NetworkError {
            message: "Connection failed".to_string(),
            code: None,
            retry_count: 0,
        };
        assert!(!network_error.requires_reauthentication());
    }

    #[test]
    fn test_error_context() {
        let context = ErrorContext::new("test_operation")
            .with_service("test_service")
            .with_request_id("req_123");

        assert_eq!(context.operation, "test_operation");
        assert_eq!(context.service, Some("test_service".to_string()));
        assert_eq!(context.request_id, Some("req_123".to_string()));
    }

    #[test]
    fn test_contextual_error() {
        let base_error = UnifiedError::NetworkError {
            message: "Connection failed".to_string(),
            code: None,
            retry_count: 0,
        };
        let contextual_error = ContextualError::with_context(base_error, "test_operation");

        assert_eq!(contextual_error.context.operation, "test_operation");
        assert!(contextual_error.to_string().contains("test_operation"));
    }
}