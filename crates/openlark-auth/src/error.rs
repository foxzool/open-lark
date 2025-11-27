//! 认证服务错误处理模块
//!
//! 基于 thiserror 最佳实践的现代化错误处理系统
//! 专注于认证场景的特定需求和用户体验

use thiserror::Error;
use openlark_core::error::CoreError;

/// 认证服务专用错误类型
///
/// 设计原则：
/// 1. 基于实际认证场景需求
/// 2. 提供清晰的错误恢复策略
/// 3. 支持自动重试和用户指导
#[derive(Debug, Error)]
pub enum AuthError {
    /// 令牌已过期
    ///
    /// 这种错误通常可以通过自动刷新令牌来解决
    #[error("访问令牌已过期")]
    TokenExpired,

    /// 令牌无效或格式错误
    ///
    /// 需要用户重新进行身份验证
    #[error("访问令牌无效")]
    TokenInvalid,

    /// 刷新令牌无效或过期
    ///
    /// 用户需要完整重新登录流程
    #[error("刷新令牌无效")]
    RefreshTokenInvalid,

    /// 应用凭证无效
    ///
    /// 应用配置错误，需要检查 app_id 和 app_secret
    #[error("应用凭证无效: {reason}")]
    InvalidCredentials {
        /// 具体错误原因
        reason: String,
    },

    /// 权限不足
    ///
    /// 应用缺少必要的权限范围
    #[error("权限不足: 缺少 {required_permissions:?} 权限")]
    PermissionDenied {
        /// 需要的权限列表
        required_permissions: Vec<String>,
    },

    /// 用户未授权应用
    ///
    /// 用户需要在授权页面确认应用访问权限
    #[error("用户未授权应用访问")]
    UserNotAuthorized,

    /// 请求频率超限
    ///
    /// 触发 API 限流，需要等待后重试
    #[error("请求频率过高，请 {wait_seconds}s 后重试")]
    RateLimited {
        /// 建议等待时间（秒）
        wait_seconds: u32,
    },

    /// 参数验证失败
    ///
    /// 输入参数不符合 API 要求
    #[error("参数验证失败: {field} - {message}")]
    Validation {
        /// 验证失败的字段名
        field: String,
        /// 错误描述
        message: String,
    },

    /// 配置错误
    ///
    /// 认证服务配置不正确
    #[error("配置错误: {message}")]
    Configuration {
        /// 错误描述
        message: String,
        /// 相关配置项（可选）
        parameter: Option<String>,
    },

    /// 网络连接错误
    ///
    /// 无法连接到认证服务器
    #[error("网络连接失败: {message}")]
    Network {
        /// 错误描述
        message: String,
        #[source]
        source: Option<reqwest::Error>,
    },

    /// 服务暂时不可用
    ///
    /// 认证服务暂时不可用，可以稍后重试
    #[error("认证服务暂时不可用: {service}")]
    ServiceUnavailable {
        /// 服务名称
        service: String,
        /// 建议重试时间（可选）
        retry_after_seconds: Option<u32>,
    },

    /// 内部系统错误
    ///
    /// 未预期的内部错误
    #[error("内部系统错误: {message}")]
    Internal {
        /// 错误描述
        message: String,
        #[source]
        source: Option<anyhow::Error>,
    },

    /// 核心模块错误
    ///
    /// 来自底层核心模块的错误
    #[error("核心模块错误: {0}")]
    Core(#[from] CoreError),
}

impl AuthError {
    /// 判断错误是否可以自动重试
    ///
    /// 返回 true 表示可以通过重试操作自动恢复
    pub fn is_retryable(&self) -> bool {
        match self {
            // 网络问题通常可以重试
            AuthError::Network { .. } => true,

            // 令牌过期可以自动刷新
            AuthError::TokenExpired => true,

            // 限流错误可以等待后重试
            AuthError::RateLimited { .. } => true,

            // 服务暂时不可用可以重试
            AuthError::ServiceUnavailable { .. } => true,

            // 核心模块中的网络和超时错误可以重试
            AuthError::Core(core) => core.is_retryable(),

            // 其他错误需要用户干预
            _ => false,
        }
    }

