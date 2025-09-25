use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_config,
};

/// 获取表单元数据请求
#[derive(Debug, Serialize, Default)]
pub struct GetFormRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 表单ID
    #[serde(skip)]
    form_id: String,
}

impl GetFormRequest {
    pub fn builder() -> GetFormRequestBuilder {
        GetFormRequestBuilder::default()
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
pub struct GetFormRequestBuilder {
    request: GetFormRequest,
}

impl GetFormRequestBuilder {
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

    pub fn build(self) -> GetFormRequest {
        self.request
    }
}

impl_executable_builder_config!(
    GetFormRequestBuilder,
    GetFormRequest,
    BaseResponse<GetFormResponse>,
    get_form
);

/// 表单信息
#[derive(Debug, Deserialize)]
pub struct Form {
    /// 表单ID
    pub form_id: String,
    /// 表单名称
    pub name: String,
    /// 表单描述
    pub description: Option<String>,
    /// 是否允许重复提交
    pub allow_resubmit: bool,
    /// 是否显示提交按钮
    pub show_submit_button: bool,
    /// 提交按钮文本
    pub submit_button_text: Option<String>,
    /// 是否分享表单
    pub shared: bool,
    /// 分享URL
    pub shared_url: Option<String>,
    /// 是否需要登录
    pub need_login: bool,
    /// 状态：启用/禁用
    pub status: String,
}

/// 获取表单元数据响应
#[derive(Debug, Deserialize)]
pub struct GetFormResponse {
    /// 表单信息
    pub form: Form,
}

impl ApiResponseTrait for GetFormResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取表单元数据
pub async fn get_form(
    request: GetFormRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<GetFormResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::GET;
    api_req.api_path = BITABLE_V1_FORM_GET
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
    fn test_get_form_request_builder() {
        let request = GetFormRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .form_id("vewxxxxxx")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.form_id, "vewxxxxxx");
    }
}
