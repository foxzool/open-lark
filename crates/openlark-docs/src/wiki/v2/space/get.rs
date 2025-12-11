//! 获取知识空间信息
//!
//! 此接口用于根据知识空间ID来查询知识空间的信息。
//! 文档参考：https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use super::super::models::WikiSpace;
use crate::common::api_endpoints::WikiApiV2;

/// 获取知识空间信息请求
pub struct GetWikiSpaceRequest {
    space_id: String,
    config: Config,
}

/// 获取知识空间信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWikiSpaceResponse {
    /// 知识空间信息
    pub data: Option<WikiSpace>,
}

impl ApiResponseTrait for GetWikiSpaceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetWikiSpaceRequest {
    /// 创建获取知识空间信息请求
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
    /// API文档: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/get
    pub async fn execute(self) -> SDKResult<GetWikiSpaceResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceGet(self.space_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<GetWikiSpaceResponse> = ApiRequest::get(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
