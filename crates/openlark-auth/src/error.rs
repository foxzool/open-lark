//! 认证服务错误处理模块
//!
//! 完全基于 CoreError 的现代化错误处理系统
//! 直接集成统一错误体系，提供类型安全和可观测性

use openlark_core::error::{
    CoreError, ErrorCode, ErrorContext, ErrorTrait, ErrorType,
    authentication_error, token_expired_error, token_invalid_error, permission_missing_error,
    user_identity_invalid_error, sso_token_invalid_error, network_error_with_details,
    validation_error,
};
use std::time::Duration;

// 导入内部结构体
use openlark_core::error::ApiError;

/// 认证服务错误类型 - 推荐使用 CoreError（CoreError为兼容性别名）
pub type AuthError = CoreError;

/// 认证服务结果类型
pub type AuthResult<T> = Result<T, AuthError>;

/// 认证错误构建器 - 专门用于认证场景的便利函数
#[derive(Debug, Copy, Clone)]
pub struct AuthErrorBuilder;

impl AuthErrorBuilder {
    /// 令牌已过期 - 可自动刷新
    pub fn token_expired(detail: impl Into<String>) -> AuthError {
        token_expired_error(detail)
    }

    /// 令牌无效 - 需要重新认证
    pub fn token_invalid(detail: impl Into<String>) -> AuthError {
        token_invalid_error(detail)
    }

    /// 缺少访问令牌
    pub fn token_missing() -> AuthError {
        authentication_error("缺少访问令牌")
    }

    /// 刷新令牌无效 - 需要完整登录流程
    pub fn refresh_token_invalid(detail: impl Into<String>) -> AuthError {
        authentication_error(format!("刷新令牌无效: {}", detail.into()))
    }

    /// 权限不足 - 缺少必要的权限范围
    pub fn permission_denied(missing_scopes: &[impl AsRef<str>]) -> AuthError {
        permission_missing_error(missing_scopes)
    }

    /// 权限不足 - 详细的权限对比信息
    pub fn scope_insufficient(
        required: &[impl AsRef<str>],
        current: &[impl AsRef<str>],
    ) -> AuthError {
        let mut ctx = ErrorContext::new();
        ctx.add_context(
            "required_scopes",
            required
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>()
                .join(","),
        );
        ctx.add_context(
            "current_scopes",
            current
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>()
                .join(","),
        );

        CoreError::Authentication {
            message: "权限范围不足".to_string(),
            code: ErrorCode::PermissionMissing,
            ctx,
        }
    }

    /// 应用凭证无效
    pub fn credentials_invalid(reason: impl Into<String>) -> AuthError {
        authentication_error(format!("应用凭证无效: {}", reason.into()))
    }

    /// 应用凭证错误 - 具体字段错误
    pub fn app_credentials_error(
        field: impl Into<String>,
        value: impl Into<String>,
        reason: impl Into<String>,
    ) -> AuthError {
        let mut ctx = ErrorContext::new();
        ctx.add_context("credential_field", field.into());
        ctx.add_context("provided_value", value.into());
        ctx.add_context("error_reason", reason.into());

        CoreError::Authentication {
            message: "应用凭证错误".to_string(),
            code: ErrorCode::AuthenticationFailed,
            ctx,
        }
    }

    /// 应用不存在或未安装
    pub fn app_not_found(app_id: impl AsRef<str>) -> AuthError {
        let mut ctx = ErrorContext::new();
        ctx.add_context("app_id", app_id.as_ref());

        CoreError::Api(ApiError {
            status: 400,
            endpoint: "auth".into(),
            message: format!("应用不存在或未安装: {}", app_id.as_ref()),
            source: None,
            code: ErrorCode::AppNotInstalled,
            ctx,
        })
    }

    /// 用户未授权应用访问权限
    pub fn user_not_authorized(user_id: impl AsRef<str>) -> AuthError {
        let mut ctx = ErrorContext::new();
        ctx.add_context("user_id", user_id.as_ref());

        CoreError::Authentication {
            message: "用户未授权应用访问权限".to_string(),
            code: ErrorCode::UserSessionInvalid,
            ctx,
        }
    }

