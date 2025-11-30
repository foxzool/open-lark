#![allow(unused_variables, unused_unsafe)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use SDKResult;use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use openlark_core::,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api::{ApiResponseTrait}
    config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    impl_executable_builder_config,
};
/// 获取表单元数据请求,
#[derive(Clone)]
pub struct GetFormRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 表单ID,
#[serde(skip)]
    form_id: String}
impl GetFormRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct GetFormRequestBuilder {
    request: GetFormRequest}
impl GetFormRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_config!(,
    GetFormRequestBuilder,
    GetFormRequest,
    Response<GetFormResponse>,
    get_form,
);
/// 表单信息
#[derive(Clone)]
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
/// 获取表单元数据响应,
#[derive(Clone)]
pub struct GetFormResponse {
    /// 表单信息
    pub form: Form,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 获取表单元数据,
pub async fn get_form(
    request: GetFormRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<GetFormResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
api_req.api_path = BITABLE_V1_FORM_GET,
        .replace("{app_token}", &request.app_token)
        .replace("{form_id}", &request.form_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_get_form_request_builder() {
let request = GetFormRequest::builder(),
            .app_token()
.form_id()
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.form_id, "vewxxxxxx");
