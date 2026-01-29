/// Wiki 知识库服务模块
///
/// 提供企业知识库和Wiki管理功能。
pub mod v1;
pub mod v2;

// 导出 V1 类型
pub use v1::node::search::SearchWikiRequest;

// 导出 V2 类型
pub use v2::models::{
    WikiSearchResult, WikiSpace, WikiSpaceMember, WikiSpaceNode, WikiSpaceSetting, WikiTask,
};
pub use v2::space::{
    CopyWikiSpaceNodeRequest, CopyWikiSpaceNodeResponse, CreateWikiSpaceMemberRequest,
    CreateWikiSpaceMemberResponse, CreateWikiSpaceNodeRequest, CreateWikiSpaceNodeResponse,
    CreateWikiSpaceRequest, CreateWikiSpaceResponse, DeleteWikiSpaceMemberRequest,
    DeleteWikiSpaceMemberResponse, GetWikiSpaceNodeRequest, GetWikiSpaceNodeResponse,
    GetWikiSpaceRequest, GetWikiSpaceResponse, ListWikiSpaceMembersRequest,
    ListWikiSpaceMembersResponse, ListWikiSpaceNodesRequest, ListWikiSpaceNodesResponse,
    ListWikiSpacesRequest, ListWikiSpacesResponse, MoveDocsToWikiRequest, MoveDocsToWikiResponse,
    MoveWikiSpaceNodeRequest, MoveWikiSpaceNodeResponse, UpdateWikiSpaceNodeTitleRequest,
    UpdateWikiSpaceNodeTitleResponse, UpdateWikiSpaceSettingRequest,
    UpdateWikiSpaceSettingResponse,
};
pub use v2::task::get::{GetWikiTaskRequest, GetWikiTaskResponse};