    /// 请求频率超限
    pub fn rate_limited(limit: u32, window: Duration, retry_after: Option<Duration>) -> AuthError {
        let mut ctx = ErrorContext::new();
        ctx.add_context("limit", limit.to_string());
        ctx.add_context("window_seconds", window.as_secs().to_string());

        CoreError::RateLimit {
            limit,
            window,
            reset_after: retry_after,
            code: ErrorCode::TooManyRequests,
            ctx,
        }
    }

    /// 参数验证失败
    pub fn validation_error(
        field: impl Into<String>,
        message: impl Into<String>,
        value: Option<impl Into<String>>,
    ) -> AuthError {
        let mut ctx = ErrorContext::new();
        if let Some(val) = value {
            ctx.add_context("provided_value", val.into());
        }

        CoreError::Validation {
            field: field.into().into(),
            message: message.into(),
            code: ErrorCode::ValidationError,
            ctx,
        }
    }

    /// 配置错误
    pub fn configuration_error(
        parameter: impl Into<String>,
        message: impl Into<String>,
    ) -> AuthError {
        let mut ctx = ErrorContext::new();
        ctx.add_context("config_parameter", parameter.into());

        CoreError::Configuration {
            message: message.into(),
            code: ErrorCode::ConfigurationError,
            ctx,
        }
    }

    /// 网络连接错误
    pub fn network_error(
        message: impl Into<String>,
        endpoint: Option<impl Into<String>>,
    ) -> AuthError {
        network_error_with_details(message, None::<String>, endpoint.map(|e| e.into()))
    }

    /// 认证服务不可用
    pub fn auth_service_unavailable(
        service: impl Into<String>,
        retry_after: Option<Duration>,
    ) -> AuthError {
        CoreError::ServiceUnavailable {
            service: service.into().into(),
            retry_after,
            code: ErrorCode::ServiceUnavailable,
            ctx: ErrorContext::new(),
        }
    }

    /// 用户身份无效
    pub fn user_identity_invalid(
        id_type: impl Into<String>,
        id_value: impl Into<String>,
    ) -> AuthError {
        user_identity_invalid_error(format!("{}:{}", id_type.into(), id_value.into()))
    }

    /// SSO 令牌无效
    pub fn sso_token_invalid(detail: impl Into<String>) -> AuthError {
        sso_token_invalid_error(detail)
    }

    /// 应用状态异常
    pub fn app_status_exception(app_id: impl AsRef<str>, status: impl Into<String>) -> AuthError {
        let status_str = status.into();
        let mut ctx = ErrorContext::new();
        ctx.add_context("app_id", app_id.as_ref());
        ctx.add_context("app_status", status_str.clone());

        CoreError::Api(ApiError {
            status: 400,
            endpoint: "auth".into(),
            message: format!("应用状态异常: {}", status_str),
            source: None,
            code: ErrorCode::AppStatusException,
            ctx,
        })
    }
}

