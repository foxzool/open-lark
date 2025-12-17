//! 根据 app_token 和 table_id，获取数据表的所有字段
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-field/list

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListFieldRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListFieldResponse {
    pub items: Vec<Field>,
    pub page_token: String,
    pub has_more: bool,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Field {
    pub field_id: String,
    pub field_name: String,
    #[serde(rename = "type")]
    pub type_: i32,
    pub property: Option<serde_json::Value>,
}

impl ApiResponseTrait for ListFieldResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct ListField {
    config: openlark_core::config::Config,
    app_token: String,
    table_id: String,
    req: ListFieldRequest,
}

impl ListField {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, table_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            table_id: table_id.into(),
            req: ListFieldRequest::default(),
        }
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.req.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.req.page_token = Some(page_token.into());
        self
    }

    pub fn view_id(mut self, view_id: impl Into<String>) -> Self {
        self.req.view_id = Some(view_id.into());
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<ListFieldResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/fields",
            self.config.base_url, self.app_token, self.table_id
        );
        let request = ApiRequest::get(&url).query(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
