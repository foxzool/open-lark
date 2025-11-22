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
};
/// 删除自定义角色请求,
#[derive(Clone)]
pub struct DeleteAppRoleRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 自定义角色的id,
#[serde(skip)]
    role_id: String}
impl DeleteAppRoleRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct DeleteAppRoleRequestBuilder {
    request: DeleteAppRoleRequest}
impl DeleteAppRoleRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    DeleteAppRoleRequestBuilder,
    AppRoleService,
    DeleteAppRoleRequest,
    Response<DeleteAppRoleResponse>,
    delete,
);
/// 删除自定义角色响应
#[derive(Clone)]
pub struct DeleteAppRoleResponse {
    /// 删除的角色ID
    pub role_id: String,
    /// 是否删除成功
    pub deleted: bool,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 删除自定义角色,
pub async fn delete_app_role(
    request: DeleteAppRoleRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<DeleteAppRoleResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::DELETE);
api_req.api_path = BITABLE_V1_ROLE_DELETE,
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
fn test_delete_app_role_request_builder() {
        let request = DeleteAppRoleRequest::builder(),
.app_token()
            .role_id()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_id, "rolxxxxxx");
#[test]
    ,
        let request = DeleteAppRoleRequest::new("bascnmBA*****yGehy8", "rolxxxxxx");

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_id, "rolxxxxxx");
#[test]
    fn test_delete_app_role_request_default() {
let request = DeleteAppRoleRequest::default();
        assert_eq!(request.app_token, "");
        assert_eq!(request.role_id, "");
#[test]
    fn test_delete_app_role_request_builder_default() {
let builder = DeleteAppRoleRequestBuilder::default();
        let request = builder.build();

        assert_eq!(request.app_token, "");
        assert_eq!(request.role_id, "");
#[test]
    fn test_delete_app_role_request_builder_chaining() {
let request = DeleteAppRoleRequest::builder(),
            .app_token()
.role_id()
            .app_token()
.role_id()
            .build();

        assert_eq!(request.app_token, "app2");
        assert_eq!(request.role_id, "role2");
#[test]
    ,
        let request = DeleteAppRoleRequest::new("test_app", "test_role");
        let debug_str = format!("{:?}", request);
assert!(debug_str.contains("DeleteAppRoleRequest"));
        assert!(debug_str.contains("test_app"));
assert!(debug_str.contains("test_role"));
    }
#[test]
    ,
        let request = DeleteAppRoleRequest::new("app_token_123", "role_456");
let serialized = serde_json::to_string(&request).unwrap();
        // Skip字段不应该出现在序列化结果中,
assert!(!serialized.contains("app_token"));
        assert!(!serialized.contains("role_id"));
assert!(!serialized.contains("api_request"));
        assert_eq!(serialized, "{}");
#[test]
    ,
        let request = DeleteAppRoleRequest::new("", "");

        assert_eq!(request.app_token, "");
        assert_eq!(request.role_id, "");
#[test]
    ,
        let request = DeleteAppRoleRequest::new("应用令牌_123", "角色_456");

        assert_eq!(request.app_token, "应用令牌_123");
        assert_eq!(request.role_id, "角色_456");
#[test]
    fn test_delete_app_role_request_with_string_types() {
let owned_app_token = String::from("owned_app");
        let owned_role_id = String::from("owned_role");
        let request1 = DeleteAppRoleRequest::new(owned_app_token, owned_role_id);

        assert_eq!(request1.app_token, "owned_app");
        assert_eq!(request1.role_id, "owned_role");
let request2 = DeleteAppRoleRequest::builder(),
            .app_token(String::from("builder_app")),
.role_id(String::from("builder_role")),
            .build();

        assert_eq!(request2.app_token, "builder_app");
        assert_eq!(request2.role_id, "builder_role");
#[test]
    fn test_delete_app_role_request_with_long_values() {
let long_token = "a".repeat(1000);
        let long_role = "b".repeat(500);
        let request = DeleteAppRoleRequest::new(&long_token, &long_role);

        assert_eq!(request.app_token, long_token);
        assert_eq!(request.role_id, long_role);
#[test]
    fn test_delete_app_role_response_deserialization() {
let json = r#"{,
            "role_id": "rolxxxxxx",
            "deleted": true}"#;
let response: DeleteAppRoleResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.role_id, "rolxxxxxx");
assert!(response.deleted);
    }
#[test]
    fn test_delete_app_role_response_deserialization_false() {
let json = r#"{,
            "role_id": "rol123456",
            "deleted": false}"#;
let response: DeleteAppRoleResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.role_id, "rol123456");
assert!(!response.deleted);
    }
#[test]
    fn test_delete_app_role_response_debug() {
let response = DeleteAppRoleResponse {,
            role_id: "debug_role".to_string(),
            deleted: true};

        let debug_str = format!("{:?}", response);
assert!(debug_str.contains("DeleteAppRoleResponse"));
        assert!(debug_str.contains("debug_role"));
assert!(debug_str.contains("true"));
    }
#[test]
    fn test_delete_app_role_response_data_format() {
let format = DeleteAppRoleResponse::data_format();
        assert!(matches!(format, ResponseFormat::Data));
#[test]
    fn test_delete_app_role_response_with_unicode_role_id() {
let json = r#"{,
            "role_id": "角色_测试_123",
            "deleted": true}"#;
let response: DeleteAppRoleResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.role_id, "角色_测试_123");
assert!(response.deleted);
    }
#[test]
    fn test_delete_app_role_response_with_empty_role_id() {
let json = r#"{,
            "role_id": "",
            "deleted": false}"#;
let response: DeleteAppRoleResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.role_id, "");
assert!(!response.deleted);
    }
#[test]
    ,
        let request = DeleteAppRoleRequest::new("test", "test");
let size = std::mem::size_of_val(&request);
        assert!(size > 0);
assert!(size < 1024);
    }
#[test]
    fn test_delete_app_role_request_builder_partial() {
let request1 = DeleteAppRoleRequest::builder(),
            .app_token()
.build();
        assert_eq!(request1.app_token, "only_app");
        assert_eq!(request1.role_id, "");
let request2 = DeleteAppRoleRequest::builder().role_id("only_role").build();
        assert_eq!(request2.app_token, "");
        assert_eq!(request2.role_id, "only_role");
#[test]
    fn test_delete_app_role_request_special_characters() {
let special_app = "app-token_123.test";
        let special_role = "role@domain#test";
        let request = DeleteAppRoleRequest::new(special_app, special_role);

        assert_eq!(request.app_token, special_app);
        assert_eq!(request.role_id, special_role);
