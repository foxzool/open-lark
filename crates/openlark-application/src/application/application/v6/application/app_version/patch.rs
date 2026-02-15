//! 更新应用审核状态

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
pub struct PatchApplicationAppVersionRequest {
    config: Arc<Config>,
    app_id: String,
    resource_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchApplicationAppVersionResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchApplicationAppVersionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchApplicationAppVersionRequest {
    pub fn new(config: Arc<Config>, app_id: impl Into<String>, resource_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
            resource_id: resource_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<PatchApplicationAppVersionResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchApplicationAppVersionResponse> {
        let path = format!("/open-apis/application/v6/applications/{}/app_versions/{}", self.app_id, self.resource_id);
        let req: ApiRequest<PatchApplicationAppVersionResponse> = ApiRequest::patch(&path);

        let _resp: openlark_core::api::Response<PatchApplicationAppVersionResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(PatchApplicationAppVersionResponse { data: None })
    }
}
