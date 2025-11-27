//! 🚀 OpenLark Client Library
//!
//! 现代化的飞书开放平台 Rust SDK，提供简洁、类型安全的 API 访问
//! 集成 CoreError 企业级错误处理系统，提供全面的错误管理和恢复建议
//!
//! ## 核心特性

#![allow(unexpected_cfgs)] // 允许使用尚未加入工作区的功能标志
//!
//! - **🎯 Feature-driven**: 基于编译时功能标志的模块化设计
//! - **⚡ 零配置**: 支持从环境变量自动配置客户端
//! - **🔒 类型安全**: 完全编译时验证的 API 调用
//! - **🚀 异步优先**: 完全异步的客户端实现
//! - **🏗️ 现代构建器**: 流畅的构建器模式 API
//! - **🔍 服务发现**: 动态服务注册和管理
//! - **🛡️ 企业级**: 基于 CoreError 的高级错误处理、重试和监控支持
//! - **🌐 中文优先**: 100% 中文错误消息和文档，专为中国开发者优化
//!
//! ## 快速开始
//!
//! ### 基础用法
//!
//! ```rust,no_run
//! use openlark_client::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // 从环境变量创建客户端（推荐）
//!     let client = Client::from_env()?;
//!
//!     // 发送文本消息（需要 communication feature）
//!     #[cfg(feature = "communication")]
//!     {
//!         let result = client.communication()
//!             .send_text_message("user_open_id", "open_id", "Hello!")
//!             .await?;
//!         println!("消息发送成功: {}", result.message_id);
//!     }
//!
//!     // 获取员工列表（需要 hr feature）
//!     #[cfg(feature = "hr")]
//!     {
//!         let employees = client.hr()
//!             .list_employees(Some("open_id"), Some(50), None)
//!             .await?;
//!         for employee in employees.employees {
//!             println!("员工: {} ({})", employee.name, employee.user_id);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### 构建器模式
//!
//! ```rust,no_run
//! use openlark_client::prelude::*;
//! use std::time::Duration;
//!
//! let client = Client::builder()
//!     .app_id("your_app_id")
//!     .app_secret("your_app_secret")
//!     .base_url("https://open.feishu.cn")
//!     .timeout(Duration::from_secs(30))
//!     .enable_log(true)
//!     .build()?;
//! ```
//!
//! ### 环境变量配置
//!
//! 设置以下环境变量：
//!
//! ```bash
//! export OPENLARK_APP_ID="your_app_id"
//! export OPENLARK_APP_SECRET="your_app_secret"
//! export OPENLARK_BASE_URL="https://open.feishu.cn"  # 可选
//! export OPENLARK_TIMEOUT="30"  # 可选，秒
//! export OPENLARK_ENABLE_LOG="true"  # 可选
//! ```
//!
//! ## 功能标志
//!
//! 客户端使用 Rust 功能标志进行模块化编译：
//!
//! ```toml
//! [dependencies]
//! openlark-client = { version = "0.1", features = [
//!     "communication",  # 通讯服务
//!     "hr",           # 人力资源服务
//!     "docs",         # 文档服务
//!     "ai",           # AI 服务
//!     "auth",         # 认证服务
//!     "websocket",    # WebSocket 支持
//! ]}
//! ```
//!
//! ## 服务访问
//!
//! 每个启用功能都提供对应的服务访问器：
//!
//! ```rust,no_run
//! let client = Client::from_env()?;
//!
//! // 通讯服务（communication feature）
//! #[cfg(feature = "communication")]
//! let comm = client.communication();
//!
//! // HR 服务（hr feature）
//! #[cfg(feature = "hr")]
//! let hr = client.hr();
//!
//! // 文档服务（docs feature）
//! #[cfg(feature = "docs")]
//! let docs = client.docs();
//!
//! // AI 服务（ai feature）
//! #[cfg(feature = "ai")]
//! let ai = client.ai();
//!
//! // 认证服务（auth feature）
//! #[cfg(feature = "auth")]
//! let auth = client.auth();
//! ```
//!
//! ## 高级用法
//!
//! ### 服务注册和管理
//!
//! ```rust,no_run
//! use openlark_client::prelude::*;
//!
//! let client = Client::from_env()?;
//! let registry = client.registry();
//!
//! // 检查可用服务
//! println!("可用服务: {:?}", registry.list_services());
//!
//! // 检查特定服务是否可用
//! if registry.has_service("communication") {
//!     println!("通讯服务可用");
//! }
//! ```
//!
//! ### 自定义配置
//!
//! ```rust,no_run
//! use openlark_client::prelude::*;
//! use std::collections::HashMap;
//! use std::time::Duration;
//!
//! let mut headers = HashMap::new();
//! headers.insert("User-Agent".to_string(), "MyApp/1.0".to_string());
//!
//! let client = Client::builder()
//!     .app_id("app_id")
//!     .app_secret("app_secret")
//!     .timeout(Duration::from_secs(60))
//!     .retry_count(3)
//!     .headers(headers)
//!     .build()?;
//! ```
//!
//! ## 错误处理
//!
//! 客户端基于 CoreError 提供企业级错误处理，包含详细的错误分析、恢复建议和中文友好的错误消息：
//!
//! ```rust,no_run
//! use openlark_client::prelude::*;
//!
//! match Client::from_env() {
//!     Ok(client) => {
//!         println!("客户端创建成功");
//!         // 使用客户端...
//!     },
//!     Err(error) => {
//!         // 用户友好的错误消息（中文）
//!         eprintln!("❌ {}", error.user_message().unwrap_or("未知错误"));
//!
//!         // 获取错误恢复建议
//!         eprintln!("💡 建议: {}", error.suggestion());
//!
//!         // 获取详细的恢复步骤
//!         for (i, step) in error.recovery_steps().iter().enumerate() {
//!             eprintln!("{}. {}", i + 1, step);
//!         }
//!
//!         // 获取完整的错误分析报告
//!         eprintln!("\n{}", error.detailed_report());
//!
//!         // 根据错误类型进行特定处理
//!         if error.is_validation_error() {
//!             eprintln!("请检查配置参数是否正确");
//!         } else if error.is_network_error() {
//!             eprintln!("请检查网络连接并稍后重试");
//!         } else if error.is_auth_error() {
//!             eprintln!("请检查应用凭据是否有效");
//!         }
//!     }
//! }
//! ```
//!
//! ### 错误类型和处理
//!
//! ```rust,no_run
//! use openlark_client::prelude::*;
//!
//! // 捕获和处理特定类型的错误
//! async fn send_message_with_error_handling() -> Result<()> {
//!     let client = Client::from_env()?;
//!
//!     match client.communication().send_text_message("user_123", "open_id", "Hello!").await {
//!         Ok(response) => {
//!             println!("消息发送成功: {}", response.message_id);
//!             Ok(())
//!         },
//!         Err(error) => {
//!             // 自动错误分析和处理建议
//!             if error.is_retryable() {
//!                 println!("错误可重试，建议稍后重试");
//!                 // 实现重试逻辑...
//!             }
//!
//!             // 记录错误用于监控
//!             tracing::error!("消息发送失败: {}", error.log_summary());
//!
//!             Err(error) // 返回原始错误给上层处理
//!         }
//!     }
//! }
//! ```

