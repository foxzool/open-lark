//! Error Handler Module
//!
//! 智能错误处理和恢复建议系统，提供错误分析、处理策略和恢复建议。
//! 整合了错误分析、重试策略、用户友好消息生成等功能。
//!
//! # 主要功能
//!
//! - **智能错误分析**: 自动分类错误类型和严重程度
//! - **恢复策略建议**: 提供具体的错误处理操作建议
//! - **重试策略生成**: 为可重试错误生成最优的重试方案
//! - **用户友好消息**: 将技术错误转换为用户可理解的消息
//!
//! # 使用示例
//!
//! ```rust
//! use openlark_core::error::prelude::*;
//!
//! let error = LarkAPIError::api_error(429, "Too Many Requests", None);
//! let advice = ErrorHelper::handle_error(&error);
//!
//! if advice.is_retryable {
//!     println!("可以重试，建议等待: {}秒", advice.retry_delay.unwrap_or(0));
//! }
//!
//! let context = ErrorHelper::create_error_context(&error);
//! context.print_details();
//! ```

use std::time::Duration;

use crate::{
    api_resp::BaseResponse,
    error::types::{
        ErrorHandlingCategory, ErrorSeverity, LarkAPIError, LarkErrorCode, NetworkErrorKind,
    },
};

/// 错误处理助手工具
///
/// 提供智能的错误分析和处理建议，帮助开发者更好地处理各种错误情况。
pub struct ErrorHelper;

impl ErrorHelper {
    /// 根据错误类型提供智能处理建议
    ///
    /// # 参数
    /// - `error`: 要分析的错误
    ///
    /// # 返回值
    /// 包含错误处理建议的结构体
    ///
    /// # 示例
    /// ```rust
    /// use openlark_core::error::prelude::*;
    ///
    /// let error = LarkAPIError::MissingAccessToken;
    /// let advice = ErrorHelper::handle_error(&error);
    /// println!("错误处理建议: {}", advice.message);
    /// ```
    pub fn handle_error(error: &LarkAPIError) -> ErrorHandlingAdvice {
        let mut advice = ErrorHandlingAdvice::default();

        match error {
            LarkAPIError::ApiError { code, message, .. } => {
                if let Some(error_code) = LarkErrorCode::from_code(*code) {
                    advice = Self::handle_api_error(error_code, message);
                } else {
                    advice.message = format!("未知API错误: {message} (错误码: {code})");
                    advice.category = ErrorHandlingCategory::Unknown;
                }
            }
            LarkAPIError::NetworkError { kind, message } => {
                advice = Self::handle_network_error(*kind, message);
            }
            LarkAPIError::RequestError(req_err) => {
                advice = Self::handle_request_error(req_err);
            }
            LarkAPIError::MissingAccessToken => {
                advice.message = "缺少访问令牌".to_string();
                advice.category = ErrorHandlingCategory::Authentication;
                advice.actions.push("配置正确的访问令牌".to_string());
                advice.is_recoverable = true;
            }
            LarkAPIError::AuthenticationError { message, .. } => {
                advice.message = format!("认证失败: {message}");
                advice.category = ErrorHandlingCategory::Authentication;
                advice.actions.extend(vec![
                    "检查访问令牌是否有效".to_string(),
                    "重新获取访问令牌".to_string(),
                ]);
                advice.is_recoverable = true;
            }
            LarkAPIError::PermissionError {
                permission_type, ..
            } => {
                advice.message = format!("权限不足: {:?} 权限错误", permission_type);
                advice.category = ErrorHandlingCategory::Permission;
                advice.actions.extend(vec![
                    "检查应用权限配置".to_string(),
                    "联系管理员添加必要权限".to_string(),
                ]);
                advice.is_recoverable = true;
            }
            LarkAPIError::IllegalParamError(msg) | LarkAPIError::ValidationError(msg) => {
                advice.message = format!("参数错误: {msg}");
                advice.category = ErrorHandlingCategory::Parameter;
                advice.actions.push("检查请求参数格式和内容".to_string());
                advice.is_recoverable = true;
            }
            LarkAPIError::BadRequest(msg) => {
                advice.message = format!("请求格式错误: {msg}");
                advice.category = ErrorHandlingCategory::Parameter;
                advice.actions.push("检查请求格式和必需参数".to_string());
                advice.is_recoverable = true;
            }
            _ => {
                advice.message = format!("系统错误: {error}");
                advice.category = ErrorHandlingCategory::System;
                advice.is_recoverable = false;
            }
        }

        advice
    }

