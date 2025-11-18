//! Error Module Prelude
//!
//! 重新导出最常用的错误处理类型和特征，简化导入路径。

// 重新导出核心错误类型
pub use super::types::{
    ApiResponseError, AuthenticationError, ErrorCategory, ErrorHandlingCategory, LarkAPIError,
    LarkErrorCode, NetworkError, ValidationError,
};

// 重新导出错误处理助手
pub use super::handler::{
    is_authentication_error, is_permission_error, is_retryable_error, ErrorHandlingAdvice,
    ErrorHelper, ErrorRecoveryStrategy,
};

// 重新导出观测性功能
pub use super::observability::{
    get_error_stats, log_error, record_error, ErrorEvent, ErrorLogger, ErrorMonitor,
};

// 重新导出类型定义
pub use super::types::{ErrorSeverity, NetworkErrorKind, PermissionType};

// 重新导出特征
pub use super::types::{ErrorClassification, ErrorDisplay, ErrorRecovery};

// 重新导出常用结果类型
pub type SDKResult<T> = Result<T, LarkAPIError>;

/// 常用宏集合
pub mod macros {
    /// 创建验证错误的便利宏
    #[macro_export]
    macro_rules! validation_error {
        ($msg:expr) => {
            $crate::error::prelude::LarkAPIError::ValidationError($msg.to_string())
        };
    }

    /// 创建认证错误的便利宏
    #[macro_export]
    macro_rules! auth_error {
        ($msg:expr) => {
            $crate::error::prelude::LarkAPIError::MissingAccessToken
        };
        ($msg:expr, $details:expr) => {
            $crate::error::prelude::LarkAPIError::AuthenticationError {
                message: $msg.to_string(),
                details: $details.to_string(),
            }
        };
    }

    /// 检查并返回错误的便利宏
    #[macro_export]
    macro_rules! ensure {
        ($condition:expr, $error:expr) => {
            if !$condition {
                return Err($error);
            }
        };
    }

    /// 记录错误的便利宏
    #[macro_export]
    macro_rules! log_and_return {
        ($error:expr) => {{
            let error = $error;
            $crate::error::prelude::log_error(&error, $crate::error::prelude::LogLevel::Error);
            return Err(error);
        }};
    }
}

// 导出宏到当前作用域
// pub use macros::*; // 暂时注释掉，避免未使用导入警告