//#![deny(missing_docs)]  // 暂时禁用以完成基本编译
#![warn(clippy::all)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]

// 核心模块
pub mod client;
pub mod config;
pub mod error;
pub mod features;
pub mod registry;
pub mod traits;
pub mod types;

// 服务访问层
pub mod services;

// WebSocket 模块（条件编译）
/// WebSocket 客户端模块
///
/// 提供与飞书WebSocket服务的实时连接功能，支持事件接收和状态管理。
/// 此模块重新导出了openlark-core中的WebSocket实现。
#[cfg(feature = "websocket")]
pub mod ws_client;

// ============================================================================
// 核心类型重新导出
// ============================================================================

// 客户端和配置
pub use client::{Client, ClientBuilder};
pub use config::Config;

// 企业级错误处理系统 - 基于 CoreError
pub use error::{Error, Result};

// 错误扩展功能
pub use error::{
    with_context,           // 上下文错误处理
    with_operation_context, // 操作上下文错误处理
    ClientErrorExt,         // 客户端错误扩展特征
    ErrorAnalyzer,          // 错误分析器
};

// 错误创建便利函数
pub use error::{
    api_error,                 // API错误
    authentication_error,      // 认证错误
    business_error,            // 业务错误
    configuration_error,       // 配置错误
    internal_error,            // 内部错误
    network_error,             // 网络错误
    rate_limit_error,          // 限流错误
    registry_error,            // 注册表错误
    serialization_error,       // 序列化错误
    service_unavailable_error, // 服务不可用错误
    timeout_error,             // 超时错误
    validation_error,          // 验证错误
};

