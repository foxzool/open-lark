#![allow(unused_variables, unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use openlark_core::{
    api::ApiRequest,
    core::{BaseResponse, ResponseFormat, api::ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    reqwest::Method,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 删除自定义角色请求
#[derive(Clone)]
pub struct DeleteAppRoleRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 自定义角色的id
    #[serde(skip)]
    role_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
}

impl DeleteAppRoleRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::DELETE, "/open-apis/bitable/v1/apps/{}/roles/{}".to_string()),
            app_token: String::new(),
            role_id: String::new(),
            user_id_type: None,
        }
    }

    pub fn builder() -> DeleteAppRoleRequestBuilder {
        DeleteAppRoleRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteAppRoleRequestBuilder {
    request: DeleteAppRoleRequest,
}

impl DeleteAppRoleRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.request.role_id = role_id.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn build(self) -> DeleteAppRoleRequest {
        self.request
    }
}

/// 删除自定义角色响应
#[derive(Clone, Serialize, Deserialize)]
pub struct DeleteAppRoleResponse {
    /// 删除的角色ID
    pub role_id: String,
    /// 是否删除成功
    pub deleted: bool,
}

impl ApiResponseTrait for DeleteAppRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除自定义角色
pub async fn delete_app_role(
    request: DeleteAppRoleRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<DeleteAppRoleResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::DELETE);
    api_req.api_path = BITABLE_V1_APP_ROLE_DELETE
        .replace("{app_token}", &request.app_token)
        .replace("{role_id}", &request.role_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert("user_id_type".to_string(), user_id_type.clone());
    }

    let api_resp: openlark_core::core::StandardResponse<DeleteAppRoleResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_app_role_request_builder() {
        let request = DeleteAppRoleRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .role_id("rolxxxxxx")
            .user_id_type("open_id")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_id, "rolxxxxxx");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_delete_app_role_request_minimal() {
        let request = DeleteAppRoleRequest::builder()
            .app_token("test-token")
            .role_id("test-role")
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.role_id, "test-role");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_delete_app_role_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = DeleteAppRoleRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.role_id, "");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_delete_app_role_response_trait() {
        assert_eq!(DeleteAppRoleResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_delete_app_role_request_empty_strings() {
        let request = DeleteAppRoleRequest::builder()
            .app_token("")
            .role_id("")
            .build();

        assert_eq!(request.app_token, "");
        assert_eq!(request.role_id, "");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_delete_app_role_request_long_ids() {
        let long_app_token = "bascn".repeat(20);
        let long_role_id = "rol".repeat(15);

        let request = DeleteAppRoleRequest::builder()
            .app_token(&long_app_token)
            .role_id(&long_role_id)
            .build();

        assert_eq!(request.app_token, long_app_token);
        assert_eq!(request.role_id, long_role_id);
    }

    #[test]
    fn test_delete_app_role_request_builder_chaining() {
        let request = DeleteAppRoleRequest::builder()
            .app_token("app123")
            .role_id("role123")
            .user_id_type("user_id")
            .build();

        assert_eq!(request.app_token, "app123");
        assert_eq!(request.role_id, "role123");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_delete_app_role_request_different_user_id_types() {
        let open_id_request = DeleteAppRoleRequest::builder()
            .app_token("app-token")
            .role_id("role-id")
            .user_id_type("open_id")
            .build();

        let user_id_request = DeleteAppRoleRequest::builder()
            .app_token("app-token")
            .role_id("role-id")
            .user_id_type("user_id")
            .build();

        let union_id_request = DeleteAppRoleRequest::builder()
            .app_token("app-token")
            .role_id("role-id")
            .user_id_type("union_id")
            .build();

        assert_eq!(open_id_request.user_id_type, Some("open_id".to_string()));
        assert_eq!(user_id_request.user_id_type, Some("user_id".to_string()));
        assert_eq!(union_id_request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_delete_app_role_request_with_string_types() {
        let owned_app_token = String::from("owned_app");
        let owned_role_id = String::from("owned_role");

        let request1 = DeleteAppRoleRequest::builder()
            .app_token(owned_app_token)
            .role_id(owned_role_id)
            .build();

        assert_eq!(request1.app_token, "owned_app");
        assert_eq!(request1.role_id, "owned_role");

        let request2 = DeleteAppRoleRequest::builder()
            .app_token(String::from("builder_app"))
            .role_id(String::from("builder_role"))
            .build();

        assert_eq!(request2.app_token, "builder_app");
        assert_eq!(request2.role_id, "builder_role");
    }

    #[test]
    fn test_delete_app_role_request_with_long_values() {
        let long_token = "a".repeat(1000);
        let long_role = "b".repeat(500);

        let request = DeleteAppRoleRequest::builder()
            .app_token(&long_token)
            .role_id(&long_role)
            .build();

        assert_eq!(request.app_token, long_token);
        assert_eq!(request.role_id, long_role);
    }

    #[test]
    fn test_delete_app_role_response_deserialization() {
        let json = r#"{
            "role_id": "rolxxxxxx",
            "deleted": true
        }"#;

        let response: DeleteAppRoleResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.role_id, "rolxxxxxx");
        assert!(response.deleted);
    }

    #[test]
    fn test_delete_app_role_response_deserialization_false() {
        let json = r#"{
            "role_id": "rol123456",
            "deleted": false
        }"#;

        let response: DeleteAppRoleResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.role_id, "rol123456");
        assert!(!response.deleted);
    }

    #[test]
    fn test_delete_app_role_response_with_unicode_role_id() {
        let json = r#"{
            "role_id": "角色_测试_123",
            "deleted": true
        }"#;

        let response: DeleteAppRoleResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.role_id, "角色_测试_123");
        assert!(response.deleted);
    }

    #[test]
    fn test_delete_app_role_response_with_empty_role_id() {
        let json = r#"{
            "role_id": "",
            "deleted": false
        }"#;

        let response: DeleteAppRoleResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.role_id, "");
        assert!(!response.deleted);
    }

    #[test]
    fn test_delete_app_role_request_special_characters() {
        let special_app = "app-token_123.test";
        let special_role = "role@domain#test";

        let request = DeleteAppRoleRequest::builder()
            .app_token(special_app)
            .role_id(special_role)
            .build();

        assert_eq!(request.app_token, special_app);
        assert_eq!(request.role_id, special_role);
    }

    #[test]
    fn test_delete_app_role_request_serialization_skip_fields() {
        let request = DeleteAppRoleRequest::builder()
            .app_token("test")
            .role_id("test")
            .build();

        let serialized = serde_json::to_value(&request).unwrap();
        // serde skip字段不应该出现在序列化结果中
        assert!(serialized.as_object().unwrap().is_empty());
    }

    #[test]
    fn test_delete_app_role_response_serialization() {
        let response = DeleteAppRoleResponse {
            role_id: "test_role".to_string(),
            deleted: true,
        };

        let serialized = serde_json::to_value(&response).unwrap();
        assert_eq!(serialized["role_id"], "test_role");
        assert_eq!(serialized["deleted"], true);
    }
}