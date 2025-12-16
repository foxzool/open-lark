//! 该接口用于更新表单中的元数据项
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/form/patch-2

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PatchFormRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_limit_once: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PatchFormResponse {
    pub form: Form,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Form {
    pub name: String,
    pub description: String,
    pub shared: bool,
    pub shared_url: String,
    pub shared_limit: String,
    pub submit_limit_once: bool,
}

impl ApiResponseTrait for PatchFormResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct PatchFormBuilder {
    api_req: ApiRequest<PatchFormRequest>,
    app_token: String,
    table_id: String,
    form_id: String,
}

impl PatchFormBuilder {
    pub fn new(app_token: impl ToString, table_id: impl ToString, form_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_form_patch".to_string();
        builder.api_req.method = "PATCH".to_string();
        builder.app_token = app_token.to_string();
        builder.table_id = table_id.to_string();
        builder.form_id = form_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}",
            builder.app_token, builder.table_id, builder.form_id
        );
        builder.api_req.body = Some(PatchFormRequest::default());
        builder
    }

    pub fn name(mut self, name: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.name = Some(name.to_string());
        }
        self
    }

    pub fn description(mut self, description: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.description = Some(description.to_string());
        }
        self
    }

    pub fn shared(mut self, shared: bool) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.shared = Some(shared);
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
