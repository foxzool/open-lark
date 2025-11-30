#![allow(unused_variables, unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

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

/// 新增自定义角色请求
#[derive(Clone)]
pub struct CreateAppRoleRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 角色名称
    role_name: String,
    /// 数据表权限
    #[serde(skip_serializing_if = "Option::is_none")]
    table_roles: Option<Vec<TableRole>>,
    /// 数据表默认权限
    #[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>,
}

impl CreateAppRoleRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::POST, "/open-apis/bitable/v1/apps/{}/roles".to_string()),
            app_token: String::new(),
            role_name: String::new(),
            table_roles: None,
            block_roles: None,
        }
    }

    pub fn builder() -> CreateAppRoleRequestBuilder {
        CreateAppRoleRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateAppRoleRequestBuilder {
    request: CreateAppRoleRequest,
}

impl CreateAppRoleRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn role_name(mut self, role_name: impl Into<String>) -> Self {
        self.request.role_name = role_name.into();
        self
    }

    pub fn table_roles(mut self, table_roles: Vec<TableRole>) -> Self {
        self.request.table_roles = Some(table_roles);
        self
    }

    pub fn block_roles(mut self, block_roles: Vec<BlockRole>) -> Self {
        self.request.block_roles = Some(block_roles);
        self
    }

    pub fn build(self) -> CreateAppRoleRequest {
        self.request
    }
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

impl TableRole {
    pub fn new(table_id: impl ToString, role: impl ToString) -> Self {
        Self {
            table_id: table_id.to_string(),
            role: role.to_string(),
            rec_rule: None,
        }
    }

    pub fn with_rec_rule(mut self, rec_rule: impl ToString) -> Self {
        self.rec_rule = Some(rec_rule.to_string());
        self
    }
}

/// 数据表默认权限
#[derive(Clone, Serialize, Deserialize)]
pub struct BlockRole {
    /// 多维表格数据表的唯一标识符
    pub block_id: String,
    /// 权限
    pub role: String,
}

impl BlockRole {
    pub fn new(block_id: impl ToString, role: impl ToString) -> Self {
        Self {
            block_id: block_id.to_string(),
            role: role.to_string(),
        }
    }
}

/// 请求体结构
#[derive(Serialize)]
struct CreateAppRoleRequestBody {
    role_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_roles: Option<Vec<TableRole>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>,
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

/// 新增自定义角色响应
#[derive(Clone)]
pub struct CreateAppRoleResponse {
    /// 新增的自定义角色信息
    pub role: AppRole,
}

impl ApiResponseTrait for CreateAppRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新增自定义角色
pub async fn create_app_role(
    request: CreateAppRoleRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<CreateAppRoleResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
    api_req.api_path = BITABLE_V1_APP_ROLE_CREATE
        .replace("{app_token}", &request.app_token);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    // 设置请求体
    let body = CreateAppRoleRequestBody {
        role_name: request.role_name,
        table_roles: request.table_roles,
        block_roles: request.block_roles,
    };

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp: openlark_core::core::StandardResponse<CreateAppRoleResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_app_role_request_builder() {
        let table_roles = vec![
            TableRole::new("tblxxxxxx", "editor")
                .with_rec_rule("all")
        ];

        let request = CreateAppRoleRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .role_name("测试自定义角色")
            .table_roles(table_roles)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_name, "测试自定义角色");
        assert!(request.table_roles.is_some());
        assert_eq!(request.table_roles.as_ref().unwrap().len(), 1);
        assert_eq!(request.table_roles.as_ref().unwrap()[0].table_id, "tblxxxxxx");
        assert_eq!(request.table_roles.as_ref().unwrap()[0].role, "editor");
        assert_eq!(request.table_roles.as_ref().unwrap()[0].rec_rule, Some("all".to_string()));
    }

    #[test]
    fn test_create_app_role_request_with_block_roles() {
        let block_roles = vec![
            BlockRole::new("block123", "reader"),
            BlockRole::new("block456", "commenter")
        ];

        let request = CreateAppRoleRequest::builder()
            .app_token("test-token")
            .role_name("测试角色")
            .block_roles(block_roles)
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.role_name, "测试角色");
        assert!(request.block_roles.is_some());
        assert_eq!(request.block_roles.as_ref().unwrap().len(), 2);
        assert_eq!(request.block_roles.as_ref().unwrap()[0].block_id, "block123");
        assert_eq!(request.block_roles.as_ref().unwrap()[1].role, "commenter");
    }

    #[test]
    fn test_create_app_role_request_minimal() {
        let request = CreateAppRoleRequest::builder()
            .app_token("test-token")
            .role_name("最小角色")
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.role_name, "最小角色");
        assert!(request.table_roles.is_none());
        assert!(request.block_roles.is_none());
    }

    #[test]
    fn test_create_app_role_response_trait() {
        assert_eq!(CreateAppRoleResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_create_app_role_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = CreateAppRoleRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.role_name, "");
        assert!(request.table_roles.is_none());
        assert!(request.block_roles.is_none());
    }

    #[test]
    fn test_table_role_builder() {
        let table_role = TableRole::new("tbl123", "editor")
            .with_rec_rule("selected");

        assert_eq!(table_role.table_id, "tbl123");
        assert_eq!(table_role.role, "editor");
        assert_eq!(table_role.rec_rule, Some("selected".to_string()));
    }

    #[test]
    fn test_table_role_minimal() {
        let table_role = TableRole::new("tbl456", "reader");

        assert_eq!(table_role.table_id, "tbl456");
        assert_eq!(table_role.role, "reader");
        assert!(table_role.rec_rule.is_none());
    }

    #[test]
    fn test_block_role_builder() {
        let block_role = BlockRole::new("block789", "commenter");

        assert_eq!(block_role.block_id, "block789");
        assert_eq!(block_role.role, "commenter");
    }

    #[test]
    fn test_create_app_role_request_builder_chaining() {
        let table_roles = vec![TableRole::new("tbl1", "editor")];
        let block_roles = vec![BlockRole::new("block1", "reader")];

        let request = CreateAppRoleRequest::builder()
            .app_token("app123")
            .role_name("链式角色")
            .table_roles(table_roles)
            .block_roles(block_roles)
            .build();

        assert_eq!(request.app_token, "app123");
        assert_eq!(request.role_name, "链式角色");
        assert!(request.table_roles.is_some());
        assert!(request.block_roles.is_some());
    }

    #[test]
    fn test_create_app_role_request_empty_collections() {
        let request = CreateAppRoleRequest::builder()
            .app_token("test-token")
            .role_name("测试角色")
            .table_roles(vec![])
            .block_roles(vec![])
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.role_name, "测试角色");
        assert!(request.table_roles.is_some());
        assert!(request.block_roles.is_some());
        assert_eq!(request.table_roles.as_ref().unwrap().len(), 0);
        assert_eq!(request.block_roles.as_ref().unwrap().len(), 0);
    }

    #[test]
    fn test_create_app_role_request_long_ids() {
        let long_app_token = "bascn".repeat(20);
        let long_role_name = "角色名称".repeat(10);
        let long_table_id = "tbl".repeat(15);
        let long_block_id = "block".repeat(10);

        let table_roles = vec![TableRole::new(&long_table_id, "editor")];
        let block_roles = vec![BlockRole::new(&long_block_id, "reader")];

        let request = CreateAppRoleRequest::builder()
            .app_token(&long_app_token)
            .role_name(&long_role_name)
            .table_roles(table_roles)
            .block_roles(block_roles)
            .build();

        assert_eq!(request.app_token, long_app_token);
        assert_eq!(request.role_name, long_role_name);
        assert_eq!(request.table_roles.as_ref().unwrap()[0].table_id, long_table_id);
        assert_eq!(request.block_roles.as_ref().unwrap()[0].block_id, long_block_id);
    }

    #[test]
    fn test_table_role_serialization() {
        let table_role = TableRole::new("tbl123", "editor")
            .with_rec_rule("all");

        let serialized = serde_json::to_value(&table_role).unwrap();

        assert_eq!(serialized["table_id"], "tbl123");
        assert_eq!(serialized["role"], "editor");
        assert_eq!(serialized["rec_rule"], "all");
    }

    #[test]
    fn test_block_role_serialization() {
        let block_role = BlockRole::new("block456", "reader");

        let serialized = serde_json::to_value(&block_role).unwrap();

        assert_eq!(serialized["block_id"], "block456");
        assert_eq!(serialized["role"], "reader");
    }
}