//! 获取勋章授予名单详情 API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

pub struct GetBadgeGrantBuilder {
    badge_id: String,
    grant_id: String,
    config: Config,
}

impl GetBadgeGrantBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            badge_id: String::new(),
            grant_id: String::new(),
            config,
        }
    }

    pub fn badge_id(mut self, badge_id: impl Into<String>) -> Self {
        self.badge_id = badge_id.into();
        self
    }

    pub fn grant_id(mut self, grant_id: impl Into<String>) -> Self {
        self.grant_id = grant_id.into();
        self
    }

    pub async fn execute(self) -> SDKResult<GetBadgeGrantResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<GetBadgeGrantResponse> {
        let url = format!("/open-apis/admin/v1/badges/{}/grants/{}", self.badge_id, self.grant_id);
        let api_request: ApiRequest<GetBadgeGrantResponse> = ApiRequest::get(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取勋章授予名单详情", "响应数据为空")
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetBadgeGrantResponse {
    pub grant_id: String,
    pub badge_id: String,
    pub user_id: String,
    pub create_time: String,
}

impl ApiResponseTrait for GetBadgeGrantResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
