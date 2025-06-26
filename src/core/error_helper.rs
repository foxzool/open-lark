use std::time::Duration;

use crate::core::{
    api_resp::BaseResponse,
    error::LarkAPIError,
    error_codes::{ErrorCategory, LarkErrorCode},
};

/// 错误处理助手工具
pub struct ErrorHelper;

impl ErrorHelper {
    /// 根据错误类型提供智能处理建议
    pub fn handle_error(error: &LarkAPIError) -> ErrorHandlingAdvice {
        let mut advice = ErrorHandlingAdvice::default();

        match error {
            LarkAPIError::ApiError { code, message, .. } => {
                if let Some(error_code) = LarkErrorCode::from_code(*code) {
                    advice = Self::handle_api_error(error_code, message);
                } else {
                    advice.message = format!("未知API错误: {} (错误码: {})", message, code);
                    advice.category = ErrorHandlingCategory::Unknown;
                }
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
            LarkAPIError::IllegalParamError(msg) => {
                advice.message = format!("参数错误: {}", msg);
                advice.category = ErrorHandlingCategory::ClientError;
                advice.actions.push("检查请求参数格式和内容".to_string());
                advice.is_recoverable = true;
            }
            _ => {
                advice.message = format!("系统错误: {}", error);
                advice.category = ErrorHandlingCategory::SystemError;
            }
        }

        advice
    }

    /// 处理API错误
    fn handle_api_error(error_code: LarkErrorCode, _message: &str) -> ErrorHandlingAdvice {
        let mut advice = ErrorHandlingAdvice::default();
        advice.error_code = Some(error_code);
        advice.message = error_code.detailed_description().to_string();

        match error_code.category() {
            ErrorCategory::Authentication => {
                advice.category = ErrorHandlingCategory::Authentication;
                advice.is_recoverable = true;
                advice.actions.extend(vec![
                    "重新获取访问令牌".to_string(),
                    "检查应用配置".to_string(),
                ]);
            }
            ErrorCategory::Permission => {
                advice.category = ErrorHandlingCategory::Permission;
                advice.is_recoverable = true;
                advice.actions.extend(vec![
                    "检查应用权限配置".to_string(),
                    "联系管理员添加必要权限".to_string(),
                ]);
            }
            ErrorCategory::RateLimit => {
                advice.category = ErrorHandlingCategory::RateLimit;
                advice.is_recoverable = true;
                advice.is_retryable = true;
                advice.retry_delay = error_code.suggested_retry_delay();
                advice.actions.push("降低请求频率或稍后重试".to_string());
            }
            ErrorCategory::Server => {
                advice.category = ErrorHandlingCategory::ServerError;
                advice.is_recoverable = true;
                advice.is_retryable = true;
                advice.retry_delay = error_code.suggested_retry_delay();
                advice.actions.push("稍后重试或联系技术支持".to_string());
            }
            ErrorCategory::Network => {
                advice.category = ErrorHandlingCategory::NetworkError;
                advice.is_recoverable = true;
                advice.is_retryable = true;
                advice.actions.push("检查网络连接".to_string());
            }
            _ => {
                advice.category = ErrorHandlingCategory::ClientError;
                advice.actions.push("检查请求参数和调用方式".to_string());
            }
        }

        if let Some(help_url) = error_code.help_url() {
            advice.help_url = Some(help_url.to_string());
        }

        advice
    }

    /// 处理网络请求错误
    fn handle_request_error(req_err: &str) -> ErrorHandlingAdvice {
        let mut advice = ErrorHandlingAdvice::default();
        advice.category = ErrorHandlingCategory::NetworkError;
        advice.is_recoverable = true;

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
            advice.message = format!("网络错误: {}", req_err);
            advice.actions.push("检查网络连接和服务状态".to_string());
        }

        advice
    }

    /// 根据响应创建处理建议
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
    pub fn format_user_error(error: &LarkAPIError) -> String {
        match error {
            LarkAPIError::ApiError { code, .. } => {
                if let Some(error_code) = LarkErrorCode::from_code(*code) {
                    error_code.detailed_description().to_string()
                } else {
                    format!("API调用失败，错误码: {}", code)
                }
            }
            _ => error.user_friendly_message(),
        }
    }

    /// 创建错误上下文信息
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
        }
    }
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

/// 错误处理类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorHandlingCategory {
    /// 认证错误
    Authentication,
    /// 权限错误
    Permission,
    /// 客户端错误
    ClientError,
    /// 服务器错误
    ServerError,
    /// 网络错误
    NetworkError,
    /// 限流错误
    RateLimit,
    /// 系统错误
    SystemError,
    /// 未知错误
    Unknown,
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
    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        if !self.use_exponential_backoff {
            return self.base_delay;
        }

        let multiplier = 2_u32.pow(attempt);
        let delay = self.base_delay * multiplier;
        std::cmp::min(delay, self.max_delay)
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
}

impl ErrorContext {
    /// 打印详细的错误信息
    pub fn print_details(&self) {
        println!("❌ 错误: {}", self.user_friendly_message);
        println!("类别: {:?}", self.category);

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
            }
        }

        if !self.suggested_actions.is_empty() {
            println!("\n💡 建议操作:");
            for (i, action) in self.suggested_actions.iter().enumerate() {
                println!("   {}. {}", i + 1, action);
            }
        }

        if let Some(url) = &self.help_url {
            println!("\n🔗 帮助文档: {}", url);
        }
    }
}

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
}
