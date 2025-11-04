//! ServiceRegistry错误类型定义

use std::fmt;

/// ServiceRegistry错误类型
#[derive(Debug, Clone)]
pub enum ServiceError {
    /// 服务未找到
    ServiceNotFound {
        service_name: String,
    },
    /// 服务已存在
    ServiceAlreadyExists {
        service_name: String,
    },
    /// 类型转换失败
    TypeMismatch {
        expected: String,
        actual: String,
    },
    /// 服务不可用
    ServiceUnavailable {
        service_name: String,
        reason: String,
    },
    /// 注册表已满
    RegistryFull {
        current: usize,
        max: usize,
    },
    /// 无效配置
    InvalidConfiguration {
        field: String,
        value: String,
    },
    /// 内部错误
    InternalError {
        message: String,
    },
    /// 健康检查失败
    HealthCheckFailed {
        service_name: String,
        error: String,
    },
}

impl ServiceError {
    /// 创建服务未找到错误
    pub fn service_not_found(name: impl Into<String>) -> Self {
        Self::ServiceNotFound {
            service_name: name.into(),
        }
    }

    /// 创建服务已存在错误
    pub fn service_already_exists(name: impl Into<String>) -> Self {
        Self::ServiceAlreadyExists {
            service_name: name.into(),
        }
    }

    /// 创建类型不匹配错误
    pub fn type_mismatch(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::TypeMismatch {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    /// 创建服务不可用错误
    pub fn service_unavailable(name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::ServiceUnavailable {
            service_name: name.into(),
            reason: reason.into(),
        }
    }

    /// 创建注册表已满错误
    pub fn registry_full(current: usize, max: usize) -> Self {
        Self::RegistryFull { current, max }
    }

    /// 创建无效配置错误
    pub fn invalid_configuration(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self::InvalidConfiguration {
            field: field.into(),
            value: value.into(),
        }
    }

    /// 创建内部错误
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::InternalError {
            message: message.into(),
        }
    }

    /// 创建健康检查失败错误
    pub fn health_check_failed(name: impl Into<String>, error: impl Into<String>) -> Self {
        Self::HealthCheckFailed {
            service_name: name.into(),
            error: error.into(),
        }
    }

    /// 创建验证错误
    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::InvalidConfiguration {
            field: "validation".to_string(),
            value: message.into(),
        }
    }

    /// 创建未找到错误（通用）
    pub fn not_found(item: impl Into<String>) -> Self {
        Self::InternalError {
            message: format!("{} not found", item.into()),
        }
    }

    /// 检查是否为可重试错误
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::ServiceUnavailable { .. } | Self::HealthCheckFailed { .. }
        )
    }

    /// 检查是否为用户错误
    pub fn is_user_error(&self) -> bool {
        matches!(
            self,
            Self::ServiceNotFound { .. }
                | Self::TypeMismatch { .. }
                | Self::InvalidConfiguration { .. }
        )
    }

    /// 检查是否为系统错误
    pub fn is_system_error(&self) -> bool {
        !self.is_user_error()
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ServiceNotFound { service_name } => {
                write!(f, "Service '{}' not found", service_name)
            }
            Self::ServiceAlreadyExists { service_name } => {
                write!(f, "Service '{}' already exists", service_name)
            }
            Self::TypeMismatch { expected, actual } => {
                write!(f, "Type mismatch: expected '{}', found '{}'", expected, actual)
            }
            Self::ServiceUnavailable {
                service_name,
                reason,
            } => {
                write!(f, "Service '{}' unavailable: {}", service_name, reason)
            }
            Self::RegistryFull { current, max } => {
                write!(f, "Registry full: {}/{} services", current, max)
            }
            Self::InvalidConfiguration { field, value } => {
                write!(f, "Invalid configuration for field '{}': {}", field, value)
            }
            Self::InternalError { message } => {
                write!(f, "Internal error: {}", message)
            }
            Self::HealthCheckFailed { service_name, error } => {
                write!(f, "Health check failed for service '{}': {}", service_name, error)
            }
        }
    }
}

impl std::error::Error for ServiceError {}

/// ServiceRegistry结果类型
pub type ServiceResult<T> = std::result::Result<T, ServiceError>;
pub type Result<T> = ServiceResult<T>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = ServiceError::service_not_found("test-service");
        assert_eq!(error.to_string(), "Service 'test-service' not found");

        let error = ServiceError::type_mismatch("TypeA", "TypeB");
        assert_eq!(
            error.to_string(),
            "Type mismatch: expected 'TypeA', found 'TypeB'"
        );
    }

    #[test]
    fn test_error_classification() {
        let user_error = ServiceError::service_not_found("test");
        assert!(user_error.is_user_error());
        assert!(!user_error.is_system_error());
        assert!(!user_error.is_retryable());

        let system_error = ServiceError::internal_error("something failed");
        assert!(!system_error.is_user_error());
        assert!(system_error.is_system_error());
        assert!(!system_error.is_retryable());

        let retryable_error = ServiceError::service_unavailable("test", "temporarily down");
        assert!(retryable_error.is_retryable());
    }

    #[test]
    fn test_registry_full_error() {
        let error = ServiceError::registry_full(100, 50);
        assert!(matches!(error, ServiceError::RegistryFull { current: 100, max: 50 }));
        assert_eq!(error.to_string(), "Registry full: 100/50 services");
    }
}