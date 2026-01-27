//! CoreHR 部门 API
//!
//! 提供部门管理相关的 API，包括创建、删除、查询、搜索等功能。

pub mod batch_get;
pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;
pub mod multi_timeline;
pub mod operation_logs;
pub mod parents;
pub mod patch;
pub mod search;
pub mod timeline;
pub mod tree;

// Re-export 公共类型
pub use batch_get::*;
pub use create::*;
pub use delete::*;
pub use get::*;
pub use list::*;
pub use models::*;
pub use multi_timeline::*;
pub use operation_logs::*;
pub use parents::*;
pub use patch::*;
pub use search::*;
pub use timeline::*;
pub use tree::*;
