//! # OpenLark 工作流模块
//!
//! OpenLark SDK 的工作流模块，提供飞书任务、审批和看板 API 的完整访问。
//!
//! ## 功能特性
//!
//! - **任务管理**: 创建、更新、删除、查询待办事项
//! - **审批流程**: 审批定义、审批实例管理
//! - **看板管理**: 看板创建、任务卡片管理
//! - **协作支持**: 添加执行者、关注者、提醒
//! - **版本支持**: 支持 v1 和 v2 两种 API 版本
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use openlark_workflow::WorkflowService;
//! use openlark_core::config::Config;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = Config::builder()
//!     .app_id("your_app_id")
//!     .app_secret("your_app_secret")
//!     .build();
//!
//! let workflow_service = WorkflowService::new(config);
//!
//! // 创建任务
//! let result = workflow_service
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

// 看板模块
#[cfg(feature = "board")]
pub mod board;

// Prelude 模块
pub mod prelude;

// 重新导出核心服务
pub use service::WorkflowService;

/// 工作流模块版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
