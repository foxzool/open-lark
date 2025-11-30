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

/// 列出自定义角色请求
#[derive(Clone)]
pub struct ListAppRoleRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 分页标记
    #[serde(skip)]
    page_token: Option<String>,
    /// 分页大小
    #[serde(skip)]
    page_size: Option<i32>,
}

impl ListAppRoleRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::GET, "/open-apis/bitable/v1/apps/{}/roles".to_string()),
            app_token: String::new(),
            user_id_type: None,
            page_token: None,
            page_size: None,
        }
    }

    pub fn builder() -> ListAppRoleRequestBuilder {
        ListAppRoleRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ListAppRoleRequestBuilder {
    request: ListAppRoleRequest,
}

impl ListAppRoleRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size.min(100)); // 限制最大100
        self
    }

    pub fn build(self) -> ListAppRoleRequest {
        self.request
    }
}

/// 自定义角色信息
#[derive(Clone, Serialize, Deserialize)]
pub struct AppRole {
    /// 自定义角色的id
    pub role_id: String,
    /// 角色名称
    pub role_name: String,
    /// 数据表权限
    #[serde(skip_serializing_if = "Option::is_none")]
    table_roles: Option<Vec<TableRole>>,
    /// 数据表默认权限
    #[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>,
}

/// 数据表权限
#[derive(Clone, Serialize, Deserialize)]
pub struct TableRole {
    /// 数据表 id
    pub table_id: String,
    /// 权限
    pub role: String,
    /// 记录权限
    #[serde(skip_serializing_if = "Option::is_none")]
    rec_rule: Option<String>,
}

/// 数据表默认权限
#[derive(Clone, Serialize, Deserialize)]
pub struct BlockRole {
    /// 多维表格数据表的唯一标识符
    pub block_id: String,
    /// 权限
    pub role: String,
}

/// 列出自定义角色响应
#[derive(Clone)]
pub struct ListAppRoleResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 角色信息列表
    pub items: Vec<AppRole>,
}

