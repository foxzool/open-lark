//! CoreError：基于 thiserror 的企业级错误处理
//!
//! 目标：展示单一映射来源（ErrorCode），简化字段、统一严重度/可重试策略，
//! 并保留可序列化的 ErrorRecord 供观测与分析使用。

use std::{borrow::Cow, time::Duration};

use super::{
    codes::ErrorCode,
    context::ErrorContext,
    traits::{ErrorSeverity, ErrorTrait, ErrorType},
    RetryPolicy,
};
use serde::Serialize;
use thiserror::Error;

type AnyError = Box<dyn std::error::Error + Send + Sync>;

/// 枚举化的构建器目标类型
#[derive(Debug, Clone, Copy)]
pub enum BuilderKind {
    Network,
    Authentication,
    Api,
    Validation,
    Configuration,
    Serialization,
    Business,
    Timeout,
    RateLimit,
    ServiceUnavailable,
    Internal,
}

/// 统一的错误构建器，避免直接依赖枚举内部字段
#[derive(Debug)]
pub struct ErrorBuilder {
    kind: BuilderKind,
    message: Option<String>,
    code: Option<ErrorCode>,
    status: Option<u16>,
    endpoint: Option<String>,
    field: Option<String>,
    source: Option<AnyError>,
    policy: Option<RetryPolicy>,
    ctx: ErrorContext,
    duration: Option<Duration>,
    operation: Option<String>,
    limit: Option<u32>,
    window: Option<Duration>,
    reset_after: Option<Duration>,
    service: Option<String>,
    retry_after: Option<Duration>,
}

impl ErrorBuilder {
    pub fn new(kind: BuilderKind) -> Self {
        Self {
            kind,
            message: None,
            code: None,
            status: None,
            endpoint: None,
            field: None,
            source: None,
            policy: None,
            ctx: ErrorContext::new(),
            duration: None,
            operation: None,
            limit: None,
            window: None,
            reset_after: None,
            service: None,
            retry_after: None,
        }
    }

    pub fn message(mut self, msg: impl Into<String>) -> Self {
        self.message = Some(msg.into());
        self
    }

    pub fn code(mut self, code: ErrorCode) -> Self {
        self.code = Some(code);
        self
    }