/// 飞书认证错误码智能映射
pub fn map_feishu_auth_error(
    feishu_code: i32,
    message: &str,
    request_id: Option<&str>,
) -> AuthError {
    let mut ctx = ErrorContext::new();
    if let Some(req_id) = request_id {
        ctx.set_request_id(req_id);
    }
    ctx.add_context("feishu_code", feishu_code.to_string());

    // 优先使用飞书通用错误码映射
    match ErrorCode::from_feishu_code(feishu_code) {
        // 令牌相关错误
        Some(ErrorCode::AccessTokenExpiredV2) => CoreError::Authentication {
            message: message.to_string(),
            code: ErrorCode::AccessTokenExpiredV2,
            ctx,
        },
        Some(
            ErrorCode::AccessTokenInvalid
            | ErrorCode::AppAccessTokenInvalid
            | ErrorCode::TenantAccessTokenInvalid,
        ) => CoreError::Authentication {
            message: message.to_string(),
            code: ErrorCode::AccessTokenInvalid,
            ctx,
        },
        Some(ErrorCode::SsoTokenInvalid) => CoreError::Authentication {
            message: message.to_string(),
            code: ErrorCode::SsoTokenInvalid,
            ctx,
        },

        // 权限相关错误
        Some(ErrorCode::PermissionMissing | ErrorCode::AccessTokenNoPermission) => {
            CoreError::Authentication {
                message: "权限不足".to_string(),
                code: ErrorCode::PermissionMissing,
                ctx,
            }
        }

        // 用户身份相关错误
        Some(
            ErrorCode::UserSessionInvalid
            | ErrorCode::UserSessionNotFound
            | ErrorCode::UserSessionTimeout,
        ) => CoreError::Authentication {
            message: "用户会话无效".to_string(),
            code: ErrorCode::UserSessionInvalid,
            ctx,
        },
        Some(ErrorCode::UserIdentityInvalid) => CoreError::Authentication {
            message: message.to_string(),
            code: ErrorCode::UserIdentityInvalid,
            ctx,
        },
        Some(ErrorCode::UserTypeNotSupportedV2 | ErrorCode::UserIdentityMismatch) => {
            CoreError::Authentication {
                message: "用户身份不匹配或类型不支持".to_string(),
                code: ErrorCode::UserIdentityInvalid,
                ctx,
            }
        }

        // 应用相关错误
        Some(ErrorCode::AppNotInstalled) => CoreError::Api(ApiError {
            status: 400,
            endpoint: "auth".into(),
            message: message.to_string(),
            source: None,
            code: ErrorCode::AppNotInstalled,
            ctx,
        }),
        Some(ErrorCode::AppPermissionDenied) => CoreError::Authentication {
            message: "应用权限不足".to_string(),
            code: ErrorCode::PermissionMissing,
            ctx,
        },
        Some(ErrorCode::AppStatusException) => CoreError::Api(ApiError {
            status: 400,
            endpoint: "auth".into(),
            message: message.to_string(),
            source: None,
            code: ErrorCode::AppStatusException,
            ctx,
        }),

        // 其他飞书错误码
        Some(mapped_code) => {
            // 根据 HTTP 状态码分类处理
            match mapped_code {
                ErrorCode::BadRequest => validation_error("", message),
                ErrorCode::Unauthorized | ErrorCode::Forbidden => CoreError::Authentication {
                    message: message.to_string(),
                    code: ErrorCode::AuthenticationFailed,
                    ctx,
                },
                ErrorCode::TooManyRequests => CoreError::RateLimit {
                    limit: 0,
                    window: Duration::from_secs(60),
                    reset_after: Some(Duration::from_secs(60)),
                    code: mapped_code,
                    ctx,
                },
                ErrorCode::InternalServerError
                | ErrorCode::BadGateway
                | ErrorCode::GatewayTimeout
                | ErrorCode::ServiceUnavailable => CoreError::ServiceUnavailable {
                    service: "auth".into(),
                    retry_after: Some(Duration::from_secs(30)),
                    code: mapped_code,
                    ctx,
                },
                _ => CoreError::Api(ApiError {
                    status: 400,
                    endpoint: "auth".into(),
                    message: message.to_string(),
                    source: None,
                    code: mapped_code,
                    ctx,
                }),
            }
        }

        // 未知错误码，根据数值范围推断类型
        None => {
            match feishu_code {
                // HTTP 状态码范围
                400..=499 => match feishu_code {
                    401 => CoreError::Authentication {
                        message: "认证失败".to_string(),
                        code: ErrorCode::AuthenticationFailed,
                        ctx,
                    },
                    403 => CoreError::Authentication {
                        message: "权限不足".to_string(),
                        code: ErrorCode::PermissionMissing,
                        ctx,
                    },
                    429 => CoreError::RateLimit {
                        limit: 0,
                        window: Duration::from_secs(60),
                        reset_after: Some(Duration::from_secs(60)),
                        code: ErrorCode::TooManyRequests,
                        ctx,
                    },
                    _ => validation_error(
                        "unknown",
                        format!("请求错误 ({}): {}", feishu_code, message),
                    ),
                },
                500..=599 => CoreError::ServiceUnavailable {
                    service: "auth".into(),
                    retry_after: Some(Duration::from_secs(30)),
                    code: ErrorCode::ServiceUnavailable,
                    ctx,
                },
                // 其他错误码
                _ => CoreError::Api(ApiError {
                    status: 500,
                    endpoint: "auth".into(),
                    message: format!("未知认证错误 ({}): {}", feishu_code, message),
                    source: None,
                    code: ErrorCode::InternalError,
                    ctx,
                }),
            }
        }
    }
}

