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

#[derive(Debug, Default)]
pub struct ListRecordBuilder {
    api_req: ApiRequest<ListRecordRequest>,
    app_token: String,
    table_id: String,
}

impl ListRecordBuilder {
    pub fn new(app_token: impl ToString, table_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_record_list".to_string();
        builder.api_req.method = "GET".to_string();
        builder.app_token = app_token.to_string();
        builder.table_id = table_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records",
            builder.app_token, builder.table_id
        );
        builder.api_req.body = None;
        builder
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        if self.api_req.url.contains('?') {
            self.api_req.url.push_str(&format!("&page_size={}", page_size));
        } else {
            self.api_req.url.push_str(&format!("?page_size={}", page_size));
        }
        self
    }

    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        if self.api_req.url.contains('?') {
            self.api_req.url.push_str(&format!("&page_token={}", page_token.to_string()));
        } else {
            self.api_req.url.push_str(&format!("?page_token={}", page_token.to_string()));
        }
        self
    }

    pub fn build(
        self,
        config: &openlark_core::config::Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let mut req = self.api_req;
        req.build(AccessTokenType::Tenant, config, option)
    }
}
