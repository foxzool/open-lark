//! Bitable App Table Field API模块
//!
//! 提供多维表格字段管理相关的功能，包括：
//! - 字段的创建、查询、更新、删除
//! - 字段类型配置和属性设置
//! - 字段顺序调整和批量操作
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::base::bitable::app_table_field::{AppTableFieldService, CreateFieldRequest};
//!
//! let service = AppTableFieldService::new(config);
//!
//! // 创建新字段
//! let response = service
//!     .create_field_builder()
//!     .app_token("app_token_xxx")
//!     .table_id("table_id_xxx")
//!     .field_name("任务优先级")
//!     .field_type("select")
//!     .property(json!({
//!         "options": [
//!             {"name": "高", "color": "red"},
//!             {"name": "中", "color": "yellow"},
//!             {"name": "低", "color": "green"}
//!         ]
//!     }))
//!     .execute(&service)
//!     .await?;
//!
//! if let Some(field) = response.field {
//!     println!("创建成功: field_id={}", field.field_id.unwrap_or_default());
//! }
//! ```

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

// 重新导出主要类型
pub use models::*;
pub use services::{AppTableFieldService, CreateFieldRequestBuilder, UpdateFieldRequestBuilder};