    /// 判断错误是否需要用户操作
    ///
    /// 返回 true 表示需要用户进行某些操作才能解决
    pub fn requires_user_action(&self) -> bool {
        match self {
            // 需要重新登录的错误
            AuthError::TokenInvalid | AuthError::RefreshTokenInvalid => true,

            // 需要用户授权的错误
            AuthError::UserNotAuthorized | AuthError::PermissionDenied { .. } => true,

            // 需要修复配置的错误
            AuthError::InvalidCredentials { .. } | AuthError::Configuration { .. } => true,

            // 需要修正输入的错误
            AuthError::Validation { .. } => true,

            // 其他错误通常不需要用户直接操作
            _ => false,
        }
    }

    /// 获取用户友好的错误消息
    ///
    /// 提供易于理解的错误描述和解决建议
    pub fn user_friendly_message(&self) -> String {
        match self {
            AuthError::TokenExpired => {
                "登录状态已过期，正在自动刷新...".to_string()
            }
            AuthError::TokenInvalid => {
                "登录状态无效，请重新登录".to_string()
            }
            AuthError::RefreshTokenInvalid => {
                "刷新令牌已失效，请完整重新登录".to_string()
            }
            AuthError::InvalidCredentials { reason } => {
                format!("应用配置错误: {}，请检查 APP_ID 和 APP_SECRET", reason)
            }
            AuthError::PermissionDenied { required_permissions } => {
                format!(
                    "应用权限不足，需要申请以下权限: {}",
                    required_permissions.join(", ")
                )
            }
            AuthError::UserNotAuthorized => {
                "您尚未授权此应用访问权限，请在授权页面确认".to_string()
            }
            AuthError::RateLimited { wait_seconds } => {
                format!("请求过于频繁，请{}秒后重试", wait_seconds)
            }
            AuthError::Validation { field, message } => {
                format!("输入参数错误: {} - {}", field, message)
            }
            AuthError::Configuration { message, .. } => {
                format!("配置错误: {}", message)
            }
            AuthError::Network { .. } => {
                "网络连接失败，请检查网络设置后重试".to_string()
            }
            AuthError::ServiceUnavailable { service, .. } => {
                format!("{}服务暂时不可用，请稍后重试", service)
            }
            AuthError::Internal { .. } => {
                "系统内部错误，请联系技术支持".to_string()
            }
            AuthError::Core(core) => {
                core.user_friendly_message()
            }
        }
    }

    /// 获取错误恢复建议
    ///
    /// 为开发者或用户提供具体的解决方案
    pub fn recovery_suggestion(&self) -> Option<String> {
        match self {
            AuthError::TokenExpired => {
                Some("系统将自动刷新令牌，无需手动操作".to_string())
            }
            AuthError::TokenInvalid | AuthError::RefreshTokenInvalid => {
                Some("请使用完整登录流程重新获取访问令牌".to_string())
            }
            AuthError::InvalidCredentials { .. } => {
                Some("请检查应用配置中的 APP_ID 和 APP_SECRET 是否正确".to_string())
            }
            AuthError::PermissionDenied { required_permissions } => {
                Some(format!(
                    "请在开放平台申请以下权限: {}",
                    required_permissions.join(", ")
                ))
            }
            AuthError::UserNotAuthorized => {
                Some("请引导用户到授权页面完成应用授权".to_string())
            }
            AuthError::RateLimited { wait_seconds } => {
                Some(format!("请等待{}秒后自动重试，或升级到更高版本获得更高限额", wait_seconds))
            }
            AuthError::Validation { field, .. } => {
                Some(format!("请检查输入的{}参数是否符合API要求", field))
            }
            AuthError::Configuration { parameter, .. } => {
                match parameter {
                    Some(param) => Some(format!("请检查配置项: {}", param)),
                    None => Some("请检查认证服务相关配置".to_string()),
                }
            }
            AuthError::Network { .. } => {
                Some("请检查网络连接，确保可以访问飞书API服务器".to_string())
            }
            AuthError::ServiceUnavailable { retry_after_seconds, .. } => {
                match retry_after_seconds {
                    Some(seconds) => Some(format!("请{}秒后自动重试", seconds)),
                    None => Some("请稍后重试，如问题持续请联系技术支持".to_string()),
                }
            }
            AuthError::Internal { .. } => {
                Some("这是系统内部错误，请记录错误详情并联系技术支持".to_string())
            }
            AuthError::Core(core) => {
                core.recovery_suggestion()
            }
        }
    }

