//! 获取表单问题列表模块

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取表单问题列表请求
#[derive(Debug, Clone)]
pub struct ListFormQuestionRequest {
    config: Config,
    api_request: ApiRequest<ListFormQuestionResponse>,
    app_token: String,
    table_id: String,
    form_id: String,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl Default for ListFormQuestionRequest {
    fn default() -> Self {
        Self {
            config: Config::default(),
            api_request: ApiRequest::get(
                "/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/questions".to_string(),
            ),
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
            page_size: None,
            page_token: None,
        }
    }
}

impl ListFormQuestionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::get(
                "/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/questions".to_string(),
            ),
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

    pub async fn execute(self) -> SDKResult<ListFormQuestionResponse> {
        let url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/questions",
            self.app_token, self.table_id, self.form_id
        );

        // 创建新的请求
        let api_request = ApiRequest::<()>::get(&url);

        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 表单问题项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormQuestionItem {
    pub question_id: String,
    pub title: String,
    pub description: Option<String>,
    pub required: bool,
    pub field_type: String,
    pub options: Vec<String>,
}

/// 获取表单问题列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// 获取表单问题列表
pub async fn list_form_questions(
    request: ListFormQuestionRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<openlark_core::api::Response<ListFormQuestionResponse>> {
    let api_path = format!(
        "/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/questions",
        &request.app_token, &request.table_id, &request.form_id
    );
    // 创建新的ApiRequest，正确的类型
    let mut api_req = ApiRequest::<()>::get(api_path);

    // 添加查询参数
    if let Some(page_size) = request.page_size {
        api_req = api_req.query("page_size", page_size.to_string());
    }
    if let Some(page_token) = &request.page_token {
        api_req = api_req.query("page_token", page_token);
    }

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}
