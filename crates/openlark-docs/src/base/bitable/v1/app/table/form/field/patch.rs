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

#[derive(Debug, Default)]
pub struct PatchFormFieldBuilder {
    api_req: ApiRequest<PatchFormFieldRequest>,
    app_token: String,
    table_id: String,
    form_id: String,
    field_id: String,
}

impl PatchFormFieldBuilder {
    pub fn new(app_token: impl ToString, table_id: impl ToString, form_id: impl ToString, field_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_form_field_patch".to_string();
        builder.api_req.method = "PATCH".to_string();
        builder.app_token = app_token.to_string();
        builder.table_id = table_id.to_string();
        builder.form_id = form_id.to_string();
        builder.field_id = field_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/fields/{}",
            builder.app_token, builder.table_id, builder.form_id, builder.field_id
        );
        builder.api_req.body = Some(PatchFormFieldRequest::default());
        builder
    }

    pub fn title(mut self, title: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.title = Some(title.to_string());
        }
        self
    }

    pub fn description(mut self, description: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.description = Some(description.to_string());
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