impl ApiResponseTrait for ListAppRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出自定义角色
pub async fn list_app_role(
    request: ListAppRoleRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<ListAppRoleResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
    api_req.api_path = BITABLE_V1_APP_ROLES
        .replace("{app_token}", &request.app_token);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert("user_id_type".to_string(), user_id_type.clone());
    }

    if let Some(page_token) = &request.page_token {
        api_req
            .query_params
            .insert("page_token".to_string(), page_token.clone());
    }

    if let Some(page_size) = &request.page_size {
        api_req
            .query_params
            .insert("page_size".to_string(), page_size.to_string());
    }

    let api_resp: openlark_core::core::StandardResponse<ListAppRoleResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_app_role_request_builder() {
        let request = ListAppRoleRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .page_size(20)
            .user_id_type("open_id")
            .page_token("next_page_token")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.page_token, Some("next_page_token".to_string()));
    }

    #[test]
    fn test_list_app_role_request_minimal() {
        let request = ListAppRoleRequest::builder()
            .app_token("test-token")
            .build();

        assert_eq!(request.app_token, "test-token");
        assert!(request.user_id_type.is_none());
        assert!(request.page_token.is_none());
        assert!(request.page_size.is_none());
    }

    #[test]
    fn test_page_size_limit() {
        let request = ListAppRoleRequest::builder()
            .app_token("test-token")
            .page_size(200) // 超过100的限制
            .build();

        assert_eq!(request.page_size, Some(100)); // 应该被限制为100
    }

    #[test]
    fn test_list_app_role_response_trait() {
        assert_eq!(ListAppRoleResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_list_app_role_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = ListAppRoleRequest::new(config);

        assert_eq!(request.app_token, "");
        assert!(request.user_id_type.is_none());
        assert!(request.page_token.is_none());
        assert!(request.page_size.is_none());
    }

    #[test]
    fn test_list_app_role_request_builder_chaining() {
        let request = ListAppRoleRequest::builder()
            .app_token("app123")
            .user_id_type("user_id")
            .page_token("page123")
            .page_size(50)
            .build();

        assert_eq!(request.app_token, "app123");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(request.page_token, Some("page123".to_string()));
        assert_eq!(request.page_size, Some(50));
    }

    #[test]
    fn test_list_app_role_response() {
        let response = ListAppRoleResponse {
            has_more: true,
            page_token: Some("next_page_token".to_string()),
            items: vec![
                AppRole {
                    role_id: "rol123".to_string(),
                    role_name: "角色1".to_string(),
                    table_roles: None,
                    block_roles: None,
                },
                AppRole {
                    role_id: "rol456".to_string(),
                    role_name: "角色2".to_string(),
                    table_roles: None,
                    block_roles: None,
                },
            ],
        };

        assert_eq!(response.has_more, true);
        assert_eq!(response.page_token, Some("next_page_token".to_string()));
        assert_eq!(response.items.len(), 2);
        assert_eq!(response.items[0].role_id, "rol123".to_string());
        assert_eq!(response.items[1].role_name, "角色2");
    }

    #[test]
    fn test_list_app_role_request_boundary_values() {
        // 测试边界值
        let request_min = ListAppRoleRequest::builder()
            .app_token("a")
            .page_size(1)
            .build();

        let request_max = ListAppRoleRequest::builder()
            .app_token("a".repeat(100))
            .page_size(100)
            .build();

        assert_eq!(request_min.app_token, "a");
        assert_eq!(request_min.page_size, Some(1));

        assert_eq!(request_max.app_token, "a".repeat(100));
        assert_eq!(request_max.page_size, Some(100));
    }

    #[test]
    fn test_list_app_role_request_different_user_id_types() {
        let open_id_request = ListAppRoleRequest::builder()
            .app_token("app-token")
            .user_id_type("open_id")
            .build();

        let user_id_request = ListAppRoleRequest::builder()
            .app_token("app-token")
            .user_id_type("user_id")
            .build();

        let union_id_request = ListAppRoleRequest::builder()
            .app_token("app-token")
            .user_id_type("union_id")
            .build();

        assert_eq!(open_id_request.user_id_type, Some("open_id".to_string()));
        assert_eq!(user_id_request.user_id_type, Some("user_id".to_string()));
        assert_eq!(union_id_request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_list_app_role_request_empty_strings() {
        let request = ListAppRoleRequest::builder()
            .app_token("")
            .page_token("")
            .build();

        assert_eq!(request.app_token, "");
        assert!(request.page_token.is_none()); // 空字符串转换为None
        assert!(request.page_size.is_none());
    }

    #[test]
    fn test_app_role_serialization() {
        let role = AppRole {
            role_id: "rol123".to_string(),
            role_name: "测试角色".to_string(),
            table_roles: None,
            block_roles: None,
        };

        let serialized = serde_json::to_value(&role).unwrap();
        assert_eq!(serialized["role_id"], "rol123");
        assert_eq!(serialized["role_name"], "测试角色");
        assert!(!serialized.contains_key("table_roles"));
        assert!(!serialized.contains_key("block_roles"));
    }

    #[test]
    fn test_table_role_serialization() {
        let table_role = TableRole {
            table_id: "tbl123".to_string(),
            role: "editor".to_string(),
            rec_rule: Some("all".to_string()),
        };

        let serialized = serde_json::to_value(&table_role).unwrap();
        assert_eq!(serialized["table_id"], "tbl123");
        assert_eq!(serialized["role"], "editor");
        assert_eq!(serialized["rec_rule"], "all");
    }

    #[test]
    fn test_list_app_role_request_with_long_values() {
        let long_app_token = "bascn".repeat(20);
        let long_page_token = "page".repeat(10);

        let request = ListAppRoleRequest::builder()
            .app_token(&long_app_token)
            .page_token(&long_page_token)
            .page_size(50)
            .build();

        assert_eq!(request.app_token, long_app_token);
        assert_eq!(request.page_token, Some(long_page_token));
        assert_eq!(request.page_size, Some(50));
    }

    #[test]
    fn test_list_app_role_response_empty_items() {
        let response = ListAppRoleResponse {
            has_more: false,
            page_token: None,
            items: vec![],
        };

        assert_eq!(response.has_more, false);
        assert!(response.page_token.is_none());
        assert_eq!(response.items.len(), 0);
    }

    #[test]
    fn test_list_app_role_request_page_size_zero() {
        let request = ListAppRoleRequest::builder()
            .app_token("test-token")
            .page_size(0)
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.page_size, Some(0));
    }
}