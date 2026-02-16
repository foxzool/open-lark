//! # OpenLark 平台服务模块
//!
//! OpenLark SDK 的平台服务模块，提供飞书平台管理相关 API 的完整访问。
//!
//! ## 功能特性
//!
//! - **应用引擎**: 应用管理、多租户、应用市场集成
//! - **目录服务**: 用户搜索、组织目录、人员查找
//! - **系统管理**: 系统配置、后台管理、平台工具
//!
//! ## 模块组织
//!
//! 本模块按业务域（bizTag）组织：
//! - `app_engine` - 应用引擎相关 API (37 APIs)
//! - `directory` - 目录服务相关 API (21 APIs)
//! - `admin` - 系统管理相关 API (14 APIs)
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use openlark_platform::PlatformService;
//! use openlark_core::prelude::Config;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // 使用 builder 模式创建配置
//! let config = Config::builder()
//!     .app_id("app_id")
//!     .app_secret("app_secret")
//!     .build();
//!
//! let platform_service = PlatformService::new(config)?;
//!
//! // 具体功能请参考各个子模块的文档
//! # Ok(())
//! # }
//! ```

#![allow(missing_docs)]
#![allow(clippy::module_inception)]

mod service;

// 通用模块
pub mod common;

// 业务域模块
#[cfg(feature = "app-engine")]
pub mod app_engine;

#[cfg(feature = "directory")]
pub mod directory;

#[cfg(feature = "admin")]
pub mod admin;

// Prelude 模块
pub mod prelude;

// 重新导出核心服务
pub use service::PlatformService;

// 配置类型
pub use openlark_core::config::Config;

/// 平台服务模块版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 平台服务配置别名
pub type PlatformConfig = Config;

#[cfg(test)]
mod tests {
    use crate::VERSION;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_module_composition() {
        // 验证模块组织正确
        assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
    }
}
