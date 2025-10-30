use open_lark_core::core::api_req::ApiRequest;
use crate::core::{,
    api_resp::ApiResponseTrait, config::Config, constants::AccessTokenType,
    http::Transport, trait_system::Service};
use serde::{Deserialize, Serialize};
/// 权限范围服务,
///
/// 用于管理通讯录的访问权限范围，包括：,
/// - 获取通讯录授权范围,
/// - 权限范围变更事件处理,
pub struct ScopeService {
    config: Config}
impl ScopeService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取通讯录授权范围,
    ///,
/// 获取应用在通讯录中的授权范围，包括可访问的部门、用户和用户组列表。,
    /// 用于了解当前应用能够访问的通讯录资源范围。,
///,
    /// # API文档,
///,
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/scope/listscope/listscope/list
    pub async fn list(&self, _req: &GetScopeRequest) -> crate::core::SDKResult<GetScopeResponse> {,
let api_req = ApiRequest {,
            http_method: reqwest::Method::GET,
            api_path: crate::core::endpoints::contact::CONTACT_V3_SCOPES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant]
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()};

        let resp = Transport::<GetScopeResponse>::request(api_req, &self.config, None).await?;
Ok(resp.data.unwrap_or_default()),
    }
/// 获取通讯录授权范围详情,
    ///,
/// 获取通讯录授权范围的详细信息，包括权限范围的具体配置和约束条件。,
    ///,
/// # API文档,
    ///,
/// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/scope/listscope/listscope/get,
    pub async fn get_authority(
        &self,
        req: &GetScopeAuthorityRequest,
    ) -> crate::core::SDKResult<GetScopeAuthorityResponse> {,
let api_req = ApiRequest {,
            http_method: reqwest::Method::GET,
            api_path: crate::core::endpoints::contact::CONTACT_V3_SCOPES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant]
            body: Vec::new(),
            query_params: {,
let mut params = std::collections::HashMap::new();
                if let Some(user_id_type) = &req.user_id_type {
                    params.insert("user_id_type", user_id_type.clone());
if let Some(department_id_type) = &req.department_id_type {,
                    params.insert("department_id_type", department_id_type.clone());
if let Some(page_size) = req.page_size {,
                    params.insert("page_size", page_size.to_string());
if let Some(page_token) = &req.page_token {,
                    params.insert("page_token", page_token.clone());
params,
            }
            ..Default::default(),
};
let resp =,
            Transport::<GetScopeAuthorityResponse>::request(api_req, &self.config, None).await?;
Ok(resp.data.unwrap_or_default()),
    }
/// 更新通讯录授权范围,
    ///,
/// 更新应用的通讯录授权范围，包括可访问的部门、用户和用户组。,
    ///,
/// # API文档,
    ///,
/// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/scope/listscope/listscope/update,
    pub async fn update_authority(
        &self,
        req: &UpdateScopeAuthorityRequest,
    ) -> crate::core::SDKResult<UpdateScopeAuthorityResponse> {,
let api_req = ApiRequest {,
            http_method: reqwest::Method::PUT,
            api_path: crate::core::endpoints::contact::CONTACT_V3_SCOPES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant]
            body: serde_json::to_vec(&req).unwrap_or_default(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()};
let resp =,
            Transport::<UpdateScopeAuthorityResponse>::request(api_req, &self.config, None).await?;
Ok(resp.data.unwrap_or_default()),
    }
impl Service for ScopeService {,
    fn config(&self) -> &Config {,
&self.config}
fn service_name() -> &'static str {,
        "scope"}
fn service_version() -> &'static str {,
        "v3"}
/// 获取权限范围请求,
#[derive(Debug, Clone)]
pub struct GetScopeRequest {
    /// 用户 ID 类型,
#[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型  ,
#[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 分页大小,
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记,
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>}
/// 获取权限范围响应,
#[derive(Debug, Clone)]
pub struct GetScopeResponse {
    /// 权限范围列表
    pub scopes: Vec<Scope>,
    /// 是否还有更多项目,
#[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记,
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> crate::core::api_resp::ResponseFormat {,
crate::core::api_resp::ResponseFormat::Data
    }
/// 权限范围,
#[derive(Debug, Clone)]
pub struct Scope {
    /// 部门列表,
#[serde(skip_serializing_if = "Option::is_none")]
    pub departments: Option<Vec<ScopeDepartment>>,
    /// 用户列表,
#[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<ScopeUser>>,
    /// 用户组列表,
#[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ScopeGroup>>}
/// 权限范围内的部门,
#[derive(Debug, Clone)]
pub struct ScopeDepartment {
    /// 部门ID,
#[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 部门名称,
#[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>}
/// 权限范围内的用户,
#[derive(Debug, Clone)]
pub struct ScopeUser {
    /// 用户ID,
#[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名称,
#[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>}
/// 权限范围内的用户组,
#[derive(Debug, Clone)]
pub struct ScopeGroup {
    /// 用户组ID,
#[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 用户组名称,
#[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>}
/// 获取权限范围详情请求,
#[derive(Debug, Clone)]
pub struct GetScopeAuthorityRequest {
    /// 用户 ID 类型,
#[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型,
#[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 分页大小,
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记,
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>}
/// 获取权限范围详情响应,
#[derive(Debug, Clone)]
pub struct GetScopeAuthorityResponse {
    /// 权限范围详情
    pub scope_authority: ScopeAuthority,
/// 权限范围详情,
#[derive(Debug, Clone)]
pub struct ScopeAuthority {
    /// 应用ID,
#[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// 应用名称,
#[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// 权限范围类型,
#[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// 权限范围详情,
#[serde(skip_serializing_if = "Option::is_none")]
    pub scope_details: Option<ScopeDetails>,
    /// 创建时间,
#[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间,
#[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>}
/// 权限范围详情,
#[derive(Debug, Clone)]
pub struct ScopeDetails {
    /// 部门权限范围,
#[serde(skip_serializing_if = "Option::is_none")]
    pub department_scope: Option<DepartmentScope>,
    /// 用户权限范围,
#[serde(skip_serializing_if = "Option::is_none")]
    pub user_scope: Option<UserScope>,
    /// 用户组权限范围,
#[serde(skip_serializing_if = "Option::is_none")]
    pub group_scope: Option<GroupScope>}
/// 部门权限范围,
#[derive(Debug, Clone)]
pub struct DepartmentScope {
    /// 是否包含所有部门,
#[serde(skip_serializing_if = "Option::is_none")]
    pub include_all: Option<bool>,
    /// 可访问的部门列表,
#[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 可访问的部门路径列表,
#[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_paths: Option<Vec<String>>}
/// 用户权限范围,
#[derive(Debug, Clone)]
pub struct UserScope {
    /// 是否包含所有用户,
#[serde(skip_serializing_if = "Option::is_none")]
    pub include_all: Option<bool>,
    /// 可访问的用户列表,
#[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 排除的用户列表,
#[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_user_ids: Option<Vec<String>>}
/// 用户组权限范围,
#[derive(Debug, Clone)]
pub struct GroupScope {
    /// 是否包含所有用户组,
#[serde(skip_serializing_if = "Option::is_none")]
    pub include_all: Option<bool>,
    /// 可访问的用户组列表,
#[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// 排除的用户组列表,
#[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_group_ids: Option<Vec<String>>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> crate::core::api_resp::ResponseFormat {,
crate::core::api_resp::ResponseFormat::Data
    }
/// 更新权限范围请求,
#[derive(Debug, Clone)]
pub struct UpdateScopeAuthorityRequest {
    /// 权限范围详情
    pub scope_details: Option<ScopeDetails>}
/// 更新权限范围响应,
#[derive(Debug, Clone)]
pub struct UpdateScopeAuthorityResponse {
    /// 更新结果
    pub result: Option<String>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> crate::core::api_resp::ResponseFormat {,
crate::core::api_resp::ResponseFormat::Data
    }
impl GetScopeAuthorityRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取权限范围详情的构建器,
#[derive(Default)]
pub struct GetScopeAuthorityRequestBuilder {
    request: GetScopeAuthorityRequest}
impl GetScopeAuthorityRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 更新权限范围的构建器,
#[derive(Default)]
pub struct UpdateScopeAuthorityRequestBuilder {
    request: UpdateScopeAuthorityRequest}
impl UpdateScopeAuthorityRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}