    pub fn status(mut self, status: u16) -> Self {
        self.status = Some(status);
        self
    }

    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    pub fn field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }

    pub fn source<E: std::error::Error + Send + Sync + 'static>(mut self, err: E) -> Self {
        self.source = Some(Box::new(err));
        self
    }

    pub fn policy(mut self, policy: RetryPolicy) -> Self {
        self.policy = Some(policy);
        self
    }

    pub fn request_id(mut self, req: impl Into<String>) -> Self {
        self.ctx.set_request_id(req);
        self
    }

    pub fn operation(mut self, op: impl Into<String>) -> Self {
        self.operation = Some(op.into());
        self.ctx.set_operation(self.operation.clone().unwrap());
        self
    }

    pub fn component(mut self, comp: impl Into<String>) -> Self {
        self.ctx.set_component(comp);
        self
    }

    pub fn user_message(mut self, msg: impl Into<String>) -> Self {
        self.ctx.set_user_message(msg);
        self
    }

    pub fn context(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.ctx.add_context(key, val);
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn window(mut self, window: Duration) -> Self {
        self.window = Some(window);
        self
    }

    pub fn reset_after(mut self, reset: Duration) -> Self {
        self.reset_after = Some(reset);
        self
    }

    pub fn service(mut self, service: impl Into<String>) -> Self {
        self.service = Some(service.into());
        self
    }

    pub fn retry_after(mut self, retry_after: Duration) -> Self {
        self.retry_after = Some(retry_after);
        self
    }

    pub fn build(self) -> CoreError {
        let msg = self.message.unwrap_or_else(|| "unknown error".to_string());
        match self.kind {
            BuilderKind::Network => CoreError::Network(NetworkError {
                message: msg,
                source: self.source,
                policy: self.policy.unwrap_or_default(),
                ctx: self.ctx,
            }),
            BuilderKind::Authentication => CoreError::Authentication {
                message: msg,
                code: self.code.unwrap_or(ErrorCode::AuthenticationFailed),
                ctx: self.ctx,
            },
            BuilderKind::Api => {
                let status = self.status.unwrap_or(500);
                CoreError::Api(ApiError {
                    status,
                    endpoint: self
                        .endpoint
                        .unwrap_or_else(|| "unknown".to_string())
                        .into(),
                    message: msg,
                    source: self.source,
                    code: self
                        .code
                        .unwrap_or_else(|| ErrorCode::from_http_status(status)),
                    ctx: self.ctx,
                })
            }
            BuilderKind::Validation => CoreError::Validation {
                field: self.field.unwrap_or_else(|| "field".to_string()).into(),
                message: msg,
                code: self.code.unwrap_or(ErrorCode::ValidationError),
                ctx: self.ctx,
            },
            BuilderKind::Configuration => CoreError::Configuration {
                message: msg,
                code: self.code.unwrap_or(ErrorCode::ConfigurationError),
                ctx: self.ctx,
            },
            BuilderKind::Serialization => CoreError::Serialization {
                message: msg,
                source: self.source,
                code: self.code.unwrap_or(ErrorCode::SerializationError),
                ctx: self.ctx,
            },
            BuilderKind::Business => CoreError::Business {
                code: self.code.unwrap_or(ErrorCode::BusinessError),
                message: msg,
                ctx: self.ctx,
            },
            BuilderKind::Timeout => CoreError::Timeout {
                duration: self.duration.unwrap_or_default(),
                operation: self.operation,
                ctx: self.ctx,
            },
            BuilderKind::RateLimit => CoreError::RateLimit {
                limit: self.limit.unwrap_or(0),
                window: self.window.unwrap_or(Duration::from_secs(1)),
                reset_after: self.reset_after,
                code: self.code.unwrap_or(ErrorCode::RateLimitExceeded),
                ctx: self.ctx,
            },
            BuilderKind::ServiceUnavailable => CoreError::ServiceUnavailable {
                service: self.service.unwrap_or_else(|| "service".to_string()).into(),
                retry_after: self.retry_after,
                code: self.code.unwrap_or(ErrorCode::ServiceUnavailable),
                ctx: self.ctx,
            },
            BuilderKind::Internal => CoreError::Internal {
                code: self.code.unwrap_or(ErrorCode::InternalError),
                message: msg,
                source: self.source,
                ctx: self.ctx,
            },
        }
    }
}

/// 轻量版核心错误
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("网络错误: {0}")]
    Network(NetworkError),

    #[error("认证失败: {message}")]
    Authentication {
        message: String,
        code: ErrorCode,
        ctx: ErrorContext,
    },

    #[error("API错误 {0}")]
    Api(ApiError),

    #[error("验证错误 {field}: {message}")]
    Validation {
        field: Cow<'static, str>,
        message: String,
        code: ErrorCode,
        ctx: ErrorContext,
    },

    #[error("配置错误: {message}")]
    Configuration {
        message: String,
        code: ErrorCode,
        ctx: ErrorContext,
    },

    #[error("序列化错误: {message}")]
    Serialization {
        message: String,
        #[source]
        source: Option<AnyError>,
        code: ErrorCode,
        ctx: ErrorContext,
    },

    #[error("业务错误 {code:?}: {message}")]
    Business {
        code: ErrorCode,
        message: String,
        ctx: ErrorContext,
    },

    #[error("超时 {operation:?} after {duration:?}")]
    Timeout {
        duration: Duration,
        operation: Option<String>,
        ctx: ErrorContext,
    },

    #[error("限流: {limit} 次/{window:?}")]
    RateLimit {
        limit: u32,
        window: Duration,
        reset_after: Option<Duration>,
        code: ErrorCode,
        ctx: ErrorContext,
    },

    #[error("服务不可用: {service}")]
    ServiceUnavailable {
        service: Cow<'static, str>,
        retry_after: Option<Duration>,
        code: ErrorCode,
        ctx: ErrorContext,
    },

    #[error("内部错误 {code:?}: {message}")]
    Internal {
        code: ErrorCode,
        message: String,
        #[source]
        source: Option<AnyError>,
        ctx: ErrorContext,
    },
}

