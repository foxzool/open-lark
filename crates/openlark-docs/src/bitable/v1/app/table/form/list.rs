//! 获取表单问题列表模块

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取表单问题列表请求
#[derive(Clone)]
pub struct ListFormQuestionRequest {
    api_request: ApiRequest<ListFormQuestionResponse>,
    app_token: String,
    table_id: String,
    form_id: String,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListFormQuestionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new()
                .method(HttpMethod::Get)
                .api_path("/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/questions".to_string())
                .config(config)
                .build(),
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
            page_size: None,
            page_token: None,
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

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub async fn execute(mut self) -> SDKResult<ListFormQuestionResponse> {
        let path = format!(
            "/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/questions",
            self.app_token, self.table_id, self.form_id
        );
        self.api_request = self.api_request.api_path(path);

        let config = self.api_request.config();
        let response = Transport::request(self.api_request, &config.clone(), None).await?;
        Ok(response)
    }
}

/// 表单问题项
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormQuestionItem {
    pub question_id: String,
    pub title: String,
    pub description: Option<String>,
    pub required: bool,
    pub field_type: String,
    pub options: Vec<String>,
}

/// 获取表单问题列表响应
#[derive(Clone)]
pub struct ListFormQuestionResponse {
    pub items: Vec<FormQuestionItem>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

impl ApiResponseTrait for ListFormQuestionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取表单问题列表请求构建器
#[derive(Default)]
pub struct ListFormQuestionRequestBuilder {
    request: ListFormQuestionRequest,
}

impl ListFormQuestionRequestBuilder {
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

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    pub fn build(self) -> ListFormQuestionRequest {
        self.request
    }
}