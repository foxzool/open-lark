//! 获取应用版本信息

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
pub struct GetApplicationVersionRequest {
    config: Arc<Config>,
    app_id: String,
    version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApplicationVersionResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetApplicationVersionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetApplicationVersionRequest {
    pub fn new(config: Arc<Config>, app_id: impl Into<String>, version_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
            version_id: version_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetApplicationVersionResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetApplicationVersionResponse> {
        let path = format!("/open-apis/application/v6/applications/{}/app_versions/{}", self.app_id, self.version_id);
        let req: ApiRequest<GetApplicationVersionResponse> = ApiRequest::get(&path);

        let _resp: openlark_core::api::Response<GetApplicationVersionResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(GetApplicationVersionResponse { data: None })
    }
}
