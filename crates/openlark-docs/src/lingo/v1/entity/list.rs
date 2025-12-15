/// 获取词条列表
///
/// 分页拉取词条列表数据，支持拉取租户内的全部词条。
/// docPath: https://open.feishu.cn/document/lingo-v1/entity/list
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::LingoApiV1, api_utils::*};

#[derive(Debug, serde::Deserialize)]
pub struct ListEntityResponse {
    pub data: Option<EntityListData>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EntityListData {
    pub items: Vec<EntityItem>,
    pub page_token: Option<String>,
    pub has_more: bool,
    pub total: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct EntityItem {
    pub entity_id: String,
    pub title: String,
    pub content: String,
    pub aliases: Vec<String>,
    pub repo_id: String,
    pub status: String,
    pub create_time: String,
    pub update_time: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, serde::Serialize)]
pub struct ListEntityParams {
    pub repo_id: Option<String>,
    pub page_size: Option<i32>,
    pub page_token: Option<String>,
    pub filter: Option<ListFilter>,
}

#[derive(Debug, serde::Serialize)]
pub struct ListFilter {
    pub classification_ids: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub status: Option<String>,
}

impl ApiResponseTrait for ListEntityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取词条列表
///
/// 分页拉取词条列表数据，支持拉取租户内的全部词条。
/// docPath: https://open.feishu.cn/document/lingo-v1/entity/list
pub async fn list_entity(
    config: &Config,
    params: Option<ListEntityParams>,
) -> SDKResult<Vec<EntityItem>> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = LingoApiV1::EntityList;

    // 创建API请求
    let api_request: ApiRequest<ListEntityResponse> = if let Some(params) = params {
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "获取词条列表")?)
    } else {
        ApiRequest::get(&api_endpoint.to_url())
    };

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    let resp: ListEntityResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.map(|data| data.items).ok_or_else(|| {
        openlark_core::error::validation_error("entity_list_data", "Entity list data is missing")
    })
}