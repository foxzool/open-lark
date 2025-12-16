//! 获取妙记信息
//!
//! doc: https://open.feishu.cn/document/server-docs/minutes-v1/minute/get

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetMinuteRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetMinuteResponse {
    pub minute: Minute,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Minute {
    pub token: String,
    pub owner_id: String,
    pub create_time: i64,
    pub title: String,
    pub cover: String,
    pub duration: i64,
    pub url: String,
}

impl ApiResponseTrait for GetMinuteResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct GetMinuteBuilder {
    api_req: ApiRequest<GetMinuteRequest>,
    minute_token: String,
}

impl GetMinuteBuilder {
    pub fn new(minute_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "minutes_minute_get".to_string();
        builder.api_req.method = "GET".to_string();
        builder.minute_token = minute_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/minutes/v1/minutes/{}",
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
