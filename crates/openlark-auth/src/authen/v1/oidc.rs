//! OIDC认证服务 (Resource: oidc)
//!
//! 提供OIDC认证功能，包括访问令牌的获取和刷新。

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::models::{AuthConfig, AuthError, AuthResult};

/// OIDC认证服务
#[derive(Debug)]
pub struct OidcService {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
}

impl OidcService {
    /// 创建新的OIDC认证服务
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// 获取OIDC访问令牌
    pub fn create_access_token(&self) -> OidcAccessTokenBuilder {
        OidcAccessTokenBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            grant_type: String::new(),
            code: String::new(),
        }
    }

    /// 刷新OIDC访问令牌
    pub fn create_refresh_access_token(&self) -> OidcRefreshTokenBuilder {
        OidcRefreshTokenBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            grant_type: String::new(),
            refresh_token: String::new(),
        }
    }
}

/// OIDC访问令牌构建器
#[derive(Debug)]
pub struct OidcAccessTokenBuilder {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
    grant_type: String,
    code: String,
}

impl OidcAccessTokenBuilder {
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

    /// 发送请求获取OIDC访问令牌
    pub async fn send(self) -> AuthResult<OidcAccessTokenResponse> {
        let url = format!(
            "{}/open-apis/authen/v1/oidc/access_token",
            self.config.base_url
        );

        let request_body = OidcAccessTokenRequest {
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
            let token_response: OidcAccessTokenResponse = response.json().await?;
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

/// OIDC刷新令牌构建器
#[derive(Debug)]
pub struct OidcRefreshTokenBuilder {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
    grant_type: String,
    refresh_token: String,
}

impl OidcRefreshTokenBuilder {
    /// 设置授权类型
    pub fn grant_type(mut self, grant_type: impl Into<String>) -> Self {
        self.grant_type = grant_type.into();
        self
    }

    /// 设置刷新令牌
    pub fn refresh_token(mut self, refresh_token: impl Into<String>) -> Self {
        self.refresh_token = refresh_token.into();
        self
    }

    /// 发送请求刷新OIDC访问令牌
    pub async fn send(self) -> AuthResult<OidcAccessTokenResponse> {
        let url = format!(
            "{}/open-apis/authen/v1/oidc/refresh_access_token",
            self.config.base_url
        );

        let request_body = OidcRefreshTokenRequest {
            grant_type: self.grant_type,
            refresh_token: self.refresh_token,
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
            let token_response: OidcAccessTokenResponse = response.json().await?;
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

/// OIDC访问令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OidcAccessTokenRequest {
    grant_type: String,
    code: String,
}

/// OIDC刷新令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OidcRefreshTokenRequest {
    grant_type: String,
    refresh_token: String,
}

/// OIDC访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcAccessTokenResponse {
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
    /// ID令牌
    pub id_token: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oidc_service_creation() {
        let config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = OidcService::new(std::sync::Arc::new(config));

        // 测试构建器创建
        let _builder = service.create_access_token();
        let _refresh_builder = service.create_refresh_access_token();
    }

    #[test]
    fn test_oidc_access_token_builder() {
        let config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = OidcService::new(std::sync::Arc::new(config));

        let builder = service
            .create_access_token()
            .grant_type("authorization_code")
            .code("test_code");

        // 构建器应该正确设置参数
        // 注意：这里不实际发送请求，只测试构建器的设置
    }
}
