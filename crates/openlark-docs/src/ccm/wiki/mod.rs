/// Wiki v1 接口模块。
pub mod v1;
/// Wiki v2 接口模块。
pub mod v2;

/// 重新导出 v1 搜索请求类型。
pub use v1::node::search::SearchWikiRequest;

/// 重新导出 v2 Wiki 模型。
pub use v2::models::{
    WikiSearchResult, WikiSpace, WikiSpaceMember, WikiSpaceNode, WikiSpaceSetting, WikiTask,
};
/// 重新导出 v2 空间管理类型。
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
/// 重新导出 v2 任务查询类型。
pub use v2::task::get::{GetWikiTaskRequest, GetWikiTaskResponse};
