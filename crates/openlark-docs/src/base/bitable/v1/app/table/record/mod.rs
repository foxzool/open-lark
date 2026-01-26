/// 记录管理模块

pub mod batch_get;
pub mod list;
pub mod batch_create;
pub mod models;
pub mod create;
pub mod delete;
pub mod update;
pub mod batch_delete;
pub mod batch_update;
pub mod get;
pub mod search;

pub use batch_create::*;
pub use batch_delete::*;
pub use batch_get::*;
pub use batch_update::*;
pub use create::*;
pub use delete::*;
pub use get::*;
pub use list::*;
pub use models::*;
pub use search::*;
pub use update::*;