    /// 处理API错误
    fn handle_api_error(error_code: LarkErrorCode, _message: &str) -> ErrorHandlingAdvice {
        let mut advice = ErrorHandlingAdvice {
            error_code: Some(error_code),
            message: error_code.detailed_description().to_string(),
            ..Default::default()
        };

        match error_code.category() {
            crate::error::types::ErrorCategory::Authentication => {
                advice.category = ErrorHandlingCategory::Authentication;
                advice.is_recoverable = true;
                advice.actions.extend(vec![
                    "重新获取访问令牌".to_string(),
                    "检查应用配置".to_string(),
                ]);
            }
            crate::error::types::ErrorCategory::Permission => {
                advice.category = ErrorHandlingCategory::Permission;
                advice.is_recoverable = true;
                advice.actions.extend(vec![
                    "检查应用权限配置".to_string(),
                    "联系管理员添加必要权限".to_string(),
                ]);
            }
            crate::error::types::ErrorCategory::Parameter => {
                advice.category = ErrorHandlingCategory::Parameter;
                advice.is_recoverable = true;
                advice.actions.push("检查请求参数和调用方式".to_string());
            }
            crate::error::types::ErrorCategory::RateLimit => {
                advice.category = ErrorHandlingCategory::RateLimit;
                advice.is_recoverable = true;
                advice.is_retryable = true;
                advice.retry_delay = error_code.suggested_retry_delay();
                advice.actions.push("降低请求频率或稍后重试".to_string());
            }
            crate::error::types::ErrorCategory::Server => {
                advice.category = ErrorHandlingCategory::Server;
                advice.is_recoverable = true;
                advice.is_retryable = true;
                advice.retry_delay = error_code.suggested_retry_delay();
                advice.actions.push("稍后重试或联系技术支持".to_string());
            }
            crate::error::types::ErrorCategory::Network => {
                advice.category = ErrorHandlingCategory::Network;
                advice.is_recoverable = true;
                advice.is_retryable = true;
                advice.retry_delay = error_code.suggested_retry_delay();
                advice.actions.push("检查网络连接".to_string());
            }
            _ => {
                advice.category = ErrorHandlingCategory::Unknown;
                advice.actions.push("检查请求参数和调用方式".to_string());
            }
        }

        advice
    }

    /// 处理网络错误
    fn handle_network_error(kind: NetworkErrorKind, message: &str) -> ErrorHandlingAdvice {
        let mut advice = ErrorHandlingAdvice {
            category: ErrorHandlingCategory::Network,
            is_recoverable: true,
            ..Default::default()
        };

        match kind {
            NetworkErrorKind::Timeout => {
                advice.message = "网络超时".to_string();
                advice.is_retryable = true;
                advice.retry_delay = Some(5);
                advice.actions.extend(vec![
                    "增加请求超时时间".to_string(),
                    "检查网络连接状况".to_string(),
                ]);
            }
            NetworkErrorKind::ConnectionRefused => {
                advice.message = "连接被拒绝".to_string();
                advice.is_retryable = true;
                advice.retry_delay = Some(10);
                advice
                    .actions
                    .extend(vec!["检查服务状态".to_string(), "确认网络连接".to_string()]);
            }
            NetworkErrorKind::DnsResolutionFailed => {
                advice.message = "DNS解析失败".to_string();
                advice.is_retryable = true;
                advice.retry_delay = Some(10);
                advice
                    .actions
                    .extend(vec!["检查DNS设置".to_string(), "确认网络配置".to_string()]);
            }
            NetworkErrorKind::SslError => {
                advice.message = "SSL证书错误".to_string();
                advice.is_retryable = false;
                advice
                    .actions
                    .extend(vec!["检查证书配置".to_string(), "更新系统时间".to_string()]);
            }
            NetworkErrorKind::Other => {
                advice.message = format!("网络错误: {message}");
                advice.is_retryable = true;
                advice.actions.push("检查网络连接和服务状态".to_string());
            }
        }

        advice
    }

