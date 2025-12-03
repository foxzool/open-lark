//! OIDC访问令牌相关API
//!
//! 实现获取OIDC访问令牌的功能

use crate::models::{OidcAccessTokenResponse, AuthConfig};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// OIDC访问令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcAccessTokenRequest {
    /// 授权码
    pub code: String,
    /// 授权码重定向URI
    pub redirect_uri: String,
    /// 客户端ID
    pub client_id: String,
    /// 客户端密钥
    pub client_secret: String,
    /// 授权类型，固定为"authorization_code"
    pub grant_type: String,
}

/// OIDC访问令牌构建器
pub struct OidcAccessTokenBuilder {
    client: reqwest::Client,
    config: Arc<AuthConfig>,
    request: OidcAccessTokenRequest,
}

impl OidcAccessTokenBuilder {
    /// 创建新的OIDC访问令牌构建器
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
            request: OidcAccessTokenRequest {
                code: String::new(),
                redirect_uri: String::new(),
                client_id: String::new(),
                client_secret: String::new(),
                grant_type: "authorization_code".to_string(),
            },
        }
    }

    /// 设置授权码
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.request.code = code.into();
        self
    }

    /// 设置重定向URI
    pub fn redirect_uri(mut self, redirect_uri: impl Into<String>) -> Self {
        self.request.redirect_uri = redirect_uri.into();
        self
    }

    /// 设置客户端ID
    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.request.client_id = client_id.into();
        self
    }

    /// 设置客户端密钥
    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.request.client_secret = client_secret.into();
        self
    }

    /// 发送请求获取OIDC访问令牌
    pub async fn send(self) -> Result<OidcAccessTokenResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/authen/v1/oidc/access_token", self.config.base_url);

        // 构建请求
        let response = self.client
            .post(&url)
            .json(&self.request)
            .send()
            .await?;

        // 解析响应
        let token_response: OidcAccessTokenResponse = response.json().await?;
        Ok(token_response)
    }
}

/// OIDC访问令牌服务
pub struct OidcAccessTokenService {
    config: Arc<AuthConfig>,
}

impl OidcAccessTokenService {
    /// 创建新的服务实例
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self { config }
    }

    /// 创建OIDC访问令牌构建器
    pub fn create(&self) -> OidcAccessTokenBuilder {
        OidcAccessTokenBuilder::new(self.config.clone())
    }
}