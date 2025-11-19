//! Error Handling Module
//!
//! 统一的错误处理模块，提供完整的错误定义、处理建议和观测性功能。
//!
//! # 模块结构
//!
//! - [`types`] - 错误类型定义和错误码
//! - [`handler`] - 错误处理逻辑和建议
//! - [`observability`] - 日志记录和监控统计
//!
//! # 快速开始
//!
//! ```rust
//! use openlark_core::error::prelude::*;
//!
//! fn handle_error(error: &LarkAPIError) {
//!     if error.is_retryable() {
//!         println!("可重试错误: {}", error.user_friendly_message());
//!     }
//! }
//! ```

// 核心模块
pub mod handler;
pub mod observability;
pub mod types;

// 便利导入
pub mod prelude;

// 重新导出核心类型，方便直接使用
pub use handler::*;
pub use observability::*;
pub use types::*;

/// 当前模块版本
pub const ERROR_MODULE_VERSION: &str = "1.0.0";

/// 默认错误处理配置
pub mod config {
    use super::observability::LogLevel;

    /// 默认重试次数
    pub const DEFAULT_RETRY_COUNT: usize = 3;

    /// 默认超时时间（秒）
    pub const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

    /// 默认日志级别
    pub const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Error;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_structure() {
        // 确保模块结构正确
        let _types = types::LarkAPIError::MissingAccessToken;
        let _handler = handler::ErrorHelper;
        let _observability = observability::ErrorLogger::default();
    }

    #[test]
    fn test_version_constant() {
        assert_eq!(ERROR_MODULE_VERSION, "1.0.0");
    }

    #[test]
    fn test_default_config() {
        assert_eq!(config::DEFAULT_RETRY_COUNT, 3);
        assert_eq!(config::DEFAULT_TIMEOUT_SECONDS, 30);
    }
}
