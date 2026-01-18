//! 创建知识空间
//!
//! 此接口用于创建知识空间。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use super::super::models::WikiSpace;
use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 创建知识空间请求（流式 Builder 模式）
pub struct CreateWikiSpaceRequest {
    config: Config,
    /// 知识空间名称
    name: String,
    /// 知识空间描述
    description: Option<String>,
}

/// 创建知识空间请求体（内部使用）
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateWikiSpaceRequestBody {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

/// 创建知识空间响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWikiSpaceResponse {
    /// 知识空间信息
    pub space: Option<WikiSpace>,
}

impl ApiResponseTrait for CreateWikiSpaceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateWikiSpaceRequest {
    /// 创建创建知识空间请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            name: String::new(),
            description: None,
        }
    }

    /// 设置知识空间名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// 设置知识空间描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateWikiSpaceResponse> {
        validate_required!(self.name, "知识空间名称不能为空");

        let api_endpoint = WikiApiV2::SpaceCreate;

        let request_body = CreateWikiSpaceRequestBody {
            name: self.name,
            description: self.description,
        };

        let api_request: ApiRequest<CreateWikiSpaceResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&request_body, "创建知识空间")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "创建知识空间")
    }
}

/// 创建知识空间请求参数（兼容旧 API，已弃用）
#[deprecated(
    since = "0.16.0",
    note = "请使用 CreateWikiSpaceRequest 的流式 Builder 模式"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWikiSpaceParams {
    /// 知识空间名称
    pub name: String,
    /// 知识空间描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
