//! 导出妙记文字记录
//!
//! doc: https://open.feishu.cn/document/minutes-v1/minute-transcript/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, LarkAPIError},
    constants::AccessTokenType,
    req_option::RequestOption,
    request_builder::UnifiedRequestBuilder,
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetMinuteTranscriptRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetMinuteTranscriptResponse {
    pub file_token: String,
    pub url: String,
}

impl ApiResponseTrait for GetMinuteTranscriptResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct GetMinuteTranscriptBuilder {
    api_req: ApiRequest<GetMinuteTranscriptRequest>,
    minute_token: String,
}

impl Default for GetMinuteTranscriptBuilder {
    fn default() -> Self {
        Self {
            api_req: ApiRequest::get(""),
            minute_token: "".to_string(),
        }
    }
}

impl GetMinuteTranscriptBuilder {
    pub fn new(minute_token: impl ToString) -> Self {
        let minute_token = minute_token.to_string();
        let url = format!(
            "https://open.feishu.cn/open-apis/minutes/v1/minutes/{}/transcript",
            minute_token
        );
        let api_req = ApiRequest::get(url);
        Self {
            api_req,
            minute_token,
        }
    }

    pub async fn build(
        mut self,
        config: &openlark_core::config::Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        UnifiedRequestBuilder::build(&mut self.api_req, AccessTokenType::Tenant, config, option)
            .await
    }
}
