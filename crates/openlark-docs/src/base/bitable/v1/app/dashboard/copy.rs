//! 该接口用于根据现有仪表盘复制出新的仪表盘
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-dashboard/copy

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CopyDashboardRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CopyDashboardResponse {
    pub block_id: String,
    pub name: String,
}

impl ApiResponseTrait for CopyDashboardResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct CopyDashboard {
    config: openlark_core::config::Config,
    app_token: String,
    block_id: String,
    req: CopyDashboardRequest,
}

impl CopyDashboard {
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            block_id: String::new(),
            req: CopyDashboardRequest::default(),
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = block_id.into();
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.req.name = name.into();
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<CopyDashboardResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/dashboards/{}/copy",
            self.config.base_url, self.app_token, self.block_id
        );
        let request = ApiRequest::post(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
