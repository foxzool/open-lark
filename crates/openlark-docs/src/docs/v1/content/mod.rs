//! 云文档内容管理模块

pub mod get;

// 导出API
pub use get::{GetDocsContentRequest, GetDocsContentParams, GetDocsContentResponse};