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
    service::bitable::v1::form::Form,
};
/// 更新表单元数据请求,
#[derive(Clone)]
pub struct PatchFormMetaRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 表单ID,
#[serde(skip)]
    form_id: String,
    /// 表单名称,
#[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// 表单描述,
#[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// 是否允许重复提交,
#[serde(skip_serializing_if = "Option::is_none")]
    allow_resubmit: Option<bool>,
    /// 是否显示提交按钮,
#[serde(skip_serializing_if = "Option::is_none")]
    show_submit_button: Option<bool>,
    /// 提交按钮文本,
#[serde(skip_serializing_if = "Option::is_none")]
    submit_button_text: Option<String>,
    /// 是否分享表单,
#[serde(skip_serializing_if = "Option::is_none")]
    shared: Option<bool>,
    /// 是否需要登录,
#[serde(skip_serializing_if = "Option::is_none")]
    need_login: Option<bool>,
    /// 状态：启用/禁用,
#[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>}
impl PatchFormMetaRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct PatchFormMetaRequestBuilder {
    request: PatchFormMetaRequest}
impl PatchFormMetaRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 更新表单元数据响应,
#[derive(Clone)]
pub struct PatchFormMetaResponse {
    /// 更新后的表单信息
    pub form: Form,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 更新表单元数据,
pub async fn patch_form_meta(
    request: PatchFormMetaRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<PatchFormMetaResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::PATCH);
api_req.api_path = BITABLE_V1_FORM_PATCH,
        .replace("{app_token}", &request.app_token)
        .replace("{form_id}", &request.form_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_patch_form_meta_request_builder() {
let request = PatchFormMetaRequest::builder(),
            .app_token()
.form_id()
            .name()
.description()
            .allow_resubmit()
.shared()
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.form_id, "vewxxxxxx");
        assert_eq!(request.name, Some("更新后的表单名称".to_string()));
        assert_eq!(request.description, Some("更新后的表单描述".to_string()));
        assert_eq!(request.allow_resubmit, Some(true));
        assert_eq!(request.shared, Some(true));
