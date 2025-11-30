
use serde_json::Value;
use reqwest::Method;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    
    
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 列出表单字段问题请求
#[derive(Clone)]
pub struct ListFormFieldQuestionRequest {
    #[serde(skip)]
    api_request: ApiRequest<Self>,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 表单ID
    #[serde(skip)]
    form_id: String,
    /// 分页标记
    #[serde(skip)]
    page_token: Option<String>,
    /// 分页大小
    #[serde(skip)]
    page_size: Option<i32>,
}

impl ListFormFieldQuestionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config),
            app_token: String::new(),
            form_id: String::new(),
            page_token: None,
            page_size: None,
        }
    }
}

#[derive(Clone)]
pub struct ListFormFieldQuestionRequestBuilder {
    request: ListFormFieldQuestionRequest,
}

impl ListFormFieldQuestionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: ListFormFieldQuestionRequest::new(config),
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn form_id(mut self, form_id: impl Into<String>) -> Self {
        self.request.form_id = form_id.into();
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    pub fn build(self) -> ListFormFieldQuestionRequest {
        // 设置查询参数
        if let Some(page_size) = &self.request.page_size {
            self.request
                .api_request
                .query_params
                .insert(page_size.to_string(), page_size.to_string());
        }

        if let Some(page_token) = &self.request.page_token {
            self.request
                .api_request
                .query_params
                .insert(page_token.to_string(), page_token.clone());
        }

        self.request
    }
}

// 应用ExecutableBuilder trait到ListFormFieldQuestionRequestBuilder
crate::impl_executable_builder_owned!(
    ListFormFieldQuestionRequestBuilder,
    super::FormFieldService,
    ListFormFieldQuestionRequest,
    Response<ListFormFieldQuestionResponse>,
    list,
);

/// 表单字段问题信息
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormFieldQuestion {
    /// 问题ID
    pub question_id: String,
    /// 问题标题
    pub title: String,
    /// 问题描述
    pub description: Option<String>,
    /// 问题类型
    pub question_type: String,
    /// 是否必填
    pub required: bool,
    /// 是否可见
    pub visible: bool,
    /// 字段ID
    pub field_id: String,
    /// 问题设置
    pub setting: Option<serde_json::Value>,
}

/// 列出表单字段问题响应
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListFormFieldQuestionResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
    /// 问题信息列表
    pub items: Vec<FormFieldQuestion>,
}

impl ApiResponseTrait for ListFormFieldQuestionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出表单字段问题
pub async fn list_form_field_questions(
    request: ListFormFieldQuestionRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<ListFormFieldQuestionResponse>> {
    let mut api_req = request.api_request;
    let api_path = format!(
        "/open-apis/bitable/v1/apps/{}/forms/{}/fields",
        &request.app_token, &request.form_id
    );
    api_req = api_req.api_path(api_path);

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