#[derive(Debug)]
pub struct NetworkError {
    pub message: String,
    pub source: Option<AnyError>,
    pub policy: RetryPolicy,
    pub ctx: ErrorContext,
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug)]
pub struct ApiError {
    pub status: u16,
    pub endpoint: Cow<'static, str>,
    pub message: String,
    pub source: Option<AnyError>,
    pub code: ErrorCode,
    pub ctx: ErrorContext,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}: {}", self.status, self.endpoint, self.message)
    }
}

impl Clone for CoreError {
    fn clone(&self) -> Self {
        match self {
            Self::Network(net) => Self::Network(NetworkError {
                message: net.message.clone(),
                source: None, // 源错误不可克隆，丢弃以便重试记录
                policy: net.policy.clone(),
                ctx: net.ctx.clone(),
            }),
            Self::Authentication { message, code, ctx } => Self::Authentication {
                message: message.clone(),
                code: *code,
                ctx: ctx.clone(),
            },
            Self::Api(api) => Self::Api(ApiError {
                status: api.status,
                endpoint: api.endpoint.clone(),
                message: api.message.clone(),
                source: None,
                code: api.code,
                ctx: api.ctx.clone(),
            }),
            Self::Validation {
                field,
                message,
                code,
                ctx,
            } => Self::Validation {
                field: field.clone(),
                message: message.clone(),
                code: *code,
                ctx: ctx.clone(),
            },
            Self::Configuration { message, code, ctx } => Self::Configuration {
                message: message.clone(),
                code: *code,
                ctx: ctx.clone(),
            },
            Self::Serialization {
                message, code, ctx, ..
            } => Self::Serialization {
                message: message.clone(),
                source: None,
                code: *code,
                ctx: ctx.clone(),
            },
            Self::Business { code, message, ctx } => Self::Business {
                code: *code,
                message: message.clone(),
                ctx: ctx.clone(),
            },
            Self::Timeout {
                duration,
                operation,
                ctx,
            } => Self::Timeout {
                duration: *duration,
                operation: operation.clone(),
                ctx: ctx.clone(),
            },
            Self::RateLimit {
                limit,
                window,
                reset_after,
                code,
                ctx,
            } => Self::RateLimit {
                limit: *limit,
                window: *window,
                reset_after: *reset_after,
                code: *code,
                ctx: ctx.clone(),
            },
            Self::ServiceUnavailable {
                service,
                retry_after,
                code,
                ctx,
            } => Self::ServiceUnavailable {
                service: service.clone(),
                retry_after: *retry_after,
                code: *code,
                ctx: ctx.clone(),
            },
            Self::Internal {
                code, message, ctx, ..
            } => Self::Internal {
                code: *code,
                message: message.clone(),
                source: None,
                ctx: ctx.clone(),
            },
        }
    }
}

impl CoreError {
    /// 统一构建器入口
    pub fn builder(kind: BuilderKind) -> ErrorBuilder {
        ErrorBuilder::new(kind)
    }

    pub fn network_builder() -> ErrorBuilder {
        ErrorBuilder::new(BuilderKind::Network)
    }

    pub fn api_builder() -> ErrorBuilder {
        ErrorBuilder::new(BuilderKind::Api)
    }

    pub fn validation_builder() -> ErrorBuilder {
        ErrorBuilder::new(BuilderKind::Validation)
    }

    pub fn authentication_builder() -> ErrorBuilder {
        ErrorBuilder::new(BuilderKind::Authentication)
    }

    pub fn business_builder() -> ErrorBuilder {
        ErrorBuilder::new(BuilderKind::Business)
    }

    /// 简单网络错误（无 source）
    pub fn network_msg(message: impl Into<String>) -> Self {
        network_error(message)
    }

    /// 简单认证错误
    pub fn authentication(message: impl Into<String>) -> Self {
        authentication_error(message)
    }