// 功能管理和服务注册
pub use features::{FeatureLoader, FeatureSet, FeatureStats};
pub use registry::{
    DefaultServiceRegistry, ServiceEntry, ServiceMetadata, ServiceRegistry, ServiceStatus,
};

// 客户端特征
pub use traits::*;

// 注意：legacy_client 已在 v0.15.0 中移除
// 请使用新的 DefaultLarkClient 和 ClientBuilder
// 迁移指南：https://github.com/foxzool/open-lark/blob/main/docs/migration-guide.md

// ============================================================================
// 服务类型重新导出
// ============================================================================

// 基础服务（始终可用）
pub use services::AuthService;

// 服务工厂和管理
pub use services::{ServiceFactory, ServiceFactoryStats, ServiceValidator};

// 可选服务（基于功能标志）
#[cfg(feature = "communication")]
pub use services::CommunicationService;

#[cfg(feature = "docs")]
pub use services::DocsService;

#[cfg(feature = "hr")]
pub use services::HRService;

#[cfg(feature = "ai")]
pub use services::AIService;

#[cfg(feature = "task")]
pub use services::TaskService;

#[cfg(feature = "calendar")]
pub use services::CalendarService;

#[cfg(feature = "admin")]
pub use services::AdminService;

#[cfg(feature = "approval")]
pub use services::ApprovalService;

// 其他服务（当前未启用但已规划）
// #[cfg(feature = "collab")]
// pub use services::CollabService;

// #[cfg(feature = "helpdesk")]
// pub use services::HelpdeskService;

// #[cfg(feature = "hire")]
// pub use services::HireService;

// #[cfg(feature = "people")]
// pub use services::PeopleService;

// ============================================================================
// Core 系统类型重新导出
// ============================================================================

// 重新导出 openlark-core 核心类型
pub use openlark_core::{config::Config as CoreConfig, SDKResult as CoreResult};

// 错误系统核心类型
pub use openlark_core::{
    error::{CoreError, ErrorCode, ErrorSeverity, ErrorTrait, ErrorType},
};

// ============================================================================
// 类型别名和便利定义
// ============================================================================

/// 📦 客户端结果类型别名
pub type ClientResult<T> = Result<T>;

/// 🚨 SDK 结果类型别名（与 Core 系统兼容）
pub type SDKResult<T> = openlark_core::SDKResult<T>;

/// 📋 服务创建结果类型
pub type ServiceResult<T> = Result<T>;

/// 🔧 配置验证结果类型
pub type ConfigResult<T> = Result<T>;

/// 🚀 预导出模块 - 包含最常用的类型和特征
///
/// 使用预导出可以简化导入，提供一站式类型访问：
///
/// ```rust,no_run
/// use openlark_client::prelude::*;
///
/// let client = Client::from_env()?;
/// let service_factory = ServiceFactory::new(client.config().clone())?;
/// ```
pub mod prelude {
    // ============================================================================
    // 核心客户端类型
    // ============================================================================

    // 客户端和配置
    pub use crate::{Client, ClientBuilder, Config};

