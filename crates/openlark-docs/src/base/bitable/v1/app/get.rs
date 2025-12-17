//! 获取多维表格元数据
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/get

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetAppRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetAppResponse {
    pub app: App,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct App {
    pub app_token: String,
    pub name: String,
    pub revision: i32,
    pub is_advanced: bool,
    pub time_zone: String,
}

impl ApiResponseTrait for GetAppResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct GetApp {
    config: openlark_core::config::Config,
    app_token: String,
}

impl GetApp {
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self {
            config,
            app_token: String::new(),
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<GetAppResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}",
            self.config.base_url, self.app_token
        );
        let request = ApiRequest::get(&url);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
