//! OpenLark Client 错误类型定义
//!
//! 统一的错误处理系统，提供类型安全的错误管理

use std::fmt;
use crate::registry::RegistryError;

/// 🚨 OpenLark 客户端错误类型
///
/// 涵盖所有可能的错误情况，提供详细的错误信息
#[derive(Debug, Clone)]
pub enum Error {
    /// ⚙️ 配置错误
    InvalidConfig(&'static str),

    /// 🌐 网络请求失败
    NetworkError(String),

    /// 📡 API调用失败
    APIError {
        /// 🔢 错误代码
        code: String,
        /// 📝 错误消息
        message: String,
    },

    /// 🔍 解析错误
    ParseError(String),

    /// 🔐 认证错误
    AuthenticationError(String),

    /// 🚫 权限错误
    PermissionError(String),

    /// ⏱️ 超时错误
    TimeoutError,

    /// 📝 未知错误
    Unknown(String),

    /// 🏷️ 服务不可用
    ServiceUnavailable(String),

    /// ⚠️ 无效参数
    InvalidParameter(String),

    /// 🔧 服务注册表错误
    RegistryError(RegistryError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidConfig(msg) => write!(f, "配置错误: {}", msg),
            Error::NetworkError(msg) => write!(f, "网络错误: {}", msg),
            Error::APIError { code, message } => write!(f, "API错误 [{}]: {}", code, message),
            Error::ParseError(msg) => write!(f, "解析错误: {}", msg),
            Error::AuthenticationError(msg) => write!(f, "认证错误: {}", msg),
            Error::PermissionError(msg) => write!(f, "权限错误: {}", msg),
            Error::TimeoutError => write!(f, "请求超时"),
            Error::Unknown(msg) => write!(f, "未知错误: {}", msg),
            Error::ServiceUnavailable(service) => write!(f, "服务不可用: {}", service),
            Error::InvalidParameter(msg) => write!(f, "参数错误: {}", msg),
            Error::RegistryError(err) => write!(f, "注册表错误: {}", err),
        }
    }
}

impl std::error::Error for Error {}

/// 📦 结果类型别名
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// 🔍 判断是否为网络错误
    pub fn is_network_error(&self) -> bool {
        matches!(self, Error::NetworkError(_))
    }

    /// 🔍 判断是否为认证错误
    pub fn is_auth_error(&self) -> bool {
        matches!(self, Error::AuthenticationError(_))
    }

    /// 🔍 判断是否为注册表错误
    pub fn is_registry_error(&self) -> bool {
        matches!(self, Error::RegistryError(_))
    }

    /// 🔍 判断是否为配置错误
    pub fn is_config_error(&self) -> bool {
        matches!(self, Error::InvalidConfig(_))
    }
}

impl From<RegistryError> for Error {
    fn from(err: RegistryError) -> Self {
        Error::RegistryError(err)
    }
}

impl From<crate::registry::feature_flags::FeatureFlagError> for Error {
    fn from(err: crate::registry::feature_flags::FeatureFlagError) -> Self {
        Error::RegistryError(RegistryError::FeatureFlagError(err))
    }
}

impl From<crate::registry::dependency_resolver::DependencyError> for Error {
    fn from(err: crate::registry::dependency_resolver::DependencyError) -> Self {
        Error::RegistryError(RegistryError::DependencyError(err))
    }
}

impl Error {
    /// 🔍 判断是否可重试
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Error::NetworkError(_) | Error::TimeoutError | Error::ServiceUnavailable(_)
        )
    }

    /// 📝 获取用户友好的错误消息
    pub fn user_message(&self) -> String {
        match self {
            Error::InvalidConfig(msg) => format!("配置不正确: {}", msg),
            Error::NetworkError(_) => "网络连接失败，请检查网络设置".to_string(),
            Error::APIError { code, message } => format!("API调用失败 [{}]: {}", code, message),
            Error::ParseError(_) => "数据解析失败，请检查数据格式".to_string(),
            Error::AuthenticationError(_) => "身份验证失败，请检查凭据".to_string(),
            Error::PermissionError(_) => "权限不足，无法执行此操作".to_string(),
            Error::TimeoutError => "请求超时，请稍后重试".to_string(),
            Error::Unknown(_) => "发生未知错误，请联系技术支持".to_string(),
            Error::ServiceUnavailable(service) => format!("{}服务当前不可用，请稍后重试", service),
            Error::InvalidParameter(msg) => format!("参数错误: {}", msg),
            Error::RegistryError(err) => format!("服务注册表错误: {}", err),
        }
    }
}

// 从标准错误类型转换
#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Error::TimeoutError
        } else if err.is_request() {
            Error::NetworkError(format!("请求失败: {}", err))
        } else {
            Error::NetworkError(format!("网络错误: {}", err))
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::ParseError(format!("JSON解析失败: {}", err))
    }
}

impl From<chrono::ParseError> for Error {
    fn from(err: chrono::ParseError) -> Self {
        Error::ParseError(format!("时间解析失败: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let config_error = Error::InvalidConfig("测试配置错误");
        assert_eq!(config_error.to_string(), "配置错误: 测试配置错误");

        let api_error = Error::APIError {
            code: "400".to_string(),
            message: "Bad Request".to_string(),
        };
        assert_eq!(api_error.to_string(), "API错误 [400]: Bad Request");
    }

    #[test]
    fn test_error_classification() {
        let network_error = Error::NetworkError("连接失败".to_string());
        assert!(network_error.is_network_error());
        assert!(!network_error.is_auth_error());
        assert!(network_error.is_retryable());

        let auth_error = Error::AuthenticationError("令牌无效".to_string());
        assert!(auth_error.is_auth_error());
        assert!(!auth_error.is_retryable());

        let config_error = Error::InvalidConfig("配置缺失");
        assert!(config_error.is_config_error());
        assert!(!config_error.is_retryable());
    }

    #[test]
    fn test_user_message() {
        let timeout_error = Error::TimeoutError;
        assert!(timeout_error.user_message().contains("超时"));

        let permission_error = Error::PermissionError("无权限访问".to_string());
        assert!(permission_error.user_message().contains("权限不足"));
    }

    #[test]
    fn test_error_conversions() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let parse_error: Error = json_err.into();
        assert!(matches!(parse_error, Error::ParseError(_)));
    }
}
