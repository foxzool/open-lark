//! CoreHR 雇佣信息 API
//!
//! 提供雇佣信息管理相关的 API，包括创建、删除、更新等功能。

pub mod models;
pub mod create;
pub mod delete;
pub mod patch;

// Re-export 公共类型
pub use create::*;
pub use delete::*;
pub use models::*;
pub use patch::*;
