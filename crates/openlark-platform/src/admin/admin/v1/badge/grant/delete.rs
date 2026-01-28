//! 删除勋章授予名单 API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config: Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

pub struct DeleteBadgeGrantBuilder {
    badge_id: String,
    grant_id: String,
    config: Config,
}

impl DeleteBadgeGrantBuilder {
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

    pub async fn execute(self) -> SDKResult<DeleteBadgeGrantResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<DeleteBadgeGrantResponse> {
        let url = format!("/open-apis/admin/v1/badges/{}/grants/{}", self.badge_id, self.grant_id);
        let api_request: ApiRequest<DeleteBadgeGrantResponse> = ApiRequest::delete(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("删除勋章授予名单", "响应数据为空")
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeleteBadgeGrantResponse {
    pub result: String,
}

impl ApiResponseTrait for DeleteBadgeGrantResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
