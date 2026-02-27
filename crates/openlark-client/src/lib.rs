//! 🚀 OpenLark Client Library
//!
//! 现代化的飞书开放平台 Rust SDK，提供简洁、类型安全的 API 访问
//! 集成 CoreError 企业级错误处理系统，提供全面的错误管理和恢复建议
//!
//! ## 核心特性

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
//!     // 单入口：meta 链式字段访问（需要对应 feature）
//!     // - 通讯：client.communication.im...
//!     // - 文档：client.docs.ccm...
//!     // - 认证：client.auth.app / client.auth.user / client.auth.oauth
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
//! fn main() -> Result<()> {
//!     let _client = Client::builder()
//!         .app_id("your_app_id")
//!         .app_secret("your_app_secret")
//!         .base_url("https://open.feishu.cn")
//!         .timeout(Duration::from_secs(30))
//!         .enable_log(true)
//!         .build()?;
//!     Ok(())
//! }
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
//! 每个启用功能都提供对应的 meta 链式入口（字段访问）：
//!
//! ```rust,no_run
//! use openlark_client::prelude::*;
//!
//! fn main() -> Result<()> {
//! let client = Client::from_env()?;
//!
//! // 通讯入口（communication feature）
//! #[cfg(feature = "communication")]
//! let _comm = &client.communication;
//!
//! // 文档入口（docs feature）
//! #[cfg(feature = "docs")]
//! let _docs = &client.docs;
//!
//! // 认证入口（auth feature）
//! #[cfg(feature = "auth")]
//! let _auth = &client.auth;
//! Ok(())
//! }
//! ```
//!
//! ## 高级用法
//!
//! ### 服务注册和管理
//!
//! ```rust,no_run
//! use openlark_client::prelude::*;
//!
//! fn main() -> Result<()> {
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
//! Ok(())
//! }
//! ```
//!
//! ### 自定义配置
//!
//! ```rust,no_run
//! use openlark_client::prelude::*;
//! use std::time::Duration;
//!
//! fn main() -> Result<()> {
//!     let _client = Client::builder()
//!         .app_id("app_id")
//!         .app_secret("app_secret")
//!         .base_url("https://open.feishu.cn")
//!         .timeout(Duration::from_secs(60))
//!         .retry_count(3)
//!         .enable_log(true)
//!         .build()?;
//!     Ok(())
//! }
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
//!         eprintln!("\n{}", ErrorAnalyzer::new(&error).detailed_report());
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
//!     // 单入口：meta 链式字段访问。这里演示“拿到入口 + 挂上错误上下文”的模式。
//!     #[cfg(feature = "communication")]
//!     let _comm = &client.communication;
//!
//!     // 具体 API 调用请使用 openlark-communication 的强类型请求/构建器并在 `.await` 处处理 Result。
//!     Ok(())
//! }
//! ```

//#![deny(missing_docs)]  // 暂时禁用以完成基本编译
// async_fn_in_trait: 保留以兼容 MSRV 1.75（该 lint 在 Rust 1.80+ 才稳定）
#![allow(async_fn_in_trait)]

// 核心模块
pub mod client;
pub mod config;
mod core_config;
pub mod error;
pub mod features;
pub mod registry;
pub mod traits;
pub mod types;

#[cfg(test)]
mod test_utils;

// meta.Project 维度的 API 调用链（数据源：api_list_export.csv）
// CardKit 由 openlark-cardkit 提供链式调用；openlark-client 仅负责挂载到 Client 上。

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
pub use features::FeatureLoader;
pub use registry::{
    DefaultServiceRegistry, ServiceEntry, ServiceMetadata, ServiceRegistry, ServiceStatus,
};

// 客户端特征
pub use traits::{LarkClient, ServiceLifecycle};

// 注意：legacy_client 已在 v0.15.0 中移除
// 请使用 `Client` 与 `ClientBuilder`
// 迁移指南：https://github.com/foxzool/open-lark/blob/main/docs/migration-guide.md

