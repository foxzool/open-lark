//! 应用票据服务 (Resource: app_ticket)
//!
//! 提供应用票据的重新推送功能。

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::models::{AppTicketResponse, AuthConfig, AuthError, AuthResult};

/// 应用票据服务
#[derive(Debug)]
pub struct AppTicketService {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
}

impl AppTicketService {
    /// 创建新的应用票据服务
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// 重新推送应用票据
    pub fn resend(&self) -> AppTicketResendBuilder {
        AppTicketResendBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            app_id: self.config.app_id.clone(),
            app_secret: self.config.app_secret.clone(),
        }
    }
}

/// 应用票据重新推送构建器
#[derive(Debug)]
pub struct AppTicketResendBuilder {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
    app_id: String,
    app_secret: String,
}

impl AppTicketResendBuilder {
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

    /// 发送请求重新推送票据
    pub async fn send(self) -> AuthResult<AppTicketResponse> {
        let url = format!(
            "{}/open-apis/auth/v3/app_ticket/resend",
            self.config.base_url
        );

        let request_body = AppTicketResendRequest {
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
            let ticket_response: AppTicketResponse = response.json().await?;
            Ok(ticket_response)
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

/// 应用票据重新推送请求
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppTicketResendRequest {
    app_id: String,
    app_secret: String,
}

/// 应用票据重新推送响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppTicketResendResponse {
    /// 应用票据
    pub app_ticket: String,
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_ticket_service_creation() {
        let config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = AppTicketService::new(std::sync::Arc::new(config));

        // 测试构建器创建
        let _builder = service.resend();
    }

    #[test]
    fn test_app_ticket_resend_builder() {
        let config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = AppTicketService::new(std::sync::Arc::new(config));

        let builder = service
            .resend()
            .app_id("custom_app_id")
            .app_secret("custom_app_secret");

        // 构建器应该正确设置参数
        // 注意：这里不实际发送请求，只测试构建器的设置
    }
}
