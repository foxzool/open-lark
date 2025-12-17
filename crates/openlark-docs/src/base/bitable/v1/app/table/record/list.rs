//! 该接口用于列出数据表中的现有记录，单次最多列出 500 行记录，支持分页获取。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/list

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListRecordRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_names: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_field_as_array: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_formula_ref: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_fields: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListRecordResponse {
    pub has_more: bool,
    pub page_token: Option<String>,
    pub total: i32,
    pub items: Vec<AppTableRecord>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppTableRecord {
    pub record_id: String,
    pub created_by: Option<User>,
    pub created_time: Option<i64>,
    pub last_modified_by: Option<User>,
    pub last_modified_time: Option<i64>,
    // Fields are dynamic key-value pairs
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

impl ApiResponseTrait for ListRecordResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct ListRecord {
    config: openlark_core::config::Config,
    app_token: String,
    table_id: String,
    req: ListRecordRequest,
}

impl ListRecord {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, table_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            table_id: table_id.into(),
            req: ListRecordRequest::default(),
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

    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.req.filter = Some(filter.into());
        self
    }

    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.req.sort = Some(sort.into());
        self
    }

    pub fn field_names(mut self, field_names: impl Into<String>) -> Self {
        self.req.field_names = Some(field_names.into());
        self
    }

    pub fn text_field_as_array(mut self, text_field_as_array: bool) -> Self {
        self.req.text_field_as_array = Some(text_field_as_array);
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.req.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn display_formula_ref(mut self, display_formula_ref: bool) -> Self {
        self.req.display_formula_ref = Some(display_formula_ref);
        self
    }

    pub fn automatic_fields(mut self, automatic_fields: bool) -> Self {
        self.req.automatic_fields = Some(automatic_fields);
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<ListRecordResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records",
            self.config.base_url, self.app_token, self.table_id
        );
        let request = ApiRequest::get(&url).query(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
