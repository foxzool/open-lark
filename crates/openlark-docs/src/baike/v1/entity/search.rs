/// 模糊搜索词条
///
/// 传入关键词，与词条名、别名、释义等信息进行模糊匹配，返回搜到的词条信息。
/// docPath: https://open.feishu.cn/document/server-docs/baike-v1/entity/search
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::BaikeApiV1, api_utils::*};

#[derive(Debug, serde::Deserialize)]
pub struct SearchEntityResponse {
    pub data: Option<SearchData>,
}

#[derive(Debug, serde::Deserialize)]
pub struct SearchData {
    pub items: Vec<EntitySearchResult>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct EntitySearchResult {
    pub entity_id: String,
    pub title: String,
    pub content: String,
    pub aliases: Vec<String>,
    pub space_id: String,
    pub search_score: f64,
    pub highlight: Option<EntityHighlight>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EntityHighlight {
    pub title_highlight: Option<String>,
    pub content_highlight: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct SearchEntityParams {
    pub query: String,
    pub space_id: Option<String>,
    pub page_size: Option<i32>,
    pub page_token: Option<String>,
    pub filter: Option<SearchFilter>,
}

#[derive(Debug, serde::Serialize)]
pub struct SearchFilter {
    pub classification_ids: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

impl ApiResponseTrait for SearchEntityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 模糊搜索词条
///
/// 传入关键词，与词条名、别名、释义等信息进行模糊匹配，返回搜到的词条信息。
/// docPath: https://open.feishu.cn/document/server-docs/baike-v1/entity/search
pub async fn search_entity(
    config: &Config,
    params: SearchEntityParams,
) -> SDKResult<Vec<EntitySearchResult>> {
    // 验证必填字段
    validate_required_field("搜索查询", Some(&params.query), "搜索查询不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = BaikeApiV1::EntitySearch;

    // 创建API请求
    let api_request: ApiRequest<SearchEntityResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "模糊搜索词条")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    let resp: SearchEntityResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.map(|data| data.items).ok_or_else(|| {
        openlark_core::error::validation_error("search_data", "Search data is missing")
    })
}