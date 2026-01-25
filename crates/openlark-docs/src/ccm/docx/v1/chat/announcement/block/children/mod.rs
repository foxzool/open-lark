/// children模块 - 群公告块子块相关API
///
/// 按照bizTag/project/version/resource/name.rs模式组织
pub mod batch_delete;
pub mod create;
pub mod get;

// batch_delete 模块显式导出
pub use batch_delete::{
    BatchDeleteChatAnnouncementBlockChildrenParams,
    BatchDeleteChatAnnouncementBlockChildrenRequest,
    BatchDeleteChatAnnouncementBlockChildrenResponse,
    CreateChatAnnouncementBlockChildrenParams,
    CreateChatAnnouncementBlockChildrenRequest,
    CreateChatAnnouncementBlockChildrenResponse,
    GetChatAnnouncementBlockChildrenParams,
    GetChatAnnouncementBlockChildrenRequest,
    GetChatAnnouncementBlockChildrenResponse,
    execute,
    execute_with_options,
    new,
};
// create 模块显式导出
pub use create::{
    BatchDeleteChatAnnouncementBlockChildrenParams,
    BatchDeleteChatAnnouncementBlockChildrenRequest,
    BatchDeleteChatAnnouncementBlockChildrenResponse,
    CreateChatAnnouncementBlockChildrenParams,
    CreateChatAnnouncementBlockChildrenRequest,
    CreateChatAnnouncementBlockChildrenResponse,
    GetChatAnnouncementBlockChildrenParams,
    GetChatAnnouncementBlockChildrenRequest,
    GetChatAnnouncementBlockChildrenResponse,
    execute,
    execute_with_options,
    new,
};
// get 模块显式导出
pub use get::{
    BatchDeleteChatAnnouncementBlockChildrenParams,
    BatchDeleteChatAnnouncementBlockChildrenRequest,
    BatchDeleteChatAnnouncementBlockChildrenResponse,
    CreateChatAnnouncementBlockChildrenParams,
    CreateChatAnnouncementBlockChildrenRequest,
    CreateChatAnnouncementBlockChildrenResponse,
    GetChatAnnouncementBlockChildrenParams,
    GetChatAnnouncementBlockChildrenRequest,
    GetChatAnnouncementBlockChildrenResponse,
    execute,
    execute_with_options,
    new,
};
