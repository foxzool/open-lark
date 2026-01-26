//! # OpenLark 用户设置模块
//!
//! OpenLark SDK 的用户设置模块，提供飞书个人设置和用户偏好相关 API 的完整访问。
//!
//! ## 功能特性
//!
//! - **个人设置**: 通知设置、隐私设置、界面设置
//! - **用户偏好**: 个人偏好、自定义选项、快捷键设置
//!
//! ## 模块组织
//!
//! 本模块按功能域组织：
//! - `settings` - 个人设置相关 API
//! - `preferences` - 用户偏好相关 API
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use openlark_user::UserService;
//! use openlark_core::prelude::Config;
//!
//! let config = Config::builder()
//!     .app_id("app_id")
//!     .app_secret("app_secret")
//!     .build();
//!
//! let user_service = UserService::new(config).unwrap();
//!
//! // 获取设置服务
//! # #[cfg(feature = "settings")]
//! let settings_service = user_service.settings();
//!
//! // 获取偏好服务
//! # #[cfg(feature = "preferences")]
//! let preferences_service = user_service.preferences();
//! ```

#![allow(missing_docs)]

mod service;

// 通用模块
pub mod common;

// 功能域模块
#[cfg(feature = "settings")]
pub mod settings;

#[cfg(feature = "preferences")]
pub mod preferences;

// Prelude 模块
pub mod prelude;

// 重新导出核心服务
pub use service::UserService;

// 配置类型
pub use openlark_core::config::Config;

/// 用户设置模块版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 用户服务配置别名
pub type UserConfig = Config;

#[cfg(test)]
mod tests {
    use super::*;

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
