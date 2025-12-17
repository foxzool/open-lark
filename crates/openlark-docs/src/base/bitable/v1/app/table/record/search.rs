//! 查询记录
//!
//! doc: https://open.feishu.cn/document/docs/bitable-v1/app-table-record/search

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchRecordRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<Sort>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_fields: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Sort {
    pub field_name: String,
    pub desc: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Filter {
    pub conjunction: String,
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Condition {
    pub field_name: String,
    pub operator: String,
    pub value: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchRecordResponse {
    pub items: Vec<AppTableRecord>,
    pub has_more: bool,
    pub page_token: Option<String>,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppTableRecord {
    pub record_id: String,
    pub created_by: Option<User>,
    pub created_time: Option<i64>,
    pub last_modified_by: Option<User>,
    pub last_modified_time: Option<i64>,
    pub fields: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub id: Option<String>,
    pub name: Option<String>,
    pub en_name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

impl ApiResponseTrait for SearchRecordResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct SearchRecord {
    config: openlark_core::config::Config,
    app_token: String,
    table_id: String,
    req: SearchRecordRequest,
}

impl SearchRecord {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, table_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            table_id: table_id.into(),
            req: SearchRecordRequest::default(),
        }
    }

    pub fn view_id(mut self, view_id: impl Into<String>) -> Self {
        self.req.view_id = Some(view_id.into());
        self
    }

    pub fn field_names(mut self, field_names: Vec<String>) -> Self {
        self.req.field_names = Some(field_names);
        self
    }

    pub fn sort(mut self, sort: Vec<Sort>) -> Self {
        self.req.sort = Some(sort);
        self
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.req.filter = Some(filter);
        self
    }

    pub fn automatic_fields(mut self, automatic_fields: bool) -> Self {
        self.req.automatic_fields = Some(automatic_fields);
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<SearchRecordResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records/search",
            self.config.base_url, self.app_token, self.table_id
        );
        let request = ApiRequest::post(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
