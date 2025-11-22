#![allow(unused_variables, unused_unsafe)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use SDKResult;use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use super::AppRoleService;
use openlark_core::,
{,
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
    service::bitable::v1::app_role::{AppRole, BlockRole, TableRole}
};
/// 更新自定义角色请求,
#[derive(Clone)]
pub struct UpdateAppRoleRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 自定义角色的id,
#[serde(skip)]
    role_id: String,
    /// 角色名称,
#[serde(skip_serializing_if = "Option::is_none")]
    role_name: Option<String>,
    /// 数据表权限,
#[serde(skip_serializing_if = "Option::is_none")]
    table_roles: Option<Vec<TableRole>>,
    /// 数据表默认权限,
#[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>}
impl UpdateAppRoleRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct UpdateAppRoleRequestBuilder {
    request: UpdateAppRoleRequest}
impl UpdateAppRoleRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    UpdateAppRoleRequestBuilder,
    AppRoleService,
    UpdateAppRoleRequest,
    Response<UpdateAppRoleResponse>,
    update,
);
/// 更新自定义角色响应
#[derive(Clone)]
pub struct UpdateAppRoleResponse {
    /// 更新后的自定义角色信息
    pub role: AppRole,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 更新自定义角色,
pub async fn update_app_role(
    request: UpdateAppRoleRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<UpdateAppRoleResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::PUT);
api_req.api_path = BITABLE_V1_ROLE_UPDATE,
        .replace("{app_token}", &request.app_token)
        .replace("{role_id}", &request.role_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
mod tests {
    use super::*;
use serde_json;
    #[test]
fn test_update_app_role_request_builder() {
        let request = UpdateAppRoleRequest::builder(),
.app_token()
            .role_id()
.role_name()
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_id, "rolxxxxxx");
        assert_eq!(request.role_name, Some("更新后的角色名称".to_string()));
#[test]
    ,
        let request = UpdateAppRoleRequest::new("bascnmBA*****yGehy8", "rolxxxxxx");

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_id, "rolxxxxxx");
        assert_eq!(request.role_name, None);
assert!(request.table_roles.is_none());
        assert!(request.block_roles.is_none());
#[test]
    fn test_update_app_role_request_default() {
let request = UpdateAppRoleRequest::default();
        assert_eq!(request.app_token, "");
        assert_eq!(request.role_id, "");
        assert_eq!(request.role_name, None);
assert!(request.table_roles.is_none());
        assert!(request.block_roles.is_none());
#[test]
    fn test_update_app_role_request_builder_default() {
let builder = UpdateAppRoleRequestBuilder::default();
        let request = builder.build();

        assert_eq!(request.app_token, "");
        assert_eq!(request.role_id, "");
        assert_eq!(request.role_name, None);
assert!(request.table_roles.is_none());
        assert!(request.block_roles.is_none());
#[test]
    fn test_update_app_role_request_builder_chaining() {
let request = UpdateAppRoleRequest::builder(),
            .app_token()
.role_id()
            .role_name()
.app_token()
            .role_id()
.role_name()
            .build();

        assert_eq!(request.app_token, "app2");
        assert_eq!(request.role_id, "role2");
        assert_eq!(request.role_name, Some("name2".to_string()));
#[test]
    ,
        let request = UpdateAppRoleRequest::new("test_app", "test_role");
        let debug_str = format!("{:?}", request);
assert!(debug_str.contains("UpdateAppRoleRequest"));
        assert!(debug_str.contains("test_app"));
assert!(debug_str.contains("test_role"));
    }
#[test]
    ,
        let request = UpdateAppRoleRequest::new("app_token_123", "role_456");
let serialized = serde_json::to_string(&request).unwrap();
        // Skip字段不应该出现在序列化结果中,
assert!(!serialized.contains("app_token"));
        assert!(!serialized.contains("role_id"));
assert!(!serialized.contains("api_request"));
        assert_eq!(serialized, "{}");
#[test]
    fn test_update_app_role_request_with_role_name() {
let request = UpdateAppRoleRequest::builder(),
            .app_token()
.role_id()
            .role_name()
.build();
        assert_eq!(request.app_token, "test_app");
        assert_eq!(request.role_id, "test_role");
        assert_eq!(request.role_name, Some("新角色名称".to_string()));
assert!(request.table_roles.is_none());
        assert!(request.block_roles.is_none());
#[test]
    fn test_update_app_role_request_with_table_roles() {
let table_roles = vec![]; // Empty vector for now since we need the actual TableRole definition,
        let request = UpdateAppRoleRequest::builder(),
.app_token()
            .role_id()
.table_roles(table_roles.clone()),
            .build();

        assert_eq!(request.app_token, "test_app");
        assert_eq!(request.role_id, "test_role");
        assert_eq!(request.role_name, None);
assert!(request.table_roles.is_some());
        assert!(request.block_roles.is_none());
#[test]
    fn test_update_app_role_request_with_block_roles() {
let block_roles = vec![]; // Empty vector for now since we need the actual BlockRole definition,
        let request = UpdateAppRoleRequest::builder(),
.app_token()
            .role_id()
.block_roles(block_roles.clone()),
            .build();

        assert_eq!(request.app_token, "test_app");
        assert_eq!(request.role_id, "test_role");
        assert_eq!(request.role_name, None);
assert!(request.table_roles.is_none());
        assert!(request.block_roles.is_some());
#[test]
    fn test_update_app_role_request_with_all_fields() {
let table_roles = vec![];
        let block_roles = vec![];
let request = UpdateAppRoleRequest::builder(),
            .app_token()
.role_id()
            .role_name()
.table_roles(table_roles.clone()),
            .block_roles(block_roles.clone()),
.build();
        assert_eq!(request.app_token, "full_app");
        assert_eq!(request.role_id, "full_role");
        assert_eq!(request.role_name, Some("完整角色".to_string()));
assert!(request.table_roles.is_some());
        assert!(request.block_roles.is_some());
#[test]
    ,
        let request = UpdateAppRoleRequest::new("", "");

        assert_eq!(request.app_token, "");
        assert_eq!(request.role_id, "");
        assert_eq!(request.role_name, None);
#[test]
    ,
        let request = UpdateAppRoleRequest::new("应用令牌_123", "角色_456");

        assert_eq!(request.app_token, "应用令牌_123");
        assert_eq!(request.role_id, "角色_456");
#[test]
    fn test_update_app_role_request_with_string_types() {
let owned_app_token = String::from("owned_app");
        let owned_role_id = String::from("owned_role");
        let request1 = UpdateAppRoleRequest::new(owned_app_token, owned_role_id);

        assert_eq!(request1.app_token, "owned_app");
        assert_eq!(request1.role_id, "owned_role");
let request2 = UpdateAppRoleRequest::builder(),
            .app_token(String::from("builder_app")),
.role_id(String::from("builder_role")),
            .role_name(String::from("builder_name")),
.build();
        assert_eq!(request2.app_token, "builder_app");
        assert_eq!(request2.role_id, "builder_role");
        assert_eq!(request2.role_name, Some("builder_name".to_string()));
#[test]
    fn test_update_app_role_request_with_long_values() {
let long_token = "a".repeat(1000);
        let long_role = "b".repeat(500);
let long_name = "c".repeat(200);
        let request = UpdateAppRoleRequest::builder(),
.app_token()
            .role_id()
.role_name()
            .build();

        assert_eq!(request.app_token, long_token);
        assert_eq!(request.role_id, long_role);
        assert_eq!(request.role_name, Some(long_name));
#[test]
    fn test_update_app_role_request_with_special_characters() {
let special_app = "app-token_123.test";
        let special_role = "role@domain#test";
let special_name = "角色名称!@#$%^&*()";
        let request = UpdateAppRoleRequest::builder(),
.app_token()
            .role_id()
.role_name()
            .build();

        assert_eq!(request.app_token, special_app);
        assert_eq!(request.role_id, special_role);
        assert_eq!(request.role_name, Some(special_name.to_string()));
#[test]
    fn test_update_app_role_request_builder_partial() {
let request1 = UpdateAppRoleRequest::builder(),
            .app_token()
.build();
        assert_eq!(request1.app_token, "only_app");
        assert_eq!(request1.role_id, "");
assert!(request1.role_name.is_none());
        let request2 = UpdateAppRoleRequest::builder().role_id("only_role").build();

        assert_eq!(request2.app_token, "");
        assert_eq!(request2.role_id, "only_role");
assert!(request2.role_name.is_none());
        let request3 = UpdateAppRoleRequest::builder(),
.role_name()
            .build();

        assert_eq!(request3.app_token, "");
        assert_eq!(request3.role_id, "");
        assert_eq!(request3.role_name, Some("only_name".to_string()));
#[test]
    fn test_update_app_role_request_serialization_with_role_name() {
let request = UpdateAppRoleRequest::builder(),
            .app_token()
.role_id()
            .role_name()
.build();
        let serialized = serde_json::to_string(&request).unwrap();
// role_name should appear in serialization,
        assert!(serialized.contains("role_name"));
assert!(serialized.contains("测试角色"));
        // Skip fields should not appear,
assert!(!serialized.contains("app_token"));
        assert!(!serialized.contains("role_id"));
assert!(!serialized.contains("api_request"));
    }
#[test]
    fn test_update_app_role_request_serialization_with_optional_fields() {
let table_roles = vec![];
        let block_roles = vec![];
let request = UpdateAppRoleRequest::builder(),
            .app_token()
.role_id()
            .role_name()
.table_roles()
            .block_roles()
.build();
        let serialized = serde_json::to_string(&request).unwrap();
// Optional fields should appear in serialization,
        assert!(serialized.contains("role_name"));
assert!(serialized.contains("table_roles"));
        assert!(serialized.contains("block_roles"));
assert!(serialized.contains("测试角色"));
    }
#[test]
    fn test_update_app_role_response_data_format() {
let format = UpdateAppRoleResponse::data_format();
        assert!(matches!(format, ResponseFormat::Data));
#[test]
    ,
        let request = UpdateAppRoleRequest::new("test", "test");
let size = std::mem::size_of_val(&request);
        assert!(size > 0);
assert!(size < 1024);
    }
#[test]
    fn test_update_app_role_request_builder_method_returns() {
let builder = UpdateAppRoleRequest::builder().app_token("测试链式");
        // 确保builder方法返回正确的类型,
let _chained = builder.role_id("role").role_name("name");
    }
#[test]
    fn test_update_app_role_request_build_with_body() {
let request = UpdateAppRoleRequest::builder(),
            .app_token()
.role_id()
            .role_name()
.build();
        // build() method should set the body in api_request,
assert!(!request.api_request.body.is_empty());
    }
