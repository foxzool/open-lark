use reqwest::Method;
use openlark_core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::{,
use SDKResult;    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::{cloud_docs::*, EndpointBuilder}
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use super::batch_create::Permission;
/// 更新协作者权限请求
#[derive(Debug, Clone)]
pub struct UpdatePermissionMemberRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token,
#[serde(skip)]
    token: String,
    /// 文档类型,
#[serde(skip)]
    obj_type: String,
    /// 协作者ID类型,
#[serde(skip)]
    member_type: String,
    /// 协作者ID,
#[serde(skip)]
    member_id: String,
    /// 新权限
    perm: Permission,
    /// 是否通知,
#[serde(skip_serializing_if = "Option::is_none")]
    need_notification: Option<bool>}
impl UpdatePermissionMemberRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct UpdatePermissionMemberRequestBuilder {
    request: UpdatePermissionMemberRequest}
impl UpdatePermissionMemberRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 协作者更新结果,
#[derive(Debug, Clone)]
pub struct PermissionMemberUpdated {
    /// 协作者ID类型
    pub member_type: String,
    /// 协作者ID
    pub member_id: String,
    /// 新权限
    pub perm: Permission,
    /// 更新时间（毫秒时间戳）
    pub update_time: Option<i64>,
    /// 原权限（如果有）
    pub old_perm: Option<Permission>,
    /// 是否通知了用户
    pub notified: Option<bool>}
/// 更新协作者权限响应,
#[derive(Debug, Clone)]
pub struct UpdatePermissionMemberResponse {
    /// 协作者信息
    pub member: PermissionMemberUpdated,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 更新协作者权限,
pub async fn update_permission_member(
    request: UpdatePermissionMemberRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<UpdatePermissionMemberResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::PUT);
api_req.api_path = format!(,
        "{}?type={}&member_type={}",
        EndpointBuilder::replace_params_from_array(
            DRIVE_V1_PERMISSIONS_MEMBER_GET,
            &[("token", &request.token), ("member_id", &request.member_id)]
        ),
        request.obj_type,
        request.member_type,
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

impl PermissionMemberUpdated {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 权限是否升级,
    pub fn w+.*{
if let Some(old_perm) = &self.old_perm {,
            self.perm.level() > old_perm.level()} else {,
false}
/// 权限是否降级,
    pub fn w+.*{
if let Some(old_perm) = &self.old_perm {,
            self.perm.level() < old_perm.level()} else {,
false}
/// 获取更新时间的格式化字符串,
    pub fn w+.*{
self.update_time,
            .map(|timestamp| format!("更新时间: {timestamp}")),
/// 获取成员类型描述,
    pub fn w+.*{
match self.member_type.as_str() {,
            "user" => "用户".to_string(),
            "chat" => "群组".to_string(),
            "department" => "部门".to_string(),
            _ => "未知".to_string()}
/// 获取权限变化描述,
    pub fn w+.*{
if let Some(old_perm) = &self.old_perm {,
            format!("{} → {}", old_perm.description(), self.perm.description()),
} else {
            format!("权限: {}", self.perm.description()),
    }
/// 获取摘要信息,
    pub fn w+.*{
let mut parts = vec![,
            format!("{} ({})", self.member_id, self.member_type_description()),
            self.permission_change_description(),
        ];
if let Some(time) = self.update_time_formatted() {,
            parts.push(time);
if self.was_notified() {,
            parts.push("已通知".to_string());

        parts.join(", "),
impl UpdatePermissionMemberResponse {
    pub fn new(config: Config) -> Self {
        Self { config }
}    }
/// 是否通知了用户,
    pub fn w+.*{
self.member.was_notified()}
/// 权限级别,
    pub fn w+.*{
self.member.perm.level()}
/// 权限是否升级,
    pub fn w+.*{
self.member.permission_upgraded()}
/// 权限是否降级,
    pub fn w+.*{
self.member.permission_downgraded()}
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_update_permission_member_request_builder() {
let request = UpdatePermissionMemberRequest::builder(),
            .token()
.as_doc()
            .user()
.to_editor()
            .with_notification()
.build();
        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.member_type, "user");
        assert_eq!(request.member_id, "user123");
        assert!(matches!(request.perm, Permission::Edit));
        assert_eq!(request.need_notification, Some(true));
#[test]
    fn test_update_permission_member_convenience_methods() {
let request = UpdatePermissionMemberRequest::for_user(,
            "doccnxxxxxx",
            "doc",
            "user123",
            Permission::Edit,
        );
        assert_eq!(request.member_type, "user");
        assert_eq!(request.member_id, "user123");
let request = UpdatePermissionMemberRequest::for_chat(,
            "doccnxxxxxx",
            "doc",
            "chat456",
            Permission::View,
        );
        assert_eq!(request.member_type, "chat");
        assert_eq!(request.member_id, "chat456");
let request = UpdatePermissionMemberRequest::for_department(,
            "doccnxxxxxx",
            "doc",
            "dept789",
            Permission::Comment,
        );
        assert_eq!(request.member_type, "department");
        assert_eq!(request.member_id, "dept789");
#[test]
    fn test_permission_member_updated_methods() {
let member = PermissionMemberUpdated {,
            member_type: "user".to_string(),
            member_id: "user123".to_string(),
            perm: Permission::Edit,
            update_time: Some(1234567890),
            old_perm: Some(Permission::View),
            notified: Some(true)};
assert!(member.is_user());
        assert!(!member.is_chat());
assert!(!member.is_department());
        assert!(member.can_edit());
assert!(!member.is_owner());
        assert!(member.was_notified());
assert!(member.has_update_time());
        assert!(member.has_old_permission());
assert!(member.permission_changed());
        assert!(member.permission_upgraded());
assert!(!member.permission_downgraded());
        assert_eq!(member.member_type_description(), "用户");
        assert_eq!(member.permission_change_description(), "阅读者 → 编辑者");
