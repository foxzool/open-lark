//! 获取知识空间列表
//!
//! 此接口用于获取有权限访问的知识空间列表。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::super::models::WikiSpace;
use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 获取知识空间列表请求（流式 Builder 模式）
pub struct ListWikiSpacesRequest {
    config: Config,
    /// 每页大小（最大 50）
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
}

/// 获取知识空间列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWikiSpacesResponse {
    /// 知识空间列表
    #[serde(default)]
    pub items: Vec<WikiSpace>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
    /// 页面标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListWikiSpacesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListWikiSpacesRequest {
    /// 创建获取知识空间列表请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置每页大小（最大 50）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListWikiSpacesResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListWikiSpacesResponse> {
        let api_endpoint = WikiApiV2::SpaceList;

        let mut api_request: ApiRequest<ListWikiSpacesResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            api_request = api_request.query("page_token", &page_token);
        }

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取知识空间列表")
    }
}

/// 获取知识空间列表请求参数（兼容旧 API，已弃用）
#[deprecated(
    since = "0.16.0",
    note = "请使用 ListWikiSpacesRequest 的流式 Builder 模式"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWikiSpacesParams {
    /// 每页大小（最大 50）
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
}
