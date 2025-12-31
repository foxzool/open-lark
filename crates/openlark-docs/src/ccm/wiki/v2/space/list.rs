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

/// 获取知识空间列表请求
pub struct ListWikiSpacesRequest {
    config: Config,
}

/// 获取知识空间列表请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWikiSpacesParams {
    /// 每页大小（最大 50）
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
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
        Self { config }
    }

    /// 执行请求
    pub async fn execute(
        self,
        params: Option<ListWikiSpacesParams>,
    ) -> SDKResult<ListWikiSpacesResponse> {
        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceList;

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<ListWikiSpacesResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 设置查询参数
        if let Some(params) = params {
            if let Some(page_size) = params.page_size {
                api_request = api_request.query("page_size", &page_size.to_string());
            }
            if let Some(page_token) = params.page_token {
                api_request = api_request.query("page_token", &page_token);
            }
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "获取知识空间列表")
    }
}
