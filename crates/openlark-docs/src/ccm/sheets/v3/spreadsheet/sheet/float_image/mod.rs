//! 浮动图片功能模块
//!
//! 提供浮动图片相关的API功能：
//! - 创建浮动图片
//! - 查询浮动图片列表
//! - 获取指定浮动图片信息
//! - 更新浮动图片
//! - 删除浮动图片

pub mod create;
pub mod delete;
pub mod get;
pub mod query;
pub mod patch;

// 重新导出所有API
pub use create::*;
pub use delete::*;
pub use get::*;
pub use query::*;
pub use patch::*;