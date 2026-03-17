/// block模块 - 群公告块相关API
///
/// 按照bizTag/project/version/resource/name.rs模式组织
pub mod batch_update;
pub mod children;
pub mod get;
pub mod list;

// 使用通配符导出所有子模块
// batch_update 模块显式导出
pub use batch_update::{
    BatchUpdateChatAnnouncementBlocksParams, BatchUpdateChatAnnouncementBlocksRequest,
    BatchUpdateChatAnnouncementBlocksResponse, BatchUpdateRequest,
};
// children 模块显式导出
pub use children::{
    BatchDeleteChatAnnouncementBlockChildrenParams,
    BatchDeleteChatAnnouncementBlockChildrenRequest,
    BatchDeleteChatAnnouncementBlockChildrenResponse, CreateChatAnnouncementBlockChildrenParams,
    CreateChatAnnouncementBlockChildrenRequest, CreateChatAnnouncementBlockChildrenResponse,
    GetChatAnnouncementBlockChildrenParams, GetChatAnnouncementBlockChildrenRequest,
    GetChatAnnouncementBlockChildrenResponse,
};
// get 模块显式导出
pub use get::{
    GetChatAnnouncementBlockParams, GetChatAnnouncementBlockRequest,
    GetChatAnnouncementBlockResponse,
};
// list 模块显式导出
pub use list::{
    GetChatAnnouncementBlocksParams, GetChatAnnouncementBlocksRequest,
    GetChatAnnouncementBlocksResponse,
};
