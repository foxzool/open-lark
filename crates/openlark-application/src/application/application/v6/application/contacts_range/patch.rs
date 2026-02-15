//! 更新应用通讯录权限范围配置

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
pub struct PatchApplicationContactsRangeRequest {
    config: Arc<Config>,
    app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchApplicationContactsRangeResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchApplicationContactsRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchApplicationContactsRangeRequest {
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<PatchApplicationContactsRangeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchApplicationContactsRangeResponse> {
        let path = format!("/open-apis/application/v6/applications/{}/contacts_range", self.app_id);
        let req: ApiRequest<PatchApplicationContactsRangeResponse> = ApiRequest::patch(&path);

        let _resp: openlark_core::api::Response<PatchApplicationContactsRangeResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(PatchApplicationContactsRangeResponse { data: None })
    }
}
