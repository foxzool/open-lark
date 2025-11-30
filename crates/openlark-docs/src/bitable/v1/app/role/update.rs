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
use serde_json::Value;

/// 更新自定义角色请求
#[derive(Clone)]
pub struct UpdateAppRoleRequest {
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
    /// 角色名称
    #[serde(skip_serializing_if = "Option::is_none")]
    role_name: Option<String>,
    /// 数据表权限
    #[serde(skip_serializing_if = "Option::is_none")]
    table_roles: Option<Vec<TableRole>>,
    /// 数据表默认权限
    #[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>,
}

impl UpdateAppRoleRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::PATCH, "/open-apis/bitable/v1/apps/{}/roles/{}".to_string()),
            app_token: String::new(),
            role_id: String::new(),
            user_id_type: None,
            role_name: None,
            table_roles: None,
            block_roles: None,
        }
    }

    pub fn builder() -> UpdateAppRoleRequestBuilder {
        UpdateAppRoleRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateAppRoleRequestBuilder {
    request: UpdateAppRoleRequest,
}

impl UpdateAppRoleRequestBuilder {
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

    pub fn role_name(mut self, role_name: impl Into<String>) -> Self {
        self.request.role_name = Some(role_name.into());
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

    pub fn build(self) -> UpdateAppRoleRequest {
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

/// 请求体结构
#[derive(Serialize)]
struct UpdateAppRoleRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    role_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_roles: Option<Vec<TableRole>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>,
}

/// 更新自定义角色响应
#[derive(Clone)]
pub struct UpdateAppRoleResponse {
    /// 更新后的角色信息
    pub role: AppRole,
}

impl ApiResponseTrait for UpdateAppRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新自定义角色
pub async fn update_app_role(
    request: UpdateAppRoleRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<UpdateAppRoleResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::PATCH);
    api_req.api_path = BITABLE_V1_APP_ROLE_UPDATE
        .replace("{app_token}", &request.app_token)
        .replace("{role_id}", &request.role_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert("user_id_type".to_string(), user_id_type.clone());
    }

    // 设置请求体
    let body = UpdateAppRoleRequestBody {
        role_name: request.role_name,
        table_roles: request.table_roles,
        block_roles: request.block_roles,
    };

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp: openlark_core::core::StandardResponse<UpdateAppRoleResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_app_role_request_builder() {
        let table_roles = vec![
            TableRole::new("tblxxxxxx", "editor")
                .with_rec_rule("all")
        ];

        let request = UpdateAppRoleRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .role_id("rolxxxxxx")
            .role_name("更新的角色名称")
            .table_roles(table_roles)
            .user_id_type("open_id")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_id, "rolxxxxxx");
        assert_eq!(request.role_name, Some("更新的角色名称".to_string()));
        assert!(request.table_roles.is_some());
        assert_eq!(request.table_roles.as_ref().unwrap().len(), 1);
        assert_eq!(request.table_roles.as_ref().unwrap()[0].table_id, "tblxxxxxx");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_update_app_role_request_minimal() {
        let request = UpdateAppRoleRequest::builder()
            .app_token("test-token")
            .role_id("test-role")
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.role_id, "test-role");
        assert!(request.role_name.is_none());
        assert!(request.table_roles.is_none());
        assert!(request.block_roles.is_none());
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_update_app_role_response_trait() {
        assert_eq!(UpdateAppRoleResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_update_app_role_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = UpdateAppRoleRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.role_id, "");
        assert!(request.role_name.is_none());
        assert!(request.table_roles.is_none());
        assert!(request.block_roles.is_none());
    }

    #[test]
    fn test_update_app_role_request_with_block_roles() {
        let block_roles = vec![
            BlockRole::new("block123", "reader"),
            BlockRole::new("block456", "commenter")
        ];

        let request = UpdateAppRoleRequest::builder()
            .app_token("test-token")
            .role_id("test-role")
            .role_name("测试角色")
            .block_roles(block_roles)
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.role_id, "test-role");
        assert_eq!(request.role_name, Some("测试角色".to_string()));
        assert!(request.block_roles.is_some());
        assert_eq!(request.block_roles.as_ref().unwrap().len(), 2);
        assert_eq!(request.block_roles.as_ref().unwrap()[0].block_id, "block123");
        assert_eq!(request.block_roles.as_ref().unwrap()[1].role, "commenter");
    }

    #[test]
    fn test_update_app_role_request_builder_chaining() {
        let table_roles = vec![TableRole::new("tbl1", "editor")];
        let block_roles = vec![BlockRole::new("block1", "reader")];

        let request = UpdateAppRoleRequest::builder()
            .app_token("app123")
            .role_id("role123")
            .role_name("链式角色")
            .table_roles(table_roles)
            .block_roles(block_roles)
            .user_id_type("user_id")
            .build();

        assert_eq!(request.app_token, "app123");
        assert_eq!(request.role_id, "role123");
        assert_eq!(request.role_name, Some("链式角色".to_string()));
        assert!(request.table_roles.is_some());
        assert!(request.block_roles.is_some());
    }

    #[test]
    fn test_update_app_role_request_empty_collections() {
        let request = UpdateAppRoleRequest::builder()
            .app_token("test-token")
            .role_id("test-role")
            .table_roles(vec![])
            .block_roles(vec![])
            .build();

        assert!(request.table_roles.is_some());
        assert!(request.block_roles.is_some());
        assert_eq!(request.table_roles.as_ref().unwrap().len(), 0);
        assert_eq!(request.block_roles.as_ref().unwrap().len(), 0);
    }

    #[test]
    fn test_update_app_role_request_long_ids() {
        let long_app_token = "bascn".repeat(20);
        let long_role_id = "rol".repeat(15);
        let long_role_name = "角色名称".repeat(10);

        let request = UpdateAppRoleRequest::builder()
            .app_token(&long_app_token)
            .role_id(&long_role_id)
            .role_name(&long_role_name)
            .build();

        assert_eq!(request.app_token, long_app_token);
        assert_eq!(request.role_id, long_role_id);
        assert_eq!(request.role_name, Some(long_role_name));
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
    fn test_block_role_builder() {
        let block_role = BlockRole::new("block789", "commenter");

        assert_eq!(block_role.block_id, "block789");
        assert_eq!(block_role.role, "commenter");
    }

    #[test]
    fn test_update_app_role_request_different_user_id_types() {
        let open_id_request = UpdateAppRoleRequest::builder()
            .app_token("app-token")
            .role_id("role-id")
            .user_id_type("open_id")
            .build();

        let user_id_request = UpdateAppRoleRequest::builder()
            .app_token("app-token")
            .role_id("role-id")
            .user_id_type("user_id")
            .build();

        let union_id_request = UpdateAppRoleRequest::builder()
            .app_token("app-token")
            .role_id("role-id")
            .user_id_type("union_id")
            .build();

        assert_eq!(open_id_request.user_id_type, Some("open_id".to_string()));
        assert_eq!(user_id_request.user_id_type, Some("user_id".to_string()));
        assert_eq!(union_id_request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_update_app_role_request_empty_strings() {
        let request = UpdateAppRoleRequest::builder()
            .app_token("")
            .role_id("")
            .role_name("")
            .build();

        assert_eq!(request.app_token, "");
        assert_eq!(request.role_id, "");
        assert!(request.role_name.is_none()); // 空字符串转换为None
    }

    #[test]
    fn test_update_app_role_request_serialization() {
        let table_roles = vec![TableRole::new("tbl123", "editor")];
        let request = UpdateAppRoleRequest::builder()
            .app_token("test")
            .role_id("test")
            .role_name("test_role")
            .table_roles(table_roles)
            .build();

        // 测试请求体序列化
        let body = UpdateAppRoleRequestBody {
            role_name: request.role_name,
            table_roles: request.table_roles,
            block_roles: request.block_roles,
        };

        let serialized = serde_json::to_value(&body).unwrap();
        assert_eq!(serialized["role_name"], "test_role");
        assert_eq!(serialized["table_roles"].as_array().unwrap().len(), 1);
        assert_eq!(serialized["table_roles"][0]["table_id"], "tbl123");
    }
}