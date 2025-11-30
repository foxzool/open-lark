//! Bitable V1 新增协作者API

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

/// 新增协作者请求
#[derive(Clone)]
pub struct CreateCollaboratorV1Request {
    api_request: ApiRequest,
    app_token: String,
    user_id: String,
    perm: String,
}

/// 新增协作者响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCollaboratorV1Response {
    pub data: CreateCollaboratorV1Data,
    pub success: bool,
}

/// 新增协作者响应数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCollaboratorV1Data {
    /// 用户ID
    pub user_id: String,
    /// 是否成功添加
    pub success: bool,
    /// 错误信息
    pub error_message: Option<String>,
}

impl CreateCollaboratorV1Request {
    /// 创建新增协作者请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config),
            app_token: String::new(),
            user_id: String::new(),
            perm: String::new(),
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置用户ID
    pub fn user_id(mut self, user_id: String) -> Self {
        self.user_id = user_id;
        self
    }

    /// 设置权限
    pub fn perm(mut self, perm: String) -> Self {
        self.perm = perm;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateCollaboratorV1Response> {
        // 构建API路径
        let path = format!("/open-apis/bitable/v1/apps/{}/collaborators", self.app_token);

        // 验证请求参数
        if self.app_token.is_empty() {
            return Err(openlark_core::error::SDKError::ValidationError("应用token不能为空".to_string()));
        }
        if self.user_id.is_empty() {
            return Err(openlark_core::error::SDKError::ValidationError("用户ID不能为空".to_string()));
        }
        if self.perm.is_empty() {
            return Err(openlark_core::error::SDKError::ValidationError("权限不能为空".to_string()));
        }

        // 构建请求体
        let request_body = serde_json::json!({
            "user_id": self.user_id,
            "perm": self.perm
        });

        // 发送请求
        let response = self.api_request
            .method(&openlark_core::http::Method::POST)
            .path(&path)
            .body(request_body)
            .execute::<CreateCollaboratorV1Response>()
            .await?;

        Ok(response)
    }
}

/// 新增协作者Builder
#[derive(Clone)]
pub struct CreateCollaboratorV1Builder {
    request: CreateCollaboratorV1Request,
}

impl CreateCollaboratorV1Builder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: CreateCollaboratorV1Request::new(config),
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置用户ID
    pub fn user_id(mut self, user_id: String) -> Self {
        self.request = self.request.user_id(user_id);
        self
    }

    /// 设置权限
    pub fn perm(mut self, perm: String) -> Self {
        self.request = self.request.perm(perm);
        self
    }

    /// 构建请求
    pub fn build(self) -> CreateCollaboratorV1Request {
        self.request
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::Config;

    #[test]
    fn test_create_collaborator_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = CreateCollaboratorV1Request::builder(config)
            .app_token("bascnmBA*****yGehy8")
            .user_id("user_123".to_string())
            .perm("view".to_string())
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.user_id, "user_123");
        assert_eq!(request.perm, "view");
    }

    #[test]
    fn test_create_collaborator_request_new() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = CreateCollaboratorV1Request::new(config)
            .app_token("test_app_token")
            .user_id("test_user_id")
            .perm("edit");

        assert_eq!(request.app_token, "test_app_token");
        assert_eq!(request.user_id, "test_user_id");
        assert_eq!(request.perm, "edit");
    }

    #[test]
    fn test_create_collaborator_response_creation() {
        let response_data = CreateCollaboratorV1Data {
            user_id: "user_123".to_string(),
            success: true,
            error_message: None,
        };

        let response = CreateCollaboratorV1Response {
            data: response_data,
            success: true,
        };

        assert_eq!(response.success, true);
        assert_eq!(response.data.user_id, "user_123");
        assert_eq!(response.data.success, true);
        assert!(response.data.error_message.is_none());
    }
}