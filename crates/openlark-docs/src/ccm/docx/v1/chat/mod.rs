/// 聊天公告模块
///
/// 提供聊天公告的管理功能，包括公告创建、获取、区块管理等。
pub mod announcement;

// 使用通配符导出所有子模块
// announcement 模块显式导出
pub use announcement::{
    BatchDeleteChatAnnouncementBlockChildrenParams,
    BatchDeleteChatAnnouncementBlockChildrenRequest,
    BatchDeleteChatAnnouncementBlockChildrenResponse,
    BatchUpdateChatAnnouncementBlocksParams,
    BatchUpdateChatAnnouncementBlocksRequest,
    BatchUpdateChatAnnouncementBlocksResponse,
    BatchUpdateRequest,
    CreateChatAnnouncementBlockChildrenParams,
    CreateChatAnnouncementBlockChildrenRequest,
    CreateChatAnnouncementBlockChildrenResponse,
    GetChatAnnouncementBlockChildrenParams,
    GetChatAnnouncementBlockChildrenRequest,
    GetChatAnnouncementBlockChildrenResponse,
    GetChatAnnouncementBlockParams,
    GetChatAnnouncementBlockRequest,
    GetChatAnnouncementBlockResponse,
    GetChatAnnouncementBlocksParams,
    GetChatAnnouncementBlocksRequest,
    GetChatAnnouncementBlocksResponse,
    GetChatAnnouncementRequest,
    GetChatAnnouncementResponse,
};
