//! 用户访问令牌服务 (Resource: access_token)
//!
//! 提供用户访问令牌的获取功能。

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::models::{AuthConfig, AuthError, AuthResult};

/// 用户访问令牌服务
#[derive(Debug)]
pub struct AccessTokenService {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
}

impl AccessTokenService {
    /// 创建新的用户访问令牌服务
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// 创建用户访问令牌
    pub fn create(&self) -> AccessTokenBuilder {
        AccessTokenBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            grant_type: String::new(),
            code: String::new(),
        }
    }
}

/// 用户访问令牌构建器
#[derive(Debug)]
pub struct AccessTokenBuilder {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
    grant_type: String,
    code: String,
}

impl AccessTokenBuilder {
    /// 设置授权类型
    pub fn grant_type(mut self, grant_type: impl Into<String>) -> Self {
        self.grant_type = grant_type.into();
        self
    }

    /// 设置授权码
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = code.into();
        self
    }

    /// 发送请求获取用户访问令牌
    pub async fn send(self) -> AuthResult<UserAccessTokenResponse> {
        let url = format!("{}/open-apis/authen/v1/access_token", self.config.base_url);

        let request_body = AccessTokenRequest {
            grant_type: self.grant_type,
            code: self.code,
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("User-Agent", "openlark-rust-sdk/0.1.0")
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response: UserAccessTokenResponse = response.json().await?;
            Ok(token_response)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AuthError::APIError {
                code: status.as_u16() as i32,
                message: format!("HTTP {}: {}", status, error_text),
            })
        }
    }
}

/// 用户访问令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AccessTokenRequest {
    grant_type: String,
    code: String,
}

/// 用户访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccessTokenResponse {
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间（秒）
    pub expires_in: i32,
    /// 权限范围
    pub scope: Option<String>,
    /// 刷新令牌
    pub refresh_token: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_token_service_creation() {
        let config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = AccessTokenService::new(std::sync::Arc::new(config));

        // 测试构建器创建
        let _builder = service.create();
    }

    #[test]
    fn test_access_token_builder() {
        let config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = AccessTokenService::new(std::sync::Arc::new(config));

        let builder = service
            .create()
            .grant_type("authorization_code")
            .code("test_code");

        // 构建器应该正确设置参数
        // 注意：这里不实际发送请求，只测试构建器的设置
    }
}
