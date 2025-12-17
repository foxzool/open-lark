//! 复制多维表格
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/copy

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CopyAppRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CopyAppResponse {
    pub app: App,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct App {
    pub app_token: String,
    pub name: String,
    pub folder_token: String,
    pub url: String,
    pub default_table_id: String,
    pub time_zone: String,
}

impl ApiResponseTrait for CopyAppResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct CopyApp {
    config: openlark_core::config::Config,
    app_token: String,
    req: CopyAppRequest,
}

impl CopyApp {
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            req: CopyAppRequest::default(),
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

    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.req.folder_token = Some(folder_token.into());
        self
    }

    pub fn without_content(mut self, without_content: bool) -> Self {
        self.req.without_content = Some(without_content);
        self
    }

    pub fn time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.req.time_zone = Some(time_zone.into());
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<CopyAppResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/copy",
            self.config.base_url, self.app_token
        );
        let request = ApiRequest::post(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
