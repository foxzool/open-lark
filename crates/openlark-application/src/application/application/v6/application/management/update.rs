//! 启停用应用

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
pub struct UpdateApplicationManagementRequest {
    config: Arc<Config>,
    app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateApplicationManagementResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for UpdateApplicationManagementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateApplicationManagementRequest {
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<UpdateApplicationManagementResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateApplicationManagementResponse> {
        let path = format!("/open-apis/application/v6/applications/{}/management", self.app_id);
        let req: ApiRequest<UpdateApplicationManagementResponse> = ApiRequest::put(&path);

        let _resp: openlark_core::api::Response<UpdateApplicationManagementResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(UpdateApplicationManagementResponse { data: None })
    }
}
