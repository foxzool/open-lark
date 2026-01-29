#![allow(clippy::module_inception)]

/// Baike 模块
pub mod baike;
pub mod lingo;
pub mod models;

// 使用通配符导出所有子模块
pub use baike::*;
pub use lingo::*;
pub use models::*;