// CardKit meta 调用链
#[cfg(feature = "cardkit")]
pub use openlark_cardkit::CardkitClient;

// Docs / Communication / Meeting meta 调用链入口（字段链式）
#[cfg(feature = "auth")]
pub use client::AuthClient;

#[cfg(feature = "docs")]
pub use openlark_docs::DocsClient;

#[cfg(feature = "communication")]
pub use openlark_communication::CommunicationClient;

#[cfg(feature = "meeting")]
pub use openlark_meeting::MeetingClient;

// 其他服务（当前未启用但已规划）
//（历史上曾尝试在 openlark-client 内重复实现业务服务包装层，但现已收敛为 meta 单入口。）

// ============================================================================
// Core 系统类型重新导出
// ============================================================================

// 重新导出 openlark-core 核心类型
pub use openlark_core::{config::Config as CoreConfig, SDKResult as CoreResult};

// 错误系统核心类型
pub use openlark_core::error::{CoreError, ErrorCode, ErrorSeverity, ErrorTrait, ErrorType};

// ============================================================================
// 类型别名和便利定义
// ============================================================================

/// 🚨 SDK 结果类型别名（与 Core 系统兼容）
pub type SDKResult<T> = openlark_core::SDKResult<T>;

/// 🚀 预导出模块 - 包含最常用的类型和特征
///
/// 使用预导出可以简化导入，提供一站式类型访问：
///
/// ```rust,no_run
/// use openlark_client::prelude::*;
///
/// fn main() -> Result<()> {
///     let client = Client::from_env()?;
///     #[cfg(feature = "docs")]
///     let _docs = &client.docs;
///     Ok(())
/// }
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

    pub use crate::FeatureLoader;

    // meta 风格链式入口（字段链式）
    #[cfg(feature = "cardkit")]
    pub use openlark_cardkit::CardkitClient;

    #[cfg(feature = "auth")]
    pub use crate::AuthClient;

    #[cfg(feature = "docs")]
    pub use openlark_docs::DocsClient;

    #[cfg(feature = "communication")]
    pub use openlark_communication::CommunicationClient;

    #[cfg(feature = "meeting")]
    pub use openlark_meeting::MeetingClient;

    // 其他服务（当前未启用但已规划）
    //（历史上曾尝试在 openlark-client 内重复实现业务服务包装层，但现已收敛为 meta 单入口。）

    // ============================================================================
    // 便利类型别名
    // ============================================================================

    /// 🚨 SDK 结果类型别名（与 Core 系统兼容）
    pub type SDKResult<T> = openlark_core::SDKResult<T>;

    // ============================================================================
    // 常用宏和便利导入
    // ============================================================================

    pub use openlark_core::{config::Config as CoreConfig, SDKResult as CoreResult};
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
pub mod utils;

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
        let features = utils::get_enabled_features();
        // auth 功能始终启用
        assert!(features.contains(&"auth"));
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

    #[test]
    fn test_check_env_config_success() {
        test_utils::with_env_vars(
            &[
                ("OPENLARK_APP_ID", Some("test_app_id")),
                ("OPENLARK_APP_SECRET", Some("test_secret")),
            ],
            || {
                let result = utils::check_env_config();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    fn test_check_env_config_missing_app_id() {
        test_utils::with_env_vars(
            &[
                ("OPENLARK_APP_ID", None),
                ("OPENLARK_APP_SECRET", Some("test_secret")),
            ],
            || {
                let result = utils::check_env_config();
                assert!(result.is_err());
            },
        );
    }

    #[test]
    fn test_check_env_config_empty_app_id() {
        test_utils::with_env_vars(
            &[
                ("OPENLARK_APP_ID", Some("")),
                ("OPENLARK_APP_SECRET", Some("test_secret")),
            ],
            || {
                let result = utils::check_env_config();
                assert!(result.is_err());
            },
        );
    }

    #[test]
    fn test_check_env_config_missing_app_secret() {
        test_utils::with_env_vars(
            &[
                ("OPENLARK_APP_ID", Some("test_app_id")),
                ("OPENLARK_APP_SECRET", None),
            ],
            || {
                let result = utils::check_env_config();
                assert!(result.is_err());
            },
        );
    }

    #[test]
    fn test_check_env_config_empty_app_secret() {
        test_utils::with_env_vars(
            &[
                ("OPENLARK_APP_ID", Some("test_app_id")),
                ("OPENLARK_APP_SECRET", Some("")),
            ],
            || {
                let result = utils::check_env_config();
                assert!(result.is_err());
            },
        );
    }

    #[test]
    fn test_check_env_config_invalid_base_url() {
        test_utils::with_env_vars(
            &[
                ("OPENLARK_APP_ID", Some("test_app_id")),
                ("OPENLARK_APP_SECRET", Some("test_secret")),
                ("OPENLARK_BASE_URL", Some("invalid_url")),
            ],
            || {
                let result = utils::check_env_config();
                assert!(result.is_err());
            },
        );
    }

    #[test]
    fn test_check_env_config_valid_base_url() {
        test_utils::with_env_vars(
            &[
                ("OPENLARK_APP_ID", Some("test_app_id")),
                ("OPENLARK_APP_SECRET", Some("test_secret")),
                ("OPENLARK_BASE_URL", Some("https://open.feishu.cn")),
            ],
            || {
                let result = utils::check_env_config();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    fn test_check_env_config_invalid_timeout() {
        test_utils::with_env_vars(
            &[
                ("OPENLARK_APP_ID", Some("test_app_id")),
                ("OPENLARK_APP_SECRET", Some("test_secret")),
                ("OPENLARK_TIMEOUT", Some("not_a_number")),
            ],
            || {
                let result = utils::check_env_config();
                assert!(result.is_err());
            },
        );
    }

    #[test]
    fn test_check_env_config_valid_timeout() {
        test_utils::with_env_vars(
            &[
                ("OPENLARK_APP_ID", Some("test_app_id")),
                ("OPENLARK_APP_SECRET", Some("test_secret")),
                ("OPENLARK_TIMEOUT", Some("30")),
            ],
            || {
                let result = utils::check_env_config();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    fn test_create_config_from_env_success() {
        test_utils::with_env_vars(
            &[
                ("OPENLARK_APP_ID", Some("test_app_id")),
                ("OPENLARK_APP_SECRET", Some("test_secret")),
                ("OPENLARK_BASE_URL", Some("https://open.feishu.cn")),
            ],
            || {
                let result = utils::create_config_from_env();
                assert!(result.is_ok());
                let config = result.unwrap();
                assert_eq!(config.app_id, "test_app_id");
                assert_eq!(config.app_secret, "test_secret");
            },
        );
    }

    #[test]
    fn test_create_config_from_env_missing_vars() {
        test_utils::with_env_vars(
            &[("OPENLARK_APP_ID", None), ("OPENLARK_APP_SECRET", None)],
            || {
                let result = utils::create_config_from_env();
                assert!(result.is_err());
            },
        );
    }

    #[test]
    fn test_get_config_summary() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_secret_key")
            .base_url("https://open.feishu.cn")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();

        let summary = utils::get_config_summary(&config);
        assert_eq!(summary.app_id, "test_app_id");
        assert!(summary.app_secret_set);
        assert_eq!(summary.base_url, "https://open.feishu.cn");
        assert!(summary.timeout > std::time::Duration::ZERO);
    }

    #[test]
    fn test_config_summary_friendly_description() {
        let summary = config::ConfigSummary {
            app_id: "test_app".to_string(),
            app_secret_set: true,
            app_type: openlark_core::constants::AppType::SelfBuild,
            enable_token_cache: true,
            base_url: "https://open.feishu.cn".to_string(),
            timeout: std::time::Duration::from_secs(30),
            retry_count: 3,
            enable_log: false,
            header_count: 0,
        };

        let description = summary.friendly_description();
        assert!(description.contains("test_app"));
        assert!(description.contains("open.feishu.cn"));
        assert!(description.contains("30s"));
    }

    #[test]
    fn test_config_summary_friendly_description_no_timeout() {
        let summary = config::ConfigSummary {
            app_id: "test_app".to_string(),
            app_secret_set: true,
            app_type: openlark_core::constants::AppType::SelfBuild,
            enable_token_cache: true,
            base_url: "https://open.feishu.cn".to_string(),
            timeout: std::time::Duration::ZERO,
            retry_count: 3,
            enable_log: false,
            header_count: 0,
        };

        let description = summary.friendly_description();
        assert!(description.contains("test_app"));
        assert!(description.contains("0ns"));
    }

    #[test]
    fn test_validate_feature_dependencies_success() {
        // auth 始终启用，应该没有依赖问题
        let result = utils::validate_feature_dependencies();
        assert!(result.is_ok());
    }

    #[test]
    fn test_diagnose_system_success() {
        test_utils::with_env_vars(
            &[
                ("OPENLARK_APP_ID", Some("test_app_id")),
                ("OPENLARK_APP_SECRET", Some("test_secret")),
            ],
            || {
                let diagnostics = utils::diagnose_system();
                assert!(
                    diagnostics.env_config_status.contains("✅")
                        || diagnostics.env_config_status.contains("❌")
                );
                assert!(diagnostics.feature_deps_status.contains("✅"));
                assert!(!diagnostics.enabled_features.is_empty());
            },
        );
    }

    #[test]
    fn test_system_diagnostics_new() {
        let diagnostics = utils::SystemDiagnostics::new();
        assert_eq!(diagnostics.env_config_status, "未检查");
        assert_eq!(diagnostics.feature_deps_status, "未检查");
        assert!(diagnostics.enabled_features.is_empty());
        assert!(diagnostics.issues.is_empty());
    }

    #[test]
    fn test_system_diagnostics_add_issue() {
        let mut diagnostics = utils::SystemDiagnostics::new();
        diagnostics.add_issue("测试类别", "测试描述");
        assert_eq!(diagnostics.issues.len(), 1);
        assert_eq!(diagnostics.issues[0].category, "测试类别");
        assert_eq!(diagnostics.issues[0].description, "测试描述");
    }

    #[test]
    fn test_system_diagnostics_health_summary_healthy() {
        let diagnostics = utils::SystemDiagnostics::new();
        let summary = diagnostics.health_summary();
        assert!(summary.contains("🟢"));
        assert!(summary.contains("健康"));
    }

    #[test]
    fn test_system_diagnostics_health_summary_with_issues() {
        let mut diagnostics = utils::SystemDiagnostics::new();
        diagnostics.add_issue("测试类别", "测试描述");
        let summary = diagnostics.health_summary();
        assert!(summary.contains("🟡"));
        assert!(summary.contains("1"));
    }

    #[test]
    fn test_system_diagnostics_has_critical_issues_true() {
        let mut diagnostics = utils::SystemDiagnostics::new();
        diagnostics.add_issue("环境变量", "配置错误");
        assert!(diagnostics.has_critical_issues());
    }

    #[test]
    fn test_system_diagnostics_has_critical_issues_false() {
        let mut diagnostics = utils::SystemDiagnostics::new();
        diagnostics.add_issue("其他问题", "一般错误");
        assert!(!diagnostics.has_critical_issues());
    }

    #[test]
    fn test_system_diagnostics_default() {
        let diagnostics: utils::SystemDiagnostics = Default::default();
        assert_eq!(diagnostics.env_config_status, "未检查");
    }

    #[test]
    fn test_diagnostic_issue_clone() {
        let issue = utils::DiagnosticIssue {
            category: "测试".to_string(),
            description: "描述".to_string(),
        };
        let cloned = issue.clone();
        assert_eq!(cloned.category, "测试");
        assert_eq!(cloned.description, "描述");
    }
}
