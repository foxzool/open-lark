//! Bitable App Form API模块
//!
//! 提供多维表格表单管理相关的功能，包括：
//! - 表单的创建、查询、更新、删除
//! - 表单字段和问题管理
//! - 表单配置和样式设置
//! - 表单提交和数据收集
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::base::bitable::app_form::{AppFormService, GetFormRequest};
//!
//! let service = AppFormService::new(config);
//!
//! // 获取表单信息
//! let response = service
//!     .get_form_builder()
//!     .app_token("app_token_xxx")
//!     .table_id("table_id_xxx")
//!     .form_id("form_id_xxx")
//!     .execute(&service)
//!     .await?;
//!
//! if let Some(form) = response.form {
//!     println!("表单名称: {}", form.title.unwrap_or_default());
//! }
//! ```

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

// 重新导出主要类型
pub use models::*;
pub use services::{
    AppFormService, GetFormRequestBuilder, PatchFormMetaRequestBuilder,
    PatchFormQuestionRequestBuilder,
};
