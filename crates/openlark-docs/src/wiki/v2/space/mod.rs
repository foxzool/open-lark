//! Wiki V2 空间管理模块

pub mod create;
pub mod get;
pub mod get_node;
pub mod list;
pub mod member;
pub mod node;
pub mod setting;

// 导出数据模型
pub use create::{CreateWikiSpaceParams, CreateWikiSpaceRequest, CreateWikiSpaceResponse};
pub use get::{GetWikiSpaceRequest, GetWikiSpaceResponse};
pub use get_node::{GetWikiSpaceNodeParams, GetWikiSpaceNodeRequest, GetWikiSpaceNodeResponse};
pub use list::{ListWikiSpacesParams, ListWikiSpacesRequest, ListWikiSpacesResponse};
pub use member::{
    create::{
        CreateWikiSpaceMemberParams, CreateWikiSpaceMemberRequest, CreateWikiSpaceMemberResponse,
    },
    delete::{DeleteWikiSpaceMemberRequest, DeleteWikiSpaceMemberResponse},
    list::{ListWikiSpaceMembersParams, ListWikiSpaceMembersRequest, ListWikiSpaceMembersResponse},
};
pub use node::{
    copy::{CopyWikiSpaceNodeParams, CopyWikiSpaceNodeRequest, CopyWikiSpaceNodeResponse},
    create::{CreateWikiSpaceNodeParams, CreateWikiSpaceNodeRequest, CreateWikiSpaceNodeResponse},
    list::{ListWikiSpaceNodesParams, ListWikiSpaceNodesRequest, ListWikiSpaceNodesResponse},
    move_docs_to_wiki::{MoveDocsToWikiParams, MoveDocsToWikiRequest, MoveDocsToWikiResponse},
    move_node::{MoveWikiSpaceNodeParams, MoveWikiSpaceNodeRequest, MoveWikiSpaceNodeResponse},
    update_title::{
        UpdateWikiSpaceNodeTitleParams, UpdateWikiSpaceNodeTitleRequest,
        UpdateWikiSpaceNodeTitleResponse,
    },
};
pub use setting::update::{UpdateWikiSpaceSettingRequest, UpdateWikiSpaceSettingResponse};
