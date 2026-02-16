//! 修改勋章授予名单 API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

pub struct UpdateBadgeGrantBuilder {
    badge_id: String,
    grant_id: String,
    user_ids: Vec<String>,
    config: Config,
}

impl UpdateBadgeGrantBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            badge_id: String::new(),
            grant_id: String::new(),
            user_ids: Vec::new(),
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

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = user_ids;
        self
    }

    pub async fn execute(self) -> SDKResult<UpdateBadgeGrantResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateBadgeGrantResponse> {
        let url = format!(
            "/open-apis/admin/v1/badges/{}/grants/{}",
            self.badge_id, self.grant_id
        );
        let request_body = UpdateBadgeGrantRequest {
            user_ids: self.user_ids,
        };
        let api_request: ApiRequest<UpdateBadgeGrantResponse> =
            ApiRequest::put(url).body(serde_json::to_value(&request_body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("修改勋章授予名单", "响应数据为空")
        })
    }
}

#[derive(Debug, Serialize)]
struct UpdateBadgeGrantRequest {
    user_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateBadgeGrantResponse {
    pub grant_id: String,
    pub badge_id: String,
    pub user_ids: Vec<String>,
    pub update_time: String,
}

impl ApiResponseTrait for UpdateBadgeGrantResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
