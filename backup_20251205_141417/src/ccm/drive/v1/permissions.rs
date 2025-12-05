
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use crate::,
{
    api::,
{,
        BaseResponse,
        ResponseFormat, HttpMethod,
        api::{ApiResponseTrait}
    config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    impl_executable_builder_owned,
};
pub struct PermissionsService {
    config: Config}
impl PermissionsService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取云文档权限设置,
    pub async fn get(
        &self,
        request: GetPermissionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<GetPermissionResponse>> {,
let mut api_req = request.api_request;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(DRIVE_V2_PERMISSIONS_PUBLIC.replace("{}", &request.token));
api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
/// 更新云文档权限设置,
    pub async fn patch(
        &self,
        request: PatchPermissionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<GetPermissionResponse>> {,
let mut api_req = request.api_request;
        api_req.set_http_method(Method::PATCH);
        api_req.set_api_path(DRIVE_V2_PERMISSIONS_PUBLIC.replace("{}", &request.token));
api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
/// 获取云文档权限设置,
pub struct GetPermissionRequest {
    api_request: ApiRequest,
    /// 文件的 token
    token: String,
    /// 文件类型，需要与文件的 token 相匹配,
///,
    /// 示例值："doc",
///,
    /// 可选值有：,
///,
    /// - doc：旧版文档,
/// - sheet：电子表格,
    /// - file：云空间文件,
/// - wiki：知识库节点,
    /// - bitable：多维表格,
/// - docx：新版文档,
    /// - mindnote：思维笔记,
/// - minutes：妙记,
    /// - slides：幻灯片
    r#type: String}
impl GetPermissionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
pub struct GetPermissionRequestBuilder {
    request: GetPermissionRequest}
impl GetPermissionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 返回的文档公共设置,
pub struct GetPermissionResponse {
    /// 返回的文档公共设置
    pub permission_public: PermissionPublic,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> api::ResponseFormat {,
api::ResponseFormat::Data
    }

/// 返回的文档公共设置,
pub struct PermissionPublic {
/// 允许内容被分享到组织外,
    ///,
/// 可选值有：,
    ///,
/// - open：打开,
    /// - closed：关闭,
/// - allow_share_partner_tenant：允许分享给关联组织,
    pub external_access_entity: Option<String>,
    /// 谁可以创建副本、打印、下载,
///,
    /// 可选值有：,
///,
    /// - anyone_can_view：拥有可阅读权限的用户,
/// - anyone_can_edit：拥有可编辑权限的用户,
    /// - only_full_access：拥有可管理权限（包括我）的用户
    pub security_entity: Option<String>,
    /// 谁可以评论,
///,
    /// 可选值有：,
///,
    /// - anyone_can_view：拥有可阅读权限的用户,
/// - anyone_can_edit：拥有可编辑权限的用户,
    pub comment_entity: Option<String>,
    /// 谁可以添加和管理协作者-组织维度,
///,
    /// 可选值有：,
///,
    /// - anyone：所有可阅读或编辑此文档的用户,
/// - same_tenant：组织内所有可阅读或编辑此文档的用户,
    pub share_entity: Option<String>,
    /// 谁可以添加和管理协作者-协作者维度,
///,
    /// 可选值有：,
///,
    /// - collaborator_can_view：拥有可阅读权限的协作者,
/// - collaborator_can_edit：拥有可编辑权限的协作者,
    /// - collaborator_full_access：拥有可管理权限（包括我）的协作者
    pub manage_collaborator_entity: Option<String>,
    /// 链接分享设置,
///,
    /// 可选值有：,
///,
    /// - tenant_readable：组织内获得链接的人可阅读,
/// - tenant_editable：组织内获得链接的人可编辑,
    /// - partner_tenant_readable：关联组织的人可阅读,
/// - partner_tenant_editable：关联组织的人可编辑,
    /// - anyone_readable：互联网上获得链接的任何人可阅读（仅external_access=“open”时有效）,
/// - anyone_editable：互联网上获得链接的任何人可编辑（仅external_access=“open”时有效）,
    /// - closed：关闭链接分享
    pub link_share_entity: Option<String>,
    /// 谁可以复制内容,
///,
    /// 可选值有：,
///,
    /// - anyone_can_view：拥有可阅读权限的用户,
/// - anyone_can_edit：拥有可编辑权限的用户,
    /// - only_full_access：拥有可管理权限（包括我）的协作者
    pub copy_entity: Option<String>,
    /// 节点是否已加锁，加锁之后不再继承父级页面的权限
    pub lock_switch: Option<bool>}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchPermissionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文件的 token,
#[serde(skip)]
    token: String,
    /// 允许内容被分享到组织外,
///,
    /// 示例值："open",
///,
    /// 可选值有：,
///,
    /// - open：打开,
/// - closed：关闭,
    /// - allow_share_partner_tenant：允许分享给关联组织（只有租户后台设置仅允许关联组织分享，,
///   才能设置为该值）,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_access_entity: Option<String>,
    /// 谁可以创建副本、打印、下载,
///,
    /// 示例值："anyone_can_view",
///,
    /// 可选值有：,
///,
    /// - anyone_can_view：拥有可阅读权限的用户,
/// - anyone_can_edit：拥有可编辑权限的用户,
    /// - only_full_access：拥有可管理权限（包括我）的用户,
#[serde(skip_serializing_if = "Option::is_none")]
    security_entity: Option<String>,
    /// 谁可以评论,
///,
    /// 示例值："anyone_can_view",
///,
    /// 可选值有：,
///,
    /// - anyone_can_view：拥有可阅读权限的用户,
/// - anyone_can_edit：拥有可编辑权限的用户,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment_entity: Option<String>,
    /// 谁可以添加和管理协作者-组织维度,
///,
    /// 示例值："anyone",
///,
    /// 可选值有：,
///,
    /// - anyone：所有可阅读或编辑此文档的用户,
/// - same_tenant：组织内所有可阅读或编辑此文档的用户,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_entity: Option<String>,
    /// 谁可以添加和管理协作者-协作者维度,
///,
    /// 示例值："collaborator_can_view",
///,
    /// 可选值有：,
///,
    /// - collaborator_can_view：拥有可阅读权限的协作者,
/// - collaborator_can_edit：拥有可编辑权限的协作者,
    /// - collaborator_full_access：拥有可管理权限（包括我）的协作者,
#[serde(skip_serializing_if = "Option::is_none")]
    manage_collaborator_entity: Option<String>,
    /// 链接分享设置,
///,
    /// 示例值："tenant_readable",
///,
    /// 可选值有：,
///,
    /// tenant_readable：组织内获得链接的人可阅读,
/// tenant_editable：组织内获得链接的人可编辑,
    /// partner_tenant_readable：关联组织的人可阅读（只有租户后台设置仅允许关联组织分享，,
/// 才能设置为该值） partner_tenant_editable：,
    /// 关联组织的人可编辑（只有租户后台设置仅允许关联组织分享，才能设置为该值）,
/// anyone_readable：互联网上获得链接的任何人可阅读（仅external_access_entity=“open”时有效）,
    /// anyone_editable：互联网上获得链接的任何人可编辑（仅external_access_entity=“open”时有效）,
/// closed：关闭链接分享,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_share_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_entity: Option<String>}
impl PatchPermissionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
pub struct PatchPermissionRequestBuilder {
    request: PatchPermissionRequest}
impl PatchPermissionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    GetPermissionRequestBuilder,
    PermissionsService,
    GetPermissionRequest,
    Response<GetPermissionResponse>,
    get,
);
impl_executable_builder_owned!(
    PatchPermissionRequestBuilder,
    PermissionsService,
    PatchPermissionRequest,
    Response<GetPermissionResponse>,
    patch,
);
#[cfg(test)]
mod tests {
use super::*;
    use api::ResponseFormat;
use SDKResult;use rstest::rstest;
    // === Helper Functions ===,
fn create_test_config() -> Config {,
        openlark_core::config::Config::builder()
.app_id()
            .app_secret()
}
fn create_test_permission_public() -> PermissionPublic {,
        PermissionPublic {
            external_access_entity: Some("open".to_string()),
            security_entity: Some("anyone_can_view".to_string()),
            comment_entity: Some("anyone_can_view".to_string()),
            share_entity: Some("anyone".to_string()),
            manage_collaborator_entity: Some("collaborator_can_view".to_string()),
            link_share_entity: Some("tenant_readable".to_string()),
            copy_entity: Some("anyone_can_view".to_string()),
            lock_switch: Some(false)}
// === Service Tests ===,
    #[test]
fn test_permissions_service_new() {
        let config = create_test_config();
let service = PermissionsService::new(config);
        // Service should be created successfully,
assert!(std::ptr::addr_of!(service).is_aligned());
    }
// === GetPermissionRequest Tests ===,
    #[test]
fn test_get_permission_request_builder_basic() {
        let request = GetPermissionRequest::builder(),
.token()
            .r#type("doc"),
;
        assert_eq!(request.token, "test_token_123");
        assert_eq!(request.r#type, "doc");
