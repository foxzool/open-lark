/// 知识空间节点管理服务
///
/// 提供飞书知识库空间节点的管理功能，包括：
/// - 创建节点
/// - 获取子节点列表
/// - 移动节点
/// - 更新节点标题
/// - 复制节点
/// - 移动文档到知识空间

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 知识空间节点信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodeInfo {
    /// 节点唯一标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_token: Option<String>,
    /// 节点标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 节点类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 父节点标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_node_token: Option<String>,
    /// 原始文档类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_type: Option<String>,
    /// 原始文档标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_token: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<NodeCreator>,
}

impl Default for NodeInfo {
    fn default() -> Self {
        Self {
            node_token: None,
            title: None,
            node_type: None,
            parent_node_token: None,
            origin_type: None,
            origin_token: None,
            create_time: None,
            update_time: None,
            creator: None,
        }
    }
}

/// 节点创建者信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodeCreator {
    /// 用户唯一标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Default for NodeCreator {
    fn default() -> Self {
        Self {
            user_id: None,
            name: None,
        }
    }
}

/// 创建知识空间节点请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNodeParams {
    /// 节点标题
    pub title: String,
    /// 父节点标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_node_token: Option<String>,
    /// 原始文档类型
    pub origin_type: String,
    /// 原始文档标识
    pub origin_token: String,
}

/// 创建知识空间节点响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateNodeResponse {
    /// 节点信息
    pub data: NodeInfo,
}

impl ApiResponseTrait for CreateNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取知识空间子节点列表请求参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetNodesParams {
    /// 父节点标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_node_token: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 获取知识空间子节点列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetNodesResponse {
    /// 节点列表
    pub items: Vec<NodeInfo>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for GetNodesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 移动知识空间节点请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveNodeParams {
    /// 目标父节点标识
    pub parent_node_token: String,
}

/// 移动知识空间节点响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MoveNodeResponse {
    /// 移动结果
    pub data: Option<MoveNodeResult>,
}

/// 移动节点结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveNodeResult {
    /// 是否成功移动
    pub success: bool,
}

impl ApiResponseTrait for MoveNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新知识空间节点标题请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNodeTitleParams {
    /// 新标题
    pub title: String,
}

/// 更新知识空间节点标题响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateNodeTitleResponse {
    /// 节点信息
    pub data: NodeInfo,
}

impl ApiResponseTrait for UpdateNodeTitleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建知识空间节点副本请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyNodeParams {
    /// 目标父节点标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_node_token: Option<String>,
    /// 新标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// 创建知识空间节点副本响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CopyNodeResponse {
    /// 节点信息
    pub data: NodeInfo,
}

impl ApiResponseTrait for CopyNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 移动云空间文档至知识空间请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDocsToWikiParams {
    /// 原始文档标识列表
    pub tokens: Vec<String>,
    /// 目标父节点标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_node_token: Option<String>,
}

/// 移动云空间文档至知识空间响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MoveDocsToWikiResponse {
    /// 移动结果
    pub data: Option<MoveDocsResult>,
}

/// 移动文档结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDocsResult {
    /// 成功移动的文档数
    pub success_count: i32,
    /// 失败的文档数
    pub failed_count: i32,
}

impl ApiResponseTrait for MoveDocsToWikiResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 知识空间节点管理服务
#[derive(Clone, Debug)]
pub struct NodesService {
    config: Config,
}

impl NodesService {
    /// 创建节点管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 创建知识空间节点
    /// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/create-space-node
    pub async fn create(&self, space_id: &str, params: CreateNodeParams) -> SDKResult<CreateNodeResponse> {
        validate_required_field("空间ID", Some(space_id), "空间ID不能为空")?;
        validate_required_field("节点标题", Some(&params.title), "节点标题不能为空")?;
        validate_required_field("原始文档类型", Some(&params.origin_type), "原始文档类型不能为空")?;
        validate_required_field("原始文档标识", Some(&params.origin_token), "原始文档标识不能为空")?;

        let api_endpoint = WikiApiV2::SpaceNodeCreate(space_id.to_string());
        let api_request: ApiRequest<CreateNodeResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建知识空间节点")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "创建知识空间节点")
    }

