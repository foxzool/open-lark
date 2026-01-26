/// 视图管理模块

pub mod list;
pub mod create;
pub mod patch;
pub mod delete;
pub mod get;

pub use create::*;
pub use delete::*;
pub use get::*;
pub use list::*;
pub use patch::*;
