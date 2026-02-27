//! 会议室（room）

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod mget;
pub mod patch;
pub mod search;

// 导出所有模块内容
pub use create::*;
pub use delete::*;
pub use get::*;
pub use list::*;
pub use mget::{MgetRoomRequest, MgetRoomResponse};
pub use patch::*;
pub use search::*;
