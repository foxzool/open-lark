//! 创建知识空间节点
//!
//! 此接口用于在知识节点里创建节点到指定位置。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::WikiApiV2, api_utils::*};
use crate::wiki::v2::models::WikiSpaceNode;

/// 创建知识空间节点请求
pub struct CreateWikiSpaceNodeRequest {
    space_id: String,
    config: Config,
}

/// 创建知识空间节点请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWikiSpaceNodeParams {
    /// 实际文档类型（例如 doc、docx、sheet 等）
    pub obj_type: String,
    /// 实际文档 token（部分场景可不传）
    pub obj_token: Option<String>,
    /// 父节点Token
    pub parent_node_token: Option<String>,
    /// 节点类型（例如 origin）
    pub node_type: Option<String>,
    /// 节点标题（可选）
    pub title: Option<String>,
}

/// 创建知识空间节点响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWikiSpaceNodeResponse {
    /// 节点信息
    pub node: Option<WikiSpaceNode>,
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
    pub async fn execute(
        self,
        params: CreateWikiSpaceNodeParams,
    ) -> SDKResult<CreateWikiSpaceNodeResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");
        validate_required!(params.obj_type, "obj_type不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceNodeCreate(self.space_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<CreateWikiSpaceNodeResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&params, "创建知识空间节点")?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "创建知识空间节点")
    }
}
