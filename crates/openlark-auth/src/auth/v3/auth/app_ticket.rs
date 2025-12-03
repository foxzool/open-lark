//! 应用票据相关API
//!
//! 实现重新获取应用票据的功能

use crate::models::{AppTicketRequest, AppTicketResponse, AuthConfig};
use std::sync::Arc;

/// 应用票据重新发送构建器
pub struct AppTicketResendBuilder {
    client: reqwest::Client,
    config: Arc<AuthConfig>,
    app_id: String,
    app_secret: String,
}

impl AppTicketResendBuilder {
    /// 创建新的应用票据重新发送构建器
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
            app_id: String::new(),
            app_secret: String::new(),
        }
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

    /// 发送请求重新获取应用票据
    pub async fn send(self) -> Result<AppTicketResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/auth/v3/app_ticket/resend", self.config.base_url);

        let request = AppTicketRequest {
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
        let ticket_response: AppTicketResponse = response.json().await?;
        Ok(ticket_response)
    }
}

/// 应用票据服务
pub struct AppTicketService {
    config: Arc<AuthConfig>,
}

impl AppTicketService {
    /// 创建新的服务实例
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self { config }
    }

    /// 重新获取应用票据
    pub fn resend(&self) -> AppTicketResendBuilder {
        AppTicketResendBuilder::new(self.config.clone())
            .from_config()
    }

    /// 自定义请求重新获取应用票据
    pub fn resend_app_ticket(
        &self,
        app_id: &str,
        app_secret: &str,
    ) -> AppTicketResendBuilder {
        AppTicketResendBuilder::new(self.config.clone())
            .app_id(app_id)
            .app_secret(app_secret)
    }
}