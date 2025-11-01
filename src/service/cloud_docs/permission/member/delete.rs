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
/// 移除协作者权限请求
#[derive(Debug, Clone)]
pub struct DeletePermissionMemberRequest {
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
    /// 是否通知,
#[serde(skip_serializing_if = "Option::is_none")]
    need_notification: Option<bool>}
impl DeletePermissionMemberRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct DeletePermissionMemberRequestBuilder {
    request: DeletePermissionMemberRequest}
impl DeletePermissionMemberRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 协作者删除结果,
#[derive(Debug, Clone)]
pub struct PermissionMemberDeleted {
    /// 协作者ID类型
    pub member_type: String,
    /// 协作者ID
    pub member_id: String,
    /// 删除时间（毫秒时间戳）
    pub delete_time: Option<i64>,
    /// 删除前的权限（如果有）
    pub old_perm: Option<Permission>,
    /// 是否通知了用户
    pub notified: Option<bool>}
/// 移除协作者权限响应,
#[derive(Debug, Clone)]
pub struct DeletePermissionMemberResponse {
    /// 协作者信息
    pub member: PermissionMemberDeleted,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 移除协作者权限,
pub async fn delete_permission_member(
    request: DeletePermissionMemberRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<DeletePermissionMemberResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::DELETE);
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

impl PermissionMemberDeleted {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 原权限是否有编辑权限,
    pub fn w+.*{
if let Some(old_perm) = &self.old_perm {,
            old_perm.can_edit()} else {,
false}
/// 获取删除时间的格式化字符串,
    pub fn w+.*{
self.delete_time,
            .map(|timestamp| format!("删除时间: {timestamp}")),
/// 获取成员类型描述,
    pub fn w+.*{
match self.member_type.as_str() {,
            "user" => "用户".to_string(),
            "chat" => "群组".to_string(),
            "department" => "部门".to_string(),
            _ => "未知".to_string()}
/// 获取原权限描述,
    pub fn w+.*{
self.old_perm,
            .as_ref()
.map(|perm| perm.description().to_string())}
/// 获取摘要信息,
    pub fn w+.*{
let mut parts = vec![format!(,
            "{} ({})",
            self.member_id,
            self.member_type_description(),
)];
        if let Some(old_perm_desc) = self.old_permission_description() {
            parts.push(format!("原权限: {old_perm_desc}"));
if let Some(time) = self.delete_time_formatted() {,
            parts.push(time);
if self.was_notified() {,
            parts.push("已通知".to_string());

        parts.join(", "),
impl DeletePermissionMemberResponse {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 是否通知了用户,
    pub fn w+.*{
self.member.was_notified()}
/// 删除的是否是所有者,
    pub fn w+.*{
self.member.was_owner()}
/// 删除的是否有编辑权限,
    pub fn w+.*{
self.member.could_edit()}
/// 获取删除时间,
    pub fn w+.*{
self.member.delete_time}
/// 操作风险级别（基于删除的权限）,
    pub fn w+.*{
if self.deleted_owner() {,
            "高风险" // 删除所有者} else if self.deleted_editor() {,
"中风险" // 删除编辑者} else {,
"低风险" // 删除查看者或评论者}
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_delete_permission_member_request_builder() {
let request = DeletePermissionMemberRequest::builder(),
            .token()
.as_doc()
            .user()
.with_notification()
            .build();

        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.member_type, "user");
        assert_eq!(request.member_id, "user123");
        assert_eq!(request.need_notification, Some(true));
#[test]
    ,
        let request = DeletePermissionMemberRequest::for_user("doccnxxxxxx", "doc", "user123");
        assert_eq!(request.member_type, "user");
        assert_eq!(request.member_id, "user123");

        let request = DeletePermissionMemberRequest::for_chat("doccnxxxxxx", "doc", "chat456");
        assert_eq!(request.member_type, "chat");
        assert_eq!(request.member_id, "chat456");
let request =,
            DeletePermissionMemberRequest::for_department("doccnxxxxxx", "doc", "dept789");
        assert_eq!(request.member_type, "department");
        assert_eq!(request.member_id, "dept789");
#[test]
    fn test_permission_member_deleted_methods() {
let member = PermissionMemberDeleted {,
            member_type: "user".to_string(),
            member_id: "user123".to_string(),
            delete_time: Some(1234567890),
            old_perm: Some(Permission::Edit),
            notified: Some(true)};
assert!(member.is_user());
        assert!(!member.is_chat());
assert!(!member.is_department());
        assert!(member.was_notified());
assert!(member.has_delete_time());
        assert!(member.has_old_permission());
assert!(!member.was_owner());
        assert!(member.could_edit());
        assert_eq!(member.member_type_description(), "用户");
assert_eq!(,
            member.old_permission_description(),
            Some("编辑者".to_string()),
);
    }
#[test]
    fn test_delete_permission_member_response_risk_level() {
let response_owner = DeletePermissionMemberResponse {,
            member: PermissionMemberDeleted {
                member_type: "user".to_string(),
                member_id: "user123".to_string(),
                delete_time: Some(1234567890),
                old_perm: Some(Permission::FullAccess),
                notified: Some(true)}
        };
        assert_eq!(response_owner.risk_level(), "高风险");
let response_editor = DeletePermissionMemberResponse {,
            member: PermissionMemberDeleted {
                member_type: "user".to_string(),
                member_id: "user123".to_string(),
                delete_time: Some(1234567890),
                old_perm: Some(Permission::Edit),
                notified: Some(true)}
        };
        assert_eq!(response_editor.risk_level(), "中风险");
let response_viewer = DeletePermissionMemberResponse {,
            member: PermissionMemberDeleted {
                member_type: "user".to_string(),
                member_id: "user123".to_string(),
                delete_time: Some(1234567890),
                old_perm: Some(Permission::View),
                notified: Some(true)}
        };
        assert_eq!(response_viewer.risk_level(), "低风险");
