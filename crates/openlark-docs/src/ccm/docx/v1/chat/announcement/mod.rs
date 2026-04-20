/// 聊天公告模块
pub mod block;
/// get 子模块。
pub mod get;

// 使用通配符导出所有子模块
// block 模块显式导出
/// 重新导出相关类型。
pub use block::{
    BatchDeleteChatAnnouncementBlockChildrenParams,
    BatchDeleteChatAnnouncementBlockChildrenRequest,
    BatchDeleteChatAnnouncementBlockChildrenResponse, BatchUpdateChatAnnouncementBlocksParams,
    BatchUpdateChatAnnouncementBlocksRequest, BatchUpdateChatAnnouncementBlocksResponse,
    BatchUpdateRequest, CreateChatAnnouncementBlockChildrenParams,
    CreateChatAnnouncementBlockChildrenRequest, CreateChatAnnouncementBlockChildrenResponse,
    GetChatAnnouncementBlockChildrenParams, GetChatAnnouncementBlockChildrenRequest,
    GetChatAnnouncementBlockChildrenResponse, GetChatAnnouncementBlockParams,
    GetChatAnnouncementBlockRequest, GetChatAnnouncementBlockResponse,
    GetChatAnnouncementBlocksParams, GetChatAnnouncementBlocksRequest,
    GetChatAnnouncementBlocksResponse,
};
// get 模块显式导出
/// 重新导出相关类型。
pub use get::{GetChatAnnouncementRequest, GetChatAnnouncementResponse};
