/// Lingo词条API
///
/// Lingo语言服务词条管理。
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::LingoApiV1, api_utils::*};
use crate::lingo::v1::LingoEntity;

#[derive(Debug, serde::Deserialize)]
pub struct LingoEntityResponse {
    pub data: Option<LingoEntity>,
}

impl ApiResponseTrait for LingoEntityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Lingo词条update
pub async fn batch_get(
    config: &Config,
) -> SDKResult<LingoEntity> {
    let api_endpoint = LingoApiV1::EntityCreate;
    
    let api_request: ApiRequest<LingoEntityResponse> = 
        ApiRequest::get(&api_endpoint.to_url());
    
    let response = Transport::request(api_request, config, None).await?;
    let resp: LingoEntityResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;
    
    resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("entity_data", "Entity data is missing")
    })
}
