//! CoreHR 员工 API
//!
//! 提供员工管理相关的 API，包括创建、删除、查询、搜索等功能。

pub mod models;
pub mod create;
pub mod delete;
pub mod list;
pub mod search;
pub mod batch_get;

// Re-export 公共类型
pub use models::*;
pub use create::*;
pub use delete::*;
pub use list::*;
pub use search::*;
pub use batch_get::*;
