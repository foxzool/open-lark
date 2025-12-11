//! 创建知识空间
//!
//! 此接口用于创建知识空间。
//! 文档参考：https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use super::super::models::WikiSpace;
use crate::common::api_endpoints::WikiApiV2;

/// 创建知识空间请求
pub struct CreateWikiSpaceRequest {
    config: Config,
}

/// 创建知识空间请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWikiSpaceParams {
    /// 知识空间名称
    pub name: String,
    /// 知识空间描述
    pub description: Option<String>,
    /// 知识空间图标
    pub icon: Option<String>,
    /// 空间类型 ("public" | "private")
    pub space_type: Option<String>,
    /// 空间域名
    pub domain: Option<String>,
}

/// 创建知识空间响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWikiSpaceResponse {
    /// 知识空间信息
    pub data: Option<WikiSpace>,
}

impl ApiResponseTrait for CreateWikiSpaceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateWikiSpaceRequest {
    /// 创建创建知识空间请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/create
    pub async fn execute(
        self,
        params: CreateWikiSpaceParams,
    ) -> SDKResult<CreateWikiSpaceResponse> {
        // 验证必填字段
        validate_required!(params.name, "知识空间名称不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceCreate;

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<CreateWikiSpaceResponse> =
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
