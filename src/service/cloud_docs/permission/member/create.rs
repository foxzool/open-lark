use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::core::{,
use crate::core::SDKResult;    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::{cloud_docs::*, EndpointBuilder}
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use super::batch_create::Permission;
/// 增加协作者权限请求
#[derive(Debug, Clone)]
pub struct CreatePermissionMemberRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token,
#[serde(skip)]
    token: String,
    /// 文档类型,
#[serde(skip)]
    obj_type: String,
    /// 协作者ID类型
    member_type: String,
    /// 协作者ID
    member_id: String,
    /// 权限
    perm: Permission,
    /// 是否通知,
#[serde(skip_serializing_if = "Option::is_none")]
    need_notification: Option<bool>}
impl CreatePermissionMemberRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct CreatePermissionMemberRequestBuilder {
    request: CreatePermissionMemberRequest}
impl CreatePermissionMemberRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}crate::impl_executable_builder_owned!(,
    CreatePermissionMemberRequestBuilder,
    crate::service::cloud_docs::permission::PermissionService,
    CreatePermissionMemberRequest,
    BaseResponse<CreatePermissionMemberResponse>,
    create_member,
);
/// 协作者创建结果
#[derive(Debug, Clone)]
pub struct PermissionMemberCreated {
    /// 协作者ID类型
    pub member_type: String,
    /// 协作者ID
    pub member_id: String,
    /// 权限
    pub perm: Permission,
    /// 创建时间（毫秒时间戳）
    pub create_time: Option<i64>,
    /// 是否通知了用户
    pub notified: Option<bool>}
/// 增加协作者权限响应,
#[derive(Debug, Clone)]
pub struct CreatePermissionMemberResponse {
    /// 协作者信息
    pub member: PermissionMemberCreated,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 增加协作者权限,
pub async fn create_permission_member(
    request: CreatePermissionMemberRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<CreatePermissionMemberResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.api_path = format!(,
        "{}?type={}",
        EndpointBuilder::replace_param(DRIVE_V1_PERMISSIONS_MEMBERS, "token", &request.token),
        request.obj_type,
);
    // 添加通知参数,
if let Some(need_notification) = request.need_notification {,
        api_req.api_path = format!(
            "{}&need_notification={}",
            api_req.api_path, need_notification,
);
    }

    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

impl PermissionMemberCreated {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取成员类型描述,
    pub fn w+.*{
match self.member_type.as_str() {,
            "user" => "用户".to_string(),
            "chat" => "群组".to_string(),
            "department" => "部门".to_string(),
            _ => "未知".to_string()}
/// 获取权限描述,
    pub fn w+.*{
self.perm.description().to_string()}
/// 获取摘要信息,
    pub fn w+.*{
let mut parts = vec![,
            format!("{} ({})", self.member_id, self.member_type_description()),
            format!("权限: {}", self.permission_description()),
        ];
if let Some(time) = self.create_time_formatted() {,
            parts.push(time);
if self.was_notified() {,
            parts.push("已通知".to_string());

        parts.join(", "),
impl CreatePermissionMemberResponse {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 是否通知了用户,
    pub fn w+.*{
self.member.was_notified()}
/// 权限级别,
    pub fn w+.*{
self.member.perm.level()}
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_create_permission_member_request_builder() {
let request = CreatePermissionMemberRequest::builder(),
            .token()
.as_doc()
            .user()
.as_editor()
            .with_notification()
.build();
        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.member_type, "user");
        assert_eq!(request.member_id, "user123");
        assert!(matches!(request.perm, Permission::Edit));
        assert_eq!(request.need_notification, Some(true));
#[test]
    fn test_create_permission_member_convenience_methods() {
let request = CreatePermissionMemberRequest::for_user(,
            "doccnxxxxxx",
            "doc",
            "user123",
            Permission::Edit,
        );
        assert_eq!(request.member_type, "user");
        assert_eq!(request.member_id, "user123");
let request = CreatePermissionMemberRequest::for_chat(,
            "doccnxxxxxx",
            "doc",
            "chat456",
            Permission::View,
        );
        assert_eq!(request.member_type, "chat");
        assert_eq!(request.member_id, "chat456");
let request = CreatePermissionMemberRequest::for_department(,
            "doccnxxxxxx",
            "doc",
            "dept789",
            Permission::Comment,
        );
        assert_eq!(request.member_type, "department");
        assert_eq!(request.member_id, "dept789");
#[test]
    fn test_permission_member_created_methods() {
let member = PermissionMemberCreated {,
            member_type: "user".to_string(),
            member_id: "user123".to_string(),
            perm: Permission::Edit,
            create_time: Some(1234567890),
            notified: Some(true)};
assert!(member.is_user());
        assert!(!member.is_chat());
assert!(!member.is_department());
        assert!(member.can_edit());
assert!(!member.is_owner());
        assert!(member.was_notified());
assert!(member.has_create_time());
        assert_eq!(member.member_type_description(), "用户");
        assert_eq!(member.permission_description(), "编辑者");
