use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::core::{,
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::{cloud_docs::*, EndpointBuilder}
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use super::batch_create::Permission;
/// 判断当前用户是否有某权限请求
#[derive(Debug, Clone)]
pub struct AuthPermissionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token,
#[serde(skip)]
    token: String,
    /// 文档类型,
#[serde(skip)]
    obj_type: String,
    /// 要检查的权限,
#[serde(skip)]
    perm: String}
impl AuthPermissionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}    }
#[derive(Debug, Clone)]
pub struct AuthPermissionRequestBuilder {
    request: AuthPermissionRequest}
impl AuthPermissionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 权限检查结果,
#[derive(Debug, Clone)]
pub struct PermissionAuth {
    /// 是否有该权限
    pub is_permitted: bool,
    /// 检查的权限类型
    pub perm: String,
    /// 用户实际权限（如果有）
    pub actual_perm: Option<String>}
/// 判断当前用户是否有某权限响应,
#[derive(Debug, Clone)]
pub struct AuthPermissionResponse {
    /// 权限检查结果
    pub auth_result: PermissionAuth,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 判断当前用户是否有某权限,
pub async fn auth_permission(
    request: AuthPermissionRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<AuthPermissionResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
api_req.api_path = format!(,
        "{}?type={}&perm={}",
        EndpointBuilder::replace_param(DRIVE_V1_PERMISSIONS_MEMBERS_AUTH, "token", &request.token),
        request.obj_type,
        request.perm,
);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

impl PermissionAuth {
    pub fn new(config: Config) -> Self {
        Self { config }
}    }
/// 权限级别比较,
    pub fn w+.*{
match self.actual_perm.as_deref() {,
            Some("view") => 1,
            Some("comment") => 2,
            Some("edit") => 3,
            Some("full_access") => 4,
            _ => 0}
/// 是否有更高级别的权限,
    pub fn w+.*{
if let Some(actual) = &self.actual_perm {,
            let checked_level = match self.perm.as_str() {
                "view" => 1,
                "comment" => 2,
                "edit" => 3,
                "full_access" => 4,
                _ => 0};
let actual_level = match actual.as_str() {,
                "view" => 1,
                "comment" => 2,
                "edit" => 3,
                "full_access" => 4,
                _ => 0};
actual_level > checked_level,
        } else {,
false}
impl AuthPermissionResponse {
    pub fn new(config: Config) -> Self {
        Self { config }
}        }

        parts.join(", "),
/// 是否可以执行指定操作,
    pub fn w+.*{
if !self.has_permission() {,
            return false;
match action {,
            "read" | "view" => true, // 有任何权限都能查看
            "comment" => self.auth_result.actual_permission_level() >= 2,
            "edit" | "write" => self.auth_result.actual_permission_level() >= 3,
            "manage" | "admin" => self.auth_result.actual_permission_level() >= 4,
            _ => false}
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_auth_permission_request_builder() {
let request = AuthPermissionRequest::builder(),
            .token()
.as_doc()
            .check_edit()
.build();
        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.perm, "edit");
#[test]
    ,
        let request = AuthPermissionRequest::new("doccnxxxxxx", "doc", Permission::Edit);
        assert_eq!(request.perm, "edit");
