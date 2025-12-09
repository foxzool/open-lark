//! Docs 云文档服务模块
//!
//! 提供企业云文档管理功能。

pub mod v1;

// 导出V1版本的主要服务
pub use v1::DocsService;

// 重新导出常用类型
pub use v1::*;