    /// 处理网络请求错误
    fn handle_request_error(req_err: &str) -> ErrorHandlingAdvice {
        let mut advice = ErrorHandlingAdvice {
            category: ErrorHandlingCategory::Network,
            is_recoverable: true,
            ..Default::default()
        };

        if req_err.contains("timeout") || req_err.contains("timed out") {
            advice.message = "请求超时".to_string();
            advice.is_retryable = true;
            advice.retry_delay = Some(5);
            advice.actions.extend(vec![
                "增加请求超时时间".to_string(),
                "检查网络连接状况".to_string(),
            ]);
        } else if req_err.contains("connect") || req_err.contains("connection") {
            advice.message = "连接失败".to_string();
            advice.is_retryable = true;
            advice.retry_delay = Some(10);
            advice.actions.extend(vec![
                "检查网络连接".to_string(),
                "确认代理设置".to_string(),
                "检查防火墙配置".to_string(),
            ]);
        } else if req_err.contains("request") {
            advice.message = "请求构建失败".to_string();
            advice.actions.push("检查请求参数格式".to_string());
        } else {
            advice.message = format!("网络错误: {req_err}");
            advice.actions.push("检查网络连接和服务状态".to_string());
        }

        advice
    }

    /// 根据响应创建处理建议
    ///
    /// # 参数
    /// - `response`: API响应
    ///
    /// # 返回值
    /// 如果响应包含错误，返回相应的处理建议；否则返回None
    pub fn analyze_response<T>(response: &BaseResponse<T>) -> Option<ErrorHandlingAdvice> {
        if response.success() {
            return None;
        }

        let mut advice = ErrorHandlingAdvice::default();

        if let Some(error_code) = response.error_code() {
            advice = Self::handle_api_error(error_code, response.msg());
        } else {
            advice.message = format!("{} (错误码: {})", response.msg(), response.code());
            advice.category = ErrorHandlingCategory::Unknown;
        }

        Some(advice)
    }

    /// 创建重试策略
    ///
    /// # 参数
    /// - `error`: 要分析重试策略的错误
    ///
    /// # 返回值
    /// 如果错误可重试，返回重试策略；否则返回None
    pub fn create_retry_strategy(error: &LarkAPIError) -> Option<RetryStrategy> {
        if !error.is_retryable() {
            return None;
        }

        let mut strategy = RetryStrategy::default();

        match error {
            LarkAPIError::ApiError { code, .. } => {
                if let Some(error_code) = LarkErrorCode::from_code(*code) {
                    strategy.max_attempts = match error_code {
                        LarkErrorCode::TooManyRequests => 3,
                        LarkErrorCode::InternalServerError => 5,
                        LarkErrorCode::ServiceUnavailable => 3,
                        LarkErrorCode::GatewayTimeout => 3,
                        _ => 3,
                    };
                    strategy.base_delay =
                        Duration::from_secs(error_code.suggested_retry_delay().unwrap_or(5));
                }
            }
            LarkAPIError::NetworkError { kind, .. } => match kind {
                NetworkErrorKind::Timeout => {
                    strategy.max_attempts = 3;
                    strategy.base_delay = Duration::from_secs(5);
                }
                NetworkErrorKind::ConnectionRefused => {
                    strategy.max_attempts = 5;
                    strategy.base_delay = Duration::from_secs(10);
                }
                NetworkErrorKind::DnsResolutionFailed => {
                    strategy.max_attempts = 3;
                    strategy.base_delay = Duration::from_secs(10);
                }
                _ => {
                    strategy.max_attempts = 3;
                    strategy.base_delay = Duration::from_secs(5);
                }
            },
            LarkAPIError::RequestError(req_err) => {
                if req_err.contains("timeout") || req_err.contains("timed out") {
                    strategy.max_attempts = 3;
                    strategy.base_delay = Duration::from_secs(5);
                } else if req_err.contains("connect") || req_err.contains("connection") {
                    strategy.max_attempts = 5;
                    strategy.base_delay = Duration::from_secs(10);
                }
            }
            _ => {
                strategy.max_attempts = 3;
                strategy.base_delay = Duration::from_secs(5);
            }
        }

        Some(strategy)
    }

    /// 格式化错误信息供用户显示
    ///
    /// # 参数
    /// - `error`: 要格式化的错误
    ///
    /// # 返回值
    /// 用户友好的错误消息字符串
    pub fn format_user_error(error: &LarkAPIError) -> String {
        error.user_friendly_message()
    }

    /// 创建错误上下文信息
    ///
    /// # 参数
    /// - `error`: 要分析的错误
    ///
    /// # 返回值
    /// 包含完整错误上下文信息的结构体
    pub fn create_error_context(error: &LarkAPIError) -> ErrorContext {
        let advice = Self::handle_error(error);
        ErrorContext {
            error_message: error.to_string(),
            user_friendly_message: Self::format_user_error(error),
            category: advice.category,
            is_recoverable: advice.is_recoverable,
            is_retryable: advice.is_retryable,
            suggested_actions: advice.actions,
            help_url: advice.help_url,
            retry_strategy: Self::create_retry_strategy(error),
            severity: error.severity(),
            error_code: advice.error_code,
        }
    }

