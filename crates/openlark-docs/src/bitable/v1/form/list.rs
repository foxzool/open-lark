#![allow(unused_variables, unused_unsafe)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use serde_json::Value;
use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use openlark_core::{,
use SDKResult;    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
/// 列出表单问题请求,
#[derive(Clone)]
pub struct ListFormQuestionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 表单ID,
#[serde(skip)]
    form_id: String,
    /// 分页标记,
#[serde(skip)]
    page_token: Option<String>,
    /// 分页大小,
#[serde(skip)]
    page_size: Option<i32>}
impl ListFormQuestionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct ListFormQuestionRequestBuilder {
    request: ListFormQuestionRequest}
impl ListFormQuestionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(page_size) = &self.request.page_size {,
            self.request,
.api_request,
                .query_params
                .insert("page_size", page_size.to_string());
self.request,
    }
// 应用ExecutableBuilder trait到ListFormQuestionRequestBuilder,
crate::impl_executable_builder_owned!(
    ListFormQuestionRequestBuilder,
    super::FormService,
    ListFormQuestionRequest,
    Response<ListFormQuestionResponse>,
    list,
);
/// 表单问题信息
#[derive(Clone)]
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
    pub setting: Option<serde_json::Value>}
/// 列出表单问题响应,
#[derive(Clone)]
pub struct ListFormQuestionResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
    /// 问题信息列表
    pub items: Vec<FormQuestion>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 列出表单问题,
pub async fn list_form_questions(
    request: ListFormQuestionRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<ListFormQuestionResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
api_req.api_path = BITABLE_V1_FORM_QUESTION,
        .replace("{app_token}", &request.app_token)
        .replace("{form_id}", &request.form_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_list_form_question_request_builder() {
let request = ListFormQuestionRequest::builder(),
            .app_token()
.form_id()
            .page_size()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.form_id, "vewxxxxxx");
        assert_eq!(request.page_size, Some(20));
