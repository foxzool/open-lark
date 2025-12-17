//! 该接口用于根据 record_id 的值检索现有记录
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/get

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetRecordRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_field_as_array: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_formula_ref: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_shared_url: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_fields: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetRecordResponse {
    pub record: AppTableRecord,
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

impl ApiResponseTrait for GetRecordResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct GetRecord {
    config: openlark_core::config::Config,
    app_token: String,
    table_id: String,
    record_id: String,
    req: GetRecordRequest,
}

impl GetRecord {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, table_id: impl Into<String>, record_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            table_id: table_id.into(),
            record_id: record_id.into(),
            req: GetRecordRequest::default(),
        }
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

    pub fn with_shared_url(mut self, with_shared_url: bool) -> Self {
        self.req.with_shared_url = Some(with_shared_url);
        self
    }

    pub fn automatic_fields(mut self, automatic_fields: bool) -> Self {
        self.req.automatic_fields = Some(automatic_fields);
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<GetRecordResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records/{}",
            self.config.base_url, self.app_token, self.table_id, self.record_id
        );
        let request = ApiRequest::get(&url).query(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
