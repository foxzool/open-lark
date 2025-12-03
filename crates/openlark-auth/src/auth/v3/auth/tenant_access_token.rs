//! 租户访问令牌相关API
//!
//! 实现自建应用和商店应用获取租户访问令牌的功能

use crate::models::{TenantAccessTokenResponse, AuthConfig};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 租户访问令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantAccessTokenRequest {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
}

/// 租户访问令牌构建器
pub struct TenantAccessTokenBuilder {
    client: reqwest::Client,
    config: Arc<AuthConfig>,
    app_id: String,
    app_secret: String,
    is_internal: bool, // 区分自建应用和商店应用
}

impl TenantAccessTokenBuilder {
    /// 创建新的租户访问令牌构建器
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
            app_id: String::new(),
            app_secret: String::new(),
            is_internal: false,
        }
    }

    /// 标记为自建应用
    pub fn internal(mut self) -> Self {
        self.is_internal = true;
        self
    }

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

    /// 从配置设置应用ID和应用密钥
    pub fn from_config(mut self) -> Self {
        self.app_id = self.config.app_id.clone();
        self.app_secret = self.config.app_secret.clone();
        self
    }

    /// 发送请求获取租户访问令牌
    pub async fn send(self) -> Result<TenantAccessTokenResponse, Box<dyn std::error::Error>> {
        let url = if self.is_internal {
            format!("{}/open-apis/auth/v3/tenant_access_token/internal", self.config.base_url)
        } else {
            format!("{}/open-apis/auth/v3/tenant_access_token", self.config.base_url)
        };

        let request = TenantAccessTokenRequest {
            app_id: self.app_id,
            app_secret: self.app_secret,
        };

        // 构建请求
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        // 解析响应
        let token_response: TenantAccessTokenResponse = response.json().await?;
        Ok(token_response)
    }
}

/// 租户访问令牌服务
pub struct TenantAccessTokenService {
    config: Arc<AuthConfig>,
}

impl TenantAccessTokenService {
    /// 创建新的服务实例
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self { config }
    }

    /// 获取自建应用租户访问令牌
    pub fn internal(&self) -> TenantAccessTokenBuilder {
        TenantAccessTokenBuilder::new(self.config.clone())
            .internal()
            .from_config()
    }

    /// 获取商店应用租户访问令牌
    pub fn store(&self) -> TenantAccessTokenBuilder {
        TenantAccessTokenBuilder::new(self.config.clone())
            .from_config()
    }

    /// 自定义请求获取自建应用租户访问令牌
    pub fn get_tenant_access_token_internal(
        &self,
        app_id: &str,
        app_secret: &str,
    ) -> TenantAccessTokenBuilder {
        TenantAccessTokenBuilder::new(self.config.clone())
            .internal()
            .app_id(app_id)
            .app_secret(app_secret)
    }

    /// 自定义请求获取商店应用租户访问令牌
    pub fn get_tenant_access_token(
        &self,
        app_id: &str,
        app_secret: &str,
    ) -> TenantAccessTokenBuilder {
        TenantAccessTokenBuilder::new(self.config.clone())
            .app_id(app_id)
            .app_secret(app_secret)
    }
}