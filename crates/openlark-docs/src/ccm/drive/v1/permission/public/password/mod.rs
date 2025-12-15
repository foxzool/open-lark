/// 公开权限密码管理模块

pub mod create;
pub mod update;

// 重新导出所有API函数
pub use create::*;
pub use update::*;