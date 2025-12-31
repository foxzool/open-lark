//! 移动云空间文档至知识空间
//!
//! 该接口允许移动云空间文档至知识空间，并挂载在指定位置。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/move_docs_to_wiki

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 移动云空间文档至知识空间请求
pub struct MoveDocsToWikiRequest {
    space_id: String,
    config: Config,
}

/// 移动云空间文档至知识空间请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDocsToWikiParams {
    /// 源文档 token
    pub obj_token: String,
    /// 源文档类型（例如 doc、docx 等）
    pub obj_type: String,
    /// 目标父节点 wiki token
    pub parent_wiki_token: String,
}

/// 移动云空间文档至知识空间响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MoveDocsToWikiResponse {
    /// 操作已完成时返回的 wiki token
    pub wiki_token: Option<String>,
    /// 操作未完成时返回的异步任务 ID
    pub task_id: Option<String>,
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
    pub async fn execute(self, params: MoveDocsToWikiParams) -> SDKResult<MoveDocsToWikiResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");
        validate_required!(params.obj_token, "源文档token不能为空");
        validate_required!(params.obj_type, "源文档类型不能为空");
        validate_required!(params.parent_wiki_token, "目标父节点wiki token不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceNodeMoveDocsToWiki(self.space_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<MoveDocsToWikiResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&params, "移动云空间文档至知识空间")?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "移动云空间文档至知识空间")
    }
}
