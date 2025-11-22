#![allow(unused_variables, unused_unsafe)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use SDKResult;use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use openlark_core::,
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
    service::bitable::v1::app_role::AppRole,
};
/// 列出自定义角色请求,
#[derive(Clone)]
pub struct ListAppRoleRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 分页标记,
#[serde(skip)]
    page_token: Option<String>,
    /// 分页大小,
#[serde(skip)]
    page_size: Option<i32>}
impl ListAppRoleRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct ListAppRoleRequestBuilder {
    request: ListAppRoleRequest}
impl ListAppRoleRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(page_size) = &self.request.page_size {,
            self.request,
.api_request,
                .query_params
                .insert("page_size", page_size.to_string());
self.request,
    }
// 应用ExecutableBuilder trait到ListAppRoleRequestBuilder,
crate::impl_executable_builder_owned!(
    ListAppRoleRequestBuilder,
    super::AppRoleService,
    ListAppRoleRequest,
    Response<ListAppRoleResponse>,
    list,
);
/// 列出自定义角色响应
#[derive(Clone)]
pub struct ListAppRoleResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
    /// 自定义角色信息列表
    pub items: Vec<AppRole>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 列出自定义角色,
pub async fn list_app_roles(
    request: ListAppRoleRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<ListAppRoleResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
    api_req.set_api_path(BITABLE_V1_ROLES.replace("{app_token}", &request.app_token));
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
mod tests {
    use super::*;
use serde_json;
    #[test]
fn test_list_app_role_request_builder() {
        let request = ListAppRoleRequest::builder(),
.app_token()
            .page_size()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.page_size, Some(20));
#[test]
    fn test_list_app_role_request_new() {
let request = ListAppRoleRequest::new("bascnmBA*****yGehy8");
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.page_token, None);
        assert_eq!(request.page_size, None);
#[test]
    fn test_list_app_role_request_default() {
let request = ListAppRoleRequest::default();
        assert_eq!(request.app_token, "");
        assert_eq!(request.page_token, None);
        assert_eq!(request.page_size, None);
#[test]
    fn test_list_app_role_request_builder_default() {
let builder = ListAppRoleRequestBuilder::default();
        let request = builder.build();

        assert_eq!(request.app_token, "");
        assert_eq!(request.page_token, None);
        assert_eq!(request.page_size, None);
#[test]
    fn test_list_app_role_request_builder_chaining() {
let request = ListAppRoleRequest::builder(),
            .app_token()
.page_token()
            .page_size()
.app_token()
            .page_token()
.page_size()
            .build();

        assert_eq!(request.app_token, "app2");
        assert_eq!(request.page_token, Some("token2".to_string()));
        assert_eq!(request.page_size, Some(20));
#[test]
    fn test_list_app_role_request_debug() {
let request = ListAppRoleRequest::new("test_app");
        let debug_str = format!("{:?}", request);
assert!(debug_str.contains("ListAppRoleRequest"));
        assert!(debug_str.contains("test_app"));
#[test]
    fn test_list_app_role_request_clone() {
let request = ListAppRoleRequest::builder(),
            .app_token()
.page_token()
            .page_size()
.build();
        let cloned = request.clone();

        assert_eq!(request.app_token, cloned.app_token);
        assert_eq!(request.page_token, cloned.page_token);
        assert_eq!(request.page_size, cloned.page_size);
#[test]
    fn test_list_app_role_request_serialization() {
let request = ListAppRoleRequest::new("app_token_123");
        let serialized = serde_json::to_string(&request).unwrap();
// Skip字段不应该出现在序列化结果中,
        assert!(!serialized.contains("app_token"));
assert!(!serialized.contains("page_token"));
        assert!(!serialized.contains("page_size"));
assert!(!serialized.contains("api_request"));
        assert_eq!(serialized, "{}");
#[test]
    fn test_list_app_role_request_with_page_token() {
let request = ListAppRoleRequest::builder(),
            .app_token()
.page_token()
            .build();

        assert_eq!(request.app_token, "test_app");
        assert_eq!(request.page_token, Some("next_page_token".to_string()));
        assert_eq!(request.page_size, None);
#[test]
    fn test_list_app_role_request_with_page_size() {
let request = ListAppRoleRequest::builder(),
            .app_token()
.page_size()
            .build();

        assert_eq!(request.app_token, "test_app");
        assert_eq!(request.page_token, None);
        assert_eq!(request.page_size, Some(50));
#[test]
    fn test_list_app_role_request_with_unicode() {
let request = ListAppRoleRequest::new("应用令牌_123");
        assert_eq!(request.app_token, "应用令牌_123");
#[test]
    fn test_list_app_role_request_with_string_types() {
let owned_app_token = String::from("owned_app");
        let request1 = ListAppRoleRequest::new(owned_app_token);

        assert_eq!(request1.app_token, "owned_app");
let request2 = ListAppRoleRequest::builder(),
            .app_token(String::from("builder_app")),
.page_token(String::from("builder_token")),
            .build();

        assert_eq!(request2.app_token, "builder_app");
        assert_eq!(request2.page_token, Some("builder_token".to_string()));
#[test]
    fn test_list_app_role_request_with_long_values() {
let long_token = "a".repeat(1000);
        let long_page_token = "b".repeat(500);
let request = ListAppRoleRequest::builder(),
            .app_token()
.page_token()
            .build();

        assert_eq!(request.app_token, long_token);
        assert_eq!(request.page_token, Some(long_page_token));
#[test]
    fn test_list_app_role_request_with_empty_values() {
let request = ListAppRoleRequest::new("");
        assert_eq!(request.app_token, "");
        assert_eq!(request.page_token, None);
        assert_eq!(request.page_size, None);
#[test]
    fn test_list_app_role_request_with_special_characters() {
let special_app = "app-token_123.test";
        let special_token = "token@domain#test";
let request = ListAppRoleRequest::builder(),
            .app_token()
.page_token()
            .build();

        assert_eq!(request.app_token, special_app);
        assert_eq!(request.page_token, Some(special_token.to_string()));
#[test]
    fn test_list_app_role_request_builder_partial() {
let request1 = ListAppRoleRequest::builder().app_token("only_app").build();
        assert_eq!(request1.app_token, "only_app");
        assert_eq!(request1.page_token, None);
        assert_eq!(request1.page_size, None);
let request2 = ListAppRoleRequest::builder().page_size(100).build();
        assert_eq!(request2.app_token, "");
        assert_eq!(request2.page_size, Some(100));
#[test]
    fn test_list_app_role_request_page_size_bounds() {
let request1 = ListAppRoleRequest::builder(),
            .app_token()
.page_size()
            .build();
        assert_eq!(request1.page_size, Some(1));
let request2 = ListAppRoleRequest::builder(),
            .app_token()
.page_size()
            .build();
        assert_eq!(request2.page_size, Some(1000));
let request3 = ListAppRoleRequest::builder(),
            .app_token()
.page_size()
            .build();
        assert_eq!(request3.page_size, Some(0));
let request4 = ListAppRoleRequest::builder(),
            .app_token()
.page_size()
            .build();
        assert_eq!(request4.page_size, Some(-1));
#[test]
    fn test_list_app_role_response_deserialization() {
let json = r#"{,
            "has_more": true,
            "page_token": "next_page_token",
            "total": 2,
            "items": [,
{,
                    "role_id": "rolxxxxxx",
                    "role_name": "自定义角色1",
                    "table_roles": []
                    "block_roles": []
                {
                    "role_id": "rolyyyyyy",
                    "role_name": "自定义角色2",
                    "table_roles": []
                    "block_roles": []
],
        }"#;
let response: ListAppRoleResponse = serde_json::from_str(json).unwrap();
        assert!(response.has_more);
        assert_eq!(response.page_token, Some("next_page_token".to_string()));
        assert_eq!(response.total, 2);
        assert_eq!(response.items.len(), 2);
        assert_eq!(response.items[0].role_id, "rolxxxxxx");
        assert_eq!(response.items[1].role_id, "rolyyyyyy");
#[test]
    fn test_list_app_role_response_deserialization_empty() {
let json = r#"{,
            "has_more": false,
            "page_token": null,
            "total": 0,
            "items": []
}"#;
let response: ListAppRoleResponse = serde_json::from_str(json).unwrap();
        assert!(!response.has_more);
        assert_eq!(response.page_token, None);
        assert_eq!(response.total, 0);
        assert_eq!(response.items.len(), 0);
#[test]
    fn test_list_app_role_response_debug() {
let response = ListAppRoleResponse {,
            has_more: true,
            page_token: Some("debug_token".to_string()),
            total: 1,
            items: vec![]
        };

        let debug_str = format!("{:?}", response);
assert!(debug_str.contains("ListAppRoleResponse"));
        assert!(debug_str.contains("debug_token"));
assert!(debug_str.contains("true"));
    }
#[test]
    fn test_list_app_role_response_data_format() {
let format = ListAppRoleResponse::data_format();
        assert!(matches!(format, ResponseFormat::Data));
#[test]
    fn test_list_app_role_response_with_unicode() {
let json = r#"{,
            "has_more": false,
            "page_token": "令牌_测试",
            "total": 1,
            "items": [,
{,
                    "role_id": "角色_123",
                    "role_name": "自定义角色_测试",
                    "table_roles": []
                    "block_roles": []
],
        }"#;
let response: ListAppRoleResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.page_token, Some("令牌_测试".to_string()));
        assert_eq!(response.items[0].role_id, "角色_123");
        assert_eq!(response.items[0].role_name, "自定义角色_测试");
#[test]
    fn test_memory_efficiency() {
let request = ListAppRoleRequest::new("test");
        let size = std::mem::size_of_val(&request);
assert!(size > 0);
        assert!(size < 1024);
#[test]
    fn test_list_app_role_request_query_params_build() {
let request = ListAppRoleRequest::builder(),
            .app_token()
.page_token()
            .page_size()
.build();
        assert!(request.api_request.query_params.contains_key("page_token"));
assert!(request.api_request.query_params.contains_key("page_size"));
        assert_eq!(
            request.api_request.query_params.get("page_token").unwrap(),
            "test_token",
);
        assert_eq!(
            request.api_request.query_params.get("page_size").unwrap(),
            "25",
);
    }
#[test]
    fn test_list_app_role_request_query_params_partial() {
let request1 = ListAppRoleRequest::builder(),
            .app_token()
.page_token()
            .build();
assert!(request1.api_request.query_params.contains_key("page_token"));
        assert!(!request1.api_request.query_params.contains_key("page_size"));
let request2 = ListAppRoleRequest::builder(),
            .app_token()
.page_size()
            .build();
assert!(!request2.api_request.query_params.contains_key("page_token"));
        assert!(request2.api_request.query_params.contains_key("page_size"));
#[test]
    fn test_list_app_role_request_builder_method_returns() {
let builder = ListAppRoleRequest::builder().app_token("测试链式");
        // 确保builder方法返回正确的类型,
let _chained = builder.page_token("token").page_size(10);
    }
