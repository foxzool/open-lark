//! 租户访问令牌服务 (Resource: tenant_access_token)
//!
//! 提供租户访问令牌的获取功能，支持自建应用和商店应用两种模式。

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::models::{AuthConfig, AuthError, AuthResult, TenantAccessTokenResponse};

/// 租户访问令牌服务
#[derive(Debug)]
pub struct TenantAccessTokenService {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
}

impl TenantAccessTokenService {
    /// 创建新的租户访问令牌服务
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// 获取自建应用租户访问令牌
    pub fn internal(&self) -> TenantAccessTokenInternalBuilder {
        TenantAccessTokenInternalBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            app_id: self.config.app_id.clone(),
            app_secret: self.config.app_secret.clone(),
        }
    }

    /// 获取商店应用租户访问令牌
    pub fn store(&self) -> TenantAccessTokenStoreBuilder {
        TenantAccessTokenStoreBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            app_access_token: String::new(),
            tenant_key: String::new(),
        }
    }
}

/// 自建应用租户访问令牌构建器
#[derive(Debug)]
pub struct TenantAccessTokenInternalBuilder {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
    app_id: String,
    app_secret: String,
}

impl TenantAccessTokenInternalBuilder {
    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 设置应用密钥
    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.app_secret = app_secret.into();
        self
    }

    /// 发送请求获取令牌
    pub async fn send(self) -> AuthResult<TenantAccessTokenResponse> {
        let url = format!(
            "{}/open-apis/auth/v3/tenant_access_token/internal",
            self.config.base_url
        );

        let request_body = TenantAccessTokenInternalRequest {
            app_id: self.app_id,
            app_secret: self.app_secret,
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
            let token_response: TenantAccessTokenResponse = response.json().await?;
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

/// 商店应用租户访问令牌构建器
#[derive(Debug)]
pub struct TenantAccessTokenStoreBuilder {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
    app_access_token: String,
    tenant_key: String,
}

impl TenantAccessTokenStoreBuilder {
    /// 设置应用访问令牌
    pub fn app_access_token(mut self, app_access_token: impl Into<String>) -> Self {
        self.app_access_token = app_access_token.into();
        self
    }

    /// 设置租户标识
    pub fn tenant_key(mut self, tenant_key: impl Into<String>) -> Self {
        self.tenant_key = tenant_key.into();
        self
    }

    /// 发送请求获取令牌
    pub async fn send(self) -> AuthResult<TenantAccessTokenResponse> {
        let url = format!(
            "{}/open-apis/auth/v3/tenant_access_token",
            self.config.base_url
        );

        let app_access_token_clone = self.app_access_token.clone();
        let request_body = TenantAccessTokenStoreRequest {
            app_access_token: self.app_access_token,
            tenant_key: self.tenant_key,
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("User-Agent", "openlark-rust-sdk/0.1.0")
            .header(
                "Authorization",
                format!("Bearer {}", app_access_token_clone),
            )
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response: TenantAccessTokenResponse = response.json().await?;
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

/// 自建应用租户访问令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TenantAccessTokenInternalRequest {
    app_id: String,
    app_secret: String,
}

/// 商店应用租户访问令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TenantAccessTokenStoreRequest {
    app_access_token: String,
    tenant_key: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tenant_access_token_service_creation() {
        let config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = TenantAccessTokenService::new(std::sync::Arc::new(config));

        // 测试构建器创建
        let _builder = service.internal();
        let _builder = service.store();
    }

    #[test]
    fn test_tenant_access_token_internal_builder() {
        let config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = TenantAccessTokenService::new(std::sync::Arc::new(config));

        let builder = service
            .internal()
            .app_id("custom_app_id")
            .app_secret("custom_app_secret");

        // 构建器应该正确设置参数
        // 注意：这里不实际发送请求，只测试构建器的设置
    }
}
