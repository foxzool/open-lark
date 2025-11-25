//! 用户信息服务 (Resource: user_info)
//!
//! 提供用户信息获取功能。

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::models::{AuthConfig, AuthError, AuthResult, UserInfoResponse};

/// 用户信息服务
#[derive(Debug)]
pub struct UserInfoService {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
}

impl UserInfoService {
    /// 创建新的用户信息服务
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// 获取用户信息
    pub fn get(&self) -> UserInfoGetBuilder {
        UserInfoGetBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            user_access_token: String::new(),
        }
    }
}

/// 用户信息获取构建器
#[derive(Debug)]
pub struct UserInfoGetBuilder {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
    user_access_token: String,
}

impl UserInfoGetBuilder {
    /// 设置用户访问令牌
    pub fn user_access_token(mut self, user_access_token: impl Into<String>) -> Self {
        self.user_access_token = user_access_token.into();
        self
    }

    /// 发送请求获取用户信息
    pub async fn send(self) -> AuthResult<UserInfoResponse> {
        let url = format!("{}/open-apis/authen/v1/user_info", self.config.base_url);

        let response = self
            .client
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.user_access_token),
            )
            .header("Content-Type", "application/json")
            .header("User-Agent", "openlark-rust-sdk/0.1.0")
            .send()
            .await?;

        if response.status().is_success() {
            let user_info: UserInfoResponse = response.json().await?;
            Ok(user_info)
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

/// 用户信息获取响应（内部数据结构）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)] // 保留用于未来API响应解析
struct UserInfoInternalResponse {
    /// 用户数据
    pub data: UserInfoResponse,
    /// 请求是否成功
    pub success: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_info_service_creation() {
        let config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = UserInfoService::new(std::sync::Arc::new(config));

        // 测试构建器创建
        let _builder = service.get();
    }

    #[test]
    fn test_user_info_get_builder() {
        let config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = UserInfoService::new(std::sync::Arc::new(config));

        let builder = service.get().user_access_token("test_user_access_token");

        // 构建器应该正确设置参数
        // 注意：这里不实际发送请求，只测试构建器的设置
    }
}
