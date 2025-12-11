//! Minutes V1 妙记管理模块
//!
//! 提供妙记基础信息、音视频、文字记录和统计数据管理功能。

pub mod get;
pub mod media;
pub mod models;
pub mod service;
pub mod statistics;
pub mod transcript;

// 导出所有子模块内容，避免命名冲突
pub use get::*;
pub use media::*;
pub use models::*;
pub use service::*;
pub use statistics::*;
pub use transcript::*;