    /// 获取知识空间子节点列表
    /// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/get-space-node-list
    pub async fn list(&self, space_id: &str, params: Option<GetNodesParams>) -> SDKResult<GetNodesResponse> {
        validate_required_field("空间ID", Some(space_id), "空间ID不能为空")?;

        let api_endpoint = WikiApiV2::SpaceNodeList(space_id.to_string());
        let mut api_request: ApiRequest<GetNodesResponse> = ApiRequest::get(&api_endpoint.to_url());

        // 构建查询参数
        if let Some(p) = params {
            if let Some(parent_node) = p.parent_node_token {
                api_request = api_request.query("parent_node_token", &parent_node);
            }
            if let Some(size) = p.page_size {
                api_request = api_request.query("page_size", &size.to_string());
            }
            if let Some(token) = p.page_token {
                api_request = api_request.query("page_token", &token);
            }
        }

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "获取知识空间子节点列表")
    }

    /// 移动知识空间节点
    /// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/move-space-node
    pub async fn move_node(&self, space_id: &str, node_token: &str, params: MoveNodeParams) -> SDKResult<MoveNodeResponse> {
        validate_required_field("空间ID", Some(space_id), "空间ID不能为空")?;
        validate_required_field("节点标识", Some(node_token), "节点标识不能为空")?;
        validate_required_field("目标父节点标识", Some(&params.parent_node_token), "目标父节点标识不能为空")?;

        let api_endpoint = WikiApiV2::SpaceNodeMove(space_id.to_string(), node_token.to_string());
        let api_request: ApiRequest<MoveNodeResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "移动知识空间节点")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "移动知识空间节点")
    }

    /// 更新知识空间节点标题
    /// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/update-space-node-title
    pub async fn update_title(&self, space_id: &str, node_token: &str, params: UpdateNodeTitleParams) -> SDKResult<UpdateNodeTitleResponse> {
        validate_required_field("空间ID", Some(space_id), "空间ID不能为空")?;
        validate_required_field("节点标识", Some(node_token), "节点标识不能为空")?;
        validate_required_field("新标题", Some(&params.title), "新标题不能为空")?;

        let api_endpoint = WikiApiV2::SpaceNodeUpdateTitle(space_id.to_string(), node_token.to_string());
        let api_request: ApiRequest<UpdateNodeTitleResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "更新知识空间节点标题")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "更新知识空间节点标题")
    }

    /// 创建知识空间节点副本
    /// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/copy-space-node
    pub async fn copy(&self, space_id: &str, node_token: &str, params: Option<CopyNodeParams>) -> SDKResult<CopyNodeResponse> {
        validate_required_field("空间ID", Some(space_id), "空间ID不能为空")?;
        validate_required_field("节点标识", Some(node_token), "节点标识不能为空")?;

        let api_endpoint = WikiApiV2::SpaceNodeCopy(space_id.to_string(), node_token.to_string());
        let mut api_request: ApiRequest<CopyNodeResponse> = ApiRequest::post(&api_endpoint.to_url());

        if let Some(p) = params {
            api_request = api_request.body(serialize_params(&p, "创建知识空间节点副本")?);
        }

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "创建知识空间节点副本")
    }

    /// 移动云空间文档至知识空间
    /// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/move-docs-to-wiki
    pub async fn move_docs_to_wiki(&self, space_id: &str, params: MoveDocsToWikiParams) -> SDKResult<MoveDocsToWikiResponse> {
        validate_required_field("空间ID", Some(space_id), "空间ID不能为空")?;

        if params.tokens.is_empty() {
            return Err(openlark_core::error::CoreError::validation(
                "tokens",
                "文档标识列表不能为空",
            ));
        }

        let api_endpoint = WikiApiV2::SpaceNodeMoveDocsToWiki(space_id.to_string());
        let api_request: ApiRequest<MoveDocsToWikiResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "移动云空间文档至知识空间")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "移动云空间文档至知识空间")
    }
}

// ==================== 构建器模式 ====================

/// 创建知识空间节点构建器
pub struct CreateNodeBuilder {
    space_id: String,
    request: CreateNodeParams,
}

