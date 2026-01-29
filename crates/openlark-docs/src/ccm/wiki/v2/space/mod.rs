/// Wiki V2 空间管理模块
///
/// 提供知识空间的创建、获取、列表、成员管理等操作。

pub mod create;
pub mod get;
pub mod get_node;
pub mod list;
pub mod member;
pub mod node;
pub mod setting;

// 显式导出 - 避免使用 glob reexport
#[allow(deprecated)]
pub use create::{CreateWikiSpaceRequest, CreateWikiSpaceResponse};

pub use get::{GetWikiSpaceRequest, GetWikiSpaceResponse};

pub use get_node::{GetWikiSpaceNodeRequest, GetWikiSpaceNodeResponse};

#[allow(deprecated)]
pub use list::{ListWikiSpacesRequest, ListWikiSpacesResponse};

pub use member::{
    CreateWikiSpaceMemberRequest, CreateWikiSpaceMemberResponse,
    DeleteWikiSpaceMemberRequest, DeleteWikiSpaceMemberResponse,
    ListWikiSpaceMembersRequest, ListWikiSpaceMembersResponse,
};

pub use node::{
    CopyWikiSpaceNodeRequest, CopyWikiSpaceNodeResponse,
    CreateWikiSpaceNodeRequest, CreateWikiSpaceNodeResponse,
    ListWikiSpaceNodesRequest, ListWikiSpaceNodesResponse,
    MoveDocsToWikiRequest, MoveDocsToWikiResponse,
    MoveWikiSpaceNodeRequest, MoveWikiSpaceNodeResponse,
    UpdateWikiSpaceNodeTitleRequest, UpdateWikiSpaceNodeTitleResponse,
};

pub use setting::{UpdateWikiSpaceSettingRequest, UpdateWikiSpaceSettingResponse};
