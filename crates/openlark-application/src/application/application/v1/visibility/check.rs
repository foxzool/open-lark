//! visibility check

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
pub struct VisibilityCheckRequest {
    config: Arc<Config>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibilityCheckResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for VisibilityCheckResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl VisibilityCheckRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn execute(self) -> SDKResult<VisibilityCheckResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<VisibilityCheckResponse> {
        let path = "/open-apis/application/application/v1/visibility/check"
            .replace("application", "application")
            .replace("security", "acs")
            .replace("personal_settings", "personal_settings");
        let req: ApiRequest<VisibilityCheckResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("visibility check", "响应数据为空")
        })
    }
}
