
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
#[derive(Clone, Debug)]
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
}#[derive(Clone, Debug)]
pub struct GetPermissionRequestBuilder {
    request: GetPermissionRequest}
impl GetPermissionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 返回的文档公共设置,
#[derive(Clone, Debug)]
pub struct GetPermissionResponse {
    /// 返回的文档公共设置
    pub permission_public: PermissionPublic,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> api::ResponseFormat {,
api::ResponseFormat::Data
    }

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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
}#[derive(Clone, Debug)]
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
.build()}
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
.build();
        assert_eq!(request.token, "test_token_123");
        assert_eq!(request.r#type, "doc");
assert!(request.api_request.query_params.contains_key("type"));
        assert_eq!(request.api_request.query_params.get("type").unwrap(), "doc");
#[rstest]
    #[case("doc", "旧版文档")]
    #[case("sheet", "电子表格")]
    #[case("file", "云空间文件")]
    #[case("wiki", "知识库节点")]
    #[case("bitable", "多维表格")]
    #[case("docx", "新版文档")]
    #[case("mindnote", "思维笔记")]
    #[case("minutes", "妙记")]
    #[case("slides", "幻灯片")]
fn test_get_permission_request_all_file_types(,
        #[case] file_type: &str,
        #[case] _description: &str,
    ) {,
let request = GetPermissionRequest::builder(),
            .token()
.r#type(file_type),
            .build();

        assert_eq!(request.r#type, file_type);
assert_eq!(,
            request.api_request.query_params.get("type").unwrap(),
            file_type,
);
    }
#[test]
    fn test_get_permission_request_builder_method_chaining() {
let request = GetPermissionRequest::builder(),
            .token()
.r#type("sheet"),
            .build();

        assert_eq!(request.token, "chain_token");
        assert_eq!(request.r#type, "sheet");
#[test]
    fn test_get_permission_request_default() {
let request = GetPermissionRequest::default();
        assert_eq!(request.token, "");
        assert_eq!(request.r#type, "");
assert!(request.api_request.query_params.is_empty());
    }
#[test]
    fn test_get_permission_request_builder_overwrite() {
let request = GetPermissionRequest::builder(),
            .token()
.r#type("doc"),
            .token("second_token") // overwrite,
.r#type("sheet") // overwrite,
            .build();

        assert_eq!(request.token, "second_token");
        assert_eq!(request.r#type, "sheet");
assert_eq!(,
            request.api_request.query_params.get("type").unwrap(),
            "sheet",
);
    }
// === PatchPermissionRequest Tests ===,
    #[test]
fn test_patch_permission_request_builder_basic() {
        let request = PatchPermissionRequest::builder(),
.token()
            .r#type("docx"),
.external_access_entity()
            .build();

        assert_eq!(request.token, "patch_token");
        assert_eq!(request.external_access_entity, Some("open".to_string()));
assert!(request.api_request.query_params.contains_key("type"));
        assert_eq!(
            request.api_request.query_params.get("type").unwrap(),
            "docx",
);
    }
#[test]
    fn test_patch_permission_request_all_entities() {
let request = PatchPermissionRequest::builder(),
            .token()
.r#type("doc"),
            .external_access_entity()
.security_entity()
            .comment_entity()
.share_entity()
            .manage_collaborator_entity()
.link_share_entity()
            .copy_entity()
.build();
        assert_eq!(request.token, "full_patch_token");
        assert_eq!(request.external_access_entity, Some("open".to_string()));
        assert_eq!(request.security_entity, Some("anyone_can_view".to_string()));
        assert_eq!(request.comment_entity, Some("anyone_can_edit".to_string()));
        assert_eq!(request.share_entity, Some("same_tenant".to_string()));
assert_eq!(,
            request.manage_collaborator_entity,
            Some("collaborator_can_edit".to_string()),
);
        assert_eq!(
            request.link_share_entity,
            Some("tenant_editable".to_string()),
);
        assert_eq!(request.copy_entity, Some("only_full_access".to_string()));
#[rstest]
    #[case("open", "打开")]
    #[case("closed", "关闭")]
    #[case("allow_share_partner_tenant", "允许分享给关联组织")]
    fn test_patch_external_access_entity_values(#[case] value: &str, #[case] _description: &str) {,
let request = PatchPermissionRequest::builder(),
            .token()
.external_access_entity()
            .build();

        assert_eq!(request.external_access_entity, Some(value.to_string()));
#[rstest]
    #[case("anyone_can_view", "拥有可阅读权限的用户")]
    #[case("anyone_can_edit", "拥有可编辑权限的用户")]
    #[case("only_full_access", "拥有可管理权限的用户")]
    fn test_patch_security_entity_values(#[case] value: &str, #[case] _description: &str) {,
let request = PatchPermissionRequest::builder(),
            .token()
.security_entity()
            .build();

        assert_eq!(request.security_entity, Some(value.to_string()));
#[rstest]
    #[case("anyone_can_view", "拥有可阅读权限的用户")]
    #[case("anyone_can_edit", "拥有可编辑权限的用户")]
    fn test_patch_comment_entity_values(#[case] value: &str, #[case] _description: &str) {,
let request = PatchPermissionRequest::builder(),
            .token()
.comment_entity()
            .build();

        assert_eq!(request.comment_entity, Some(value.to_string()));
#[rstest]
    #[case("anyone", "所有可阅读或编辑此文档的用户")]
    #[case("same_tenant", "组织内所有可阅读或编辑此文档的用户")]
    fn test_patch_share_entity_values(#[case] value: &str, #[case] _description: &str) {,
let request = PatchPermissionRequest::builder(),
            .token()
.share_entity()
            .build();

        assert_eq!(request.share_entity, Some(value.to_string()));
#[rstest]
    #[case("collaborator_can_view", "拥有可阅读权限的协作者")]
    #[case("collaborator_can_edit", "拥有可编辑权限的协作者")]
    #[case("collaborator_full_access", "拥有可管理权限的协作者")]
fn test_patch_manage_collaborator_entity_values(,
        #[case] value: &str,
        #[case] _description: &str,
    ) {,
let request = PatchPermissionRequest::builder(),
            .token()
.manage_collaborator_entity()
            .build();

        assert_eq!(request.manage_collaborator_entity, Some(value.to_string()));
#[rstest]
    #[case("tenant_readable", "组织内获得链接的人可阅读")]
    #[case("tenant_editable", "组织内获得链接的人可编辑")]
    #[case("partner_tenant_readable", "关联组织的人可阅读")]
    #[case("partner_tenant_editable", "关联组织的人可编辑")]
    #[case("anyone_readable", "互联网上获得链接的任何人可阅读")]
    #[case("anyone_editable", "互联网上获得链接的任何人可编辑")]
    #[case("closed", "关闭链接分享")]
    fn test_patch_link_share_entity_values(#[case] value: &str, #[case] _description: &str) {,
let request = PatchPermissionRequest::builder(),
            .token()
.link_share_entity()
            .build();

        assert_eq!(request.link_share_entity, Some(value.to_string()));
#[test]
    fn test_patch_permission_request_default() {
let request = PatchPermissionRequest::default();
        assert_eq!(request.token, "");
        assert_eq!(request.external_access_entity, None);
        assert_eq!(request.security_entity, None);
        assert_eq!(request.comment_entity, None);
        assert_eq!(request.share_entity, None);
        assert_eq!(request.manage_collaborator_entity, None);
        assert_eq!(request.link_share_entity, None);
        assert_eq!(request.copy_entity, None);
#[test]
    fn test_patch_permission_request_partial_update() {
let request = PatchPermissionRequest::builder(),
            .token()
.external_access_entity()
            .security_entity("only_full_access"),
// Intentionally skip other fields,
            .build();

        assert_eq!(request.token, "partial_token");
        assert_eq!(request.external_access_entity, Some("closed".to_string()));
assert_eq!(,
            request.security_entity,
            Some("only_full_access".to_string()),
);
        assert_eq!(request.comment_entity, None);
        assert_eq!(request.share_entity, None);
        assert_eq!(request.manage_collaborator_entity, None);
        assert_eq!(request.link_share_entity, None);
        assert_eq!(request.copy_entity, None);
// === Response Structure Tests ===,
    #[test]
fn test_get_permission_response_api_trait() {
        let format = GetPermissionResponse::data_format();
        assert_eq!(format, ResponseFormat::Data);
#[test]
    fn test_permission_public_creation() {
let permission = create_test_permission_public();
        assert_eq!(permission.external_access_entity, Some("open".to_string()));
assert_eq!(,
            permission.security_entity,
            Some("anyone_can_view".to_string()),
);
        assert_eq!(
            permission.comment_entity,
            Some("anyone_can_view".to_string()),
);
        assert_eq!(permission.share_entity, Some("anyone".to_string()));
assert_eq!(,
            permission.manage_collaborator_entity,
            Some("collaborator_can_view".to_string()),
);
        assert_eq!(
            permission.link_share_entity,
            Some("tenant_readable".to_string()),
);
        assert_eq!(permission.copy_entity, Some("anyone_can_view".to_string()));
        assert_eq!(permission.lock_switch, Some(false));
#[test]
    fn test_permission_public_optional_fields() {
let permission = PermissionPublic {,
            external_access_entity: None,
            security_entity: None,
            comment_entity: None,
            share_entity: None,
            manage_collaborator_entity: None,
            link_share_entity: None,
            copy_entity: None,
            lock_switch: None};

        assert_eq!(permission.external_access_entity, None);
        assert_eq!(permission.security_entity, None);
        assert_eq!(permission.comment_entity, None);
        assert_eq!(permission.share_entity, None);
        assert_eq!(permission.manage_collaborator_entity, None);
        assert_eq!(permission.link_share_entity, None);
        assert_eq!(permission.copy_entity, None);
        assert_eq!(permission.lock_switch, None);
#[test]
    fn test_get_permission_response_creation() {
let permission_public = create_test_permission_public();
        let response = GetPermissionResponse { permission_public };
assert_eq!(,
            response.permission_public.external_access_entity,
            Some("open".to_string()),
);
        assert_eq!(response.permission_public.lock_switch, Some(false));
// === Serialization Tests ===,
    #[test]
fn test_patch_permission_request_serialization() {
        let request = PatchPermissionRequest::builder(),
.token()
            .external_access_entity()
.security_entity()
            .build();
let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"external_access_entity\":\"open\""));
assert!(json.contains("\"security_entity\":\"anyone_can_view\""));
        // Should not contain token (it's skipped),
assert!(!json.contains("ser_token"));
    }
#[test]
    fn test_patch_permission_request_serialization_skip_none() {
let request = PatchPermissionRequest::builder(),
            .token()
.external_access_entity("closed"),
            // Other fields are None and should be skipped,
.build();
        let json = serde_json::to_string(&request).unwrap();
assert!(json.contains("\"external_access_entity\":\"closed\""));
        assert!(!json.contains("security_entity"));
assert!(!json.contains("comment_entity"));
        assert!(!json.contains("share_entity"));
#[test]
    fn test_get_permission_response_deserialization() {
let json = r#"{,
            "permission_public": {
                "external_access_entity": "open",
                "security_entity": "anyone_can_view",
                "comment_entity": "anyone_can_edit",
                "share_entity": "same_tenant",
                "manage_collaborator_entity": "collaborator_can_view",
                "link_share_entity": "tenant_readable",
                "copy_entity": "only_full_access",
                "lock_switch": true}
        }"#;
let response: GetPermissionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response.permission_public.external_access_entity,
            Some("open".to_string()),
);
        assert_eq!(
            response.permission_public.security_entity,
            Some("anyone_can_view".to_string()),
);
        assert_eq!(
            response.permission_public.comment_entity,
            Some("anyone_can_edit".to_string()),
);
        assert_eq!(
            response.permission_public.share_entity,
            Some("same_tenant".to_string()),
);
        assert_eq!(
            response.permission_public.manage_collaborator_entity,
            Some("collaborator_can_view".to_string()),
);
        assert_eq!(
            response.permission_public.link_share_entity,
            Some("tenant_readable".to_string()),
);
        assert_eq!(
            response.permission_public.copy_entity,
            Some("only_full_access".to_string()),
);
        assert_eq!(response.permission_public.lock_switch, Some(true));
#[test]
    fn test_permission_public_deserialization_partial() {
let json = r#"{,
            "external_access_entity": "closed",
            "lock_switch": false}"#;
let permission: PermissionPublic = serde_json::from_str(json).unwrap();
        assert_eq!(
            permission.external_access_entity,
            Some("closed".to_string()),
);
        assert_eq!(permission.lock_switch, Some(false));
        assert_eq!(permission.security_entity, None);
        assert_eq!(permission.comment_entity, None);
        assert_eq!(permission.share_entity, None);
// === Edge Case Tests ===,
    #[test]
fn test_get_permission_request_unicode_token() {
        let unicode_token = "测试令牌_🔑_token";
let request = GetPermissionRequest::builder(),
            .token()
.r#type("doc"),
            .build();

        assert_eq!(request.token, unicode_token);
#[test]
    fn test_patch_permission_request_unicode_values() {
let request = PatchPermissionRequest::builder(),
            .token()
.external_access_entity()
            .security_entity()
.build();
        assert_eq!(request.external_access_entity, Some("测试值".to_string()));
        assert_eq!(request.security_entity, Some("权限值".to_string()));
#[test]
    fn test_get_permission_request_empty_values() {
let request = GetPermissionRequest::builder().token("").r#type("").build();
        assert_eq!(request.token, "");
        assert_eq!(request.r#type, "");
        assert_eq!(request.api_request.query_params.get("type").unwrap(), "");
#[test]
    fn test_patch_permission_request_empty_values() {
let request = PatchPermissionRequest::builder(),
            .token()
.external_access_entity()
            .security_entity()
.build();
        assert_eq!(request.token, "");
        assert_eq!(request.external_access_entity, Some("".to_string()));
        assert_eq!(request.security_entity, Some("".to_string()));
#[test]
    fn test_long_token_handling() {
let long_token = "a".repeat(1000);
        let request = GetPermissionRequest::builder(),
.token()
            .r#type("doc"),
.build();
        assert_eq!(request.token, long_token);
#[test]
    fn test_special_characters_in_token() {
let special_token = "token-with_special.chars@123#";
        let request = PatchPermissionRequest::builder(),
.token()
            .external_access_entity()
.build();
        assert_eq!(request.token, special_token);
// === Debug Tests ===,
    #[test]
fn test_debug_implementations() {
    pub fn new(config: Config) -> Self {
        Self { config }
}        let get_request = GetPermissionRequest::builder(),
.token()
            .r#type("doc"),
.build();
        let patch_request = PatchPermissionRequest::builder(),
.token()
            .external_access_entity()
.build();
        let permission = create_test_permission_public();
let response = GetPermissionResponse {,
            permission_public: permission};
// Should not panic,
        let _debug_get = format!("{:?}", get_request);
        let _debug_patch = format!("{:?}", patch_request);
        let _debug_response = format!("{:?}", response);
        let _debug_permission = format!("{:?}", response.permission_public);
// === Builder Pattern Tests ===,
    #[test]
fn test_get_permission_builder_reuse() {
        let builder = GetPermissionRequest::builder(),
.token()
            .r#type("sheet");
// Build multiple requests from same builder state,
        let request1 = builder.build();

        assert_eq!(request1.token, "reuse_test");
        assert_eq!(request1.r#type, "sheet");
#[test]
    fn test_patch_permission_builder_complex_chaining() {
let request = PatchPermissionRequest::builder(),
            .token()
.r#type("bitable"),
            .external_access_entity()
.security_entity()
            .comment_entity()
.share_entity()
            .manage_collaborator_entity()
.link_share_entity()
            .copy_entity()
.build();
        // Verify all chained values
        assert_eq!(request.token, "complex_chain");
        assert_eq!(request.external_access_entity, Some("open".to_string()));
        assert_eq!(request.security_entity, Some("anyone_can_view".to_string()));
        assert_eq!(request.comment_entity, Some("anyone_can_edit".to_string()));
        assert_eq!(request.share_entity, Some("anyone".to_string()));
assert_eq!(,
            request.manage_collaborator_entity,
            Some("collaborator_can_edit".to_string()),
);
        assert_eq!(
            request.link_share_entity,
            Some("anyone_editable".to_string()),
);
        assert_eq!(request.copy_entity, Some("anyone_can_view".to_string()));
// Verify query param was set,
        assert_eq!(
            request.api_request.query_params.get("type").unwrap(),
            "bitable",
);
    }
// === Request Body Tests ===,
    #[test]
fn test_patch_permission_request_body_generation() {
        let request = PatchPermissionRequest::builder(),
.token()
            .external_access_entity()
.security_entity()
            .build();
// Body should be populated after build(),
        assert!(!request.api_request.body.is_empty());
// Parse body to verify content,
        let body_str = String::from_utf8(request.api_request.body).unwrap();
let parsed: serde_json::Value = serde_json::from_str(&body_str).unwrap();
        assert_eq!(parsed["external_access_entity"] "closed");
        assert_eq!(parsed["security_entity"] "only_full_access");
// Token should not be in body (serde skip),
        assert!(parsed.get("token").is_none());