impl CreateNodeBuilder {
    /// 创建新的构建器
    pub fn new(space_id: impl Into<String>, title: impl Into<String>, origin_type: impl Into<String>, origin_token: impl Into<String>) -> Self {
        Self {
            space_id: space_id.into(),
            request: CreateNodeParams {
                title: title.into(),
                parent_node_token: None,
                origin_type: origin_type.into(),
                origin_token: origin_token.into(),
            },
        }
    }

    /// 设置父节点标识
    pub fn parent_node_token(mut self, parent_node_token: impl Into<String>) -> Self {
        self.request.parent_node_token = Some(parent_node_token.into());
        self
    }

    /// 执行创建知识空间节点操作
    pub async fn execute(self, service: &NodesService) -> SDKResult<CreateNodeResponse> {
        service.create(&self.space_id, self.request).await
    }
}

/// 获取知识空间子节点列表构建器
pub struct GetNodesBuilder {
    space_id: String,
    request: GetNodesParams,
}

impl Default for GetNodesBuilder {
    fn default() -> Self {
        Self {
            space_id: String::new(),
            request: GetNodesParams {
                parent_node_token: None,
                page_size: None,
                page_token: None,
            },
        }
    }
}

impl GetNodesBuilder {
    /// 创建新的构建器
    pub fn new(space_id: impl Into<String>) -> Self {
        Self {
            space_id: space_id.into(),
            request: GetNodesParams::default(),
        }
    }

    /// 设置父节点标识
    pub fn parent_node_token(mut self, parent_node_token: impl Into<String>) -> Self {
        self.request.parent_node_token = Some(parent_node_token.into());
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    /// 执行获取知识空间子节点列表操作
    pub async fn execute(self, service: &NodesService) -> SDKResult<GetNodesResponse> {
        service.list(&self.space_id, Some(self.request)).await
    }
}

/// 移动知识空间节点构建器
pub struct MoveNodeBuilder {
    space_id: String,
    node_token: String,
    request: MoveNodeParams,
}

impl MoveNodeBuilder {
    /// 创建新的构建器
    pub fn new(space_id: impl Into<String>, node_token: impl Into<String>, parent_node_token: impl Into<String>) -> Self {
        Self {
            space_id: space_id.into(),
            node_token: node_token.into(),
            request: MoveNodeParams {
                parent_node_token: parent_node_token.into(),
            },
        }
    }

    /// 执行移动知识空间节点操作
    pub async fn execute(self, service: &NodesService) -> SDKResult<MoveNodeResponse> {
        service.move_node(&self.space_id, &self.node_token, self.request).await
    }
}

/// 更新知识空间节点标题构建器
pub struct UpdateNodeTitleBuilder {
    space_id: String,
    node_token: String,
    request: UpdateNodeTitleParams,
}

impl UpdateNodeTitleBuilder {
    /// 创建新的构建器
    pub fn new(space_id: impl Into<String>, node_token: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            space_id: space_id.into(),
            node_token: node_token.into(),
            request: UpdateNodeTitleParams {
                title: title.into(),
            },
        }
    }

    /// 执行更新知识空间节点标题操作
    pub async fn execute(self, service: &NodesService) -> SDKResult<UpdateNodeTitleResponse> {
        service.update_title(&self.space_id, &self.node_token, self.request).await
    }
}

/// 创建知识空间节点副本构建器
pub struct CopyNodeBuilder {
    space_id: String,
    node_token: String,
    request: Option<CopyNodeParams>,
}

impl CopyNodeBuilder {
    /// 创建新的构建器
    pub fn new(space_id: impl Into<String>, node_token: impl Into<String>) -> Self {
        Self {
            space_id: space_id.into(),
            node_token: node_token.into(),
            request: None,
        }
    }

    /// 设置目标父节点标识
    pub fn parent_node_token(mut self, parent_node_token: impl Into<String>) -> Self {
        if self.request.is_none() {
            self.request = Some(CopyNodeParams {
                parent_node_token: None,
                title: None,
            });
        }
        self.request.as_mut().unwrap().parent_node_token = Some(parent_node_token.into());
        self
    }

