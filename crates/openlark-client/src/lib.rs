//! OpenLark Client Library
//!
//! 🚀 现代化的飞书开放平台 Rust SDK，提供简洁、类型安全的 API 访问
//!
//! ## 核心特性
//!
//! - **Feature-driven**: 基于编译时功能标志的模块化设计
//! - **零配置**: 支持从环境变量自动配置客户端
//! - **类型安全**: 完全编译时验证的 API 调用
//! - **异步优先**: 完全异步的客户端实现
//! - **现代构建器**: 流畅的构建器模式 API
//! - **服务发现**: 动态服务注册和管理
//! - **企业级**: 高级错误处理、重试和监控支持
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
//! 客户端提供统一的错误处理：
//!
//! ```rust,no_run
//! use openlark_client::prelude::*;
//!
//! match Client::from_env() {
//!     Ok(client) => {
//!         println!("客户端创建成功");
//!         // 使用客户端...
//!     },
//!     Err(Error::InvalidConfig(msg)) => {
//!         eprintln!("配置错误: {}", msg);
//!     },
//!     Err(error) => {
//!         eprintln!("其他错误: {}", error);
//!     }
//! }
//! ```

#![deny(missing_docs)]
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
#[cfg(feature = "websocket")]
pub mod ws_client;

// 重新导出核心类型
pub use client::{Client, ClientBuilder};
pub use config::Config;
pub use error::{Error, Result};
pub use features::{FeatureLoader, FeatureSet, FeatureStats};
pub use registry::{ServiceRegistry, ServiceDescriptor};
pub use traits::*;

// 重新导出服务类型
#[cfg(feature = "communication")]
pub use services::CommunicationService;

#[cfg(feature = "hr")]
pub use services::HRService;

#[cfg(feature = "docs")]
pub use services::DocsService;

#[cfg(feature = "ai")]
pub use services::AIService;

#[cfg(feature = "auth")]
pub use services::AuthService;

// 重新导出 openlark-core 核心类型
pub use openlark_core::{
    SDKResult as CoreResult,
    config::Config as CoreConfig,
};

/// 🚀 预导出模块 - 包含最常用的类型和特征
///
/// 使用预导出可以简化导入：
///
/// ```rust,no_run
/// use openlark_client::prelude::*;
///
/// let client = Client::from_env()?;
/// ```
pub mod prelude {
    // 核心类型
    pub use crate::{Client, ClientBuilder, Config, Error, Result};

    // 服务特征
    pub use crate::traits::{LarkClient, ServiceTrait, ServiceLifecycle};

    // 服务注册
    pub use crate::{ServiceRegistry, ServiceDescriptor};

    // 服务类型
    #[cfg(feature = "communication")]
    pub use crate::services::CommunicationService;

    #[cfg(feature = "hr")]
    pub use crate::services::HRService;

    #[cfg(feature = "docs")]
    pub use crate::services::DocsService;

    #[cfg(feature = "ai")]
    pub use crate::services::AIService;

    #[cfg(feature = "auth")]
    pub use crate::services::AuthService;

    // 功能管理
    pub use crate::{FeatureLoader, FeatureSet};

    // 便利类型别名
    /// 📦 客户端结果类型别名
    pub type ClientResult<T> = Result<T>;
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

    /// 检查环境变量配置
    pub fn check_env_config() -> Result<()> {
        let app_id = env::var("OPENLARK_APP_ID")
            .map_err(|_| Error::InvalidConfig("OPENLARK_APP_ID 环境变量未设置"))?;

        let app_secret = env::var("OPENLARK_APP_SECRET")
            .map_err(|_| Error::InvalidConfig("OPENLARK_APP_SECRET 环境变量未设置"))?;

        if app_id.is_empty() {
            return Err(Error::InvalidConfig("OPENLARK_APP_ID 不能为空"));
        }

        if app_secret.is_empty() {
            return Err(Error::InvalidConfig("OPENLARK_APP_SECRET 不能为空"));
        }

        Ok(())
    }

    /// 获取启用的功能列表
    pub fn get_enabled_features() -> Vec<&'static str> {
        FeatureLoader::get_enabled_services()
    }

    /// 验证功能依赖
    pub fn validate_feature_dependencies() -> Result<Vec<crate::features::DependencyIssue>> {
        FeatureLoader::validate_feature_dependencies().map_err(|e| {
            Error::InvalidConfig("功能依赖验证失败")
        })
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
        let features = utils::get_enabled_features();
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
        let _config = Config::builder()
            .app_id("test")
            .app_secret("test")
            .build();
    }
}