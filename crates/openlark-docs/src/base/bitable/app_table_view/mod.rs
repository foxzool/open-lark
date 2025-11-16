//! Bitable App Table View API模块
//!
//! 提供多维表格视图管理相关的功能，包括：
//! - 视图的创建、查询、更新、删除
//! - 视图类型配置和样式设置
//! - 视图权限管理和共享设置
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::base::bitable::app_table_view::{AppTableViewService, CreateViewRequest};
//!
//! let service = AppTableViewService::new(config);
//!
//! // 创建新视图
//! let response = service
//!     .create_view_builder()
//!     .app_token("app_token_xxx")
//!     .table_id("table_id_xxx")
//!     .view_name("任务看板")
//!     .view_type("kanban")
//!     .execute(&service)
//!     .await?;
//!
//! if let Some(view) = response.view {
//!     println!("创建成功: view_id={}", view.view_id.unwrap_or_default());
//! }
//! ```

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

// 重新导出主要类型
pub use models::*;
pub use services::{AppTableViewService, CreateViewRequestBuilder, UpdateViewRequestBuilder};
