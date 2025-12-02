//! 获取表单模块

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, HttpMethod, Response},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取表单请求
#[derive(Debug, Clone)]
pub struct GetFormRequest {
    config: Config,
    api_request: ApiRequest<GetFormResponse>,
    app_token: String,
    table_id: String,
    form_id: String,
}

impl GetFormRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::get("https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}"),
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
        let url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}",
            self.app_token, self.table_id, self.form_id
        );

        // 创建新的请求
        let api_request = ApiRequest::<()>::get(&url);

        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}

/// 表单结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Form {
    pub form_id: String,
    pub name: String,
    pub items: Vec<FormItem>,
}

/// 表单项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormItem {
    pub field_name: String,
    pub field_type: String,
    pub required: bool,
}

/// 获取表单响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFormResponse {
    pub form: Form,
}

impl ApiResponseTrait for GetFormResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取表单
pub async fn get_form(
    request: GetFormRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetFormResponse>> {
    let api_path = format!(
        "/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}",
        &request.app_token, &request.table_id, &request.form_id
    );
    // 创建新的ApiRequest，正确的类型
    let api_req = ApiRequest::<()>::get(api_path);

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}