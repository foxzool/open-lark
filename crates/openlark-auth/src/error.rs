//! 错误处理模块
//!
//! 定义认证模块的专用错误类型。

use std::fmt;
use thiserror::Error;

/// 认证模块专用错误类型
#[derive(Debug, Error)]
pub enum AuthError {
    /// 参数错误
    #[error("参数错误: {parameter} - {reason}")]
    InvalidParameter {
        /// 参数名
        parameter: String,
        /// 错误原因
        reason: String,
    },

    /// 令牌相关错误
    #[error("令牌错误: {0}")]
    TokenError(String),

    /// 缓存相关错误
    #[error("缓存错误: {0}")]
    CacheError(String),

    /// 验证相关错误
    #[error("验证错误: {0}")]
    ValidationError(String),

    /// 配置相关错误
    #[error("配置错误: {0}")]
    ConfigError(String),

    /// 网络相关错误
    #[error("网络错误: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// 序列化错误
    #[error("序列化错误: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// 时间相关错误
    #[error("时间错误: {0}")]
    TimeError(String),

    /// 加密相关错误
    #[error("加密错误: {0}")]
    CryptoError(String),

    /// OAuth相关错误
    #[error("OAuth错误: {0}")]
    OAuthError(String),

    /// 权限相关错误
    #[error("权限错误: {0}")]
    PermissionError(String),

    /// 限流错误
    #[error("请求频率过高，请稍后重试")]
    RateLimitError,

    /// 未知错误
    #[error("未知错误: {0}")]
    UnknownError(String),
}

impl AuthError {
    /// 检查错误是否可重试
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            AuthError::NetworkError(_) |
            AuthError::CacheError(_) |
            AuthError::TokenError(_) if self.to_string().contains("temporary")
        )
    }

    /// 获取用户友好的错误消息
    pub fn user_friendly_message(&self) -> String {
        match self {
            AuthError::TokenError(msg) if msg.contains("expired") => {
                "登录已过期，请重新登录".to_string()
            }
            AuthError::NetworkError(_) => "网络连接失败，请检查网络设置".to_string(),
            AuthError::ValidationError(msg) => {
                format!("输入验证失败: {}", msg)
            }
            AuthError::ConfigError(msg) => {
                format!("配置错误: {}", msg)
            }
            _ => {
                format!("操作失败: {}", self)
            }
        }
    }
}

/// 认证模块结果类型
pub type AuthResult<T> = Result<T, AuthError>;
