//! Docx V1 API 模块
//!
//! 提供群公告和文档内容管理功能。

pub mod chat;
pub mod document;
pub mod models;
pub mod service;

// 重新导出主要服务
pub use service::DocxService;
pub use models::*;