    /// 设置新标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        if self.request.is_none() {
            self.request = Some(CopyNodeParams {
                parent_node_token: None,
                title: None,
            });
        }
        self.request.as_mut().unwrap().title = Some(title.into());
        self
    }

    /// 执行创建知识空间节点副本操作
    pub async fn execute(self, service: &NodesService) -> SDKResult<CopyNodeResponse> {
        service.copy(&self.space_id, &self.node_token, self.request).await
    }
}

/// 移动云空间文档至知识空间构建器
pub struct MoveDocsToWikiBuilder {
    space_id: String,
    request: MoveDocsToWikiParams,
}

impl MoveDocsToWikiBuilder {
    /// 创建新的构建器
    pub fn new(space_id: impl Into<String>) -> Self {
        Self {
            space_id: space_id.into(),
            request: MoveDocsToWikiParams {
                tokens: Vec::new(),
                parent_node_token: None,
            },
        }
    }

    /// 添加文档标识
    pub fn add_token(mut self, token: impl Into<String>) -> Self {
        self.request.tokens.push(token.into());
        self
    }

    /// 设置文档标识列表
    pub fn tokens(mut self, tokens: Vec<String>) -> Self {
        self.request.tokens = tokens;
        self
    }

    /// 设置目标父节点标识
    pub fn parent_node_token(mut self, parent_node_token: impl Into<String>) -> Self {
        self.request.parent_node_token = Some(parent_node_token.into());
        self
    }

    /// 执行移动云空间文档至知识空间操作
    pub async fn execute(self, service: &NodesService) -> SDKResult<MoveDocsToWikiResponse> {
        service.move_docs_to_wiki(&self.space_id, self.request).await
    }
}

impl NodesService {
    /// 创建创建知识空间节点构建器
    pub fn create_node_builder(&self, space_id: impl Into<String>, title: impl Into<String>, origin_type: impl Into<String>, origin_token: impl Into<String>) -> CreateNodeBuilder {
        CreateNodeBuilder::new(space_id, title, origin_type, origin_token)
    }

    /// 创建获取知识空间子节点列表构建器
    pub fn get_nodes_builder(&self, space_id: impl Into<String>) -> GetNodesBuilder {
        GetNodesBuilder::new(space_id)
    }

    /// 创建移动知识空间节点构建器
    pub fn move_node_builder(&self, space_id: impl Into<String>, node_token: impl Into<String>, parent_node_token: impl Into<String>) -> MoveNodeBuilder {
        MoveNodeBuilder::new(space_id, node_token, parent_node_token)
    }

    /// 创建更新知识空间节点标题构建器
    pub fn update_node_title_builder(&self, space_id: impl Into<String>, node_token: impl Into<String>, title: impl Into<String>) -> UpdateNodeTitleBuilder {
        UpdateNodeTitleBuilder::new(space_id, node_token, title)
    }

    /// 创建创建知识空间节点副本构建器
    pub fn copy_node_builder(&self, space_id: impl Into<String>, node_token: impl Into<String>) -> CopyNodeBuilder {
        CopyNodeBuilder::new(space_id, node_token)
    }

