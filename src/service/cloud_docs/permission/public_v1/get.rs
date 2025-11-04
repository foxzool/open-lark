use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::core::{,
use crate::core::SDKResult;    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::{cloud_docs::*, EndpointBuilder}
    http::Transport,
    query_params::QueryParams,
    req_option::RequestOption,
    SDKResult,
};
/// 获取云文档权限设置请求,
#[derive(Debug, Clone)]
pub struct GetPermissionPublicRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token,
#[serde(skip)]
    token: String,
    /// 文档类型,
#[serde(skip)]
    obj_type: String}
impl GetPermissionPublicRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct GetPermissionPublicRequestBuilder {
    request: GetPermissionPublicRequest}
impl GetPermissionPublicRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 公开访问设置,
#[derive(Debug, Clone)]
pub struct PublicSettings {
    /// 链接分享设置
    pub link_share_setting: String,
    /// 密码保护（如果有）
    pub password_switch: bool,
    /// 是否允许复制
    pub allow_copy: bool,
    /// 是否允许评论
    pub allow_comment: bool,
    /// 是否允许保存副本
    pub allow_save_copy: bool,
    /// 访问权限
    pub access_setting: Option<String>,
    /// 水印设置
    pub watermark_setting: Option<String>}
/// 获取云文档权限设置响应,
#[derive(Debug, Clone)]
pub struct GetPermissionPublicResponse {
    /// 公开访问设置
    pub permission_public: PublicSettings,
    /// 外部访问设置（如果有）
    pub external_access: Option<serde_json::Value>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 获取云文档权限设置,
pub async fn get_permission_public(
    request: GetPermissionPublicRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<GetPermissionPublicResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
api_req.set_api_path(EndpointBuilder::replace_param(,
        DRIVE_V1_PERMISSIONS_PUBLIC,
        "token",
        &request.token,
    ));
// 添加查询参数,
    api_req
.query_params
        .insert(QueryParams::TYPE, request.obj_type);

    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp)}

impl PublicSettings {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取权限摘要,
    pub fn w+.*{
let mut features = Vec::new();
        if self.allow_copy {,
features.push("允许复制");
        }
if self.allow_comment {,
            features.push("允许评论");
if self.allow_save_copy {,
            features.push("允许保存副本");
if self.password_switch {,
            features.push("密码保护");
if features.is_empty() {,
            "基础权限".to_string()} else {
            features.join(", ")}
    }
/// 获取安全级别,
    pub fn w+.*{
if self.link_share_setting == "closed" {,
            "最安全"} else if self.password_switch {,
"较安全"} else if self.is_tenant_accessible() {,
"中等安全"} else if self.is_anyone_accessible() {,
"较低安全"} else {,
"未知"}
impl GetPermissionPublicResponse {
    pub fn new(config: Config) -> Self {
        Self { config }
}if self.permission_public.allow_copy && self.permission_public.is_anyone_accessible() {,
            recommendations.push("建议限制复制权限以防止内容泄露".to_string());
if self.permission_public.is_editable() && self.permission_public.is_anyone_accessible() {,
            recommendations.push("建议将编辑权限限制在组织内".to_string());
if recommendations.is_empty() {,
            recommendations.push("当前权限设置合理".to_string());
recommendations,
    }
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_get_permission_public_request_builder() {
let request = GetPermissionPublicRequest::builder(),
            .token()
.as_doc()
            .build();

        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
#[test]
    fn test_convenience_methods() {
let request = GetPermissionPublicRequest::for_doc("doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
let request = GetPermissionPublicRequest::for_sheet("shtcnxxxxxx");
        assert_eq!(request.obj_type, "sheet");
let request = GetPermissionPublicRequest::for_bitable("bblcnxxxxxx");
        assert_eq!(request.obj_type, "bitable");
let request = GetPermissionPublicRequest::for_wiki("wikicnxxxxxx");
        assert_eq!(request.obj_type, "wiki");
#[test]
    fn test_public_settings_methods() {
let settings = PublicSettings {,
            link_share_setting: "tenant_editable".to_string(),
            password_switch: true,
            allow_copy: true,
            allow_comment: true,
            allow_save_copy: false,
            access_setting: None,
            watermark_setting: None};
assert!(settings.is_link_share_enabled());
        assert!(settings.is_tenant_accessible());
assert!(!settings.is_anyone_accessible());
        assert!(settings.is_editable());
assert!(!settings.is_readonly());
        assert!(settings.has_password_protection());
        assert_eq!(settings.share_level_description(), "组织内可编辑");
        assert_eq!(settings.security_level(), "较安全");
