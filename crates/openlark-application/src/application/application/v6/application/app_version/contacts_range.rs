//! 获取应用版本中开发者申请的通讯录权限范围

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
pub struct GetApplicationVersionContactsRangeRequest {
    config: Arc<Config>,
    app_id: String,
    version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApplicationVersionContactsRangeResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetApplicationVersionContactsRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetApplicationVersionContactsRangeRequest {
    pub fn new(config: Arc<Config>, app_id: impl Into<String>, version_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
            version_id: version_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetApplicationVersionContactsRangeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetApplicationVersionContactsRangeResponse> {
        let path = format!("/open-apis/application/v6/applications/{}/app_versions/{}/contacts_range", self.app_id, self.version_id);
        let req: ApiRequest<GetApplicationVersionContactsRangeResponse> = ApiRequest::get(&path);

        let _resp: openlark_core::api::Response<GetApplicationVersionContactsRangeResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(GetApplicationVersionContactsRangeResponse { data: None })
    }
}
