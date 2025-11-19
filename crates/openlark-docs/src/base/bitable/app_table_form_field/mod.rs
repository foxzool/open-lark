//! Bitable App Table Form Field API模块
//!
//! 提供多维表格表单字段管理相关的功能，包括：
//! - 表单字段的列表查询和更新
//! - 表单字段类型配置
//! - 表单字段验证规则设置
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::base::bitable::app_table_form_field::{AppTableFormFieldService, ListFormFieldRequest};
//!
//! let service = AppTableFormFieldService::new(config);
//!
//! // 列出表单字段
//! let response = service
//!     .list_form_field_builder()
//!     .app_token("app_token_xxx")
//!     .table_id("table_id_xxx")
//!     .form_id("form_id_xxx")
//!     .execute(&service)
//!     .await?;
//!
//! if let Some(fields) = response.fields {
//!     println!("表单字段数量: {}", fields.len());
//! }
//! ```

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

// 重新导出主要类型
pub use models::*;
pub use services::{
    AppTableFormFieldService, ListFormFieldRequestBuilder, PatchFormFieldRequestBuilder,
};
