//! 提取潜在的词条
//!
//! docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/entity/extract

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ExtractEntityReqBody {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ExtractEntityResponse {
    pub entity_word: Vec<ExtractedWord>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ExtractedWord {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
}

impl ApiResponseTrait for ExtractEntityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 提取潜在词条请求
pub struct ExtractEntityRequest {
    config: Config,
    req: ExtractEntityReqBody,
}

impl ExtractEntityRequest {
    pub fn new(config: Config, text: impl Into<String>) -> Self {
        Self {
            config,
            req: ExtractEntityReqBody { text: text.into() },
        }
    }

    pub async fn send(self) -> SDKResult<ExtractEntityResponse> {
        use crate::common::api_endpoints::BaikeApiV1;
        validate_required!(self.req.text, "text 不能为空");

        let api_request: ApiRequest<ExtractEntityResponse> =
            ApiRequest::post(&BaikeApiV1::EntityExtract.to_url())
                .body(serde_json::to_value(&self.req)?);

        let response: Response<ExtractEntityResponse> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
