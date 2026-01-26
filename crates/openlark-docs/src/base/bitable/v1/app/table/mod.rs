/// 表格管理模块

pub mod list;
pub mod batch_create;
pub mod create;
pub mod patch;
pub mod delete;
pub mod batch_delete;

pub use batch_create::*;
pub use batch_delete::*;
pub use create::*;
pub use delete::*;
pub use field::*;
pub use form::*;
pub use list::*;
pub use patch::*;
pub use record::*;
pub use view::*;