/// 认证错误扩展特征 - 提供认证特定的错误分析能力
pub trait AuthErrorExt {
    /// 判断是否为令牌相关错误
    fn is_token_error(&self) -> bool;
    /// 判断是否为权限相关错误
    fn is_permission_error(&self) -> bool;
    /// 判断是否为凭证相关错误
    fn is_credential_error(&self) -> bool;
    /// 判断是否为用户身份相关错误
    fn is_user_identity_error(&self) -> bool;
    /// 判断是否为应用相关错误
    fn is_application_error(&self) -> bool;

    /// 判断是否应该自动刷新令牌
    fn should_refresh_token(&self) -> bool;
    /// 判断是否需要用户重新认证
    fn requires_user_reauth(&self) -> bool;
    /// 判断是否需要用户干预
    fn requires_user_action(&self) -> bool;

    /// 获取认证错误的具体类型
    fn auth_error_type(&self) -> AuthErrorType;
    /// 获取建议的恢复操作
    fn recovery_actions(&self) -> Vec<RecoveryAction>;
    /// 获取用于监控的错误指标
    fn auth_metrics(&self) -> AuthMetrics;
}

/// 认证错误类型分类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthErrorType {
    /// 令牌已过期
    TokenExpired,
    /// 令牌无效
    TokenInvalid,
    /// 权限不足
    PermissionDenied,
    /// 凭证无效
    CredentialInvalid,
    /// 用户身份问题
    UserIdentityInvalid,
    /// 应用问题
    ApplicationError,
    /// 配置错误
    ConfigurationError,
    /// 网络问题
    NetworkError,
    /// 服务不可用
    ServiceUnavailable,
    /// 请求限流
    RateLimited,
    /// 参数验证错误
    ValidationError,
    /// 未知错误
    Unknown,
}

/// 恢复操作建议
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryAction {
    /// 自动刷新令牌
    RefreshToken,
    /// 用户重新认证
    ReAuthenticate,
    /// 检查权限配置
    CheckPermissions,
    /// 检查应用配置
    CheckAppConfig,
    /// 等待后重试
    RetryLater(Duration),
    /// 联系管理员
    ContactAdmin,
    /// 检查网络连接
    CheckNetwork,
    /// 修正输入参数
    FixInput,
}

/// 认证错误监控指标
#[derive(Debug, Clone)]
pub struct AuthMetrics {
    /// 错误代码
    pub error_code: ErrorCode,
    /// 错误类型
    pub error_type: AuthErrorType,
    /// 是否可重试
    pub retryable: bool,
    /// 错误严重程度
    pub severity: String,
    /// 请求追踪ID
    pub request_id: Option<String>,
    /// 操作名称
    pub operation: Option<String>,
    /// 飞书原始错误码
    pub feishu_code: Option<i32>,
}

impl AuthErrorExt for AuthError {
    fn is_token_error(&self) -> bool {
        matches!(
            self.code(),
            ErrorCode::AccessTokenExpiredV2
                | ErrorCode::AccessTokenInvalid
                | ErrorCode::AccessTokenFormatInvalid
                | ErrorCode::TenantAccessTokenInvalid
                | ErrorCode::AppAccessTokenInvalid
        )
    }

    fn is_permission_error(&self) -> bool {
        matches!(
            self.code(),
            ErrorCode::PermissionMissing
                | ErrorCode::AccessTokenNoPermission
                | ErrorCode::AppPermissionDenied
                | ErrorCode::Forbidden
        )
    }

    fn is_credential_error(&self) -> bool {
        matches!(
            self.code(),
            ErrorCode::AuthenticationFailed
                | ErrorCode::InvalidSignature
                | ErrorCode::AppTicketInvalid
        )
    }

