/// Bitable 更新表单元数据
///
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-form/patch
/// doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/form/patch-2
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

use super::models::Form;

/// 更新表单元数据请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PatchFormRequest {
    config: Config,
    app_token: String,
    table_id: String,
    form_id: String,

    name: Option<String>,
    description: Option<String>,
    shared: Option<bool>,
    shared_limit: Option<String>,
    submit_limit_once: Option<bool>,
}

impl PatchFormRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
            name: None,
            description: None,
            shared: None,
            shared_limit: None,
            submit_limit_once: None,
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    pub fn form_id(mut self, form_id: String) -> Self {
        self.form_id = form_id;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn shared(mut self, shared: bool) -> Self {
        self.shared = Some(shared);
        self
    }

    pub fn shared_limit(mut self, shared_limit: String) -> Self {
        self.shared_limit = Some(shared_limit);
        self
    }

    pub fn submit_limit_once(mut self, submit_limit_once: bool) -> Self {
        self.submit_limit_once = Some(submit_limit_once);
        self
    }

    pub async fn execute(self) -> SDKResult<PatchFormResponse> {
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "app_token 不能为空"));
        }
        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "table_id 不能为空"));
        }
        if self.form_id.trim().is_empty() {
            return Err(validation_error("form_id", "form_id 不能为空"));
        }

        if self.name.is_none()
            && self.description.is_none()
            && self.shared.is_none()
            && self.shared_limit.is_none()
            && self.submit_limit_once.is_none()
        {
            return Err(validation_error("body", "至少需要提供一个要更新的字段"));
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::FormPatch(self.app_token, self.table_id, self.form_id);

        let api_request: ApiRequest<PatchFormResponse> = ApiRequest::patch(&api_endpoint.to_url())
            .body(serde_json::to_vec(&PatchFormRequestBody {
                name: self.name,
                description: self.description,
                shared: self.shared,
                shared_limit: self.shared_limit,
                submit_limit_once: self.submit_limit_once,
            })?);

        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("response", "响应数据为空"))
    }
}

/// 更新表单元数据 Builder
pub struct PatchFormRequestBuilder {
    request: PatchFormRequest,
}

impl PatchFormRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: PatchFormRequest::new(config),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    pub fn table_id(mut self, table_id: String) -> Self {
        self.request = self.request.table_id(table_id);
        self
    }

    pub fn form_id(mut self, form_id: String) -> Self {
        self.request = self.request.form_id(form_id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.request = self.request.name(name);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.request = self.request.description(description);
        self
    }

    pub fn shared(mut self, shared: bool) -> Self {
        self.request = self.request.shared(shared);
        self
    }

    pub fn shared_limit(mut self, shared_limit: String) -> Self {
        self.request = self.request.shared_limit(shared_limit);
        self
    }

    pub fn submit_limit_once(mut self, submit_limit_once: bool) -> Self {
        self.request = self.request.submit_limit_once(submit_limit_once);
        self
    }

    pub fn build(self) -> PatchFormRequest {
        self.request
    }
}

#[derive(Serialize)]
struct PatchFormRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    submit_limit_once: Option<bool>,
}

/// 更新表单元数据响应（data）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchFormResponse {
    pub form: Form,
}

impl ApiResponseTrait for PatchFormResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
