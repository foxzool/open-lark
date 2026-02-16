//! 修改勋章信息 API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

pub struct UpdateBadgeBuilder {
    badge_id: String,
    name: Option<String>,
    description: Option<String>,
    config: Config,
}

impl UpdateBadgeBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            badge_id: String::new(),
            name: None,
            description: None,
            config,
        }
    }

    pub fn badge_id(mut self, badge_id: impl Into<String>) -> Self {
        self.badge_id = badge_id.into();
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub async fn execute(self) -> SDKResult<UpdateBadgeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateBadgeResponse> {
        let url = format!("/open-apis/admin/v1/badges/{}", self.badge_id);
        let request_body = UpdateBadgeRequest {
            name: self.name,
            description: self.description,
        };
        let api_request: ApiRequest<UpdateBadgeResponse> =
            ApiRequest::put(url).body(serde_json::to_value(&request_body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("修改勋章信息", "响应数据为空"))
    }
}

#[derive(Debug, Serialize)]
struct UpdateBadgeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateBadgeResponse {
    pub badge_id: String,
    pub name: String,
    pub description: Option<String>,
    pub update_time: String,
}

impl ApiResponseTrait for UpdateBadgeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
