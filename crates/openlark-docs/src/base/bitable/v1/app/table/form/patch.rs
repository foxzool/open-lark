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

#[derive(Debug)]
pub struct PatchForm {
    config: openlark_core::config::Config,
    app_token: String,
    table_id: String,
    form_id: String,
    req: PatchFormRequest,
}

impl PatchForm {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, table_id: impl Into<String>, form_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            table_id: table_id.into(),
            form_id: form_id.into(),
            req: PatchFormRequest::default(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.req.name = Some(name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.req.description = Some(description.into());
        self
    }

    pub fn shared(mut self, shared: bool) -> Self {
        self.req.shared = Some(shared);
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<PatchFormResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}",
            self.config.base_url, self.app_token, self.table_id, self.form_id
        );
        let request = ApiRequest::patch(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