    /// 获取错误代码
    ///
    /// 返回标准化的错误代码，便于程序化处理
    pub fn error_code(&self) -> &'static str {
        match self {
            AuthError::TokenExpired => "AUTH_TOKEN_EXPIRED",
            AuthError::TokenInvalid => "AUTH_TOKEN_INVALID",
            AuthError::RefreshTokenInvalid => "AUTH_REFRESH_TOKEN_INVALID",
            AuthError::InvalidCredentials { .. } => "AUTH_INVALID_CREDENTIALS",
            AuthError::PermissionDenied { .. } => "AUTH_PERMISSION_DENIED",
            AuthError::UserNotAuthorized => "AUTH_USER_NOT_AUTHORIZED",
            AuthError::RateLimited { .. } => "AUTH_RATE_LIMITED",
            AuthError::Validation { .. } => "AUTH_VALIDATION_ERROR",
            AuthError::Configuration { .. } => "AUTH_CONFIGURATION_ERROR",
            AuthError::Network { .. } => "AUTH_NETWORK_ERROR",
            AuthError::ServiceUnavailable { .. } => "AUTH_SERVICE_UNAVAILABLE",
            AuthError::Internal { .. } => "AUTH_INTERNAL_ERROR",
            AuthError::Core(_) => "AUTH_CORE_ERROR",
        }
    }

    /// 创建令牌过期错误
    pub fn token_expired() -> Self {
        Self::TokenExpired
    }

    /// 创建令牌无效错误
    pub fn token_invalid() -> Self {
        Self::TokenInvalid
    }

    /// 创建权限不足错误
    pub fn permission_denied(perms: Vec<String>) -> Self {
        Self::PermissionDenied {
            required_permissions: perms,
        }
    }

    /// 创建参数验证错误
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }

    /// 创建配置错误
    pub fn configuration(message: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
            parameter: None,
        }
    }

    /// 创建网络错误
    pub fn network(message: impl Into<String>, source: Option<reqwest::Error>) -> Self {
        Self::Network {
            message: message.into(),
            source,
        }
    }

    /// 创建限流错误
    pub fn rate_limited(wait_seconds: u32) -> Self {
        Self::RateLimited { wait_seconds }
    }

    /// 创建服务不可用错误
    pub fn service_unavailable(service: impl Into<String>, retry_after_seconds: Option<u32>) -> Self {
        Self::ServiceUnavailable {
            service: service.into(),
            retry_after_seconds,
        }
    }
}

/// 认证服务结果类型
pub type AuthResult<T> = Result<T, AuthError>;

/// 为 CoreError 添加缺失的方法（如果不存在）
pub trait CoreErrorExt {
    fn is_retryable(&self) -> bool;
    fn user_friendly_message(&self) -> String;
    fn recovery_suggestion(&self) -> Option<String>;
}

impl CoreErrorExt for CoreError {
    fn is_retryable(&self) -> bool {
        // 基于 CoreError 的具体实现判断是否可重试
        // 这里需要根据实际的 CoreError 实现来调整
        match self {
            // 假设 CoreError 有相应的变体
            CoreError::Network { .. } => true,
            CoreError::Timeout { .. } => true,
            CoreError::RateLimit { .. } => true,
            CoreError::ServiceUnavailable { .. } => true,
            _ => false,
        }
    }

    fn user_friendly_message(&self) -> String {
        // 基于 CoreError 生成用户友好消息
        match self {
            CoreError::Network { .. } => "网络连接异常，请检查网络设置".to_string(),
            CoreError::Authentication { .. } => "认证失败，请检查登录状态".to_string(),
            CoreError::Api { .. } => "API请求失败，请稍后重试".to_string(),
            CoreError::Validation { .. } => "输入参数有误，请检查后重试".to_string(),
            CoreError::Business { .. } => "业务逻辑错误，请联系技术支持".to_string(),
            CoreError::Internal { .. } => "系统内部错误，请联系技术支持".to_string(),
            // 其他变体...
            _ => "操作失败，请稍后重试".to_string(),
        }
    }

    fn recovery_suggestion(&self) -> Option<String> {
        match self {
            CoreError::Network { .. } => Some("检查网络连接后重试".to_string()),
            CoreError::Authentication { .. } => Some("重新登录可解决此问题".to_string()),
            CoreError::Api { status, .. } if *status >= 500 => Some("服务器异常，请稍后重试".to_string()),
            CoreError::RateLimit { .. } => Some("请求频率过高，请稍后重试".to_string()),
            _ => None,
        }
    }
}