    /// 生成错误恢复建议
    ///
    /// # 参数
    /// - `error`: 要分析的错误
    ///
    /// # 返回值
    /// 错误恢复策略枚举
    pub fn generate_recovery_strategy(error: &LarkAPIError) -> ErrorRecoveryStrategy {
        match error.handling_category() {
            ErrorHandlingCategory::Authentication => ErrorRecoveryStrategy::Reauthenticate,
            ErrorHandlingCategory::Permission => ErrorRecoveryStrategy::RequestPermission,
            ErrorHandlingCategory::Parameter => ErrorRecoveryStrategy::ValidateAndRetry,
            ErrorHandlingCategory::Network | ErrorHandlingCategory::RateLimit => {
                ErrorRecoveryStrategy::RetryWithBackoff
            }
            ErrorHandlingCategory::Server => ErrorRecoveryStrategy::RetryWithDelay,
            _ => ErrorRecoveryStrategy::ManualIntervention,
        }
    }
}

/// 错误恢复策略
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorRecoveryStrategy {
    /// 重新认证
    Reauthenticate,
    /// 请求权限
    RequestPermission,
    /// 验证并重试
    ValidateAndRetry,
    /// 带退避的重试
    RetryWithBackoff,
    /// 带延迟的重试
    RetryWithDelay,
    /// 需要手动干预
    ManualIntervention,
}

/// 错误处理建议
#[derive(Debug, Clone)]
pub struct ErrorHandlingAdvice {
    /// 错误消息
    pub message: String,
    /// 错误类别
    pub category: ErrorHandlingCategory,
    /// 错误码（如果是API错误）
    pub error_code: Option<LarkErrorCode>,
    /// 是否可恢复
    pub is_recoverable: bool,
    /// 是否可重试
    pub is_retryable: bool,
    /// 建议的重试延迟（秒）
    pub retry_delay: Option<u64>,
    /// 建议的操作
    pub actions: Vec<String>,
    /// 帮助文档链接
    pub help_url: Option<String>,
}

impl Default for ErrorHandlingAdvice {
    fn default() -> Self {
        Self {
            message: String::new(),
            category: ErrorHandlingCategory::Unknown,
            error_code: None,
            is_recoverable: false,
            is_retryable: false,
            retry_delay: None,
            actions: Vec::new(),
            help_url: None,
        }
    }
}

/// 重试策略
#[derive(Debug, Clone)]
pub struct RetryStrategy {
    /// 最大重试次数
    pub max_attempts: u32,
    /// 基础延迟时间
    pub base_delay: Duration,
    /// 是否使用指数退避
    pub use_exponential_backoff: bool,
    /// 最大延迟时间
    pub max_delay: Duration,
}

impl Default for RetryStrategy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_secs(5),
            use_exponential_backoff: true,
            max_delay: Duration::from_secs(60),
        }
    }
}

impl RetryStrategy {
    /// 计算指定尝试次数的延迟时间
    ///
    /// # 参数
    /// - `attempt`: 当前尝试次数（从0开始）
    ///
    /// # 返回值
    /// 计算得出的延迟时间
    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        if !self.use_exponential_backoff {
            return self.base_delay;
        }

        let multiplier = 2_u32.pow(attempt);
        let delay = self.base_delay * multiplier;
        std::cmp::min(delay, self.max_delay)
    }

    /// 创建线性退避策略
    ///
    /// # 参数
    /// - `max_attempts`: 最大重试次数
    /// - `base_delay`: 基础延迟时间
    ///
    /// # 返回值
    /// 新的重试策略
    pub fn linear_backoff(max_attempts: u32, base_delay: Duration) -> Self {
        Self {
            max_attempts,
            base_delay,
            use_exponential_backoff: false,
            max_delay: Duration::from_secs(300), // 5分钟
        }
    }

    /// 创建指数退避策略
    ///
    /// # 参数
    /// - `max_attempts`: 最大重试次数
    /// - `base_delay`: 基础延迟时间
    ///
    /// # 返回值
    /// 新的重试策略
    pub fn exponential_backoff(max_attempts: u32, base_delay: Duration) -> Self {
        Self {
            max_attempts,
            base_delay,
            use_exponential_backoff: true,
            max_delay: Duration::from_secs(300), // 5分钟
        }
    }
}

