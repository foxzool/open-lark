//! 该接口用于在数据表中新增一个字段
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-field/create

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateFieldRequest {
    pub field_name: String,
    #[serde(rename = "type")]
    pub type_: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateFieldResponse {
    pub field: Field,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Field {
    pub field_id: String,
    pub field_name: String,
    #[serde(rename = "type")]
    pub type_: i32,
    pub property: Option<serde_json::Value>,
}

impl ApiResponseTrait for CreateFieldResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct CreateField {
    config: openlark_core::config::Config,
    app_token: String,
    table_id: String,
    req: CreateFieldRequest,
}

impl CreateField {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, table_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            table_id: table_id.into(),
            req: CreateFieldRequest::default(),
        }
    }

    pub fn field_name(mut self, field_name: impl Into<String>) -> Self {
        self.req.field_name = field_name.into();
        self
    }

    pub fn type_(mut self, type_: i32) -> Self {
        self.req.type_ = type_;
        self
    }

    pub fn property(mut self, property: serde_json::Value) -> Self {
        self.req.property = Some(property);
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<CreateFieldResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/fields",
            self.config.base_url, self.app_token, self.table_id
        );
        let request = ApiRequest::post(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
