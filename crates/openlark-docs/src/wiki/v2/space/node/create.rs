//! 创建知识空间节点
//!
//! 此接口用于在知识节点里创建节点到指定位置。
//! 文档参考：https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::wiki::v2::models::WikiSpaceNode;
use crate::common::api_endpoints::WikiApiV2;

/// 创建知识空间节点请求
pub struct CreateWikiSpaceNodeRequest {
    space_id: String,
    config: Config,
}

/// 创建知识空间节点请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWikiSpaceNodeParams {
    /// 节点标题
    pub title: String,
    /// 节点类型
    pub node_type: String,
    /// 父节点Token
    pub parent_node_token: Option<String>,
    /// 节点内容
    pub content: Option<String>,
}

/// 创建知识空间节点响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWikiSpaceNodeResponse {
    /// 节点信息
    pub data: Option<WikiSpaceNode>,
}

impl ApiResponseTrait for CreateWikiSpaceNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateWikiSpaceNodeRequest {
    /// 创建创建知识空间节点请求
    pub fn new(config: Config) -> Self {
        Self {
            space_id: String::new(),
            config,
        }
    }

    /// 设置知识空间ID
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.space_id = space_id.into();
        self
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/create
    pub async fn execute(self, params: CreateWikiSpaceNodeParams) -> SDKResult<CreateWikiSpaceNodeResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");
        validate_required!(params.title, "节点标题不能为空");
        validate_required!(params.node_type, "节点类型不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceNodeCreate(self.space_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<CreateWikiSpaceNodeResponse> =
            ApiRequest::post(&api_endpoint.to_url());

        // 设置请求体
        api_request.body = Some(openlark_core::api::RequestData::Json(serde_json::to_value(&params)?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}