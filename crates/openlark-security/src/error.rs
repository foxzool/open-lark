//! 安全认证服务错误处理模块

use openlark_core::error::prelude::*;
use thiserror::Error;

/// 安全认证服务专用错误类型
#[derive(Debug, Error)]
pub enum SecurityError {
    /// 核心错误 - 来自openlark-core
    #[error("Core error: {0}")]
    Core(#[from] LarkAPIError),

    /// 认证失败
    #[error("Authentication failed: {reason}")]
    AuthenticationFailed { reason: String, code: Option<i32> },

    /// 授权失败
    #[error("Authorization failed: {permission}")]
    AuthorizationFailed {
        permission: String,
        resource: String,
    },

    /// 令牌错误
    #[error("Token error: {message}")]
    TokenError {
        message: String,
        token_type: Option<TokenType>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    },

    /// 权限不足
    #[error("Insufficient permissions: {required_permission}")]
    InsufficientPermissions {
        required_permission: String,
        current_permissions: Vec<String>,
    },

    /// 会话过期
    #[error("Session expired: {session_id}")]
    SessionExpired {
        session_id: String,
        expires_at: chrono::DateTime<chrono::Utc>,
    },

    /// 账户被锁定
    #[error("Account locked: {user_id} - {reason}")]
    AccountLocked {
        user_id: String,
        reason: String,
        locked_until: Option<chrono::DateTime<chrono::Utc>>,
    },

    /// 密码错误
    #[error("Password error: {message}")]
    PasswordError {
        message: String,
        attempts_remaining: Option<u32>,
    },

    /// 多因子认证失败
    #[error("MFA failed: {method} - {reason}")]
    MFAFailed { method: String, reason: String },

    /// 安全策略违规
    #[error("Security policy violation: {policy} - {reason}")]
    SecurityPolicyViolation { policy: String, reason: String },

    /// IP地址被限制
    #[error("IP address restricted: {ip_address}")]
    IPRestricted {
        ip_address: String,
        restriction_type: String,
    },

    /// 设备验证失败
    #[error("Device verification failed: {device_id}")]
    DeviceVerificationFailed { device_id: String, reason: String },

    /// 网络错误
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON序列化错误
    #[error("JSON serialization error: {0}")]
    JsonSerialization(#[from] serde_json::Error),

    /// 请求参数错误
    #[error("Invalid parameter: {parameter} - {reason}")]
    InvalidParameter { parameter: String, reason: String },

    /// 其他错误
    #[error("Other error: {message}")]
    Other { message: String },

    /// 参数缺失
    #[error("Missing required parameter: {parameter}")]
    MissingParameter { parameter: String, message: String },

    /// 认证错误（通用）
    #[error("Authentication error: {message}")]
    AuthenticationError {
        message: String,
        error_code: Option<String>,
    },

    /// 权限拒绝
    #[error("Permission denied: {resource} - {action}")]
    PermissionDenied {
        resource: String,
        action: String,
        message: String,
    },
}

// 重新导出models中的TokenType
pub use super::models::TokenType;

impl SecurityError {
    /// 获取用户友好的错误消息
    pub fn user_friendly_message(&self) -> String {
        match self {
            SecurityError::Core(core_err) => {
                // 使用LarkAPIError的user_friendly_message方法
                core_err.user_friendly_message()
            }

            SecurityError::AuthenticationFailed { reason, .. } => {
                format!("登录失败: {}", reason)
            }

            SecurityError::AuthorizationFailed {
                permission,
                resource,
            } => {
                format!("权限不足，无法访问{}。需要权限: {}", resource, permission)
            }

            SecurityError::TokenError {
                message,
                token_type,
                ..
            } => {
                let type_name = token_type
                    .as_ref()
                    .map_or("令牌".to_string(), |t| format!("{:?}", t));
                format!("{}错误: {}", type_name, message)
            }

            SecurityError::InsufficientPermissions {
                required_permission,
                ..
            } => {
                format!("权限不足，需要权限: {}", required_permission)
            }

            SecurityError::SessionExpired { .. } => "会话已过期，请重新登录".to_string(),

            SecurityError::AccountLocked { reason, .. } => {
                format!("账户已被锁定: {}", reason)
            }

            SecurityError::PasswordError { message, .. } => {
                format!("密码错误: {}", message)
            }

            SecurityError::MFAFailed { method, reason } => {
                format!("多因子认证失败 ({}): {}", method, reason)
            }

            SecurityError::SecurityPolicyViolation { policy, reason } => {
                format!("违反安全策略 ({}) : {}", policy, reason)
            }

            SecurityError::IPRestricted {
                ip_address,
                restriction_type,
            } => {
                format!("IP地址 {} 被限制: {}", ip_address, restriction_type)
            }

            SecurityError::DeviceVerificationFailed { device_id, reason } => {
                format!("设备验证失败 ({}): {}", device_id, reason)
            }

            SecurityError::Network(err) => {
                format!("网络连接失败: {}", err)
            }

            SecurityError::JsonSerialization(err) => {
                format!("数据格式错误: {}", err)
            }

            SecurityError::InvalidParameter { parameter, reason } => {
                format!("参数错误: {} - {}", parameter, reason)
            }

            SecurityError::Other { message } => {
                format!("安全错误: {}", message)
            }

            SecurityError::MissingParameter { parameter, message } => {
                format!("缺少必需参数: {} - {}", parameter, message)
            }

            SecurityError::AuthenticationError { message, .. } => {
                format!("认证失败: {}", message)
            }

            SecurityError::PermissionDenied {
                resource,
                action,
                message,
            } => {
                format!("权限被拒绝: 无法{} {} - {}", action, resource, message)
            }
        }
    }

    /// 检查是否为可重试的错误
    pub fn is_retryable(&self) -> bool {
        match self {
            SecurityError::Core(core_err) => {
                // 使用LarkAPIError的is_retryable方法
                core_err.is_retryable()
            }
            SecurityError::Network(_) => true,
            SecurityError::TokenError { .. } => true,
            SecurityError::SessionExpired { .. } => true,
            _ => false,
        }
    }

    /// 检查是否为认证错误
    pub fn is_auth_error(&self) -> bool {
        match self {
            SecurityError::Core(core_err) => {
                // 使用LarkAPIError的方法检查是否为认证相关错误
                matches!(
                    core_err,
                    LarkAPIError::MissingAccessToken | LarkAPIError::AuthenticationError { .. }
                )
            }
            SecurityError::AuthenticationFailed { .. } => true,
            SecurityError::AuthorizationFailed { .. } => true,
            SecurityError::TokenError { .. } => true,
            SecurityError::SessionExpired { .. } => true,
            SecurityError::AccountLocked { .. } => true,
            SecurityError::PasswordError { .. } => true,
            SecurityError::MissingParameter { .. } => true,
            SecurityError::AuthenticationError { .. } => true,
            SecurityError::PermissionDenied { .. } => true,
            _ => false,
        }
    }

    /// 检查是否为安全策略错误
    pub fn is_security_policy_error(&self) -> bool {
        match self {
            SecurityError::SecurityPolicyViolation { .. } => true,
            SecurityError::IPRestricted { .. } => true,
            SecurityError::DeviceVerificationFailed { .. } => true,
            SecurityError::MFAFailed { .. } => true,
            _ => false,
        }
    }
}

/// 安全认证服务Result类型
pub type SecurityResult<T> = Result<T, SecurityError>;

/// 安全服务错误上下文
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// 请求ID
    pub request_id: Option<String>,
    /// 操作类型
    pub operation: String,
    /// 用户ID
    pub user_id: Option<String>,
    /// 资源ID
    pub resource_id: Option<String>,
    /// IP地址
    pub ip_address: Option<String>,
    /// 设备ID
    pub device_id: Option<String>,
    /// 会话ID
    pub session_id: Option<String>,
    /// 额外的上下文信息
    pub extra: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    /// 创建新的错误上下文
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            request_id: None,
            operation: operation.into(),
            user_id: None,
            resource_id: None,
            ip_address: None,
            device_id: None,
            session_id: None,
            extra: std::collections::HashMap::new(),
        }
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

    /// 设置资源ID
    pub fn with_resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.resource_id = Some(resource_id.into());
        self
    }

    /// 设置IP地址
    pub fn with_ip_address(mut self, ip_address: impl Into<String>) -> Self {
        self.ip_address = Some(ip_address.into());
        self
    }

    /// 设置设备ID
    pub fn with_device_id(mut self, device_id: impl Into<String>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }

    /// 设置会话ID
    pub fn with_session_id(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    /// 添加额外信息
    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }
}
