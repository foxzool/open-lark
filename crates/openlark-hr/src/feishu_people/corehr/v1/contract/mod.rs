//! CoreHR 合同 API
//!
//! 提供合同管理相关的 API，包括创建、删除、查询、搜索等功能。

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;
pub mod patch;
pub mod search;

// Re-export 公共类型
pub use create::*;
pub use delete::*;
pub use get::*;
pub use list::*;
pub use models::*;
pub use patch::*;
pub use search::*;
