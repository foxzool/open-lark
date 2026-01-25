/// Wiki V2 API 模块
pub mod space;
pub mod task;

// 导出数据模型
pub mod models;

// 导出数据模型
pub use models::{
    WikiSearchResult, WikiSpace, WikiSpaceMember, WikiSpaceNode, WikiSpaceSetting, WikiTask,
};

// 导出API服务 - 使用glob导入避免复杂的路径指定
// space 模块显式导出
pub use space::{
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
    node_token,
    obj_token,
    obj_type,
    page_size,
    page_token,
    parent_wiki_token,
    space_id,
    task_id,
    task_type,
};

use openlark_core::config::Config;

use crate::ccm::wiki::v1::node::search::SearchWikiRequest;

/// Wiki 知识库服务
#[derive(Clone, Debug)]
pub struct WikiService {
    config: Config,
}

impl WikiService {
    /// 创建新的Wiki服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取知识空间列表请求构建器
    pub fn list_spaces(&self) -> ListWikiSpacesRequest {
        ListWikiSpacesRequest::new(self.config.clone())
    }

    /// 获取知识空间信息请求构建器
    pub fn get_space(&self, space_id: impl Into<String>) -> GetWikiSpaceRequest {
        GetWikiSpaceRequest::new(self.config.clone()).space_id(space_id)
    }

    /// 创建知识空间请求构建器
    pub fn create_space(&self) -> CreateWikiSpaceRequest {
        CreateWikiSpaceRequest::new(self.config.clone())
    }

    /// 更新知识空间设置请求构建器
    pub fn update_space_setting(
        &self,
        space_id: impl Into<String>,
    ) -> UpdateWikiSpaceSettingRequest {
        UpdateWikiSpaceSettingRequest::new(self.config.clone()).space_id(space_id)
    }

    /// 获取知识空间节点信息请求构建器
    pub fn get_space_node(&self) -> GetWikiSpaceNodeRequest {
        GetWikiSpaceNodeRequest::new(self.config.clone())
    }

    /// 获取任务结果请求构建器
    pub fn get_task(&self, task_id: impl Into<String>) -> task::get::GetWikiTaskRequest {
        task::get::GetWikiTaskRequest::new(self.config.clone()).task_id(task_id)
    }

    /// 搜索 Wiki 请求构建器
    pub fn search_wiki(&self) -> SearchWikiRequest {
        SearchWikiRequest::new(self.config.clone())
    }

    // === 空间成员管理 API ===

    /// 获取知识空间成员列表请求构建器
    pub fn list_space_members(&self, space_id: impl Into<String>) -> ListWikiSpaceMembersRequest {
        ListWikiSpaceMembersRequest::new(self.config.clone()).space_id(space_id)
    }

    /// 添加知识空间成员请求构建器
    pub fn create_space_member(&self, space_id: impl Into<String>) -> CreateWikiSpaceMemberRequest {
        CreateWikiSpaceMemberRequest::new(self.config.clone()).space_id(space_id)
    }

    /// 删除知识空间成员请求构建器
    pub fn delete_space_member(&self, space_id: impl Into<String>) -> DeleteWikiSpaceMemberRequest {
        DeleteWikiSpaceMemberRequest::new(self.config.clone()).space_id(space_id)
    }

    // === 空间节点管理 API ===

    /// 获取知识空间节点列表请求构建器
    pub fn list_space_nodes(&self, space_id: impl Into<String>) -> ListWikiSpaceNodesRequest {
        ListWikiSpaceNodesRequest::new(self.config.clone()).space_id(space_id)
    }

    /// 创建知识空间节点请求构建器
    pub fn create_space_node(&self, space_id: impl Into<String>) -> CreateWikiSpaceNodeRequest {
        CreateWikiSpaceNodeRequest::new(self.config.clone()).space_id(space_id)
    }

    /// 移动知识空间节点请求构建器
    pub fn move_space_node(
        &self,
        space_id: impl Into<String>,
        node_token: impl Into<String>,
    ) -> MoveWikiSpaceNodeRequest {
        MoveWikiSpaceNodeRequest::new(self.config.clone())
            .space_id(space_id)
            .node_token(node_token)
    }

    /// 复制知识空间节点请求构建器
    pub fn copy_space_node(
        &self,
        space_id: impl Into<String>,
        node_token: impl Into<String>,
    ) -> CopyWikiSpaceNodeRequest {
        CopyWikiSpaceNodeRequest::new(self.config.clone())
            .space_id(space_id)
            .node_token(node_token)
    }

    /// 更新知识空间节点标题请求构建器
    pub fn update_space_node_title(
        &self,
        space_id: impl Into<String>,
        node_token: impl Into<String>,
    ) -> UpdateWikiSpaceNodeTitleRequest {
        UpdateWikiSpaceNodeTitleRequest::new(self.config.clone())
            .space_id(space_id)
            .node_token(node_token)
    }

    /// 移动云空间文档至知识空间请求构建器
    pub fn move_docs_to_wiki(&self, space_id: impl Into<String>) -> MoveDocsToWikiRequest {
        MoveDocsToWikiRequest::new(self.config.clone()).space_id(space_id)
    }
}
