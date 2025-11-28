//! 列出自定义角色模块

use openlark_core::{
    core::{
        BaseResponse,
        ResponseFormat,
        api::ApiResponseTrait,
    },
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 列出自定义角色请求
#[derive(Clone)]
pub struct ListRolesRequest {
    api_request: openlark_core::api::ApiRequest,
    /// 应用的 app_token
    pub app_token: String,
    /// 用户 ID 类型
    pub user_id_type: Option<String>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ListRolesRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                reqwest::Method::GET,
                BASE_V2_APP_ROLE_LIST.to_string(),
            ),
            app_token: String::new(),
            user_id_type: None,
            page_size: None,
            page_token: None,
        }
    }

    pub fn builder() -> ListRolesRequestBuilder {
        ListRolesRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ListRolesRequestBuilder {
    request: ListRolesRequest,
}

impl ListRolesRequestBuilder {
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

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size.min(100)); // 限制最大100
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    pub fn build(self) -> ListRolesRequest {
        self.request
    }
}

/// 角色信息
#[derive(Clone, Serialize, Deserialize)]
pub struct RoleInfo {
    /// 角色的ID
    pub role_id: String,
    /// 角色名称
    pub name: String,
    /// 角色描述
    pub description: Option<String>,
    /// 角色权限范围
    pub permission_scopes: Option<Vec<String>>,
    /// 是否为系统角色
    pub is_system: Option<bool>,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

/// 列出自定义角色响应
#[derive(Clone)]
pub struct ListRolesResponse {
    /// 角色列表
    pub items: Option<Vec<RoleInfo>>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
    /// 总数量
    pub total: Option<i32>,
}

impl ApiResponseTrait for ListRolesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_roles_request_builder() {
        let request = ListRolesRequest::builder()
            .app_token("bascnxxxxxxxxxxxxxxx")
            .user_id_type("user_id")
            .page_size(50)
            .page_token("page_token")
            .build();

        assert_eq!(request.app_token, "bascnxxxxxxxxxxxxxxx");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("page_token".to_string()));
    }

    #[test]
    fn test_page_size_limit() {
        let request = ListRolesRequest::builder()
            .app_token("bascnxxxxxxxxxxxxxxx")
            .page_size(200) // 超过100的限制
            .build();

        assert_eq!(request.page_size, Some(100)); // 应该被限制为100
    }

    #[test]
    fn test_role_info_serialization() {
        let role = RoleInfo {
            role_id: "rolxxxxxxxxxxxxxxx".to_string(),
            name: "管理员".to_string(),
            description: Some("系统管理员角色".to_string()),
            permission_scopes: Some(vec!["read".to_string(), "write".to_string()]),
            is_system: Some(true),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            updated_at: "2023-01-01T00:00:00Z".to_string(),
        };

        let serialized = serde_json::to_value(&role).unwrap();
        let expected = serde_json::json!({
            "role_id": "rolxxxxxxxxxxxxxxx",
            "name": "管理员",
            "description": "系统管理员角色",
            "permission_scopes": ["read", "write"],
            "is_system": true,
            "created_at": "2023-01-01T00:00:00Z",
            "updated_at": "2023-01-01T00:00:00Z"
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_optional_fields_serialization() {
        // 测试可选字段
        let role = RoleInfo {
            role_id: "rolxxxxxxxxxxxxxxx".to_string(),
            name: "测试角色".to_string(),
            description: None,
            permission_scopes: None,
            is_system: None,
            created_at: "2023-01-01T00:00:00Z".to_string(),
            updated_at: "2023-01-01T00:00:00Z".to_string(),
        };

        let serialized = serde_json::to_value(&role).unwrap();
        let expected = serde_json::json!({
            "role_id": "rolxxxxxxxxxxxxxxx",
            "name": "测试角色",
            "created_at": "2023-01-01T00:00:00Z",
            "updated_at": "2023-01-01T00:00:00Z"
        });

        assert_eq!(serialized, expected);
    }
}