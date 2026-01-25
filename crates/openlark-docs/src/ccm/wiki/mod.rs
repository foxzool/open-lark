/// Wiki 知识库服务模块
///
/// 提供企业知识库和Wiki管理功能。
pub mod v1;
pub mod v2;

// 导出V2版本的主要服务
pub use v2::WikiService;

// 重新导出V1搜索API（保留 deprecated 类型用于向后兼容）
#[allow(deprecated)]
pub use v1::node::search::SearchWikiParams;
pub use v1::node::search::{SearchWikiRequest, SearchWikiResponse};

// 重新导出V2版本的所有内容
// v2 模块显式导出
pub use v2::{
    CopyWikiSpaceNodeParams,
    CopyWikiSpaceNodeRequest,
    CopyWikiSpaceNodeResponse,
    CreateWikiSpaceMemberParams,
    CreateWikiSpaceMemberRequest,
    CreateWikiSpaceMemberResponse,
    CreateWikiSpaceNodeParams,
    CreateWikiSpaceNodeRequest,
    CreateWikiSpaceNodeResponse,
    CreateWikiSpaceParams,
    CreateWikiSpaceRequest,
    CreateWikiSpaceResponse,
    DeleteWikiSpaceMemberParams,
    DeleteWikiSpaceMemberRequest,
    DeleteWikiSpaceMemberResponse,
    GetWikiSpaceNodeParams,
    GetWikiSpaceNodeRequest,
    GetWikiSpaceNodeResponse,
    GetWikiSpaceRequest,
    GetWikiSpaceResponse,
    GetWikiTaskRequest,
    GetWikiTaskResponse,
    ListWikiSpaceMembersParams,
    ListWikiSpaceMembersRequest,
    ListWikiSpaceMembersResponse,
    ListWikiSpaceNodesParams,
    ListWikiSpaceNodesRequest,
    ListWikiSpaceNodesResponse,
    ListWikiSpacesParams,
    ListWikiSpacesRequest,
    ListWikiSpacesResponse,
    MoveDocsToWikiParams,
    MoveDocsToWikiRequest,
    MoveDocsToWikiResponse,
    MoveWikiSpaceNodeParams,
    MoveWikiSpaceNodeRequest,
    MoveWikiSpaceNodeResponse,
    SearchWikiParams,
    SearchWikiRequest,
    SearchWikiResponse,
    UpdateWikiSpaceNodeTitleParams,
    UpdateWikiSpaceNodeTitleRequest,
    UpdateWikiSpaceNodeTitleResponse,
    UpdateWikiSpaceSettingRequest,
    UpdateWikiSpaceSettingResponse,
    WikiSearchResult,
    WikiSpace,
    WikiSpaceMember,
    WikiSpaceNode,
    WikiSpaceSetting,
    WikiTask,
    description,
    execute,
    execute_with_options,
    member_id,
    name,
    need_notification,
    new,
    node_id,
    node_token,
    obj_token,
    obj_type,
    page_size,
    page_token,
    parent_wiki_token,
    query,
    space_id,
    task_id,
    task_type,
};
