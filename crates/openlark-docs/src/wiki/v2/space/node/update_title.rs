//! 更新知识空间节点标题
//!
//! 更新知识空间中节点的标题。
//! 文档参考：https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-nodes/updateTitle

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

/// 更新知识空间节点标题请求
pub struct UpdateWikiSpaceNodeTitleRequest {
    space_id: String,
    node_token: String,
    config: Config,
}

/// 更新知识空间节点标题请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWikiSpaceNodeTitleParams {
    /// 新的节点标题
    pub title: String,
}

/// 更新知识空间节点标题响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWikiSpaceNodeTitleResponse {
    /// 更新后的节点信息
    pub data: Option<WikiSpaceNode>,
}

impl ApiResponseTrait for UpdateWikiSpaceNodeTitleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateWikiSpaceNodeTitleRequest {
    /// 创建更新知识空间节点标题请求
    pub fn new(config: Config) -> Self {
        Self {
            space_id: String::new(),
            node_token: String::new(),
            config,
        }
    }

    /// 设置知识空间ID
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.space_id = space_id.into();
        self
    }

    /// 设置节点Token
    pub fn node_token(mut self, node_token: impl Into<String>) -> Self {
        self.node_token = node_token.into();
        self
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-nodes/updateTitle
    pub async fn execute(self, params: UpdateWikiSpaceNodeTitleParams) -> SDKResult<UpdateWikiSpaceNodeTitleResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");
        validate_required!(self.node_token, "节点Token不能为空");
        validate_required!(params.title, "节点标题不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceNodeUpdateTitle(self.space_id.clone(), self.node_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<UpdateWikiSpaceNodeTitleResponse> =
            ApiRequest::put(&api_endpoint.to_url());

        // 设置请求体
        api_request.body = Some(openlark_core::api::RequestData::Json(serde_json::to_value(&params)?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}