    fn is_user_identity_error(&self) -> bool {
        matches!(
            self.code(),
            ErrorCode::UserSessionInvalid
                | ErrorCode::UserSessionNotFound
                | ErrorCode::UserSessionTimeout
                | ErrorCode::UserIdentityInvalid
                | ErrorCode::UserIdInvalid
                | ErrorCode::OpenIdInvalid
                | ErrorCode::UnionIdInvalid
                | ErrorCode::UserTypeNotSupportedV2
                | ErrorCode::UserIdentityMismatch
        )
    }

    fn is_application_error(&self) -> bool {
        matches!(
            self.code(),
            ErrorCode::AppNotInstalled | ErrorCode::AppStatusException
        )
    }

    fn should_refresh_token(&self) -> bool {
        matches!(self.code(), ErrorCode::AccessTokenExpiredV2) && !self.is_credential_error()
    }

    fn requires_user_reauth(&self) -> bool {
        self.is_token_error() && !self.should_refresh_token()
            || self.is_user_identity_error()
            || self.is_permission_error()
    }

    fn requires_user_action(&self) -> bool {
        self.is_credential_error()
            || self.is_permission_error()
            || self.is_application_error()
            || self.error_type() == ErrorType::Validation
    }

    fn auth_error_type(&self) -> AuthErrorType {
        match self.code() {
            ErrorCode::AccessTokenExpiredV2 => AuthErrorType::TokenExpired,
            ErrorCode::AccessTokenInvalid
            | ErrorCode::TenantAccessTokenInvalid
            | ErrorCode::AppAccessTokenInvalid
            | ErrorCode::AccessTokenFormatInvalid => AuthErrorType::TokenInvalid,
            ErrorCode::PermissionMissing
            | ErrorCode::AccessTokenNoPermission
            | ErrorCode::AppPermissionDenied
            | ErrorCode::Forbidden => AuthErrorType::PermissionDenied,
            ErrorCode::AuthenticationFailed
            | ErrorCode::InvalidSignature
            | ErrorCode::AppTicketInvalid => AuthErrorType::CredentialInvalid,
            ErrorCode::UserSessionInvalid
            | ErrorCode::UserSessionNotFound
            | ErrorCode::UserSessionTimeout
            | ErrorCode::UserIdentityInvalid
            | ErrorCode::SsoTokenInvalid => AuthErrorType::UserIdentityInvalid,
            ErrorCode::AppNotInstalled | ErrorCode::AppStatusException => {
                AuthErrorType::ApplicationError
            }
            ErrorCode::ConfigurationError => AuthErrorType::ConfigurationError,
            ErrorCode::TooManyRequests => AuthErrorType::RateLimited,
            ErrorCode::ValidationError | ErrorCode::MissingRequiredParameter => {
                AuthErrorType::ValidationError
            }
            ErrorCode::ServiceUnavailable
            | ErrorCode::BadGateway
            | ErrorCode::GatewayTimeout
            | ErrorCode::InternalServerError => AuthErrorType::ServiceUnavailable,
            _ => match self {
                CoreError::Network { .. } => AuthErrorType::NetworkError,
                CoreError::Configuration { .. } => AuthErrorType::ConfigurationError,
                CoreError::ServiceUnavailable { .. } => AuthErrorType::ServiceUnavailable,
                CoreError::RateLimit { .. } => AuthErrorType::RateLimited,
                CoreError::Validation { .. } => AuthErrorType::ValidationError,
                _ => AuthErrorType::Unknown,
            },
        }
    }