/// 错误上下文信息
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// 原始错误消息
    pub error_message: String,
    /// 用户友好的错误消息
    pub user_friendly_message: String,
    /// 错误类别
    pub category: ErrorHandlingCategory,
    /// 是否可恢复
    pub is_recoverable: bool,
    /// 是否可重试
    pub is_retryable: bool,
    /// 建议的操作
    pub suggested_actions: Vec<String>,
    /// 帮助文档链接
    pub help_url: Option<String>,
    /// 重试策略
    pub retry_strategy: Option<RetryStrategy>,
    /// 错误严重程度
    pub severity: ErrorSeverity,
    /// 错误码
    pub error_code: Option<LarkErrorCode>,
}

impl ErrorContext {
    /// 打印详细的错误信息
    ///
    /// 以用户友好的格式打印错误详情，包括建议操作和重试信息
    pub fn print_details(&self) {
        let severity_icon = match self.severity {
            ErrorSeverity::Info => "ℹ️",
            ErrorSeverity::Warning => "⚠️",
            ErrorSeverity::Error => "❌",
            ErrorSeverity::Critical => "🚨",
        };

        println!("{} 错误: {}", severity_icon, self.user_friendly_message);
        println!("类别: {:?}", self.category);
        println!("严重程度: {:?}", self.severity);

        if self.is_recoverable {
            println!("✅ 此错误可以恢复");
        } else {
            println!("⚠️ 此错误可能需要人工干预");
        }

        if self.is_retryable {
            println!("🔄 此错误可以重试");
            if let Some(strategy) = &self.retry_strategy {
                println!("   建议最大重试次数: {}", strategy.max_attempts);
                println!("   基础延迟时间: {:?}", strategy.base_delay);
                if strategy.use_exponential_backoff {
                    println!("   退避策略: 指数退避");
                } else {
                    println!("   退避策略: 线性退避");
                }
            }
        }

        if !self.suggested_actions.is_empty() {
            println!("\n💡 建议操作:");
            for (i, action) in self.suggested_actions.iter().enumerate() {
                println!("   {}. {}", i + 1, action);
            }
        }

        if let Some(url) = &self.help_url {
            println!("\n🔗 帮助文档: {url}");
        }
    }

    /// 转换为JSON格式
    ///
    /// # 返回值
    /// JSON格式的错误上下文字符串
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        // 简单的JSON格式化，避免复杂的序列化依赖
        let json = format!(
            r#"{{
  "error_message": "{}",
  "user_friendly_message": "{}",
  "category": "{:?}",
  "is_recoverable": {},
  "is_retryable": {},
  "severity": "{:?}",
  "suggested_actions": {},
  "error_code": {:?}
}}"#,
            self.error_message,
            self.user_friendly_message,
            self.category,
            self.is_recoverable,
            self.is_retryable,
            self.severity,
            serde_json::to_string(&self.suggested_actions).unwrap_or_default(),
            self.error_code
        );
        Ok(json)
    }

    /// 获取错误的摘要信息
    ///
    /// # 返回值
    /// 简短的错误摘要字符串
    pub fn summary(&self) -> String {
        format!(
            "{} ({:?}, {}): {} - {} actions suggested",
            self.user_friendly_message,
            self.category,
            self.severity,
            if self.is_retryable {
                "retryable"
            } else {
                "not retryable"
            },
            self.suggested_actions.len()
        )
    }
}

// ============================================================================
// 便利函数
// ============================================================================

/// 检查错误是否可重试
///
/// # 参数
/// - `error`: 要检查的错误
///
/// # 返回值
/// true如果错误可重试，false otherwise
pub fn is_retryable_error(error: &LarkAPIError) -> bool {
    error.is_retryable()
}

/// 检查错误是否为权限错误
///
/// # 参数
/// - `error`: 要检查的错误
///
/// # 返回值
/// true如果错误为权限错误，false otherwise
pub fn is_permission_error(error: &LarkAPIError) -> bool {
    error.is_permission_error()
}

