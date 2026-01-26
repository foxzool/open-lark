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
#[allow(deprecated)]
pub use space::*;

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
    pub fn list_spaces(&self) -> space::list::ListWikiSpacesRequest {
        space::list::ListWikiSpacesRequest::new(self.config.clone())
    }

    /// 获取知识空间信息请求构建器
    pub fn get_space(&self, space_id: impl Into<String>) -> space::get::GetWikiSpaceRequest {
        space::get::GetWikiSpaceRequest::new(self.config.clone()).space_id(space_id)
    }

    /// 创建知识空间请求构建器
    pub fn create_space(&self) -> space::create::CreateWikiSpaceRequest {
        space::create::CreateWikiSpaceRequest::new(self.config.clone())
    }

    /// 更新知识空间设置请求构建器
    pub fn update_space_setting(
        &self,
        space_id: impl Into<String>,
    ) -> space::setting::update::UpdateWikiSpaceSettingRequest {
        space::setting::update::UpdateWikiSpaceSettingRequest::new(self.config.clone())
            .space_id(space_id)
    }

    /// 获取知识空间节点信息请求构建器
    /// 注意：此功能需要通过 list_space_nodes 查询
    pub fn get_space_node(&self) -> space::node::list::ListWikiSpaceNodesRequest {
        space::node::list::ListWikiSpaceNodesRequest::new(self.config.clone())
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
    pub fn list_space_members(
        &self,
        space_id: impl Into<String>,
    ) -> space::member::list::ListWikiSpaceMembersRequest {
        space::member::list::ListWikiSpaceMembersRequest::new(self.config.clone())
            .space_id(space_id)
    }

    /// 添加知识空间成员请求构建器
    pub fn create_space_member(
        &self,
        space_id: impl Into<String>,
    ) -> space::member::create::CreateWikiSpaceMemberRequest {
        space::member::create::CreateWikiSpaceMemberRequest::new(self.config.clone())
            .space_id(space_id)
    }

    /// 删除知识空间成员请求构建器
    pub fn delete_space_member(
        &self,
        space_id: impl Into<String>,
    ) -> space::member::delete::DeleteWikiSpaceMemberRequest {
        space::member::delete::DeleteWikiSpaceMemberRequest::new(self.config.clone())
            .space_id(space_id)
    }

    // === 空间节点管理 API ===

    /// 获取知识空间节点列表请求构建器
    pub fn list_space_nodes(
        &self,
        space_id: impl Into<String>,
    ) -> space::node::list::ListWikiSpaceNodesRequest {
        space::node::list::ListWikiSpaceNodesRequest::new(self.config.clone()).space_id(space_id)
    }

    /// 创建知识空间节点请求构建器
    pub fn create_space_node(
        &self,
        space_id: impl Into<String>,
    ) -> space::node::create::CreateWikiSpaceNodeRequest {
        space::node::create::CreateWikiSpaceNodeRequest::new(self.config.clone()).space_id(space_id)
    }

    /// 移动知识空间节点请求构建器
    pub fn move_space_node(
        &self,
        space_id: impl Into<String>,
        node_token: impl Into<String>,
    ) -> space::node::r#move::MoveWikiSpaceNodeRequest {
        space::node::r#move::MoveWikiSpaceNodeRequest::new(self.config.clone())
            .space_id(space_id)
            .node_token(node_token)
    }

    /// 复制知识空间节点请求构建器
    pub fn copy_space_node(
        &self,
        space_id: impl Into<String>,
        node_token: impl Into<String>,
    ) -> space::node::copy::CopyWikiSpaceNodeRequest {
        space::node::copy::CopyWikiSpaceNodeRequest::new(self.config.clone())
            .space_id(space_id)
            .node_token(node_token)
    }

    /// 更新知识空间节点标题请求构建器
    pub fn update_space_node_title(
        &self,
        space_id: impl Into<String>,
        node_token: impl Into<String>,
    ) -> space::node::update_title::UpdateWikiSpaceNodeTitleRequest {
        space::node::update_title::UpdateWikiSpaceNodeTitleRequest::new(self.config.clone())
            .space_id(space_id)
            .node_token(node_token)
    }

    /// 移动云空间文档至知识空间请求构建器
    pub fn move_docs_to_wiki(
        &self,
        space_id: impl Into<String>,
    ) -> space::node::move_docs_to_wiki::MoveDocsToWikiRequest {
        space::node::move_docs_to_wiki::MoveDocsToWikiRequest::new(self.config.clone())
            .space_id(space_id)
    }
}
