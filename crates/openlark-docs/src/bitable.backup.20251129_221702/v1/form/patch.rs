#![allow(unused_variables, unused_unsafe)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use serde_json::Value;
use SDKResult;use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use super::FormService;
use openlark_core::,
{,
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
    impl_executable_builder_owned,
    service::bitable::v1::form::FormQuestion,
};
/// 更新表单问题请求,
#[derive(Clone)]
pub struct PatchFormQuestionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 表单ID,
#[serde(skip)]
    form_id: String,
    /// 问题ID,
#[serde(skip)]
    question_id: String,
    /// 问题标题,
#[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    /// 问题描述,
#[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// 是否必填,
#[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
    /// 是否可见,
#[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    /// 问题设置,
#[serde(skip_serializing_if = "Option::is_none")]
    setting: Option<serde_json::Value>}
impl PatchFormQuestionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct PatchFormQuestionRequestBuilder {
    request: PatchFormQuestionRequest}
impl PatchFormQuestionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    PatchFormQuestionRequestBuilder,
    FormService,
    PatchFormQuestionRequest,
    Response<PatchFormQuestionResponse>,
    patch,
);
/// 更新表单问题响应
#[derive(Clone)]
pub struct PatchFormQuestionResponse {
    /// 更新后的问题信息
    pub question: FormQuestion,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 更新表单问题,
pub async fn patch_form_question(
    request: PatchFormQuestionRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<PatchFormQuestionResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::PATCH);
api_req.api_path = BITABLE_V1_FORM_PATCH,
        .replace("{app_token}", &request.app_token)
        .replace("{form_id}", &request.form_id)
        .replace("{question_id}", &request.question_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_patch_form_question_request_builder() {
let request = PatchFormQuestionRequest::builder(),
            .app_token()
.form_id()
            .question_id()
.title()
            .required()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.form_id, "vewxxxxxx");
        assert_eq!(request.question_id, "qstxxxxxx");
        assert_eq!(request.title, Some("更新后的问题标题".to_string()));
        assert_eq!(request.required, Some(true));
