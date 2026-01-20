//! # OpenLark 任务模块
//!
//! OpenLark SDK 的任务/待办事项模块，提供飞书任务 API 的完整访问。
//!
//! ## 功能特性
//!
//! - **任务管理**: 创建、更新、删除、查询待办事项
//! - **协作支持**: 添加执行者、关注者、提醒
//! - **任务清单**: v2 API 的任务清单管理
//! - **自定义字段**: v2 API 的自定义字段支持
//! - **版本支持**: 支持 v1 和 v2 两种 API 版本
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use openlark_task::TaskService;
//! use openlark_core::config::Config;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = Config::builder()
//!     .app_id("your_app_id")
//!     .app_secret("your_app_secret")
//!     .build();
//!
//! let task_service = TaskService::new(config);
//!
//! // 创建任务
//! let result = task_service
//!     .v2()
//!     .task()
//!     .create()
//!     .summary("完成项目文档")
//!     .execute()
//!     .await?;
//! # Ok(())
//! # }
//! ```

#![allow(missing_docs)]

mod service;

// 通用模块
pub mod common;

// 版本模块
#[cfg(feature = "v1")]
pub mod v1;

#[cfg(feature = "v2")]
pub mod v2;

// Prelude 模块
pub mod prelude;

// 重新导出核心服务
pub use service::TaskService;

/// 任务模块版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
