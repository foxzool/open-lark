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
pub struct GetApplicationContactsRangeConfigurationRequest {
    config: Arc<Config>,
    app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApplicationContactsRangeConfigurationResponse {
    pub data: Option<ContactsRangeConfigurationData>,
}

impl ApiResponseTrait for GetApplicationContactsRangeConfigurationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactsRangeConfigurationData {
    pub app_id: String,
    pub contacts_range: serde_json::Value,
}

impl GetApplicationContactsRangeConfigurationRequest {
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetApplicationContactsRangeConfigurationResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetApplicationContactsRangeConfigurationResponse> {
        let path = format!("/open-apis/application/v6/applications/{}/contacts_range_configuration", self.app_id);
        let req: ApiRequest<GetApplicationContactsRangeConfigurationResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取应用通讯录权限范围配置", "响应数据为空")
        })
    }
}