    /// 简单 API 错误（便于兼容旧 CoreError::api_error）
    pub fn api_error(
        status: i32,
        endpoint: impl Into<String>,
        message: impl Into<String>,
        request_id: Option<impl Into<String>>,
    ) -> Self {
        api_error(status as u16, endpoint, message, request_id.map(|id| id.into()))
    }

    /// 仅带 message 的验证错误（默认字段 general）
    pub fn validation_msg(message: impl Into<String>) -> Self {
        validation_error("general", message)
    }

    /// 用户可读 message（兼容旧 API）
    pub fn message(&self) -> String {
        self.to_string()
    }

    /// 错误类型别名（兼容旧 kind）
    pub fn kind(&self) -> ErrorType {
        self.error_type()
    }

    /// 直接判断 API 错误（便捷别名）
    pub fn is_api_error(&self) -> bool {
        matches!(self, Self::Api(_))
    }

    // === 兼容旧 CoreError 的工厂 ===
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        let mut ctx = ErrorContext::new();
        let field = field.into();
        ctx.add_context("field", field.clone());
        Self::Validation {
            field: field.into(),
            message: message.into(),
            code: ErrorCode::ValidationError,
            ctx,
        }
    }

    pub fn api_data_error(message: impl Into<String>) -> Self {
        Self::Api(ApiError {
            status: 500,
            endpoint: "data_error".into(),
            message: format!("no data: {}", message.into()),
            source: None,
            code: ErrorCode::InternalServerError,
            ctx: ErrorContext::new(),
        })
    }

    pub fn code(&self) -> ErrorCode {
        match self {
            Self::Network(_) => ErrorCode::NetworkConnectionFailed,
            Self::Authentication { code, .. } => *code,
            Self::Api(api) => api.code,
            Self::Validation { code, .. } => *code,
            Self::Configuration { code, .. } => *code,
            Self::Serialization { code, .. } => *code,
            Self::Business { code, .. } => *code,
            Self::Timeout { .. } => ErrorCode::NetworkTimeout,
            Self::RateLimit { code, .. } => *code,
            Self::ServiceUnavailable { code, .. } => *code,
            Self::Internal { code, .. } => *code,
        }
    }

    pub fn severity(&self) -> ErrorSeverity {
        self.code().severity()
    }

    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Network(net) => net.policy.is_retryable(),
            Self::Api(api) => matches!(api.status, 429 | 500..=599),
            Self::Timeout { .. } => true,
            Self::RateLimit { .. } => true,
            Self::ServiceUnavailable { .. } => true,
            _ => false,
        }
    }

    pub fn retry_delay(&self, attempt: u32) -> Option<Duration> {
        match self {
            Self::Network(net) => net.policy.retry_delay(attempt),
            Self::RateLimit { window, .. } => Some(*window),
            Self::ServiceUnavailable { retry_after, .. } => *retry_after,
            Self::Api(api) if matches!(api.status, 429 | 500..=599) => {
                Some(Duration::from_secs(1 << attempt.min(5)))
            }
            _ => None,
        }
    }

    pub fn ctx(&self) -> &ErrorContext {
        match self {
            Self::Network(net) => &net.ctx,
            Self::Authentication { ctx, .. }
            | Self::Api(ApiError { ctx, .. })
            | Self::Validation { ctx, .. }
            | Self::Configuration { ctx, .. }
            | Self::Serialization { ctx, .. }
            | Self::Business { ctx, .. }
            | Self::Timeout { ctx, .. }
            | Self::RateLimit { ctx, .. }
            | Self::ServiceUnavailable { ctx, .. }
            | Self::Internal { ctx, .. } => ctx,
        }
    }

    /// 观测记录（可序列化）——供日志/指标/告警统一使用
    pub fn record(&self) -> ErrorRecord {
        ErrorRecord::from(self)
    }

    // === 快速构造器（示例） ===
    pub fn network<E>(source: E, ctx: ErrorContext) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Network(NetworkError {
            message: "网络连接失败".to_string(),
            source: Some(Box::new(source)),
            policy: RetryPolicy::default(),
            ctx,
        })
    }

    pub fn api(
        status: u16,
        endpoint: impl Into<Cow<'static, str>>,
        message: impl Into<String>,
        ctx: ErrorContext,
    ) -> Self {
        Self::Api(ApiError {
            status,
            endpoint: endpoint.into(),
            message: message.into(),
            source: None,
            code: ErrorCode::from_http_status(status),
            ctx,
        })
    }
}

