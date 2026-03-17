/// children模块 - 群公告块子块相关API
///
/// 按照bizTag/project/version/resource/name.rs模式组织
pub mod batch_delete;
pub mod create;
pub mod get;

// 使用通配符导出所有子模块
// batch_delete 模块显式导出
pub use batch_delete::{
    BatchDeleteChatAnnouncementBlockChildrenParams,
    BatchDeleteChatAnnouncementBlockChildrenRequest,
    BatchDeleteChatAnnouncementBlockChildrenResponse,
};
// create 模块显式导出
pub use create::{
    CreateChatAnnouncementBlockChildrenParams,
    CreateChatAnnouncementBlockChildrenRequest,
    CreateChatAnnouncementBlockChildrenResponse,
};
// get 模块显式导出
pub use get::{
    GetChatAnnouncementBlockChildrenParams,
    GetChatAnnouncementBlockChildrenRequest,
    GetChatAnnouncementBlockChildrenResponse,
};
