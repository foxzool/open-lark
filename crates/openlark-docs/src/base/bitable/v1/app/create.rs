//! 在指定目录下创建多维表格
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/create

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateAppRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateAppResponse {
    pub app: App,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct App {
    pub app_token: String,
    pub name: String,
    pub folder_token: String,
    pub url: String,
    pub default_table_id: String,
}

impl ApiResponseTrait for CreateAppResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct CreateApp {
    config: openlark_core::config::Config,
    req: CreateAppRequest,
}

impl CreateApp {
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self {
            config,
            req: CreateAppRequest::default(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.req.name = Some(name.into());
        self
    }

    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.req.folder_token = Some(folder_token.into());
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<CreateAppResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps",
            self.config.base_url
        );
        let request = ApiRequest::post(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
