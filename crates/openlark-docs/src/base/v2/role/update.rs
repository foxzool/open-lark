//! 更新自定义角色模块

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

/// 更新自定义角色请求
#[derive(Clone)]
pub struct UpdateRoleRequest {
    api_request: openlark_core::api::ApiRequest,
    /// 应用的 app_token
    pub app_token: String,
    /// 角色的ID
    pub role_id: String,
    /// 角色名称
    pub name: Option<String>,
    /// 角色描述
    pub description: Option<String>,
    /// 角色权限范围
    pub permission_scopes: Option<Vec<String>>,
}

impl UpdateRoleRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                reqwest::Method::PUT,
                BASE_V2_APP_ROLE_UPDATE.to_string(),
            ),
            app_token: String::new(),
            role_id: String::new(),
            name: None,
            description: None,
            permission_scopes: None,
        }
    }

    pub fn builder() -> UpdateRoleRequestBuilder {
        UpdateRoleRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateRoleRequestBuilder {
    request: UpdateRoleRequest,
}

impl UpdateRoleRequestBuilder {
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

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = Some(name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn permission_scopes(mut self, permission_scopes: Vec<impl Into<String>>) -> Self {
        self.request.permission_scopes = Some(permission_scopes.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn build(self) -> UpdateRoleRequest {
        self.request
    }
}

/// 更新自定义角色请求体
#[derive(Serialize)]
struct UpdateRoleRequestBody {
    /// 角色名称
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// 角色描述
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// 角色权限范围
    #[serde(skip_serializing_if = "Option::is_none")]
    permission_scopes: Option<Vec<String>>,
}

/// 更新自定义角色响应
#[derive(Clone)]
pub struct UpdateRoleResponse {
    /// 更新的角色信息
    pub role: UpdateRoleResponseData,
}

/// 更新自定义角色响应数据
#[derive(Clone)]
pub struct UpdateRoleResponseData {
    /// 角色的ID
    pub role_id: String,
    /// 角色名称
    pub name: String,
    /// 角色描述
    pub description: Option<String>,
    /// 角色权限范围
    pub permission_scopes: Option<Vec<String>>,
    /// 更新时间
    pub updated_at: String,
}

impl ApiResponseTrait for UpdateRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_role_request_builder() {
        let permission_scopes = vec!["read", "write", "delete"];

        let request = UpdateRoleRequest::builder()
            .app_token("bascnxxxxxxxxxxxxxxx")
            .role_id("rolxxxxxxxxxxxxxxx")
            .name("更新的角色")
            .description("这是一个更新的自定义角色")
            .permission_scopes(permission_scopes)
            .build();

        assert_eq!(request.app_token, "bascnxxxxxxxxxxxxxxx");
        assert_eq!(request.role_id, "rolxxxxxxxxxxxxxxx");
        assert_eq!(request.name, Some("更新的角色".to_string()));
        assert_eq!(request.description, Some("这是一个更新的自定义角色".to_string()));
        assert_eq!(request.permission_scopes, Some(vec!["read", "write", "delete"]));
    }

    #[test]
    fn test_update_role_request_body_serialization() {
        let body = UpdateRoleRequestBody {
            name: Some("更新的角色".to_string()),
            description: Some("这是一个更新的自定义角色".to_string()),
            permission_scopes: Some(vec!["read".to_string(), "write".to_string(), "delete".to_string()]),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = serde_json::json!({
            "name": "更新的角色",
            "description": "这是一个更新的自定义角色",
            "permission_scopes": ["read", "write", "delete"]
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_partial_update() {
        // 测试部分更新
        let body = UpdateRoleRequestBody {
            name: None,
            description: Some("仅更新描述".to_string()),
            permission_scopes: None,
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = serde_json::json!({
            "description": "仅更新描述"
        });

        assert_eq!(serialized, expected);
    }
}