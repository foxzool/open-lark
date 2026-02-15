//! 查询用户或部门是否在应用的可用或禁用名单

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
pub struct CheckApplicationVisibilityWhiteBlackListRequest {
    config: Arc<Config>,
    app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckApplicationVisibilityWhiteBlackListResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for CheckApplicationVisibilityWhiteBlackListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CheckApplicationVisibilityWhiteBlackListRequest {
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<CheckApplicationVisibilityWhiteBlackListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CheckApplicationVisibilityWhiteBlackListResponse> {
        let path = format!("/open-apis/application/v6/applications/{}/visibility/check_white_black_list", self.app_id);
        let req: ApiRequest<CheckApplicationVisibilityWhiteBlackListResponse> = ApiRequest::post(&path);

        let _resp: openlark_core::api::Response<CheckApplicationVisibilityWhiteBlackListResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(CheckApplicationVisibilityWhiteBlackListResponse { data: None })
    }
}
