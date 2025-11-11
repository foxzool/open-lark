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
/// 协作者权限,
#[derive(Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum Permission {
/// 所有者,
    FullAccess,
    /// 编辑者
    Edit,
    /// 阅读者,
#[default]
    View,
    /// 评论者
    Comment}
/// 协作者信息,
#[derive(Debug, Clone)]
pub struct Collaborator {
    /// 协作者ID类型
    pub member_type: String,
    /// 协作者ID
    pub member_id: String,
    /// 权限
    pub perm: Permission,
/// 批量增加协作者权限请求,
#[derive(Debug, Clone)]
pub struct BatchCreatePermissionMemberRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token,
#[serde(skip)]
    token: String,
    /// 文档类型,
#[serde(skip)]
    obj_type: String,
    /// 协作者列表
    members: Vec<Collaborator>,
    /// 是否通知,
#[serde(skip_serializing_if = "Option::is_none")]
    need_notification: Option<bool>}
impl BatchCreatePermissionMemberRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct BatchCreatePermissionMemberRequestBuilder {
    request: BatchCreatePermissionMemberRequest}
impl BatchCreatePermissionMemberRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}crate::impl_executable_builder_owned!(,
    BatchCreatePermissionMemberRequestBuilder,
    crate::service::cloud_docs::permission::PermissionService,
    BatchCreatePermissionMemberRequest,
    BaseResponse<BatchCreatePermissionMemberResponse>,
    batch_create_member,
);
/// 成员操作结果
#[derive(Debug, Clone)]
pub struct MemberResult {
    /// 协作者ID类型
    pub member_type: String,
    /// 协作者ID
    pub member_id: String,
    /// 权限
    pub perm: Permission,
    /// 操作结果
    pub result: String,
    /// 错误码（如果有）
    pub code: Option<i32>,
    /// 错误信息（如果有）
    pub msg: Option<String>}
/// 批量增加协作者权限响应,
#[derive(Debug, Clone)]
pub struct BatchCreatePermissionMemberResponse {
    /// 操作结果列表
    pub members: Vec<MemberResult>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 批量增加协作者权限,
pub async fn batch_create_permission_member(
    request: BatchCreatePermissionMemberRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<BatchCreatePermissionMemberResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.api_path = format!(,
        "{}?type={}",
        EndpointBuilder::replace_param(
            DRIVE_V1_PERMISSIONS_MEMBERS_BATCH_CREATE,
            "token",
            &request.token
        ),
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

impl Permission {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 是否有编辑权限,
    pub fn can_edit(&self) -> bool {
        matches!(self, Permission::Edit | Permission::FullAccess)}
/// 是否有评论权限,
    pub fn can_comment(&self) -> bool {
        !matches!(self, Permission::View)}
/// 是否是所有者,
    pub fn is_owner(&self) -> bool {
        matches!(self, Permission::FullAccess)}
/// 权限描述,
    pub fn w+.*{
match self {,
            Permission::FullAccess => "所有者",
            Permission::Edit => "编辑者",
            Permission::Comment => "评论者",
            Permission::View => "阅读者"}
impl MemberResult {
    pub fn new(config: Config) -> Self {
        Self { config }
}    }
impl BatchCreatePermissionMemberResponse {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_batch_create_permission_member_request_builder() {
let request = BatchCreatePermissionMemberRequest::builder(),
            .token()
.as_doc()
            .add_user("user123", Permission::Edit)
            .add_chat()
.with_notification()
            .build();

        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.members.len(), 2);
        assert_eq!(request.need_notification, Some(true));
#[test]
    fn test_permission_methods() {
assert!(Permission::Edit.can_edit());
        assert!(Permission::FullAccess.can_edit());
assert!(!Permission::View.can_edit());
        assert!(Permission::Edit.can_comment());
assert!(!Permission::View.can_comment());
        assert!(Permission::FullAccess.is_owner());
assert!(!Permission::Edit.is_owner());
        assert_eq!(Permission::FullAccess.level(), 4);
        assert_eq!(Permission::View.level(), 1);
