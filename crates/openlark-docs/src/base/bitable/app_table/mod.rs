//! Bitable App Table API模块
//!
//! 提供多维表格数据表管理相关的功能，包括：
//! - 数据表的创建、查询、更新、删除
//! - 批量操作支持
//! - 完整的字段管理
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::base::bitable::app_table::{AppTableService, CreateTableRequest};
//!
//! let service = AppTableService::new(config);
//!
//! // 创建新的数据表
//! let response = service
//!     .create_table_builder()
//!     .app_token("app_token_xxx")
//!     .name("项目任务表")
//!     .description("用于跟踪项目任务进度")
//!     .is_advanced(true)
//!     .execute(&service)
//!     .await?;
//!
//! if let Some(table) = response.table {
//!     println!("创建成功: table_id={}", table.table_id.unwrap_or_default());
//! }
//! ```

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

// 重新导出主要类型
pub use models::*;
pub use services::{AppTableService, CreateTableRequestBuilder, UpdateTableRequestBuilder};
