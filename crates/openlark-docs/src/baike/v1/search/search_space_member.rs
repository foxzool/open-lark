/// 搜索词条
///
/// 在知识库中搜索词条。
/// docPath: https://open.feishu.cn/document/baike-v1/search/entity
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::BaikeApiV1, api_utils::*};

#[derive(Debug, serde::Deserialize)]
pub struct SearchResponse {
    pub data: Option<SearchData>,
}

#[derive(Debug, serde::Deserialize)]
pub struct SearchData {
    pub items: Vec<EntityItem>,
    pub total: i32,
    pub has_more: bool,
    pub page_token: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EntityItem {
    pub entity_id: String,
    pub title: String,
    pub content: String,
    pub space_id: String,
    pub create_time: String,
}

impl ApiResponseTrait for SearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索词条
///
/// 在知识库中搜索词条。
/// docPath: https://open.feishu.cn/document/baike-v1/search/entity
pub async fn search_space_member(
    config: &Config,
    query: &str,
    space_id: Option<&str>,
) -> SDKResult<SearchData> {
    let mut params = serde_json::json!({
        "query": query,
    });

    if let Some(id) = space_id {
        params["space_id"] = serde_json::Value::String(id.to_string());
    }

    let api_endpoint = BaikeApiV1::Spacemember;

    let api_request: ApiRequest<SearchResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(Ok(serde_json::to_vec(&params).map_err(|e| {
                openlark_core::error::validation_error("params", &format!("序列化参数失败: {}", e))
            })?));

    let response = Transport::request(api_request, config, None).await?;
    let resp: SearchResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("search_data", "Search data is missing")
    })
}