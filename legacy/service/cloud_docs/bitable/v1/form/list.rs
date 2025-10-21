use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 列出表单问题请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct ListFormQuestionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
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

impl ListFormQuestionRequest {
    pub fn builder() -> ListFormQuestionRequestBuilder {
        ListFormQuestionRequestBuilder::default()
    }

    pub fn new(app_token: impl ToString, form_id: impl ToString) -> Self {
        Self {
            app_token: app_token.to_string(),
            form_id: form_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct ListFormQuestionRequestBuilder {
    request: ListFormQuestionRequest,
}

impl ListFormQuestionRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 表单ID
    pub fn form_id(mut self, form_id: impl ToString) -> Self {
        self.request.form_id = form_id.to_string();
        self
    }

    /// 分页标记
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    pub fn build(mut self) -> ListFormQuestionRequest {
        if let Some(page_token) = &self.request.page_token {
            self.request
                .api_request
                .query_params
                .insert("page_token", page_token.clone());
        }
        if let Some(page_size) = &self.request.page_size {
            self.request
                .api_request
                .query_params
                .insert("page_size", page_size.to_string());
        }
        self.request
    }
}

// 应用ExecutableBuilder trait到ListFormQuestionRequestBuilder
crate::impl_executable_builder_owned!(
    ListFormQuestionRequestBuilder,
    super::FormService,
    ListFormQuestionRequest,
    BaseResponse<ListFormQuestionResponse>,
    list
);

/// 表单问题信息
#[derive(Debug, Deserialize)]
pub struct FormQuestion {
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

/// 列出表单问题响应
#[derive(Debug, Deserialize)]
pub struct ListFormQuestionResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
    /// 问题信息列表
    pub items: Vec<FormQuestion>,
}

impl ApiResponseTrait for ListFormQuestionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出表单问题
pub async fn list_form_questions(
    request: ListFormQuestionRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<ListFormQuestionResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::GET;
    api_req.api_path = BITABLE_V1_FORM_QUESTION
        .replace("{app_token}", &request.app_token)
        .replace("{form_id}", &request.form_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_list_form_question_request_builder() {
        let request = ListFormQuestionRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .form_id("vewxxxxxx")
            .page_size(20)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.form_id, "vewxxxxxx");
        assert_eq!(request.page_size, Some(20));
    }
}
