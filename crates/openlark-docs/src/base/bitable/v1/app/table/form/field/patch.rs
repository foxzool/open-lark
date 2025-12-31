/// Bitable 更新表单问题
///
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-form-field/patch
/// doc: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-form-field/patch
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

use super::models::PatchFormFieldRequest;

/// 更新表单问题请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PatchFormFieldQuestionRequest {
    config: Config,
    app_token: String,
    table_id: String,
    form_id: String,
    field_id: String,
    body: PatchFormFieldRequest,
}

impl PatchFormFieldQuestionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
            field_id: String::new(),
            body: PatchFormFieldRequest::new(),
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

    pub fn field_id(mut self, field_id: String) -> Self {
        self.field_id = field_id;
        self
    }

    pub fn pre_field_id(mut self, pre_field_id: String) -> Self {
        self.body.pre_field_id = Some(pre_field_id);
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.body.title = Some(title);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.body.description = Some(description);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.body.required = Some(required);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.body.visible = Some(visible);
        self
    }

    pub async fn execute(self) -> SDKResult<PatchFormFieldQuestionResponse> {
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "app_token 不能为空"));
        }
        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "table_id 不能为空"));
        }
        if self.form_id.trim().is_empty() {
            return Err(validation_error("form_id", "form_id 不能为空"));
        }
        if self.field_id.trim().is_empty() {
            return Err(validation_error("field_id", "field_id 不能为空"));
        }
        self.body
            .validate()
            .map_err(|msg| validation_error("body", msg))?;

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::FormFieldPatch(
            self.app_token,
            self.table_id,
            self.form_id,
            self.field_id,
        );

        let api_request: ApiRequest<PatchFormFieldQuestionResponse> =
            ApiRequest::patch(&api_endpoint.to_url()).body(serde_json::to_vec(&self.body)?);

        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("response", "响应数据为空"))
    }
}

/// 更新表单问题 Builder
pub struct PatchFormFieldQuestionBuilder {
    request: PatchFormFieldQuestionRequest,
}

impl PatchFormFieldQuestionBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: PatchFormFieldQuestionRequest::new(config),
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

    pub fn field_id(mut self, field_id: String) -> Self {
        self.request = self.request.field_id(field_id);
        self
    }

    pub fn pre_field_id(mut self, pre_field_id: String) -> Self {
        self.request = self.request.pre_field_id(pre_field_id);
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.request = self.request.title(title);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.request = self.request.description(description);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.request = self.request.required(required);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.request = self.request.visible(visible);
        self
    }

    pub fn build(self) -> PatchFormFieldQuestionRequest {
        self.request
    }
}

/// 更新后的表单问题（响应 field）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchedFormFieldQuestion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_field_id: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub required: bool,
    pub visible: bool,
}

/// 更新表单问题响应（data）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchFormFieldQuestionResponse {
    pub field: PatchedFormFieldQuestion,
}

impl ApiResponseTrait for PatchFormFieldQuestionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
