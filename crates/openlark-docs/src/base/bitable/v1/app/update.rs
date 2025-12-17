//! 更新多维表格元数据
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/update

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateAppRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_advanced: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateAppResponse {
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

impl ApiResponseTrait for UpdateAppResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct UpdateApp {
    config: openlark_core::config::Config,
    app_token: String,
    req: UpdateAppRequest,
}

impl UpdateApp {
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            req: UpdateAppRequest::default(),
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.req.name = Some(name.into());
        self
    }

    pub fn is_advanced(mut self, is_advanced: bool) -> Self {
        self.req.is_advanced = Some(is_advanced);
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<UpdateAppResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}",
            self.config.base_url, self.app_token
        );
        let request = ApiRequest::put(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
