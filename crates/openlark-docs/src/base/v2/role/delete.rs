//! Base V2 删除自定义角色API

#![allow(unused_variables, unused_imports, dead_code, non_snake_case)]
#![allow(clippy::too_many_arguments)]

use openlark_core::{
    api::ApiRequest,
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::{RoleService};

/// 删除自定义角色请求
#[derive(Clone)]
pub struct DeleteRoleV2Request {
    api_request: ApiRequest,
    app_token: String,
    role_id: String,
}

/// 删除自定义角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteRoleV2Response {
    pub data: DeleteRoleV2Data,
    pub success: bool,
}

/// 删除角色响应数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteRoleV2Data {
    /// 角色ID
    pub role_id: String,
    /// 删除时间
    pub deleted_time: String,
    /// 是否成功删除
    pub success: bool,
}

impl DeleteRoleV2Request {
    /// 创建删除自定义角色请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config),
            app_token: String::new(),
            role_id: String::new(),
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置角色 ID
    pub fn role_id(mut self, role_id: String) -> Self {
        self.role_id = role_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteRoleV2Response> {
        // 构建API路径
        let path = format!("/open-apis/base/v2/apps/{}/roles/{}", self.app_token, self.role_id);

        // 验证请求参数
        if self.app_token.is_empty() {
            return Err(openlark_core::error::SDKError::ValidationError("应用token不能为空".to_string()));
        }
        if self.role_id.is_empty() {
            return Err(openlark_core::error::SDKError::ValidationError("角色ID不能为空".to_string()));
        }

        // 发送请求
        let response = self.api_request
            .method(&openlark_core::http::Method::DELETE)
            .path(&path)
            .execute::<DeleteRoleV2Response>()
            .await?;

        Ok(response)
    }
}

/// 删除自定义角色Builder
#[derive(Clone)]
pub struct DeleteRoleV2Builder {
    request: DeleteRoleV2Request,
}

impl DeleteRoleV2Builder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: DeleteRoleV2Request::new(config),
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置角色 ID
    pub fn role_id(mut self, role_id: String) -> Self {
        self.request = self.request.role_id(role_id);
        self
    }

    /// 构建请求
    pub fn build(self) -> DeleteRoleV2Request {
        self.request
    }
}

impl RoleService {
    /// 创建删除自定义角色请求构建器
    pub fn delete_role_v2_builder(&self, config: Config) -> DeleteRoleV2Builder {
        DeleteRoleV2Builder::new(config)
    }

    /// 创建删除自定义角色请求
    pub fn delete_role_v2(&self, app_token: String, role_id: String) -> DeleteRoleV2Request {
        DeleteRoleV2Request::new(self.config.clone())
            .app_token(app_token)
            .role_id(role_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::Config;

    #[test]
    fn test_delete_role_v2_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = DeleteRoleV2Request::builder(config)
            .app_token("bascnmBA*****yGehy8")
            .role_id("role_123")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_id, "role_123");
    }

    #[test]
    fn test_delete_role_v2_request_new() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = DeleteRoleV2Request::new(config)
            .app_token("test_app_token")
            .role_id("test_role_id");

        assert_eq!(request.app_token, "test_app_token");
        assert_eq!(request.role_id, "test_role_id");
    }

    #[test]
    fn test_delete_role_v2_response_creation() {
        let response_data = DeleteRoleV2Data {
            role_id: "role_123".to_string(),
            deleted_time: "2023-12-01T10:00:00Z".to_string(),
            success: true,
        };

        let response = DeleteRoleV2Response {
            data: response_data,
            success: true,
        };

        assert_eq!(response.success, true);
        assert_eq!(response.data.role_id, "role_123");
        assert_eq!(response.data.deleted_time, "2023-12-01T10:00:00Z");
        assert_eq!(response.data.success, true);
    }

    #[test]
    fn test_delete_role_v2_request_serialization() {
        let request = DeleteRoleV2Request::builder(
            Config::builder()
                .app_id("test_app_id")
                .app_secret("test_app_secret")
                .build()
                .unwrap()
        )
        .app_token("app_token_123")
        .role_id("role_id_456")
        .build();

        assert_eq!(request.app_token, "app_token_123");
        assert_eq!(request.role_id, "role_id_456");
    }

    #[test]
    fn test_delete_role_v2_data_serialization() {
        let data = DeleteRoleV2Data {
            role_id: "role_test".to_string(),
            deleted_time: "2023-01-01T00:00:00Z".to_string(),
            success: true,
        };

        let serialized = serde_json::to_value(&data).unwrap();
        let expected = serde_json::json!({
            "role_id": "role_test",
            "deleted_time": "2023-01-01T00:00:00Z",
            "success": true
        });

        assert_eq!(serialized, expected);
    }
}