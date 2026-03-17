//! 会议室（room）

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod mget;
pub mod patch;
pub mod search;

// 导出所有模块内容
// create 模块显式导出
pub use create::{CreateRoomRequest, CreateRoomResponse};
// delete 模块显式导出
pub use delete::{DeleteRoomRequest, DeleteRoomResponse};
// get 模块显式导出
pub use get::{GetRoomRequest, GetRoomResponse};
// list 模块显式导出
pub use list::{ListRoomRequest, ListRoomRequestBuilder, ListRoomResponse, RoomItem};
pub use mget::{MgetRoomRequest, MgetRoomResponse};
// patch 模块显式导出
pub use patch::{PatchRoomRequest, PatchRoomResponse};
// search 模块显式导出
pub use search::SearchRoomRequest;
