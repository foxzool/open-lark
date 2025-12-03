//! OIDC刷新访问令牌相关API
//!
//! 实现刷新OIDC访问令牌的功能

use crate::models::{OidcRefreshAccessTokenResponse, AuthConfig};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// OIDC刷新访问令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcRefreshAccessTokenRequest {
    /// 刷新令牌
    pub refresh_token: String,
    /// 授权类型，固定为"refresh_token"
    pub grant_type: String,
}

/// OIDC刷新访问令牌构建器
pub struct OidcRefreshAccessTokenBuilder {
    client: reqwest::Client,
    config: Arc<AuthConfig>,
    request: OidcRefreshAccessTokenRequest,
}

impl OidcRefreshAccessTokenBuilder {
    /// 创建新的OIDC刷新访问令牌构建器
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
            request: OidcRefreshAccessTokenRequest {
                refresh_token: String::new(),
                grant_type: "refresh_token".to_string(),
            },
        }
    }

    /// 设置刷新令牌
    pub fn refresh_token(mut self, refresh_token: impl Into<String>) -> Self {
        self.request.refresh_token = refresh_token.into();
        self
    }

    /// 发送请求刷新OIDC访问令牌
    pub async fn send(self) -> Result<OidcRefreshAccessTokenResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/authen/v1/oidc/refresh_access_token", self.config.base_url);

        // 构建请求
        let response = self.client
            .post(&url)
            .json(&self.request)
            .send()
            .await?;

        // 解析响应
        let token_response: OidcRefreshAccessTokenResponse = response.json().await?;
        Ok(token_response)
    }
}

/// OIDC刷新访问令牌服务
pub struct OidcRefreshAccessTokenService {
    config: Arc<AuthConfig>,
}

impl OidcRefreshAccessTokenService {
    /// 创建新的服务实例
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self { config }
    }

    /// 创建OIDC刷新访问令牌构建器
    pub fn create(&self) -> OidcRefreshAccessTokenBuilder {
        OidcRefreshAccessTokenBuilder::new(self.config.clone())
    }
}