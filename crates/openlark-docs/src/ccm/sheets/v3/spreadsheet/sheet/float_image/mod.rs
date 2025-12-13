/// 浮动图片管理模块
pub mod create;
pub mod delete;
pub mod get;
pub mod patch;
pub mod query;

// 重新导出所有API函数
pub use create::*;
pub use delete::*;
pub use get::*;
pub use patch::*;
pub use query::*;
