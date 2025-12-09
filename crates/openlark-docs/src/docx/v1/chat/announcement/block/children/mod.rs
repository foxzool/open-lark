//! 群公告块子内容模块

pub mod create;
pub mod get;
pub mod batch_delete;

// 导出所有API
pub use create::{CreateChatAnnouncementBlockChildrenRequest, CreateChatAnnouncementBlockChildrenParams, CreateChatAnnouncementBlockChildrenResponse};
pub use get::{GetChatAnnouncementBlockChildrenRequest, GetChatAnnouncementBlockChildrenParams, GetChatAnnouncementBlockChildrenResponse};
pub use batch_delete::{BatchDeleteChatAnnouncementBlockChildrenRequest, BatchDeleteChatAnnouncementBlockChildrenParams, BatchDeleteChatAnnouncementBlockChildrenResponse};