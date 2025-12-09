//! Docs V1 API 模块
//!
//! 提供云文档内容管理功能。

pub mod content;
pub mod models;
pub mod service;

// 导出API和模型
pub use content::get::{GetDocsContentRequest, GetDocsContentParams, GetDocsContentResponse};
pub use models::*;

// 重新导出主要服务
pub use service::DocsService;