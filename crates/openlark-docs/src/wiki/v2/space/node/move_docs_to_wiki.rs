//! 移动云空间文档至知识空间
//!
//! 该接口允许移动云空间文档至知识空间，并挂载在指定位置。
//! 文档参考：https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/move_docs_to_wiki

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::WikiApiV2;

/// 移动云空间文档至知识空间请求
pub struct MoveDocsToWikiRequest {
    space_id: String,
    config: Config,
}

/// 移动云空间文档至知识空间请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDocsToWikiParams {
    /// 源文档Token列表
    pub obj_tokens: Vec<String>,
    /// 目标父节点Token
    pub parent_node_token: String,
}

/// 移动云空间文档至知识空间响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDocsToWikiResponse {
    /// 移动结果信息
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for MoveDocsToWikiResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl MoveDocsToWikiRequest {
    /// 创建移动云空间文档至知识空间请求
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
    /// API文档: https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/move_docs_to_wiki
    pub async fn execute(self, params: MoveDocsToWikiParams) -> SDKResult<MoveDocsToWikiResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");
        validate_required!(params.obj_tokens, "源文档Token列表不能为空");
        validate_required!(params.parent_node_token, "目标父节点Token不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceNodeMoveDocsToWiki(self.space_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<MoveDocsToWikiResponse> =
            ApiRequest::post(&api_endpoint.to_url());

        // 设置请求体
        api_request.body = Some(openlark_core::api::RequestData::Json(serde_json::to_value(
            &params,
        )?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
