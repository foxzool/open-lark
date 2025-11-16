//! Bitable App Table Record API模块
//!
//! 提供多维表格数据记录管理相关的功能，包括：
//! - 记录的创建、查询、更新、删除
//! - 批量记录操作支持
//! - 记录搜索和过滤
//! - 字段值更新和类型转换
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::base::bitable::app_table_record::{AppTableRecordService, CreateRecordRequest};
//!
//! let service = AppTableRecordService::new(config);
//!
//! // 创建新记录
//! let response = service
//!     .create_record_builder()
//!     .app_token("app_token_xxx")
//!     .table_id("table_id_xxx")
//!     .fields(json!({
//!         "title": "新任务",
//!         "status": "未开始",
//!         "priority": "高"
//!     }))
//!     .execute(&service)
//!     .await?;
//!
//! if let Some(record) = response.record {
//!     println!("创建成功: record_id={}", record.record_id.unwrap_or_default());
//! }
//! ```

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

// 重新导出主要类型
pub use models::*;
pub use services::{AppTableRecordService, CreateRecordRequestBuilder, UpdateRecordRequestBuilder};
