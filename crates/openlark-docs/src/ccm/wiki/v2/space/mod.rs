/// Wiki V2 空间管理模块
pub mod create;
pub mod get;
pub mod get_node;
pub mod list;
pub mod member;
pub mod node;
pub mod setting;

#[allow(deprecated)]
pub use create::CreateWikiSpaceParams;
pub use create::{CreateWikiSpaceRequest, CreateWikiSpaceResponse};
pub use get::{GetWikiSpaceRequest, GetWikiSpaceResponse};
pub use get_node::{GetWikiSpaceNodeParams, GetWikiSpaceNodeRequest, GetWikiSpaceNodeResponse};
#[allow(deprecated)]
pub use list::ListWikiSpacesParams;
pub use list::{ListWikiSpacesRequest, ListWikiSpacesResponse};
pub use member::{
    create::{
        CreateWikiSpaceMemberParams, CreateWikiSpaceMemberRequest, CreateWikiSpaceMemberResponse,
    },
    delete::{
        DeleteWikiSpaceMemberParams, DeleteWikiSpaceMemberRequest, DeleteWikiSpaceMemberResponse,
    },
    list::{ListWikiSpaceMembersParams, ListWikiSpaceMembersRequest, ListWikiSpaceMembersResponse},
};
#[allow(deprecated)]
pub use node::move_docs_to_wiki::MoveDocsToWikiParams;
pub use node::{
    copy::{CopyWikiSpaceNodeParams, CopyWikiSpaceNodeRequest, CopyWikiSpaceNodeResponse},
    create::{CreateWikiSpaceNodeParams, CreateWikiSpaceNodeRequest, CreateWikiSpaceNodeResponse},
    list::{ListWikiSpaceNodesParams, ListWikiSpaceNodesRequest, ListWikiSpaceNodesResponse},
    move_docs_to_wiki::{MoveDocsToWikiRequest, MoveDocsToWikiResponse},
    r#move::{MoveWikiSpaceNodeParams, MoveWikiSpaceNodeRequest, MoveWikiSpaceNodeResponse},
    update_title::{
        UpdateWikiSpaceNodeTitleParams, UpdateWikiSpaceNodeTitleRequest,
        UpdateWikiSpaceNodeTitleResponse,
    },
};
pub use setting::update::{UpdateWikiSpaceSettingRequest, UpdateWikiSpaceSettingResponse};
