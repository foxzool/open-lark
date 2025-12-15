/// 词条list操作
///
/// 知识库词条管理。
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::BaikeApiV1, api_utils::*};

#[derive(Debug, serde::Deserialize)]
pub struct EntityResponse {
    pub data: Option<EntityData>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EntityListResponse {
    pub data: Option<Vec<EntityData>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EntityData {
    pub entity_id: String,
    pub title: String,
    pub content: String,
    pub space_id: String,
    pub status: String,
}

impl ApiResponseTrait for EntityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for EntityListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取词条列表
///
/// 分页拉取词条列表数据，支持拉取租户内的全部词条。
/// docPath: https://open.feishu.cn/document/server-docs/baike-v1/entity/list
pub async fn list_entities(
    config: &Config,
) -> SDKResult<Vec<EntityData>> {
    let api_endpoint = BaikeApiV1::EntityList;

    let api_request: ApiRequest<EntityListResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, None).await?;
    let resp: EntityListResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("entity_list_data", "Entity list data is missing")
    })
}
