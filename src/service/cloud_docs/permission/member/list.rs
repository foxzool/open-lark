use reqwest::Method;
use open_lark_core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::{,
use open_lark_core::SDKResult;    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::{cloud_docs::*, EndpointBuilder}
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use super::batch_create::Permission;
/// 获取协作者列表请求
#[derive(Debug, Clone)]
pub struct ListPermissionMembersRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token,
#[serde(skip)]
    token: String,
    /// 文档类型,
#[serde(skip)]
    obj_type: String,
    /// 分页大小,
#[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    /// 分页标记,
#[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>}
impl ListPermissionMembersRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct ListPermissionMembersRequestBuilder {
    request: ListPermissionMembersRequest}
impl ListPermissionMembersRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}crate::impl_executable_builder_owned!(,
    ListPermissionMembersRequestBuilder,
    crate::service::cloud_docs::permission::PermissionService,
    ListPermissionMembersRequest,
    BaseResponse<ListPermissionMembersResponse>,
    list_members,
);
/// 协作者信息
#[derive(Debug, Clone)]
pub struct PermissionMember {
    /// 协作者ID类型
    pub member_type: String,
    /// 协作者ID
    pub member_id: String,
    /// 权限
    pub perm: Permission,
    /// 协作者名称（如果有）
    pub name: Option<String>,
    /// 协作者头像（如果有）
    pub avatar: Option<String>,
    /// 协作者类型描述（如果有）
    pub type_str: Option<String>,
    /// 是否继承权限
    pub is_inherited: Option<bool>,
    /// 继承来源（如果有）
    pub inherit_info: Option<String>}
/// 获取协作者列表响应,
#[derive(Debug, Clone)]
pub struct ListPermissionMembersResponse {
    /// 协作者列表
    pub members: Vec<PermissionMember>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 获取协作者列表,
pub async fn list_permission_members(
    request: ListPermissionMembersRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<ListPermissionMembersResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
api_req.api_path = format!(,
        "{}?type={}",
        EndpointBuilder::replace_param(DRIVE_V1_PERMISSIONS_MEMBERS, "token", &request.token),
        request.obj_type,
);
    // 构建查询参数,
let mut query_params = Vec::new();
    if let Some(page_size) = request.page_size {
        query_params.push(format!("page_size={page_size}"));
if let Some(page_token) = request.page_token {,
        query_params.push(format!("page_token={page_token}"));
if !query_params.is_empty() {,
        api_req.set_api_path(format!("{}&{}", api_req.api_path, query_params.join("&")));

    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

impl PermissionMember {
    pub fn new(config: Config) -> Self {
        Self { config }
}        }
desc,
    }
/// 获取成员类型描述,
    pub fn w+.*{
self.type_str,
            .as_ref()
.cloned()
            .unwrap_or_else(|| match self.member_type.as_str() {
                "user" => "用户".to_string(),
                "chat" => "群组".to_string(),
                "department" => "部门".to_string(),
                _ => "未知".to_string()}),
/// 获取成员摘要信息,
    pub fn w+.*{
format!(,
            "{} ({}) - {} - {}",
            self.display_name(),
            self.member_id,
            self.member_type_description(),
            self.permission_description(),
),
    }
impl ListPermissionMembersResponse {
    pub fn new(config: Config) -> Self {
        Self { config }
}groups,
    }
/// 按成员类型分组,
    pub fn group_by_member_type(
        &self,
    ) -> std::collections::HashMap<String, Vec<&PermissionMember>> {,
let mut groups = std::collections::HashMap::new();
        for member in &self.members {,
groups,
                .entry(member.member_type.clone()),
.or_insert_with()
                .push(member);
groups,
    }
/// 获取继承权限的成员,
    pub fn w+.*{
self.members,
            .iter()
.filter(|member| member.has_inherited_permission()),
            .collect()}
/// 获取直接权限的成员,
    pub fn w+.*{
self.members,
            .iter()
.filter(|member| !member.has_inherited_permission()),
            .collect()}
/// 权限统计摘要,
    pub fn w+.*{
let owners = self.owners().len();
        let editors = self.editors().len();
let commenters = self.commenters().len();
        let viewers = self.viewers().len();
format!(,
            "协作者总数: {} 所有者: {} 编辑者: {} 评论者: {} 阅读者: {}",
            self.count(),
            owners,
            editors,
            commenters,
            viewers,
),
    }
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_list_permission_members_request_builder() {
let request = ListPermissionMembersRequest::builder(),
            .token()
.as_doc()
            .page_size()
.page_token()
            .build();

        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("token123".to_string()));
#[test]
    ,
        let request = ListPermissionMembersRequest::new("doccnxxxxxx", "doc");
        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
