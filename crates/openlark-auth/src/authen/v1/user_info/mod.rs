//! 用户信息API模块
//!
//! 实现获取登录用户信息的功能

use crate::models::{UserInfoResponse, AuthConfig};
use std::sync::Arc;

/// 用户信息构建器
pub struct UserInfoBuilder {
    client: reqwest::Client,
    config: Arc<AuthConfig>,
    user_access_token: Option<String>,
}

impl UserInfoBuilder {
    /// 创建新的用户信息构建器
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
            user_access_token: None,
        }
    }

    /// 设置用户访问令牌
    pub fn user_access_token(mut self, token: impl Into<String>) -> Self {
        self.user_access_token = Some(token.into());
        self
    }

    /// 发送请求获取用户信息
    pub async fn send(self) -> Result<UserInfoResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/authen/v1/user_info", self.config.base_url);

        // 构建请求
        let mut request = self.client.get(&url);

        // 添加授权头（如果提供了用户访问令牌）
        if let Some(token) = &self.user_access_token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?;

        // 解析响应
        let user_response: UserInfoResponse = response.json().await?;
        Ok(user_response)
    }
}

/// 用户信息服务
pub struct UserInfoService {
    config: Arc<AuthConfig>,
}

impl UserInfoService {
    /// 创建新的服务实例
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self { config }
    }

    /// 获取用户信息
    pub fn get(&self) -> UserInfoBuilder {
        UserInfoBuilder::new(self.config.clone())
    }
}