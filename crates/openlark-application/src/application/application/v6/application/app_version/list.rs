//! 获取应用版本列表

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
pub struct ListApplicationVersionsRequest {
    config: Arc<Config>,
    app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListApplicationVersionsResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ListApplicationVersionsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListApplicationVersionsRequest {
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<ListApplicationVersionsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListApplicationVersionsResponse> {
        let path = format!("/open-apis/application/v6/applications/{}/app_versions", self.app_id);
        let req: ApiRequest<ListApplicationVersionsResponse> = ApiRequest::get(&path);

        let _resp: openlark_core::api::Response<ListApplicationVersionsResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(ListApplicationVersionsResponse { data: None })
    }
}
