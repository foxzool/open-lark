//! 获取妙记信息
//!
//! doc: https://open.feishu.cn/document/server-docs/minutes-v1/minute/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, LarkAPIError},
    constants::AccessTokenType,
    req_option::RequestOption,
    request_builder::UnifiedRequestBuilder,
};
use reqwest::RequestBuilder;
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

#[derive(Debug)]
pub struct GetMinuteBuilder {
    api_req: ApiRequest<GetMinuteRequest>,
    minute_token: String,
}

impl Default for GetMinuteBuilder {
    fn default() -> Self {
        Self {
            api_req: ApiRequest::get(""),
            minute_token: "".to_string(),
        }
    }
}

impl GetMinuteBuilder {
    pub fn new(minute_token: impl ToString) -> Self {
        let minute_token = minute_token.to_string();
        let url = format!(
            "https://open.feishu.cn/open-apis/minutes/v1/minutes/{}",
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
