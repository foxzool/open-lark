/// 公开权限管理模块

pub mod get;
pub mod patch;
pub mod password;

// 重新导出所有API函数
pub use get::*;
pub use patch::*;
pub use password::*;