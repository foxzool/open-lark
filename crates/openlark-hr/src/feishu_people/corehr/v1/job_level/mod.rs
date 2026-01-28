//! CoreHR 职级（job_level）相关 API
//!
//! 包含创建、删除、查询、更新职级等功能

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;
pub mod patch;

// Re-export 公共类型
pub use create::*;
pub use delete::*;
pub use get::*;
pub use list::*;
pub use models::*;
pub use patch::*;
