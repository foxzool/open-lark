//! 获取妙记统计数据
//!
//! doc: https://open.feishu.cn/document/server-docs/minutes-v1/minute-statistics/get

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
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

#[derive(Debug, Default)]
pub struct GetMinuteStatisticsBuilder {
    api_req: ApiRequest<GetMinuteStatisticsRequest>,
    minute_token: String,
}

impl GetMinuteStatisticsBuilder {
    pub fn new(minute_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "minutes_minute_statistics_get".to_string();
        builder.api_req.method = "GET".to_string();
        builder.minute_token = minute_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/minutes/v1/minutes/{}/statistics",
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
