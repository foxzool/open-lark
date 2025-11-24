//! 安全认证服务预导入模块
//!
//! 重新导出最常用的类型和特征，简化导入路径。

// 重新导出错误处理
pub use crate::error::{ErrorContext, SecurityError, SecurityResult};

// 重新导出数据模型
pub use crate::models::*;

// 重新导出服务特征和实现
pub use crate::service::{
    AuthenticationRequest, AuthenticationResponse, CreateSessionRequest, DefaultSecurityService,
    PermissionCheckRequest, SecurityEventFilters, SecurityService, SecurityServiceConfig, UserInfo,
};

// 重新导出认证服务
pub use crate::auth::{AuthService, DefaultAuthService};

// 重新导出认证服务（从service子模块）
pub use crate::auth::service::{
    ActiveSession, ClientInfo, ClientType, LoginRequest, LoginResponse, LoginStatus, LoginType,
    LogoutRequest, LogoutResponse, TokenRefreshResponse, TokenValidationResponse, UserAuthInfo,
};

// 重新导出设备相关类型（从models模块）
pub use crate::models::{DeviceInfo, DeviceType};

// 重新导出访问控制系统（feature-gated）
#[cfg(feature = "acs")]
pub use crate::acs::{
    AccessControlService, CreatePolicyRequest, DefaultAccessControlService, DeletePolicyResponse,
    GrantPermissionRequest, GrantPermissionResponse, PolicyEvaluationRequest,
    PolicyEvaluationResponse, RevokePermissionRequest, RevokePermissionResponse,
    UpdatePolicyRequest, UserPermission,
};

// 重新导出审计服务 (feature-gated)
#[cfg(feature = "audit")]
pub use crate::audit::{
    AuditLogQuery, AuditLogQueryResponse, AuditService, AuditStatistics, AuditStatisticsFilters,
    DefaultAuditService, ExportFormat, ExportResponse, ExportStatus, SortDirection,
    StatisticsPeriod,
};

// 重新导出令牌管理服务 (feature-gated)
#[cfg(feature = "token")]
pub use crate::token::{
    ActiveToken, CreateTokenRequest, DefaultTokenService, ListTokensResponse, RevokeTokenResponse,
    RevokeUserTokensResponse, TokenFilters, TokenResponse, TokenService, TokenServiceConfig,
    TokenVerificationResponse,
};

// 重新导出版本管理服务 (feature-gated)
#[cfg(any(feature = "v1", feature = "v2", feature = "v3"))]
pub use crate::versions::{
    ApiEndpoint, ApiVersion, ChangeType, ChangelogEntry, CompatibilityCheckRequest,
    CompatibilityCheckResponse, DefaultVersionService, MigrationGuide, SupportLevel, VersionInfo,
    VersionService, VersionStatus,
};

// 常用宏定义
#[macro_export]
macro_rules! security_error {
    (authentication_failed, $reason:expr) => {
        $crate::error::SecurityError::AuthenticationFailed {
            reason: $reason.to_string(),
            code: None,
        }
    };
    (authorization_failed, $permission:expr, $resource:expr) => {
        $crate::error::SecurityError::AuthorizationFailed {
            permission: $permission.to_string(),
            resource: $resource.to_string(),
        }
    };
    (token_error, $message:expr) => {
        $crate::error::SecurityError::TokenError {
            message: $message.to_string(),
            token_type: None,
            expires_at: None,
        }
    };
    (invalid_parameter, $param:expr, $reason:expr) => {
        $crate::error::SecurityError::InvalidParameter {
            parameter: $param.to_string(),
            reason: $reason.to_string(),
        }
    };
}

#[macro_export]
macro_rules! ensure_security_condition {
    ($condition:expr, $error:expr) => {
        if !$condition {
            return Err($error);
        }
    };
}

#[macro_export]
macro_rules! security_log {
    ($level:expr, $($arg:tt)*) => {
        tracing::$level!($($arg)*);
    };
}

// 便捷函数
pub fn create_error_context(operation: &str) -> ErrorContext {
    ErrorContext::new(operation)
}

pub fn create_security_service_from_env() -> SecurityResult<DefaultSecurityService> {
    let config = SecurityServiceConfig {
        app_id: std::env::var("SECURITY_APP_ID").unwrap_or_else(|_| "default_app".to_string()),
        app_secret: std::env::var("SECURITY_APP_SECRET")
            .unwrap_or_else(|_| "default_secret".to_string()),
        base_url: std::env::var("LARK_BASE_URL")
            .unwrap_or_else(|_| "https://open.feishu.cn".to_string()),
        session_timeout: std::env::var("SESSION_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3600),
        token_timeout: std::env::var("TOKEN_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(7200),
        enable_mfa: std::env::var("ENABLE_MFA")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false),
        enable_ip_restriction: std::env::var("ENABLE_IP_RESTRICTION")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false),
        allowed_ips: std::env::var("ALLOWED_IPS")
            .unwrap_or_else(|_| "".to_string())
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect(),
    };

    Ok(DefaultSecurityService::new(config))
}