/// 检查错误是否为认证错误
///
/// # 参数
/// - `error`: 要检查的错误
///
/// # 返回值
/// true如果错误为认证错误，false otherwise
pub fn is_authentication_error(error: &LarkAPIError) -> bool {
    error.is_authentication_error()
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_helper_api_error() {
        let error = LarkAPIError::api_error(403, "Forbidden", None);
        let advice = ErrorHelper::handle_error(&error);

        assert_eq!(advice.category, ErrorHandlingCategory::Permission);
        assert!(advice.is_recoverable);
        assert!(!advice.actions.is_empty());
    }

    #[test]
    fn test_retry_strategy() {
        let error = LarkAPIError::api_error(429, "Too Many Requests", None);
        let strategy = ErrorHelper::create_retry_strategy(&error);

        assert!(strategy.is_some());
        let strategy = strategy.unwrap();
        assert_eq!(strategy.max_attempts, 3);
    }

    #[test]
    fn test_error_context() {
        let error = LarkAPIError::MissingAccessToken;
        let context = ErrorHelper::create_error_context(&error);

        assert_eq!(context.category, ErrorHandlingCategory::Authentication);
        assert!(context.is_recoverable);
    }

    #[test]
    fn test_recovery_strategy() {
        let auth_error = LarkAPIError::MissingAccessToken;
        let strategy = ErrorHelper::generate_recovery_strategy(&auth_error);
        assert_eq!(strategy, ErrorRecoveryStrategy::Reauthenticate);

        let permission_error = LarkAPIError::permission_error(
            "Denied",
            crate::error::types::PermissionType::Application,
        );
        let strategy = ErrorHelper::generate_recovery_strategy(&permission_error);
        assert_eq!(strategy, ErrorRecoveryStrategy::RequestPermission);

        let network_error = LarkAPIError::network_error("Timeout", NetworkErrorKind::Timeout);
        let strategy = ErrorHelper::generate_recovery_strategy(&network_error);
        assert_eq!(strategy, ErrorRecoveryStrategy::RetryWithBackoff);
    }

    #[test]
    fn test_convenience_functions() {
        let auth_error = LarkAPIError::MissingAccessToken;
        assert!(is_authentication_error(&auth_error));
        assert!(!is_permission_error(&auth_error));
        assert!(!is_retryable_error(&auth_error));

        let network_error = LarkAPIError::network_error("Timeout", NetworkErrorKind::Timeout);
        assert!(!is_authentication_error(&network_error));
        assert!(!is_permission_error(&network_error));
        assert!(is_retryable_error(&network_error));
    }

    #[test]
    fn test_retry_strategy_creation() {
        let linear = RetryStrategy::linear_backoff(5, Duration::from_secs(10));
        assert_eq!(linear.max_attempts, 5);
        assert_eq!(linear.base_delay, Duration::from_secs(10));
        assert!(!linear.use_exponential_backoff);

        let exponential = RetryStrategy::exponential_backoff(3, Duration::from_secs(2));
        assert_eq!(exponential.max_attempts, 3);
        assert_eq!(exponential.base_delay, Duration::from_secs(2));
        assert!(exponential.use_exponential_backoff);
    }

    #[test]
    fn test_error_context_methods() {
        let error = LarkAPIError::api_error(429, "Rate limit", None);
        let context = ErrorHelper::create_error_context(&error);

        let summary = context.summary();
        assert!(summary.contains("retryable"));
        assert!(summary.contains("Rate limit"));

        // Test that print_details doesn't panic
        context.print_details();
    }

    #[test]
    fn test_network_error_handling() {
        let timeout_error = LarkAPIError::network_error("Timeout", NetworkErrorKind::Timeout);
        let advice = ErrorHelper::handle_error(&timeout_error);

        assert_eq!(advice.category, ErrorHandlingCategory::Network);
        assert!(advice.is_retryable);
        assert_eq!(advice.retry_delay, Some(5));

        let ssl_error = LarkAPIError::network_error("SSL Error", NetworkErrorKind::SslError);
        let advice = ErrorHelper::handle_error(&ssl_error);

        assert_eq!(advice.category, ErrorHandlingCategory::Network);
        assert!(!advice.is_retryable);
    }

    #[test]
    fn test_authentication_error_handling() {
        let auth_error = LarkAPIError::auth_error_with_details("Invalid token", "Token expired");
        let advice = ErrorHelper::handle_error(&auth_error);

        assert_eq!(advice.category, ErrorHandlingCategory::Authentication);
        assert!(advice.is_recoverable);
        assert!(!advice.actions.is_empty());
    }

    #[test]
    fn test_permission_error_handling() {
        let perm_error = LarkAPIError::permission_error(
            "Access denied",
            crate::error::types::PermissionType::Document,
        );
        let advice = ErrorHelper::handle_error(&perm_error);

        assert_eq!(advice.category, ErrorHandlingCategory::Permission);
        assert!(advice.is_recoverable);
        assert!(!advice.actions.is_empty());
    }
}
