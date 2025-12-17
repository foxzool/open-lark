//! 根据 app_token，获取多维表格下的所有仪表盘
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-dashboard/list

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListDashboardRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListDashboardResponse {
    pub dashboards: Vec<Dashboard>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Dashboard {
    pub block_id: String,
    pub name: String,
}

impl ApiResponseTrait for ListDashboardResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct ListDashboard {
    config: openlark_core::config::Config,
    app_token: String,
    req: ListDashboardRequest,
}

impl ListDashboard {
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            req: ListDashboardRequest::default(),
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.req.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.req.page_token = Some(page_token.into());
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<ListDashboardResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/dashboards",
            self.config.base_url, self.app_token
        );
        let request = ApiRequest::get(&url).query(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
