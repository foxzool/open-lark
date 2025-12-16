//! 导出妙记文字记录
//!
//! doc: https://open.feishu.cn/document/minutes-v1/minute-transcript/get

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
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

#[derive(Debug, Default)]
pub struct GetMinuteTranscriptBuilder {
    api_req: ApiRequest<GetMinuteTranscriptRequest>,
    minute_token: String,
}

impl GetMinuteTranscriptBuilder {
    pub fn new(minute_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "minutes_minute_transcript_get".to_string();
        builder.api_req.method = "GET".to_string();
        builder.minute_token = minute_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/minutes/v1/minutes/{}/transcript",
            builder.minute_token
        );
        builder.api_req.body = None;
        builder
    }

    pub fn build(
        self,
        config: &openlark_core::config::Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let mut req = self.api_req;
        req.build(AccessTokenType::Tenant, config, option)
    }
}
