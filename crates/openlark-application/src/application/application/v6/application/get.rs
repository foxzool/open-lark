//! 获取应用信息

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
pub struct GetApplicationRequest {
    config: Arc<Config>,
    app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApplicationResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetApplicationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetApplicationRequest {
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetApplicationResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetApplicationResponse> {
        let path = format!("/open-apis/application/v6/applications/{}", self.app_id);
        let req: ApiRequest<GetApplicationResponse> = ApiRequest::get(&path);

        let _resp: openlark_core::api::Response<GetApplicationResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(GetApplicationResponse { data: None })
    }
}
