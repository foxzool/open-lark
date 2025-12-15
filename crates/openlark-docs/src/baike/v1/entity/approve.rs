/// 词条approve操作
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

/// 词条approve操作
pub async fn approve_entity(
    id: &str,
    config: &Config,
) -> SDKResult<EntityData> {
    let api_endpoint = BaikeApiV1::EntityApprove(id.to_string());
    
    let api_request: ApiRequest<EntityResponse> = 
        ApiRequest::post(&api_endpoint.to_url());
    
    let response = Transport::request(api_request, config, None).await?;
    let resp: EntityResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;
    
    resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("entity_data", "Entity data is missing")
    })
}
