//! Docx 群公告服务模块
//!
//! 提供企业群公告管理功能。

pub mod v1;

// 导出V1版本的主要服务
pub use v1::DocxService;

// 重新导出常用类型
pub use v1::*;