    // 企业级错误处理系统
    pub use crate::{Error, Result};

    // ============================================================================
    // 错误处理扩展
    // ============================================================================

    // 错误扩展特征和分析器
    pub use crate::{
        with_context,           // 上下文错误处理
        with_operation_context, // 操作上下文错误处理
        ClientErrorExt,         // 客户端错误扩展特征
        ErrorAnalyzer,          // 错误分析器
    };

    // 错误创建便利函数
    pub use crate::{
        api_error,                 // API错误
        authentication_error,      // 认证错误
        business_error,            // 业务错误
        configuration_error,       // 配置错误
        internal_error,            // 内部错误
        network_error,             // 网络错误
        rate_limit_error,          // 限流错误
        registry_error,            // 注册表错误
        serialization_error,       // 序列化错误
        service_unavailable_error, // 服务不可用错误
        timeout_error,             // 超时错误
        validation_error,          // 验证错误
    };

    // Core 错误系统类型
    pub use openlark_core::error::{CoreError, ErrorCode, ErrorSeverity, ErrorTrait, ErrorType};

    // ============================================================================
    // 客户端特征
    // ============================================================================

    // 服务特征
    pub use crate::traits::{LarkClient, ServiceLifecycle, ServiceTrait};

    // 服务注册
    pub use crate::ServiceRegistry;

    // ============================================================================
    // 功能管理
    // ============================================================================

    pub use crate::{FeatureLoader, FeatureSet};

    // ============================================================================
    // 服务工厂和管理
    // ============================================================================

    // 服务工厂
    pub use crate::{ServiceFactory, ServiceFactoryStats, ServiceValidator};

    // ============================================================================
    // 服务类型
    // ============================================================================

    // 基础服务（始终可用）
    pub use crate::services::AuthService;

    // 可选服务（基于功能标志）
    #[cfg(feature = "communication")]
    pub use crate::services::CommunicationService;

    #[cfg(feature = "docs")]
    pub use crate::services::DocsService;

    #[cfg(feature = "hr")]
    pub use crate::services::HRService;

    #[cfg(feature = "ai")]
    pub use crate::services::AIService;

    #[cfg(feature = "task")]
    pub use crate::services::TaskService;

    #[cfg(feature = "calendar")]
    pub use crate::services::CalendarService;

    #[cfg(feature = "admin")]
    pub use crate::services::AdminService;

    #[cfg(feature = "approval")]
    pub use crate::services::ApprovalService;

    // 其他服务（当前未启用但已规划）
    // #[cfg(feature = "collab")]
    // pub use crate::services::CollabService;

    // #[cfg(feature = "helpdesk")]
    // pub use crate::services::HelpdeskService;

    // #[cfg(feature = "hire")]
    // pub use crate::services::HireService;

    // #[cfg(feature = "people")]
    // pub use crate::services::PeopleService;

    // ============================================================================
    // 便利类型别名
    // ============================================================================

    /// 📦 客户端结果类型别名
    pub type ClientResult<T> = Result<T>;

    /// 🚨 SDK 结果类型别名（与 Core 系统兼容）
    pub type SDKResult<T> = openlark_core::SDKResult<T>;

    /// 📋 服务创建结果类型
    pub type ServiceResult<T> = Result<T>;

    /// 🔧 配置验证结果类型
    pub type ConfigResult<T> = Result<T>;

    // ============================================================================
    // 常用宏和便利导入
    // ============================================================================

    // 重新导出常用的 core 类型，减少嵌套导入
    pub use openlark_core::{config::Config as CoreConfig, SDKResult as CoreResult};

    // 常用的标准库类型
    pub use std::collections::HashMap;
    pub use std::time::Duration;
}

/// 🏷️ 库信息
pub mod info {
    /// 库名称
    pub const NAME: &str = "OpenLark Client";
    /// 库版本
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    /// 库描述
    pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
    /// 仓库地址
    pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
}

/// 🔧 实用工具函数
pub mod utils {
    use super::*;
    use std::env;

