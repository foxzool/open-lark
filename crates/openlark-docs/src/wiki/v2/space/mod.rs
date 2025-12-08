//! Wiki V2 空间管理模块

pub mod list;
pub mod get;
pub mod create;
pub mod setting;
pub mod get_node;
pub mod node;
pub mod member;

// 导出数据模型
pub use list::{ListWikiSpacesRequest, ListWikiSpacesParams, ListWikiSpacesResponse};
pub use get::{GetWikiSpaceRequest, GetWikiSpaceResponse};
pub use create::{CreateWikiSpaceRequest, CreateWikiSpaceParams, CreateWikiSpaceResponse};
pub use setting::update::{UpdateWikiSpaceSettingRequest, UpdateWikiSpaceSettingResponse};
pub use get_node::{GetWikiSpaceNodeRequest, GetWikiSpaceNodeResponse, GetWikiSpaceNodeParams};
pub use member::{
    list::{ListWikiSpaceMembersRequest, ListWikiSpaceMembersParams, ListWikiSpaceMembersResponse},
    create::{CreateWikiSpaceMemberRequest, CreateWikiSpaceMemberParams, CreateWikiSpaceMemberResponse},
    delete::{DeleteWikiSpaceMemberRequest, DeleteWikiSpaceMemberResponse}
};
pub use node::{
    list::{ListWikiSpaceNodesRequest, ListWikiSpaceNodesParams, ListWikiSpaceNodesResponse},
    create::{CreateWikiSpaceNodeRequest, CreateWikiSpaceNodeParams, CreateWikiSpaceNodeResponse},
    move_node::{MoveWikiSpaceNodeRequest, MoveWikiSpaceNodeParams, MoveWikiSpaceNodeResponse},
    copy::{CopyWikiSpaceNodeRequest, CopyWikiSpaceNodeParams, CopyWikiSpaceNodeResponse},
    update_title::{UpdateWikiSpaceNodeTitleRequest, UpdateWikiSpaceNodeTitleParams, UpdateWikiSpaceNodeTitleResponse},
    move_docs_to_wiki::{MoveDocsToWikiRequest, MoveDocsToWikiParams, MoveDocsToWikiResponse}
};