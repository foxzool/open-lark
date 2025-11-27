//! 与 CoreErrorV3 对齐的精简便利函数

use crate::error::{
    context::ErrorContext,
    core_v3::{ApiError, CoreErrorV3, NetworkError},
    ErrorCode, RetryPolicy,
};
use std::time::Duration;

pub fn network_error_v3(message: impl Into<String>) -> CoreErrorV3 {
    CoreErrorV3::Network(NetworkError {
        message: message.into(),
        source: None,
        policy: RetryPolicy::default(),
        ctx: ErrorContext::new(),
    })
}

pub fn network_error_with_details_v3(
    message: impl Into<String>,
    request_id: Option<impl Into<String>>,
    endpoint: Option<impl Into<String>>,
) -> CoreErrorV3 {
    let mut ctx = ErrorContext::new();
    if let Some(req) = request_id {
        ctx.set_request_id(req);
    }
    if let Some(ep) = endpoint {
        ctx.add_context("endpoint", ep);
    }
    CoreErrorV3::Network(NetworkError {
        message: message.into(),
        source: None,
        policy: RetryPolicy::default(),
        ctx,
    })
}

pub fn authentication_error_v3(message: impl Into<String>) -> CoreErrorV3 {
    CoreErrorV3::Authentication {
        message: message.into(),
        code: ErrorCode::AuthenticationFailed,
        ctx: ErrorContext::new(),
    }
}

/// 访问令牌无效/格式错误
pub fn token_invalid_error_v3(detail: impl Into<String>) -> CoreErrorV3 {
    CoreErrorV3::Authentication {
        message: detail.into(),
        code: ErrorCode::AccessTokenInvalid,
        ctx: ErrorContext::new(),
    }
}

/// 访问令牌已过期（飞书通用错误码 99991677）
pub fn token_expired_error_v3(detail: impl Into<String>) -> CoreErrorV3 {
    CoreErrorV3::Authentication {
        message: detail.into(),
        code: ErrorCode::AccessTokenExpiredV2,
        ctx: ErrorContext::new(),
    }
}

/// 缺少权限 scope
pub fn permission_missing_error_v3(scopes: &[impl AsRef<str>]) -> CoreErrorV3 {
    let mut ctx = ErrorContext::new();
    if !scopes.is_empty() {
        ctx.add_context(
            "scopes",
            scopes
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>()
                .join(","),
        );
    }
    CoreErrorV3::Authentication {
        message: "缺少所需权限".to_string(),
        code: ErrorCode::PermissionMissing,
        ctx,
    }
}

/// SSO 令牌无效
pub fn sso_token_invalid_error_v3(detail: impl Into<String>) -> CoreErrorV3 {
    CoreErrorV3::Authentication {
        message: detail.into(),
        code: ErrorCode::SsoTokenInvalid,
        ctx: ErrorContext::new(),
    }
}

/// 用户身份标识非法
pub fn user_identity_invalid_error_v3(id_desc: impl Into<String>) -> CoreErrorV3 {
    let mut ctx = ErrorContext::new();
    ctx.add_context("identity", id_desc.into());
    CoreErrorV3::Validation {
        field: "identity".into(),
        message: "用户身份标识非法或不支持".to_string(),
        code: ErrorCode::UserIdentityInvalid,
        ctx,
    }
}

pub fn validation_error_v3(field: impl Into<String>, message: impl Into<String>) -> CoreErrorV3 {
    let mut ctx = ErrorContext::new();
    let field = field.into();
    ctx.add_context("field", field.clone());
    CoreErrorV3::Validation {
        field: field.into(),
        message: message.into(),
        code: ErrorCode::ValidationError,
        ctx,
    }
}

pub fn api_error_v3(
    status: u16,
    endpoint: impl Into<String>,
    message: impl Into<String>,
    request_id: Option<impl Into<String>>,
) -> CoreErrorV3 {
    let mut ctx = ErrorContext::new();
    if let Some(req) = request_id {
        ctx.set_request_id(req);
    }
    CoreErrorV3::Api(ApiError {
        status,
        endpoint: endpoint.into().into(),
        message: message.into(),
        source: None,
        code: ErrorCode::from_http_status(status),
        ctx,
    })
}

pub fn business_error_v3(message: impl Into<String>) -> CoreErrorV3 {
    CoreErrorV3::Business {
        code: ErrorCode::BusinessError,
        message: message.into(),
        ctx: ErrorContext::new(),
    }
}

pub fn configuration_error_v3(message: impl Into<String>) -> CoreErrorV3 {
    CoreErrorV3::Configuration {
        message: message.into(),
        code: ErrorCode::ConfigurationError,
        ctx: ErrorContext::new(),
    }
}

pub fn timeout_error_v3(duration: Duration, operation: Option<String>) -> CoreErrorV3 {
    let mut ctx = ErrorContext::new();
    if let Some(op) = &operation {
        ctx.set_operation(op.clone());
    }
    CoreErrorV3::Timeout {
        duration,
        operation,
        ctx,
    }
}

pub fn rate_limit_error_v3(
    limit: u32,
    window: Duration,
    reset_after: Option<Duration>,
) -> CoreErrorV3 {
    let mut ctx = ErrorContext::new();
    ctx.add_context("limit", limit.to_string());
    ctx.add_context("window_secs", window.as_secs().to_string());
    CoreErrorV3::RateLimit {
        limit,
        window,
        reset_after,
        code: ErrorCode::RateLimitExceeded,
        ctx,
    }
}

pub fn service_unavailable_error_v3(
    service: impl Into<String>,
    retry_after: Option<Duration>,
) -> CoreErrorV3 {
    let mut ctx = ErrorContext::new();
    let svc = service.into();
    ctx.add_context("service", svc.clone());
    CoreErrorV3::ServiceUnavailable {
        service: svc.into(),
        retry_after,
        code: ErrorCode::ServiceUnavailable,
        ctx,
    }
}

pub fn serialization_error_v3(
    message: impl Into<String>,
    source: Option<serde_json::Error>,
) -> CoreErrorV3 {
    CoreErrorV3::Serialization {
        message: message.into(),
        source: source.map(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        code: ErrorCode::SerializationError,
        ctx: ErrorContext::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_error_v3_sets_code_and_retryable() {
        let err = api_error_v3(503, "/health", "down", Some("req-1"));
        assert_eq!(err.code(), ErrorCode::ServiceUnavailable);
        assert!(err.is_retryable());
        assert_eq!(err.ctx().request_id(), Some("req-1"));
    }
}