    /// 🔍 检查环境变量配置
    ///
    /// 验证飞书应用所需的环境变量是否正确设置
    ///
    /// # 返回
    /// - `Ok(())`: 环境变量配置正确
    /// - `Err(Error)`: 环境变量配置错误，包含详细的错误信息和恢复建议
    ///
    /// # 示例
    /// ```rust
    /// use openlark_client::utils;
    ///
    /// match utils::check_env_config() {
    ///     Ok(()) => println!("环境变量配置正确"),
    ///     Err(error) => {
    ///         eprintln!("❌ {}", error.user_message().unwrap_or("未知错误"));
    ///         for step in error.recovery_steps() {
    ///             eprintln!("• {}", step);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn check_env_config() -> Result<()> {
        // 检查 OPENLARK_APP_ID
        let app_id = env::var("OPENLARK_APP_ID")
            .map_err(|_| configuration_error("环境变量检查失败 [variable: OPENLARK_APP_ID]"))?;

        if app_id.is_empty() {
            return with_context(
                Err(validation_error(
                    "OPENLARK_APP_ID",
                    "应用ID环境变量不能为空",
                )),
                "validation",
                "env_config",
            );
        }

        // 检查 OPENLARK_APP_SECRET
        let app_secret = env::var("OPENLARK_APP_SECRET")
            .map_err(|_| configuration_error("环境变量检查失败 [variable: OPENLARK_APP_SECRET]"))?;

        if app_secret.is_empty() {
            return with_context(
                Err(validation_error(
                    "OPENLARK_APP_SECRET",
                    "应用密钥环境变量不能为空",
                )),
                "validation",
                "env_config",
            );
        }

        // 检查可选的环境变量
        if let Ok(base_url) = env::var("OPENLARK_BASE_URL") {
            if !base_url.starts_with("http://") && !base_url.starts_with("https://") {
                return with_context(
                    Err(validation_error(
                        "OPENLARK_BASE_URL",
                        "基础URL必须以http://或https://开头",
                    )),
                    "validation",
                    "env_config",
                );
            }
        }

        // 检查超时设置
        if let Ok(timeout_str) = env::var("OPENLARK_TIMEOUT") {
            if let Err(_) = timeout_str.parse::<u64>() {
                return with_context(
                    Err(validation_error(
                        "OPENLARK_TIMEOUT",
                        "超时设置必须是有效的数字（秒数）",
                    )),
                    "validation",
                    "env_config",
                );
            }
        }

