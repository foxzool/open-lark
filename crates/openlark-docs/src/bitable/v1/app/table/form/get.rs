//! 获取表单模块

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取表单请求
#[derive(Clone)]
pub struct GetFormRequest {
    api_request: ApiRequest<GetFormResponse>,
    app_token: String,
    table_id: String,
    form_id: String,
}

impl GetFormRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new()
                .method(HttpMethod::Get)
                .api_path("/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}".to_string())
                .config(config)
                .build(),
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
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

    pub async fn execute(mut self) -> SDKResult<GetFormResponse> {
        let path = format!(
            "/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}",
            self.app_token, self.table_id, self.form_id
        );
        self.api_request = self.api_request.api_path(path);

        let config = self.api_request.config();
        let response = Transport::request(self.api_request, &config.clone(), None).await?;
        Ok(response)
    }
}

/// 表单结构
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Form {
    pub form_id: String,
    pub name: String,
    pub items: Vec<FormItem>,
}

/// 表单项
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormItem {
    pub field_name: String,
    pub field_type: String,
    pub required: bool,
}

/// 获取表单响应
#[derive(Clone)]
pub struct GetFormResponse {
    pub form: Form,
}

impl ApiResponseTrait for GetFormResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}