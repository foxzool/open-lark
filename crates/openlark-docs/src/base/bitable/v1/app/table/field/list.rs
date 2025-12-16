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

#[derive(Debug, Default)]
pub struct ListFieldBuilder {
    api_req: ApiRequest<ListFieldRequest>,
    app_token: String,
    table_id: String,
}

impl ListFieldBuilder {
    pub fn new(app_token: impl ToString, table_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_field_list".to_string();
        builder.api_req.method = "GET".to_string();
        builder.app_token = app_token.to_string();
        builder.table_id = table_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/fields",
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

    pub fn view_id(mut self, view_id: impl ToString) -> Self {
        if self.api_req.url.contains('?') {
            self.api_req.url.push_str(&format!("&view_id={}", view_id.to_string()));
        } else {
            self.api_req.url.push_str(&format!("?view_id={}", view_id.to_string()));
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
