//! 该接口用于更新表单中的问题项
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/form/patch

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PatchFormFieldRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_field_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PatchFormFieldResponse {
    pub field: FormField,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FormField {
    pub field_id: String,
    pub title: String,
    pub description: String,
    pub required: bool,
    pub visible: bool,
}

impl ApiResponseTrait for PatchFormFieldResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct PatchFormField {
    config: openlark_core::config::Config,
    app_token: String,
    table_id: String,
    form_id: String,
    field_id: String,
    req: PatchFormFieldRequest,
}

impl PatchFormField {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, table_id: impl Into<String>, form_id: impl Into<String>, field_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            table_id: table_id.into(),
            form_id: form_id.into(),
            field_id: field_id.into(),
            req: PatchFormFieldRequest::default(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.req.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.req.description = Some(description.into());
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<PatchFormFieldResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/fields/{}",
            self.config.base_url, self.app_token, self.table_id, self.form_id, self.field_id
        );
        let request = ApiRequest::patch(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
