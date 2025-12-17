//! 获取妙记统计数据
//!
//! doc: https://open.feishu.cn/document/server-docs/minutes-v1/minute-statistics/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, LarkAPIError},
    constants::AccessTokenType,
    req_option::RequestOption,
    request_builder::UnifiedRequestBuilder,
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetMinuteStatisticsRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetMinuteStatisticsResponse {
    pub statistics: MinuteStatistics,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MinuteStatistics {
    pub pv: i32,
    pub uv: i32,
    pub user_ids: Option<Vec<String>>,
}

impl ApiResponseTrait for GetMinuteStatisticsResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct GetMinuteStatisticsBuilder {
    api_req: ApiRequest<GetMinuteStatisticsRequest>,
    minute_token: String,
}

impl Default for GetMinuteStatisticsBuilder {
    fn default() -> Self {
        Self {
            api_req: ApiRequest::get(""),
            minute_token: "".to_string(),
        }
    }
}

impl GetMinuteStatisticsBuilder {
    pub fn new(minute_token: impl ToString) -> Self {
        let minute_token = minute_token.to_string();
        let url = format!(
            "https://open.feishu.cn/open-apis/minutes/v1/minutes/{}/statistics",
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