    fn recovery_actions(&self) -> Vec<RecoveryAction> {
        match self.auth_error_type() {
            AuthErrorType::TokenExpired => {
                vec![RecoveryAction::RefreshToken]
            }
            AuthErrorType::TokenInvalid | AuthErrorType::UserIdentityInvalid => {
                vec![RecoveryAction::ReAuthenticate]
            }
            AuthErrorType::PermissionDenied => {
                vec![
                    RecoveryAction::CheckPermissions,
                    RecoveryAction::ContactAdmin,
                ]
            }
            AuthErrorType::CredentialInvalid => {
                vec![RecoveryAction::CheckAppConfig]
            }
            AuthErrorType::ApplicationError => {
                vec![RecoveryAction::CheckAppConfig, RecoveryAction::ContactAdmin]
            }
            AuthErrorType::RateLimited => {
                let retry_delay = self.retry_delay(0).unwrap_or(Duration::from_secs(60));
                vec![RecoveryAction::RetryLater(retry_delay)]
            }
            AuthErrorType::NetworkError | AuthErrorType::ServiceUnavailable => {
                vec![
                    RecoveryAction::CheckNetwork,
                    RecoveryAction::RetryLater(Duration::from_secs(5)),
                ]
            }
            AuthErrorType::ValidationError => {
                vec![RecoveryAction::FixInput]
            }
            AuthErrorType::ConfigurationError => {
                vec![RecoveryAction::CheckAppConfig]
            }
            AuthErrorType::Unknown => {
                vec![
                    RecoveryAction::RetryLater(Duration::from_secs(10)),
                    RecoveryAction::ContactAdmin,
                ]
            }
        }
    }

    fn auth_metrics(&self) -> AuthMetrics {
        AuthMetrics {
            error_code: self.code(),
            error_type: self.auth_error_type(),
            retryable: self.is_retryable(),
            severity: format!("{:?}", self.severity()),
            request_id: self.context().request_id().map(|s| s.to_string()),
            operation: self.context().operation().map(|s| s.to_string()),
            feishu_code: self
                .context()
                .get_context("feishu_code")
                .and_then(|s| s.parse().ok()),
        }
    }
}

/// 认证错误分析器 - 提供高级错误分析功能
#[derive(Debug, Copy, Clone)]
pub struct AuthErrorAnalyzer;

impl AuthErrorAnalyzer {
    /// 分析单个认证错误
    pub fn analyze(error: &AuthError) -> AuthAnalysisReport {
        AuthAnalysisReport {
            error_type: error.auth_error_type(),
            is_token_related: error.is_token_error(),
            is_permission_related: error.is_permission_error(),
            is_credential_related: error.is_credential_error(),
            is_user_identity_related: error.is_user_identity_error(),
            requires_reauth: error.requires_user_reauth(),
            should_refresh: error.should_refresh_token(),
            recovery_actions: error.recovery_actions(),
            metrics: error.auth_metrics(),
        }
    }

    /// 分析批量认证错误，检测模式
    pub fn analyze_batch(errors: &[AuthError]) -> AuthBatchAnalysis {
        let mut report = AuthBatchAnalysis {
            total_errors: errors.len(),
            error_types: std::collections::HashMap::new(),
            token_errors: 0,
            permission_errors: 0,
            credential_errors: 0,
            retryable_errors: 0,
            user_action_required: 0,
            most_common_error: None,
        };

        for error in errors {
            let error_type = error.auth_error_type();
            *report.error_types.entry(error_type.clone()).or_insert(0) += 1;

            if error.is_token_error() {
                report.token_errors += 1;
            }
            if error.is_permission_error() {
                report.permission_errors += 1;
            }
            if error.is_credential_error() {
                report.credential_errors += 1;
            }
            if error.is_retryable() {
                report.retryable_errors += 1;
            }
            if error.requires_user_action() {
                report.user_action_required += 1;
            }
        }

        report.most_common_error = report
            .error_types
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(error_type, count)| (error_type.clone(), *count));

        report
    }

    /// 检测潜在的安全问题
    pub fn detect_security_patterns(errors: &[AuthError]) -> Option<SecurityPattern> {
        let credential_errors = errors.iter().filter(|e| e.is_credential_error()).count();

        let permission_errors = errors.iter().filter(|e| e.is_permission_error()).count();

        let user_identity_errors = errors.iter().filter(|e| e.is_user_identity_error()).count();

        // 检测攻击模式
        if credential_errors > errors.len() / 2 {
            Some(SecurityPattern::CredentialStuffing {
                attempts: credential_errors,
                time_window: Duration::from_secs(300), // 5分钟窗口
            })
        } else if permission_errors > errors.len() / 3 {
            Some(SecurityPattern::PrivilegeEscalation {
                attempts: permission_errors,
                affected_resources: permission_errors,
            })
        } else if user_identity_errors > 10 {
            Some(SecurityPattern::IdentityAttacks {
                attempts: user_identity_errors,
            })
        } else {
            None
        }
    }
}