/// 统一的可观测记录
#[derive(Debug, Serialize)]
#[serde_with::skip_serializing_none]
pub struct ErrorRecord {
    pub code: ErrorCode,
    pub severity: ErrorSeverity,
    pub retryable: bool,
    pub retry_delay_ms: Option<u64>,
    pub message: String,
    pub context: std::collections::HashMap<String, String>,
    pub request_id: Option<String>,
    pub operation: Option<String>,
    pub component: Option<String>,
    pub backtrace: Option<String>,
}

impl From<&CoreError> for ErrorRecord {
    fn from(err: &CoreError) -> Self {
        let ctx = err.ctx();
        Self {
            code: err.code(),
            severity: err.severity(),
            retryable: err.is_retryable(),
            retry_delay_ms: err.retry_delay(0).map(|d| d.as_millis() as u64),
            message: err.to_string(),
            context: ctx.all_context().clone(),
            request_id: ctx.request_id().map(|s| s.to_string()),
            operation: ctx.operation().map(|s| s.to_string()),
            component: ctx.component().map(|s| s.to_string()),
            backtrace: ctx.backtrace().map(|bt| format!("{bt:?}")),
        }
    }
}

impl From<reqwest::Error> for CoreError {
    fn from(source: reqwest::Error) -> Self {
        Self::Network(NetworkError {
            message: source.to_string(),
            source: Some(Box::new(source)),
            policy: RetryPolicy::default(),
            ctx: ErrorContext::new(),
        })
    }
}

impl From<serde_json::Error> for CoreError {
    fn from(source: serde_json::Error) -> Self {
        Self::Serialization {
            message: format!("JSON序列化错误: {}", source),
            source: Some(Box::new(source)),
            code: ErrorCode::SerializationError,
            ctx: ErrorContext::new(),
        }
    }
}

impl ErrorTrait for CoreError {
    fn severity(&self) -> ErrorSeverity {
        self.severity()
    }

    fn is_retryable(&self) -> bool {
        self.is_retryable()
    }

    fn retry_delay(&self, attempt: u32) -> Option<Duration> {
        self.retry_delay(attempt)
    }

    fn user_message(&self) -> Option<&str> {
        match self {
            Self::Network(_) => Some("网络连接异常，请稍后重试"),
            Self::Authentication { .. } => Some("认证失败，请重新登录"),
            Self::Api(api) => Some(api.message.as_str()),
            Self::Validation { message, .. } => Some(message.as_str()),
            Self::Configuration { .. } => Some("配置错误，请检查环境"),
            Self::Serialization { message, .. } => Some(message.as_str()),
            Self::Business { message, .. } => Some(message.as_str()),
            Self::Timeout { .. } => Some("请求超时，请稍后重试"),
            Self::RateLimit { .. } => Some("请求过于频繁，请稍候"),
            Self::ServiceUnavailable { .. } => Some("服务暂不可用，请稍后重试"),
            Self::Internal { message, .. } => Some(message.as_str()),
        }
    }

    fn context(&self) -> &ErrorContext {
        self.ctx()
    }

    fn error_type(&self) -> ErrorType {
        match self {
            Self::Network(_) => ErrorType::Network,
            Self::Authentication { .. } => ErrorType::Authentication,
            Self::Api(_) => ErrorType::Api,
            Self::Validation { .. } => ErrorType::Validation,
            Self::Configuration { .. } => ErrorType::Configuration,
            Self::Serialization { .. } => ErrorType::Serialization,
            Self::Business { .. } => ErrorType::Business,
            Self::Timeout { .. } => ErrorType::Timeout,
            Self::RateLimit { .. } => ErrorType::RateLimit,
            Self::ServiceUnavailable { .. } => ErrorType::ServiceUnavailable,
            Self::Internal { .. } => ErrorType::Internal,
        }
    }

