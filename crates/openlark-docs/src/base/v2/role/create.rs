//! 新增自定义角色模块

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

/// 新增自定义角色请求
#[derive(Clone)]
pub struct CreateRoleRequest {
    api_request: openlark_core::api::ApiRequest,
    /// 应用的 app_token
    pub app_token: String,
    /// 角色名称
    pub name: String,
    /// 角色描述
    pub description: Option<String>,
    /// 角色权限范围
    pub permission_scopes: Option<Vec<String>>,
}

impl CreateRoleRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                reqwest::Method::POST,
                BASE_V2_APP_ROLE_CREATE.to_string(),
            ),
            app_token: String::new(),
            name: String::new(),
            description: None,
            permission_scopes: None,
        }
    }

    pub fn builder() -> CreateRoleRequestBuilder {
        CreateRoleRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateRoleRequestBuilder {
    request: CreateRoleRequest,
}

impl CreateRoleRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = name.into();
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

    pub fn build(self) -> CreateRoleRequest {
        self.request
    }
}

/// 新增自定义角色请求体
#[derive(Serialize)]
struct CreateRoleRequestBody {
    /// 角色名称
    name: String,
    /// 角色描述
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// 角色权限范围
    #[serde(skip_serializing_if = "Option::is_none")]
    permission_scopes: Option<Vec<String>>,
}

/// 新增自定义角色响应
#[derive(Clone)]
pub struct CreateRoleResponse {
    /// 创建的角色信息
    pub role: CreateRoleResponseData,
}

/// 新增自定义角色响应数据
#[derive(Clone)]
pub struct CreateRoleResponseData {
    /// 角色的ID
    pub role_id: String,
    /// 角色名称
    pub name: String,
    /// 角色描述
    pub description: Option<String>,
    /// 角色权限范围
    pub permission_scopes: Option<Vec<String>>,
    /// 创建时间
    pub created_at: String,
}

impl ApiResponseTrait for CreateRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_role_request_builder() {
        let permission_scopes = vec!["read", "write"];

        let request = CreateRoleRequest::builder()
            .app_token("bascnxxxxxxxxxxxxxxx")
            .name("自定义角色")
            .description("这是一个自定义角色")
            .permission_scopes(permission_scopes)
            .build();

        assert_eq!(request.app_token, "bascnxxxxxxxxxxxxxxx");
        assert_eq!(request.name, "自定义角色");
        assert_eq!(request.description, Some("这是一个自定义角色".to_string()));
        assert_eq!(request.permission_scopes, Some(vec!["read", "write"]));
    }

    #[test]
    fn test_create_role_request_body_serialization() {
        let body = CreateRoleRequestBody {
            name: "自定义角色".to_string(),
            description: Some("这是一个自定义角色".to_string()),
            permission_scopes: Some(vec!["read".to_string(), "write".to_string()]),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = serde_json::json!({
            "name": "自定义角色",
            "description": "这是一个自定义角色",
            "permission_scopes": ["read", "write"]
        });

        assert_eq!(serialized, expected);
    }
}