/// 单个认证错误分析报告
#[derive(Debug, Clone)]
pub struct AuthAnalysisReport {
    pub error_type: AuthErrorType,
    pub is_token_related: bool,
    pub is_permission_related: bool,
    pub is_credential_related: bool,
    pub is_user_identity_related: bool,
    pub requires_reauth: bool,
    pub should_refresh: bool,
    pub recovery_actions: Vec<RecoveryAction>,
    pub metrics: AuthMetrics,
}

/// 批量认证错误分析报告
#[derive(Debug, Clone)]
pub struct AuthBatchAnalysis {
    pub total_errors: usize,
    pub error_types: std::collections::HashMap<AuthErrorType, usize>,
    pub token_errors: usize,
    pub permission_errors: usize,
    pub credential_errors: usize,
    pub retryable_errors: usize,
    pub user_action_required: usize,
    pub most_common_error: Option<(AuthErrorType, usize)>,
}

/// 安全攻击模式检测结果
#[derive(Debug, Clone)]
pub enum SecurityPattern {
    /// 凭证填充攻击
    CredentialStuffing {
        attempts: usize,
        time_window: Duration,
    },
    /// 权限提升攻击
    PrivilegeEscalation {
        attempts: usize,
        affected_resources: usize,
    },
    /// 身份攻击
    IdentityAttacks { attempts: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_error_creation() {
        let error = AuthErrorBuilder::token_expired("测试令牌过期");
        assert!(error.is_token_error());
        assert!(error.should_refresh_token());
        assert!(!error.requires_user_reauth());
        assert_eq!(error.auth_error_type(), AuthErrorType::TokenExpired);
    }

    #[test]
    fn test_permission_error_creation() {
        let missing_scopes = ["contacts:readonly", "messages:write"];
        let error = AuthErrorBuilder::permission_denied(&missing_scopes);

        assert!(error.is_permission_error());
        assert!(error.requires_user_action());
        assert_eq!(error.auth_error_type(), AuthErrorType::PermissionDenied);
    }

    #[test]
    fn test_feishu_error_mapping() {
        let error = map_feishu_auth_error(99991677, "令牌已过期", Some("req-123"));

        assert!(error.is_token_error());
        assert!(error.should_refresh_token());
        assert_eq!(error.context().request_id(), Some("req-123"));
    }

    #[test]
    fn test_auth_error_analysis() {
        let error = AuthErrorBuilder::credentials_invalid("APP_SECRET错误");
        let analysis = AuthErrorAnalyzer::analyze(&error);

        assert!(analysis.is_credential_related);
        assert!(analysis.requires_reauth);
        assert!(!analysis.should_refresh);
    }

    #[test]
    fn test_security_pattern_detection() {
        let errors = vec![
            AuthErrorBuilder::credentials_invalid("错误1"),
            AuthErrorBuilder::credentials_invalid("错误2"),
            AuthErrorBuilder::permission_denied(&["scope1"]),
        ];

        let pattern = AuthErrorAnalyzer::detect_security_patterns(&errors);
        assert!(matches!(
            pattern,
            Some(SecurityPattern::CredentialStuffing { .. })
        ));
    }

    #[test]
    fn test_recovery_actions() {
        let token_expired = AuthErrorBuilder::token_expired("过期");
        let actions = token_expired.recovery_actions();
        assert!(actions.contains(&RecoveryAction::RefreshToken));

        let permission_denied = AuthErrorBuilder::permission_denied(&["scope1"]);
        let actions = permission_denied.recovery_actions();
        assert!(actions.contains(&RecoveryAction::CheckPermissions));
    }

    #[test]
    fn test_auth_metrics() {
        let error = map_feishu_auth_error(99991677, "令牌过期", Some("req-456"));
        let metrics = error.auth_metrics();

        assert_eq!(metrics.request_id, Some("req-456".to_string()));
        assert_eq!(metrics.feishu_code, Some(99991677));
        assert!(metrics.retryable);
        assert_eq!(metrics.error_type, AuthErrorType::TokenExpired);
    }
}
