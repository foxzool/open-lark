//! Wiki 知识库服务
//!
//! 提供企业知识库和Wiki管理的统一管理接口。
//!
//! ## 使用示例
//!
//! ```rust
//! use openlark_core::config::Config;
//! use openlark_docs::wiki::v2::service::WikiService;
//! use openlark_docs::wiki::v2::space::list::ListWikiSpacesParams;
//! use openlark_docs::wiki::v2::space::create::CreateWikiSpaceParams;
//!
//! let config = Config::builder()
//!     .app_id("app_id")
//!     .app_secret("app_secret")
//!     .build();
//!
//! let wiki = WikiService::new(config);
//!
//! // 获取知识空间列表
//! let params = ListWikiSpacesParams {
//!     site_type: Some(1),
//!     page_size: Some(20),
//!     page_token: None,
//! };
//! let spaces = wiki.list_spaces()
//!     .execute(Some(params))
//!     .await?;
//!
//! // 创建新知识空间
//! let create_params = CreateWikiSpaceParams {
//!     name: "我的知识库".to_string(),
//!     description: Some("团队知识管理空间".to_string()),
//!     space_type: Some("private".to_string()),
//!     icon: Some("📚".to_string()),
//!     domain: None,
//! };
//!
//! let new_space = wiki.create_space()
//!     .execute(create_params)
//!     .await?;
//! ```

use openlark_core::config::Config;

// 导入所有API请求类型
use super::{
    space::{list::ListWikiSpacesRequest, get::GetWikiSpaceRequest, create::CreateWikiSpaceRequest},
    setting::update::UpdateWikiSpaceSettingRequest,
    space::get_node::GetWikiSpaceNodeRequest,
    space::member::{list::ListWikiSpaceMembersRequest, create::CreateWikiSpaceMemberRequest, delete::DeleteWikiSpaceMemberRequest},
    space::node::{list::ListWikiSpaceNodesRequest, create::CreateWikiSpaceNodeRequest, copy::CopyWikiSpaceNodeRequest, move_node::MoveWikiSpaceNodeRequest, update_title::UpdateWikiSpaceNodeTitleRequest, move_docs_to_wiki::MoveDocsToWikiRequest},
};
use super::task::get::GetWikiTaskRequest;
use crate::wiki::v1::node::search::SearchWikiRequest;

/// Wiki 知识库服务
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
    pub fn update_space_setting(&self, space_id: impl Into<String>) -> UpdateWikiSpaceSettingRequest {
        UpdateWikiSpaceSettingRequest::new(self.config.clone()).space_id(space_id)
    }

    /// 获取知识空间节点信息请求构建器
    pub fn get_space_node(&self) -> GetWikiSpaceNodeRequest {
        GetWikiSpaceNodeRequest::new(self.config.clone())
    }

    /// 获取任务结果请求构建器
    pub fn get_task(&self, task_id: impl Into<String>) -> GetWikiTaskRequest {
        GetWikiTaskRequest::new(self.config.clone()).task_id(task_id)
    }

    /// 搜索Wiki请求构建器 (V1)
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
    pub fn move_space_node(&self, space_id: impl Into<String>, node_token: impl Into<String>) -> MoveWikiSpaceNodeRequest {
        MoveWikiSpaceNodeRequest::new(self.config.clone()).space_id(space_id).node_token(node_token)
    }

    /// 复制知识空间节点请求构建器
    pub fn copy_space_node(&self, space_id: impl Into<String>, node_token: impl Into<String>) -> CopyWikiSpaceNodeRequest {
        CopyWikiSpaceNodeRequest::new(self.config.clone()).space_id(space_id).node_token(node_token)
    }

    /// 更新知识空间节点标题请求构建器
    pub fn update_space_node_title(&self, space_id: impl Into<String>, node_token: impl Into<String>) -> UpdateWikiSpaceNodeTitleRequest {
        UpdateWikiSpaceNodeTitleRequest::new(self.config.clone()).space_id(space_id).node_token(node_token)
    }

    /// 移动云空间文档至知识空间请求构建器
    pub fn move_docs_to_wiki(&self, space_id: impl Into<String>) -> MoveDocsToWikiRequest {
        MoveDocsToWikiRequest::new(self.config.clone()).space_id(space_id)
    }
}