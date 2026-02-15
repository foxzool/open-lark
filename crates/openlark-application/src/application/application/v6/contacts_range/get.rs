//! 获取应用通讯录权限范围配置

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
pub struct GetAppContactsRangeRequest {
    config: Arc<Config>,
    app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAppContactsRangeResponse {
    pub data: Option<ContactsRangeData>,
}

impl ApiResponseTrait for GetAppContactsRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactsRangeData {
    pub app_id: String,
    pub contacts_range: serde_json::Value,
}

impl GetAppContactsRangeRequest {
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetAppContactsRangeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetAppContactsRangeResponse> {
        let path = format!("/open-apis/application/v6/applications/{}/contacts_range", self.app_id);
        let req: ApiRequest<GetAppContactsRangeResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取应用通讯录权限范围配置", "响应数据为空")
        })
    }
}