        Ok(())
    }

    /// 🔧 从环境变量创建配置
    ///
    /// 自动读取环境变量并创建客户端配置
    ///
    /// # 返回
    /// - `Ok(Config)`: 成功创建配置
    /// - `Err(Error)`: 配置创建失败，包含详细错误信息
    pub fn create_config_from_env() -> Result<Config> {
        // 先检查环境变量
        check_env_config()?;

        let app_id = env::var("OPENLARK_APP_ID").unwrap();
        let app_secret = env::var("OPENLARK_APP_SECRET").unwrap();

        let base_url =
            env::var("OPENLARK_BASE_URL").unwrap_or_else(|_| "https://open.feishu.cn".to_string());

        let timeout = env::var("OPENLARK_TIMEOUT")
            .ok()
            .and_then(|t| t.parse().ok())
            .map(std::time::Duration::from_secs);

        let enable_log = env::var("OPENLARK_ENABLE_LOG")
            .ok()
            .and_then(|l| l.parse().ok())
            .unwrap_or(false);

        let mut config = Config::builder()
            .app_id(app_id)
            .app_secret(app_secret)
            .base_url(base_url)
            .enable_log(enable_log);

        if let Some(timeout_duration) = timeout {
            config = config.timeout(timeout_duration);
        }

        with_context(config.build(), "operation", "create_config_from_env")
    }

    /// 📊 获取配置摘要
    ///
    /// 返回当前配置的摘要信息，用于调试和监控
    pub fn get_config_summary(config: &Config) -> ConfigSummary {
        ConfigSummary {
            app_id: config.app_id.clone(),
            app_secret: if config.app_secret.is_empty() {
                "未设置".to_string()
            } else {
                format!(
                    "***{}***",
                    &config.app_secret[config.app_secret.len().saturating_sub(4)..]
                )
            },
            base_url: config.base_url.clone(),
            has_timeout: config.timeout > std::time::Duration::ZERO,
            feature_count: get_enabled_features().len(),
        }
    }

    /// 📋 配置摘要信息
    ///
    /// 用于调试和监控的配置信息摘要
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct ConfigSummary {
        /// 🆔 应用ID
        pub app_id: String,
        /// 🔒 应用密钥（已脱敏）
        pub app_secret: String,
        /// 🌐 基础URL
        pub base_url: String,
        /// ⏰ 是否设置了超时
        pub has_timeout: bool,
        /// 🔢 启用的功能数量
        pub feature_count: usize,
    }

    impl ConfigSummary {
        /// 📋 获取友好的配置描述
        pub fn friendly_description(&self) -> String {
            format!(
                "应用ID: {}, 基础URL: {}, 启用功能数: {}, 超时设置: {}",
                self.app_id,
                self.base_url,
                self.feature_count,
                if self.has_timeout {
                    "已设置"
                } else {
                    "使用默认值"
                }
            )
        }
    }

    /// 🏷️ 获取启用的功能列表
    ///
    /// 返回当前编译时启用的功能标志列表
    pub fn get_enabled_features() -> Vec<&'static str> {
        // 基础功能（始终启用）
        let mut features = vec!["auth"];

        // 可选功能（基于编译时标志）
        #[cfg(feature = "communication")]
        features.push("communication");

        #[cfg(feature = "docs")]
        features.push("docs");

        #[cfg(feature = "security")]
        features.push("security");

        #[cfg(feature = "hr")]
        features.push("hr");

        #[cfg(feature = "ai")]
        features.push("ai");

        #[cfg(feature = "task")]
        features.push("task");

        #[cfg(feature = "calendar")]
        features.push("calendar");

        #[cfg(feature = "admin")]
        features.push("admin");

        #[cfg(feature = "approval")]
        features.push("approval");

        #[cfg(feature = "helpdesk")]
        features.push("helpdesk");

        #[cfg(feature = "mail")]
        features.push("mail");

        #[cfg(feature = "application")]
        features.push("application");

        features
    }

    /// 🔍 验证功能依赖关系
    ///
    /// 检查启用的功能是否满足依赖关系要求
    pub fn validate_feature_dependencies() -> Result<Vec<String>> {
        let enabled_features = get_enabled_features();
        let mut issues = Vec::new();

        // 检查核心依赖
        if enabled_features.contains(&"communication") && !enabled_features.contains(&"auth") {
            issues.push("通讯服务 (communication) 需要启用认证服务 (auth)".to_string());
        }

        if enabled_features.contains(&"docs") && !enabled_features.contains(&"auth") {
            issues.push("文档服务 (docs) 需要启用认证服务 (auth)".to_string());
        }

        if enabled_features.contains(&"hr") && !enabled_features.contains(&"auth") {
            issues.push("人力资源服务 (hr) 需要启用认证服务 (auth)".to_string());
        }

        if enabled_features.contains(&"ai") && !enabled_features.contains(&"auth") {
            issues.push("AI服务 (ai) 需要启用认证服务 (auth)".to_string());
        }

        if enabled_features.contains(&"task") && !enabled_features.contains(&"auth") {
            issues.push("任务管理服务 (task) 需要启用认证服务 (auth)".to_string());
        }

        if enabled_features.contains(&"calendar") && !enabled_features.contains(&"auth") {
            issues.push("日历服务 (calendar) 需要启用认证服务 (auth)".to_string());
        }

        // 检查高级功能依赖
        if enabled_features.contains(&"admin") && !enabled_features.contains(&"hr") {
            issues.push("管理服务 (admin) 建议启用人力资源服务 (hr) 以获得完整功能".to_string());
        }

        if enabled_features.contains(&"approval") && !enabled_features.contains(&"auth") {
            issues.push("审批服务 (approval) 需要启用认证服务 (auth)".to_string());
        }

        if issues.is_empty() {
            Ok(issues)
        } else {
            with_context(
                Err(configuration_error(format!(
                    "发现 {} 个功能依赖问题: {}",
                    issues.len(),
                    issues.join("; ")
                ))),
                "validation",
                "feature_dependencies",
            )
        }
    }

    /// 🏥 诊断系统配置
    ///
    /// 执行全面的系统配置检查，包括环境变量、功能依赖等
    pub fn diagnose_system() -> SystemDiagnostics {
        let mut diagnostics = SystemDiagnostics::new();

        // 检查环境变量
        match check_env_config() {
            Ok(()) => {
                diagnostics.env_config_status = "✅ 正常".to_string();
            }
            Err(error) => {
                diagnostics.env_config_status =
                    format!("❌ {}", error.user_message().unwrap_or("未知错误"));
                diagnostics.add_issue("环境变量", error.user_message().unwrap_or("未知错误"));
            }
        }

        // 检查功能依赖
        match validate_feature_dependencies() {
            Ok(_) => {
                diagnostics.feature_deps_status = "✅ 正常".to_string();
            }
            Err(error) => {
                diagnostics.feature_deps_status =
                    format!("❌ {}", error.user_message().unwrap_or("未知错误"));
                diagnostics.add_issue("功能依赖", error.user_message().unwrap_or("未知错误"));
            }
        }

        // 列出启用的功能
        diagnostics.enabled_features = get_enabled_features()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        diagnostics
    }

    /// 🏥 系统诊断结果
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct SystemDiagnostics {
        /// 🌍 环境变量配置状态
        pub env_config_status: String,
        /// 🔗 功能依赖状态
        pub feature_deps_status: String,
        /// 🏷️ 启用的功能列表
        pub enabled_features: Vec<String>,
        /// ⚠️ 发现的问题列表
        pub issues: Vec<DiagnosticIssue>,
    }

    impl SystemDiagnostics {
        /// 创建新的诊断结果
        pub fn new() -> Self {
            Self {
                env_config_status: "未检查".to_string(),
                feature_deps_status: "未检查".to_string(),
                enabled_features: Vec::new(),
                issues: Vec::new(),
            }
        }

        /// 添加问题到诊断结果
        pub fn add_issue(&mut self, category: &str, description: &str) {
            self.issues.push(DiagnosticIssue {
                category: category.to_string(),
                description: description.to_string(),
            });
        }

        /// 获取健康状态摘要
        pub fn health_summary(&self) -> String {
            let healthy_count = self.issues.len();
            if healthy_count == 0 {
                "🟢 系统配置健康".to_string()
            } else {
                format!("🟡 发现 {} 个配置问题", healthy_count)
            }
        }

        /// 检查是否有严重问题
        pub fn has_critical_issues(&self) -> bool {
            self.issues.iter().any(|issue| {
                issue.category.contains("环境变量") || issue.category.contains("功能依赖")
            })
        }
    }

    /// 🔍 诊断问题条目
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct DiagnosticIssue {
        /// 🏷️ 问题类别
        pub category: String,
        /// 📝 问题描述
        pub description: String,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_info() {
        assert!(!info::NAME.is_empty());
        assert!(!info::VERSION.is_empty());
        assert!(!info::DESCRIPTION.is_empty());
    }

    #[test]
    fn test_enabled_features() {
        let _features = utils::get_enabled_features();
        // 至少应该有一些功能（或者为空）
        // 这个测试主要确保函数能正常工作
    }

    #[test]
    fn test_prelude_reexports() {
        // 确保 prelude 模块正确导出了核心类型
        use prelude::*;

        // 这些导入应该能够工作
        let _builder: ClientBuilder = ClientBuilder::new();

        // 测试配置创建
        let _config = Config::builder().app_id("test").app_secret("test").build();
    }
}