    fn error_code(&self) -> Option<&str> {
        None
    }
}

// ============================================================================
// 便利函数（保持向后兼容）
// ============================================================================

/// 创建网络错误
pub fn network_error(message: impl Into<String>) -> CoreError {
    CoreError::Network(NetworkError {
        message: message.into(),
        source: None,
        policy: RetryPolicy::default(),
        ctx: ErrorContext::new(),
    })
}

/// 创建认证错误
pub fn authentication_error(message: impl Into<String>) -> CoreError {
    CoreError::Authentication {
        message: message.into(),
        code: ErrorCode::AuthenticationFailed,
        ctx: ErrorContext::new(),
    }
}

/// 创建API错误
pub fn api_error(
    status: u16,
    endpoint: impl Into<String>,
    message: impl Into<String>,
    request_id: Option<String>,
) -> CoreError {
    CoreError::Api(ApiError {
        status,
        endpoint: endpoint.into().into(),
        message: message.into(),
        source: None,
        code: ErrorCode::from_http_status(status),
        ctx: {
            let mut ctx = ErrorContext::new();
            if let Some(req_id) = request_id {
                ctx.set_request_id(req_id);
            }
            ctx
        },
    })
}

/// 创建验证错误
pub fn validation_error(field: impl Into<String>, message: impl Into<String>) -> CoreError {
    CoreError::Validation {
        field: field.into().into(),
        message: message.into(),
        code: ErrorCode::ValidationError,
        ctx: ErrorContext::new(),
    }
}

/// 创建序列化错误
pub fn serialization_error<T: std::error::Error + Send + Sync + 'static>(
    message: impl Into<String>,
    source: Option<T>,
) -> CoreError {
    CoreError::Serialization {
        message: message.into(),
        source: source.map(|e| Box::new(e) as AnyError),
        code: ErrorCode::SerializationError,
        ctx: ErrorContext::new(),
    }
}

/// 创建业务错误
pub fn business_error(message: impl Into<String>) -> CoreError {
    CoreError::Business {
        message: message.into(),
        code: ErrorCode::BusinessError,
        ctx: ErrorContext::new(),
    }
}

/// 创建配置错误
pub fn configuration_error(message: impl Into<String>) -> CoreError {
    CoreError::Configuration {
        message: message.into(),
        code: ErrorCode::ConfigurationError,
        ctx: ErrorContext::new(),
    }
}

/// 创建超时错误
pub fn timeout_error(timeout: Duration, operation: Option<String>) -> CoreError {
    CoreError::Timeout {
        duration: timeout,
        operation,
        ctx: ErrorContext::new(),
    }
}

/// 创建限流错误
pub fn rate_limit_error(
    limit: u32,
    window: Duration,
    retry_after: Option<Duration>,
) -> CoreError {
    CoreError::RateLimit {
        limit,
        window,
        reset_after: retry_after,
        code: ErrorCode::TooManyRequests,
        ctx: ErrorContext::new(),
    }
}

/// 创建服务不可用错误
pub fn service_unavailable_error(
    service: impl Into<String>,
    retry_after: Option<Duration>,
) -> CoreError {
    CoreError::ServiceUnavailable {
        service: service.into().into(),
        retry_after,
        code: ErrorCode::ServiceUnavailable,
        ctx: ErrorContext::new(),
    }
}

/// 创建权限缺失错误
pub fn permission_missing_error(scopes: &[impl AsRef<str>]) -> CoreError {
    let mut ctx = ErrorContext::new();
    ctx.add_context("required_scopes", scopes.iter().map(|s| s.as_ref()).collect::<Vec<_>>().join(","));

    CoreError::Authentication {
        message: "权限范围不足".to_string(),
        code: ErrorCode::PermissionMissing,
        ctx,
    }
}

/// 创建SSO令牌无效错误
pub fn sso_token_invalid_error(detail: impl Into<String>) -> CoreError {
    let mut ctx = ErrorContext::new();
    ctx.add_context("detail", detail.into());

    CoreError::Authentication {
        message: "SSO令牌无效".to_string(),
        code: ErrorCode::SsoTokenInvalid,
        ctx,
    }
}

