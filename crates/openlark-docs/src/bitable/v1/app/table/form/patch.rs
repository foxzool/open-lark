//! Bitable 更新表单问题API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/table/form/patch
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod, RequestData, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新表单问题请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PatchFormQuestionRequest {
    config: Config,
    app_token: String,
    table_id: String,
    form_id: String,
    question_id: String,
    title: Option<String>,
    description: Option<String>,
    required: Option<bool>,
}

impl Default for PatchFormQuestionRequest {
    fn default() -> Self {
        Self {
            config: Config::default(),
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
            question_id: String::new(),
            title: None,
            description: None,
            required: None,
        }
    }
}

impl PatchFormQuestionRequest {
    pub fn new(config: Config) -> Self {
        let mut request = Self::default();
        request.config = config;
        request
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

    pub async fn execute(self) -> SDKResult<PatchFormQuestionResponse> {
        let url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/questions/{}",
            self.app_token, self.table_id, self.form_id, self.question_id
        );

        let body = serde_json::json!({
            "title": self.title,
            "description": self.description,
            "required": self.required
        });

        // 创建PATCH请求
        let api_request: ApiRequest<()> = ApiRequest {
            method: HttpMethod::Patch,
            url: url.clone(),
            headers: std::collections::HashMap::new(),
            query: std::collections::HashMap::new(),
            body: Some(RequestData::Json(body)),
            timeout: None,
            _phantom: std::marker::PhantomData,
        };

        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 更新表单问题响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchFormQuestionResponse {
    pub question: Value,
}

impl ApiResponseTrait for PatchFormQuestionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新表单问题请求构建器
pub struct PatchFormQuestionRequestBuilder {
    request: PatchFormQuestionRequest,
}

impl Default for PatchFormQuestionRequestBuilder {
    fn default() -> Self {
        Self {
            request: PatchFormQuestionRequest::default(),
        }
    }
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

/// 更新表单问题
pub async fn patch_form_question(
    request: PatchFormQuestionRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<openlark_core::api::Response<PatchFormQuestionResponse>> {
    let api_path = format!(
        "/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/questions/{}",
        &request.app_token, &request.table_id, &request.form_id, &request.question_id
    );

    let body = serde_json::json!({
        "title": request.title,
        "description": request.description,
        "required": request.required
    });

    // 创建PATCH请求
    let api_req: ApiRequest<()> = ApiRequest {
        method: HttpMethod::Patch,
        url: api_path.to_string(),
        headers: std::collections::HashMap::new(),
        query: std::collections::HashMap::new(),
        body: Some(RequestData::Json(body)),
        timeout: None,
        _phantom: std::marker::PhantomData,
    };

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}
