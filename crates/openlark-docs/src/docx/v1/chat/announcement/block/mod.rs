//! 群公告块模块

pub mod list;
pub mod get;
pub mod batch_update;
pub mod children;

// 导出所有API
pub use list::{ListChatAnnouncementBlocksRequest, ListChatAnnouncementBlocksParams, ListChatAnnouncementBlocksResponse};
pub use get::{GetChatAnnouncementBlockRequest, GetChatAnnouncementBlockParams, GetChatAnnouncementBlockResponse};
pub use batch_update::{BatchUpdateChatAnnouncementBlocksRequest, BatchUpdateChatAnnouncementBlocksParams, BatchUpdateChatAnnouncementBlocksResponse};
pub use children::*;