/// 创建用户身份无效错误
pub fn user_identity_invalid_error(desc: impl Into<String>) -> CoreError {
    let mut ctx = ErrorContext::new();
    ctx.add_context("description", desc.into());

    CoreError::Authentication {
        message: "用户身份无效".to_string(),
        code: ErrorCode::UserIdentityInvalid,
        ctx,
    }
}

/// 创建访问令牌无效错误
pub fn token_invalid_error(detail: impl Into<String>) -> CoreError {
    let mut ctx = ErrorContext::new();
    ctx.add_context("detail", detail.into());

    CoreError::Authentication {
        message: "访问令牌无效".to_string(),
        code: ErrorCode::AccessTokenInvalid,
        ctx,
    }
}

/// 创建访问令牌过期错误
pub fn token_expired_error(detail: impl Into<String>) -> CoreError {
    let mut ctx = ErrorContext::new();
    ctx.add_context("detail", detail.into());

    CoreError::Authentication {
        message: "访问令牌过期".to_string(),
        code: ErrorCode::AccessTokenExpiredV2,
        ctx,
    }
}

/// 创建带详细信息的网络错误
pub fn network_error_with_details(
    message: impl Into<String>,
    endpoint: Option<String>,
    request_id: Option<String>,
) -> CoreError {
    let mut ctx = ErrorContext::new();
    if let Some(ep) = endpoint {
        ctx.add_context("endpoint", ep);
    }
    if let Some(req_id) = request_id {
        ctx.set_request_id(req_id);
    }

    CoreError::Network(NetworkError {
        message: message.into(),
        source: None,
        policy: RetryPolicy::default(),
        ctx,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn api_error_has_code_and_severity() {
        let err = CoreError::api(503, "/ping", "service down", ErrorContext::new());

        assert_eq!(err.code(), ErrorCode::ServiceUnavailable);
        assert!(err.is_retryable());
        assert_eq!(err.severity(), ErrorSeverity::Critical);
        assert!(err.retry_delay(1).is_some());
    }

    #[test]
    fn record_contains_context() {
        let mut ctx = ErrorContext::new();
        ctx.add_context("endpoint", "/user");
        ctx.set_request_id("req-1");

        let err = CoreError::network(std::io::Error::new(std::io::ErrorKind::Other, "boom"), ctx);

        let rec = err.record();
        assert_eq!(rec.code, ErrorCode::NetworkConnectionFailed);
        assert_eq!(rec.context.get("endpoint"), Some(&"/user".to_string()));
        assert_eq!(rec.request_id.as_deref(), Some("req-1"));
    }

    #[test]
    fn core_error_to_record() {
        let err = CoreError::api(503, "/ping", "svc down", ErrorContext::new());
        let rec: ErrorRecord = (&err).into();
        assert_eq!(rec.code, ErrorCode::ServiceUnavailable);
        assert!(rec.retryable);
        assert!(rec.message.contains("API错误"));
    }

    #[test]
    fn builder_creates_api_error_with_context() {
        let err = CoreError::api_builder()
            .status(404)
            .endpoint("/users/1")
            .message("not found")
            .request_id("req-123")
            .build();

        assert!(err.is_api_error());
        assert_eq!(err.context().request_id(), Some("req-123"));
        assert_eq!(err.code(), ErrorCode::NotFound);
    }

    #[test]
    fn rate_limit_retry_delay() {
        let err = CoreError::RateLimit {
            limit: 10,
            window: Duration::from_secs(60),
            reset_after: Some(Duration::from_secs(30)),
            code: ErrorCode::RateLimitExceeded,
            ctx: ErrorContext::new(),
        };

        assert!(err.is_retryable());
        assert_eq!(err.retry_delay(0), Some(Duration::from_secs(60)));
    }

    #[test]
    fn from_reqwest_error() {
        // 无法直接构造 reqwest::Error（构造函数为私有），跳过具体实例化，只验证 From trait 存在
        fn assert_from_reqwest<E: Into<CoreError>>() {}
        assert_from_reqwest::<reqwest::Error>();
    }
}