// 常用结果类型别名
pub type AuthResult<T> = std::result::Result<T, SecurityError>;
pub type TokenResult<T> = std::result::Result<T, SecurityError>;
pub type AcsResult<T> = std::result::Result<T, SecurityError>;
pub type AuditResult<T> = std::result::Result<T, SecurityError>;
pub type VersionResult<T> = std::result::Result<T, SecurityError>;

// 异步特征别名
pub use async_trait::async_trait;

// 时间处理便捷函数
pub fn now_utc() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

pub fn duration_hours(hours: i64) -> chrono::Duration {
    chrono::Duration::hours(hours)
}

pub fn duration_days(days: i64) -> chrono::Duration {
    chrono::Duration::days(days)
}

// JSON处理便捷函数
pub fn to_json_value<T: serde::Serialize>(value: &T) -> serde_json::Value {
    serde_json::to_value(value).unwrap_or(serde_json::Value::Null)
}

pub fn from_json_value<T: for<'de> serde::Deserialize<'de>>(
    value: serde_json::Value,
) -> AuthResult<T> {
    serde_json::from_value(value).map_err(|e| SecurityError::JsonSerialization(e))
}

// UUID生成便捷函数
pub fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn generate_short_uuid() -> String {
    let uuid = uuid::Uuid::new_v4();
    uuid.simple().to_string()
}

// 字符串处理便捷函数
pub fn mask_sensitive_data(data: &str, visible_chars: usize) -> String {
    if data.len() <= visible_chars {
        data.to_string()
    } else {
        format!("{}***", &data[..visible_chars])
    }
}

pub fn mask_email(email: &str) -> String {
    if let Some(at_pos) = email.find('@') {
        let username = &email[..at_pos];
        let domain = &email[at_pos..];

        if username.len() > 2 {
            format!("{}***{}", &username[..2], domain)
        } else {
            format!("***{}", domain)
        }
    } else {
        mask_sensitive_data(email, 2)
    }
}

pub fn mask_phone(phone: &str) -> String {
    if phone.len() > 4 {
        format!("{}***", &phone[..4])
    } else {
        mask_sensitive_data(phone, 2)
    }
}

// 常用验证函数
pub fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

pub fn is_valid_phone(phone: &str) -> bool {
    phone
        .chars()
        .all(|c| c.is_ascii_digit() || c == '+' || c == '-' || c == ' ')
}

pub fn is_strong_password(password: &str) -> bool {
    password.len() >= 8
        && password.chars().any(|c| c.is_ascii_uppercase())
        && password.chars().any(|c| c.is_ascii_lowercase())
        && password.chars().any(|c| c.is_ascii_digit())
        && password
            .chars()
            .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c))
}

// 常用常量
pub const DEFAULT_SESSION_TIMEOUT: u64 = 3600; // 1小时
pub const DEFAULT_TOKEN_TIMEOUT: u64 = 7200; // 2小时
pub const MAX_LOGIN_ATTEMPTS: u32 = 5;
pub const LOCKOUT_DURATION_MINUTES: u64 = 30;

// 安全相关的常量
pub const PASSWORD_MIN_LENGTH: usize = 8;
pub const PASSWORD_MAX_LENGTH: usize = 128;
pub const TOKEN_MIN_LENGTH: usize = 32;
pub const USERNAME_MAX_LENGTH: usize = 64;
pub const EMAIL_MAX_LENGTH: usize = 254;
pub const PHONE_MAX_LENGTH: usize = 20;

// HTTP状态码常量
pub const HTTP_OK: u16 = 200;
pub const HTTP_CREATED: u16 = 201;
pub const HTTP_BAD_REQUEST: u16 = 400;
pub const HTTP_UNAUTHORIZED: u16 = 401;
pub const HTTP_FORBIDDEN: u16 = 403;
pub const HTTP_NOT_FOUND: u16 = 404;
pub const HTTP_CONFLICT: u16 = 409;
pub const HTTP_TOO_MANY_REQUESTS: u16 = 429;
pub const HTTP_INTERNAL_SERVER_ERROR: u16 = 500;
