//! 删除应用模块
//!
//! 提供多维表格的删除功能。

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

/// 删除应用请求
#[derive(Clone)]
pub struct DeleteAppRequest {
    api_request: openlark_core::api::ApiRequest,
    /// 多维表格的 app_token
    pub app_token: String,
}

impl DeleteAppRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                reqwest::Method::DELETE,
                DELETE_APP.to_string(),
            ),
            app_token: String::new(),
        }
    }

    pub fn builder() -> DeleteAppRequestBuilder {
        DeleteAppRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteAppRequestBuilder {
    request: DeleteAppRequest,
}

impl DeleteAppRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn build(self) -> DeleteAppRequest {
        self.request
    }
}

/// 删除应用响应
#[derive(Clone)]
pub struct DeleteAppResponse {
    /// 是否成功删除
    pub success: bool,
    /// 删除的应用信息
    pub app: DeleteAppInfo,
}

/// 删除应用信息
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteAppInfo {
    /// 多维表格的 app_token
    pub app_token: String,
    /// 删除时间
    pub deleted_time: String,
    /// 多维表格名称（删除前）
    pub name: Option<String>,
}

impl ApiResponseTrait for DeleteAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_app_request_builder() {
        let request = DeleteAppRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
    }

    #[test]
    fn test_delete_app_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = DeleteAppRequest::new(config);

        assert_eq!(request.app_token, "");
    }

    #[test]
    fn test_delete_app_response_creation() {
        let app_info = DeleteAppInfo {
            app_token: "bascnmBA*****yGehy8".to_string(),
            deleted_time: "2023-12-01T10:00:00Z".to_string(),
            name: Some("测试表格".to_string()),
        };

        let response = DeleteAppResponse {
            success: true,
            app: app_info,
        };

        assert_eq!(response.success, true);
        assert_eq!(response.app.app_token, "bascnmBA*****yGehy8");
        assert_eq!(response.app.deleted_time, "2023-12-01T10:00:00Z");
        assert_eq!(response.app.name, Some("测试表格".to_string()));
    }

    #[test]
    fn test_delete_app_info_serialization() {
        let app_info = DeleteAppInfo {
            app_token: "app_token_123".to_string(),
            deleted_time: "2023-01-01T00:00:00Z".to_string(),
            name: Some("示例多维表格".to_string()),
        };

        let serialized = serde_json::to_value(&app_info).unwrap();
        let expected = serde_json::json!({
            "app_token": "app_token_123",
            "deleted_time": "2023-01-01T00:00:00Z",
            "name": "示例多维表格"
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_delete_app_info_serialization_without_name() {
        let app_info = DeleteAppInfo {
            app_token: "app_token_456".to_string(),
            deleted_time: "2023-01-01T00:00:00Z".to_string(),
            name: None,
        };

        let serialized = serde_json::to_value(&app_info).unwrap();
        let expected = serde_json::json!({
            "app_token": "app_token_456",
            "deleted_time": "2023-01-01T00:00:00Z"
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_delete_app_response_serialization() {
        let response = DeleteAppResponse {
            success: true,
            app: DeleteAppInfo {
                app_token: "test_app".to_string(),
                deleted_time: "2023-06-15T14:30:00Z".to_string(),
                name: Some("重要项目表".to_string()),
            },
        };

        let serialized = serde_json::to_value(&response).unwrap();
        let expected = serde_json::json!({
            "success": true,
            "app": {
                "app_token": "test_app",
                "deleted_time": "2023-06-15T14:30:00Z",
                "name": "重要项目表"
            }
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_delete_app_request_builder_chaining() {
        // 测试构建器方法链式调用
        let request = DeleteAppRequest::builder()
            .app_token("app_token_789")
            .build();

        assert_eq!(request.app_token, "app_token_789");
    }

    #[test]
    fn test_api_response_trait() {
        // 测试API响应特征实现
        assert_eq!(DeleteAppResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_delete_app_request_with_different_tokens() {
        // 测试不同app_token的处理
        let tokens = vec![
            "token_with_123",
            "token_ABC_DEF",
            "token_with-special_chars",
            "simpletoken"
        ];

        for token in tokens {
            let request = DeleteAppRequest::builder()
                .app_token(token)
                .build();
            assert_eq!(request.app_token, token);
        }
    }

    #[test]
    fn test_delete_app_info_edge_cases() {
        // 测试边界情况
        let app_info_empty = DeleteAppInfo {
            app_token: "".to_string(),
            deleted_time: "".to_string(),
            name: Some("".to_string()),
        };

        let app_info_long = DeleteAppInfo {
            app_token: "a".repeat(1000),
            deleted_time: "2023-01-01T00:00:00Z".to_string(),
            name: Some("很长的表格名称".repeat(100)),
        };

        assert_eq!(app_info_empty.app_token, "");
        assert_eq!(app_info_long.app_token.len(), 1000);
    }
}