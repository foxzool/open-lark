//! Bitable 列出表单问题
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-form-field/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 表单问题项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FormFieldQuestion {
    pub field_id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub required: bool,
    pub visible: bool,
}

/// 列出表单问题请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ListFormFieldQuestionRequest {
    config: Config,
    app_token: String,
    table_id: String,
    form_id: String,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListFormFieldQuestionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
            page_size: None,
            page_token: None,
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

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    pub async fn execute(self) -> SDKResult<ListFormFieldQuestionResponse> {
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "app_token 不能为空"));
        }
        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "table_id 不能为空"));
        }
        if self.form_id.trim().is_empty() {
            return Err(validation_error("form_id", "form_id 不能为空"));
        }
        if let Some(page_size) = self.page_size {
            if page_size < 1 || page_size > 100 {
                return Err(validation_error("page_size", "page_size 必须在 1~100 之间"));
            }
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::FormFieldList(
            self.app_token.clone(),
            self.table_id.clone(),
            self.form_id.clone(),
        );

        let mut api_request: ApiRequest<ListFormFieldQuestionResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            api_request = api_request.query("page_token", page_token);
        }

        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("response", "响应数据为空"))
    }
}

/// 列出表单问题 Builder
pub struct ListFormFieldQuestionRequestBuilder {
    request: ListFormFieldQuestionRequest,
}

impl ListFormFieldQuestionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: ListFormFieldQuestionRequest::new(config),
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

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    pub fn page_token(mut self, page_token: String) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    pub fn build(self) -> ListFormFieldQuestionRequest {
        self.request
    }
}

/// 列出表单问题响应（data）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListFormFieldQuestionResponse {
    pub items: Vec<FormFieldQuestion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    pub has_more: bool,
    pub total: i32,
}

impl ApiResponseTrait for ListFormFieldQuestionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
