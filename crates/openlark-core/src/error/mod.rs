//! 现代化错误处理系统
//!
//! 基于thiserror的完整错误处理解决方案，提供类型安全和开发者友好的API

use std::time::Duration;
use uuid::Uuid;

// 暴露错误体系（CoreError 为主要错误类型）
pub use self::codes::{ErrorCategory, ErrorCode};
pub use self::context::{ErrorContext, ErrorContextBuilder};
pub use self::core::{RecoveryStrategy, RetryPolicy};
pub use self::core::{
    api_error, authentication_error, business_error, configuration_error, network_error,
    network_error_with_details, permission_missing_error, rate_limit_error, serialization_error,
    service_unavailable_error, sso_token_invalid_error, timeout_error, token_expired_error,
    token_invalid_error, user_identity_invalid_error, validation_error,
};
pub use self::core::{ApiError, BuilderKind, CoreError, ErrorBuilder, ErrorRecord};
pub use self::kinds::ErrorKind;
pub use self::traits::{ErrorContextTrait, ErrorFormatTrait, ErrorTrait, FullErrorTrait};
pub use self::traits::{ErrorSeverity, ErrorType};

// 主要类型别名（推荐使用）
pub type SDKResult<T> = Result<T, CoreError>;
pub type ErrorId = Uuid;
pub type LarkAPIError = CoreError;

// 兼容性类型别名（逐步迁移中，后续可移除）
#[deprecated(note = "使用 CoreError 替代")]
pub type LegacyCoreError = CoreError;

// 核心模块
pub mod codes;
pub mod context;
pub mod core; // 主实现（含重试策略与错误定义）
pub mod kinds;
pub mod traits;

// 预设导入
pub mod prelude;

/// 系统版本信息
pub const ERROR_SYSTEM_VERSION: &str = "2.1.0-modern";

/// 系统能力声明
pub mod capabilities {
    pub const CORE_ERROR: bool = true;
    pub const ERROR_CONTEXT: bool = true;
    pub const RETRY_POLICY: bool = true;
    pub const ERROR_ANALYSIS: bool = true;
    pub const MODERN_FORMATTING: bool = true;
    pub const BUILDER_PATTERN: bool = false;
    pub const ERROR_CLASSIFICATION: bool = true;
    pub const JSON_EXPORT: bool = true;
    pub const OBSERVABILITY: bool = true;
    pub const RECOVERY_SUGGESTIONS: bool = true;
    // 注意：无需向后兼容
    pub const LEGACY_COMPATIBILITY: bool = false;
}

/// 默认配置
pub mod defaults {
    use super::*;

    pub fn default_retry_policy() -> RetryPolicy {
        RetryPolicy::exponential(3, Duration::from_secs(1))
    }

    pub fn default_error_context() -> ErrorContext {
        ErrorContext::new()
    }
}

// 现代化便利函数
// modern_convenience 功能已收敛到核心错误系统中

/// 错误分析工具
pub mod analysis {
    use super::*;

    /// 深度分析错误
    pub fn analyze_error<E: ErrorTrait>(error: &E) -> ErrorAnalysisResult {
        ErrorAnalysisResult {
            error_type: error.error_type(),
            severity: error.severity(),
            is_retryable: error.is_retryable(),
            retry_delay: error.retry_delay(0),
            user_error: error.is_user_error(),
            system_error: error.is_system_error(),
            recovery_action: recommend_action(error),
            context_summary: summarize_context(error.context()),
        }
    }

    pub struct ErrorAnalysisResult {
        pub error_type: ErrorType,
        pub severity: ErrorSeverity,
        pub is_retryable: bool,
        pub retry_delay: Option<Duration>,
        pub user_error: bool,
        pub system_error: bool,
        pub recovery_action: &'static str,
        pub context_summary: String,
    }

    fn recommend_action<E: ErrorTrait>(error: &E) -> &'static str {
        match error.error_type() {
            ErrorType::Network | ErrorType::Timeout | ErrorType::ServiceUnavailable => "retry",
            ErrorType::Authentication => "reauthenticate",
            ErrorType::Validation => "fix_input",
            ErrorType::Configuration => "fix_config",
            ErrorType::RateLimit => "slowdown",
            _ => "inspect",
        }
    }

    fn summarize_context(context: &ErrorContext) -> String {
        let mut parts = Vec::new();

        if let Some(req_id) = context.request_id() {
            parts.push(format!("RequestID: {}", req_id));
        }

        if let Some(op) = context.operation() {
            parts.push(format!("Operation: {}", op));
        }

        if let Some(comp) = context.component() {
            parts.push(format!("Component: {}", comp));
        }

        let context_count = context.context_len();
        if context_count > 0 {
            parts.push(format!("Context: {} items", context_count));
        }

        if parts.is_empty() {
            "No context".to_string()
        } else {
            parts.join(" | ")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::analysis::*;
    use super::*;

    #[test]
    fn test_modern_error_creation() {
        let error = api_error(404, "/api/users/123", "用户不存在", Some("req-123".to_string()));

        assert_eq!(error.error_type(), ErrorType::Api);
        assert_eq!(error.severity(), ErrorSeverity::Warning);
        assert!(!error.is_retryable());
        assert!(!error.is_user_error());
        assert_eq!(error.context().request_id(), Some("req-123"));
    }

    #[test]
    fn test_detailed_error_creation() {
        let error = network_error_with_details(
            "连接超时",
            Some("https://api.example.com".to_string()), // endpoint
            Some("req-456".to_string()),                 // request_id
        );

        assert!(error.is_network_error());
        assert_eq!(error.context().request_id(), Some("req-456"));
    }

    #[test]
    fn test_error_analysis() {
        let error = validation_error("email", "邮箱格式不正确");

        let analysis = analyze_error(&error);

        assert_eq!(analysis.error_type, ErrorType::Validation);
        assert_eq!(analysis.severity, ErrorSeverity::Warning);
        assert!(!analysis.is_retryable);
        assert!(analysis.user_error);
        assert!(!analysis.system_error);
        assert!(analysis.context_summary.contains("Context"));
    }

    #[test]
    fn test_capabilities() {
        assert!(capabilities::CORE_ERROR);
        assert!(capabilities::ERROR_CONTEXT);
        assert!(!capabilities::BUILDER_PATTERN);
        assert!(capabilities::ERROR_ANALYSIS);
        assert!(!capabilities::LEGACY_COMPATIBILITY);
    }

    #[test]
    fn test_version() {
        assert_eq!(ERROR_SYSTEM_VERSION, "2.1.0-modern");
    }
}