    /// 创建移动云空间文档至知识空间构建器
    pub fn move_docs_to_wiki_builder(&self, space_id: impl Into<String>) -> MoveDocsToWikiBuilder {
        MoveDocsToWikiBuilder::new(space_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nodes_service_creation() {
        let config = openlark_core::config::Config::default();
        let service = NodesService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_node_info_default_creation() {
        let node_info = NodeInfo::default();
        assert_eq!(node_info.node_token, None);
        assert_eq!(node_info.title, None);
        assert_eq!(node_info.node_type, None);
        assert_eq!(node_info.parent_node_token, None);
        assert_eq!(node_info.origin_type, None);
        assert_eq!(node_info.origin_token, None);
    }

    #[test]
    fn test_node_creator_default_creation() {
        let creator = NodeCreator::default();
        assert_eq!(creator.user_id, None);
        assert_eq!(creator.name, None);
    }

    #[test]
    fn test_create_node_builder() {
        let builder = CreateNodeBuilder::new("space123", "测试节点", "doc", "token456")
            .parent_node_token("parent789");

        assert_eq!(builder.space_id, "space123");
        assert_eq!(builder.request.title, "测试节点");
        assert_eq!(builder.request.origin_type, "doc");
        assert_eq!(builder.request.origin_token, "token456");
        assert_eq!(builder.request.parent_node_token, Some("parent789".to_string()));
    }

    #[test]
    fn test_get_nodes_builder() {
        let builder = GetNodesBuilder::new("space123")
            .parent_node_token("parent456")
            .page_size(20)
            .page_token("token789");

        assert_eq!(builder.space_id, "space123");
        assert_eq!(builder.request.parent_node_token, Some("parent456".to_string()));
        assert_eq!(builder.request.page_size, Some(20));
        assert_eq!(builder.request.page_token, Some("token789".to_string()));
    }

    #[test]
    fn test_move_node_builder() {
        let builder = MoveNodeBuilder::new("space123", "node456", "parent789");

        assert_eq!(builder.space_id, "space123");
        assert_eq!(builder.node_token, "node456");
        assert_eq!(builder.request.parent_node_token, "parent789");
    }

    #[test]
    fn test_update_node_title_builder() {
        let builder = UpdateNodeTitleBuilder::new("space123", "node456", "新标题");

        assert_eq!(builder.space_id, "space123");
        assert_eq!(builder.node_token, "node456");
        assert_eq!(builder.request.title, "新标题");
    }

    #[test]
    fn test_copy_node_builder() {
        let builder = CopyNodeBuilder::new("space123", "node456")
            .parent_node_token("parent789")
            .title("复制节点");

        assert_eq!(builder.space_id, "space123");
        assert_eq!(builder.node_token, "node456");
        assert_eq!(builder.request.as_ref().unwrap().parent_node_token, Some("parent789".to_string()));
        assert_eq!(builder.request.as_ref().unwrap().title, Some("复制节点".to_string()));
    }

    #[test]
    fn test_move_docs_to_wiki_builder() {
        let builder = MoveDocsToWikiBuilder::new("space123")
            .add_token("token1")
            .add_token("token2")
            .parent_node_token("parent456");

        assert_eq!(builder.space_id, "space123");
        assert_eq!(builder.request.tokens.len(), 2);
        assert_eq!(builder.request.tokens[0], "token1");
        assert_eq!(builder.request.tokens[1], "token2");
        assert_eq!(builder.request.parent_node_token, Some("parent456".to_string()));
    }

    #[test]
    fn test_response_trait_implementations() {
        assert_eq!(CreateNodeResponse::data_format(), ResponseFormat::Data);
        assert_eq!(GetNodesResponse::data_format(), ResponseFormat::Data);
        assert_eq!(MoveNodeResponse::data_format(), ResponseFormat::Data);
        assert_eq!(UpdateNodeTitleResponse::data_format(), ResponseFormat::Data);
        assert_eq!(CopyNodeResponse::data_format(), ResponseFormat::Data);
        assert_eq!(MoveDocsToWikiResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_serialization() {
        // 测试NodeInfo序列化
        let node_info = NodeInfo {
            node_token: Some("node123".to_string()),
            title: Some("测试节点".to_string()),
            node_type: Some("doc".to_string()),
            parent_node_token: Some("parent456".to_string()),
            origin_type: Some("original".to_string()),
            origin_token: Some("origin789".to_string()),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&node_info).unwrap();
        let deserialized: NodeInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(node_info.node_token, deserialized.node_token);
        assert_eq!(node_info.title, deserialized.title);
        assert_eq!(node_info.node_type, deserialized.node_type);
        assert_eq!(node_info.parent_node_token, deserialized.parent_node_token);
    }

    #[test]
    fn test_builder_default_creation() {
        let builder = GetNodesBuilder::default();
        assert_eq!(builder.space_id, "");
        assert_eq!(builder.request.parent_node_token, None);
        assert_eq!(builder.request.page_size, None);
        assert_eq!(builder.request.page_token, None);
    }

    #[test]
    fn test_multiple_tokens_in_builder() {
        let builder = MoveDocsToWikiBuilder::new("space123")
            .tokens(vec!["token1".to_string(), "token2".to_string(), "token3".to_string()]);

        assert_eq!(builder.request.tokens.len(), 3);
        assert_eq!(builder.request.tokens[0], "token1");
        assert_eq!(builder.request.tokens[1], "token2");
        assert_eq!(builder.request.tokens[2], "token3");
    }
}