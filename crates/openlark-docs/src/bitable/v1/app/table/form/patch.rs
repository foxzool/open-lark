//! 更新表单问题模块

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新表单问题请求
#[derive(Clone)]
pub struct PatchFormQuestionRequest {
    api_request: ApiRequest<PatchFormQuestionResponse>,
    app_token: String,
    table_id: String,
    form_id: String,
    question_id: String,
    title: Option<String>,
    description: Option<String>,
    required: Option<bool>,
}

impl PatchFormQuestionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new()
                .method(HttpMethod::Patch)
                .api_path("/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/questions/{}".to_string())
                .config(config)
                .build(),
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
            question_id: String::new(),
            title: None,
            description: None,
            required: None,
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
        self.table_id = table_id.into();
        self
    }

    pub fn form_id(mut self, form_id: impl Into<String>) -> Self {
        self.form_id = form_id.into();
        self
    }

    pub fn question_id(mut self, question_id: impl Into<String>) -> Self {
        self.question_id = question_id.into();
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    pub async fn execute(mut self) -> SDKResult<PatchFormQuestionResponse> {
        let path = format!(
            "/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/questions/{}",
            self.app_token, self.table_id, self.form_id, self.question_id
        );
        self.api_request = self.api_request.api_path(path);

        let body = serde_json::json!({
            "title": self.title,
            "description": self.description,
            "required": self.required
        });

        self.api_request = self.api_request.body(serde_json::to_vec(&body)?);

        let config = self.api_request.config();
        let response = Transport::request(self.api_request, &config.clone(), None).await?;
        Ok(response)
    }
}

/// 更新表单问题响应
#[derive(Clone)]
pub struct PatchFormQuestionResponse {
    pub question: Value,
}

impl ApiResponseTrait for PatchFormQuestionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新表单问题请求构建器
#[derive(Default)]
pub struct PatchFormQuestionRequestBuilder {
    request: PatchFormQuestionRequest,
}

impl PatchFormQuestionRequestBuilder {
    pub fn new(app_token: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.request.app_token = app_token.into();
        builder
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
        self.request.table_id = table_id.into();
        self
    }

    pub fn form_id(mut self, form_id: impl Into<String>) -> Self {
        self.request.form_id = form_id.into();
        self
    }

    pub fn question_id(mut self, question_id: impl Into<String>) -> Self {
        self.request.question_id = question_id.into();
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.request.required = Some(required);
        self
    }

    pub fn build(self) -> PatchFormQuestionRequest {
        self.request
    }
}