//! Bitable V1 批量新增协作者API

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

/// 批量新增协作者请求
#[derive(Clone)]
pub struct BatchCreateCollaboratorV1Request {
    api_request: ApiRequest,
    app_token: String,
    user_ids: Vec<String>,
}

/// 批量新增协作者响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateCollaboratorV1Response {
    pub data: BatchCreateCollaboratorV1Data,
    pub success: bool,
}

/// 批量新增协作者响应数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateCollaboratorV1Data {
    /// 成功添加的用户ID列表
    pub success_user_ids: Vec<String>,
    /// 失败的用户ID列表
    pub failed_user_ids: Vec<String>,
    /// 操作结果详情
    pub results: Vec<CollaboratorResult>,
}

/// 协作者操作结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CollaboratorResult {
    /// 用户ID
    pub user_id: String,
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error_message: Option<String>,
}

impl BatchCreateCollaboratorV1Request {
    /// 创建批量新增协作者请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config),
            app_token: String::new(),
            user_ids: Vec::new(),
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置用户ID列表
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = user_ids;
        self
    }

    /// 添加用户ID
    pub fn add_user_id(mut self, user_id: String) -> Self {
        self.user_ids.push(user_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchCreateCollaboratorV1Response> {
        // 构建API路径
        let path = format!("/open-apis/bitable/v1/apps/{}/collaborators/batch_create", self.app_token);

        // 验证请求参数
        if self.app_token.is_empty() {
            return Err(openlark_core::error::SDKError::ValidationError("应用token不能为空".to_string()));
        }
        if self.user_ids.is_empty() {
            return Err(openlark_core::error::SDKError::ValidationError("用户ID列表不能为空".to_string()));
        }
        if self.user_ids.len() > 100 {
            return Err(openlark_core::error::SDKError::ValidationError("用户ID列表不能超过100个".to_string()));
        }

        // 构建请求体
        let request_body = serde_json::json!({
            "user_ids": self.user_ids
        });

        // 发送请求
        let response = self.api_request
            .method(&openlark_core::http::Method::POST)
            .path(&path)
            .body(request_body)
            .execute::<BatchCreateCollaboratorV1Response>()
            .await?;

        Ok(response)
    }
}

/// 批量新增协作者Builder
#[derive(Clone)]
pub struct BatchCreateCollaboratorV1Builder {
    request: BatchCreateCollaboratorV1Request,
}

impl BatchCreateCollaboratorV1Builder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchCreateCollaboratorV1Request::new(config),
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置用户ID列表
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.request = self.request.user_ids(user_ids);
        self
    }

    /// 添加用户ID
    pub fn add_user_id(mut self, user_id: String) -> Self {
        self.request = self.request.add_user_id(user_id);
        self
    }

    /// 构建请求
    pub fn build(self) -> BatchCreateCollaboratorV1Request {
        self.request
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::Config;

    #[test]
    fn test_batch_create_collaborator_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = BatchCreateCollaboratorV1Request::builder(config)
            .app_token("bascnmBA*****yGehy8")
            .user_ids(vec!["user_1".to_string(), "user_2".to_string()])
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.user_ids.len(), 2);
        assert_eq!(request.user_ids[0], "user_1");
        assert_eq!(request.user_ids[1], "user_2");
    }

    #[test]
    fn test_batch_create_collaborator_request_new() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = BatchCreateCollaboratorV1Request::new(config)
            .app_token("test_app_token")
            .add_user_id("user_1".to_string())
            .add_user_id("user_2".to_string());

        assert_eq!(request.app_token, "test_app_token");
        assert_eq!(request.user_ids.len(), 2);
    }

    #[test]
    fn test_batch_create_collaborator_response_creation() {
        let success_user_ids = vec!["user_1".to_string(), "user_2".to_string()];
        let failed_user_ids = vec!["user_3".to_string()];
        let results = vec![
            CollaboratorResult {
                user_id: "user_1".to_string(),
                success: true,
                error_message: None,
            },
            CollaboratorResult {
                user_id: "user_2".to_string(),
                success: true,
                error_message: None,
            },
            CollaboratorResult {
                user_id: "user_3".to_string(),
                success: false,
                error_message: Some("用户不存在".to_string()),
            },
        ];

        let response_data = BatchCreateCollaboratorV1Data {
            success_user_ids: success_user_ids.clone(),
            failed_user_ids: failed_user_ids.clone(),
            results: results.clone(),
        };

        let response = BatchCreateCollaboratorV1Response {
            data: response_data,
            success: true,
        };

        assert_eq!(response.success, true);
        assert_eq!(response.data.success_user_ids, success_user_ids);
        assert_eq!(response.data.failed_user_ids, failed_user_ids);
        assert_eq!(response.data.results.len(), 3);
    }
}