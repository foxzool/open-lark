//! rule get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RuleGetRequest {
    config: Arc<Config>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleGetResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for RuleGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl RuleGetRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn execute(self) -> SDKResult<RuleGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RuleGetResponse> {
        let path = "/open-apis/security/acs/v1/rule/get"
            .replace("application", "application")
            .replace("security", "acs")
            .replace("personal_settings", "personal_settings");
        let req: ApiRequest<RuleGetResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("rule get", "响应数据为空")
        })
    }
}
