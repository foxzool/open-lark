//! 登录预授权码相关API
//!
//! 实现获取登录预授权码的功能

use crate::models::{AuthorizationIndexResponse, AuthConfig};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 登录预授权码请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationIndexRequest {
    /// 应用ID
    pub app_id: String,
    /// 重定向URI
    pub redirect_uri: String,
    /// 授权范围
    pub scope: Option<String>,
    /// 状态参数
    pub state: Option<String>,
}

/// 登录预授权码构建器
pub struct AuthorizationIndexBuilder {
    client: reqwest::Client,
    config: Arc<AuthConfig>,
    app_id: String,
    redirect_uri: String,
    scope: Option<String>,
    state: Option<String>,
}

impl AuthorizationIndexBuilder {
    /// 创建新的登录预授权码构建器
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
            app_id: String::new(),
            redirect_uri: String::new(),
            scope: None,
            state: None,
        }
    }

    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 设置重定向URI
    pub fn redirect_uri(mut self, redirect_uri: impl Into<String>) -> Self {
        self.redirect_uri = redirect_uri.into();
        self
    }

    /// 设置授权范围
    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }

    /// 设置状态参数
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// 从配置设置应用ID
    pub fn from_config(mut self) -> Self {
        self.app_id = self.config.app_id.clone();
        self
    }

    /// 发送请求获取登录预授权码
    pub async fn send(self) -> Result<AuthorizationIndexResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/open-apis/authen/v1/authorize", self.config.base_url);

        // 构建请求 - 使用GET方法，参数作为查询字符串
        let mut request = self.client.get(&url);

        // 添加查询参数
        if let Some(scope) = &self.scope {
            request = request.query(&[("scope", scope)]);
        }
        if let Some(state) = &self.state {
            request = request.query(&[("state", state)]);
        }

        request = request
            .query(&[("app_id", &self.app_id)])
            .query(&[("redirect_uri", &self.redirect_uri)]);

        let response = request.send().await?;

        // 解析响应
        let auth_response: AuthorizationIndexResponse = response.json().await?;
        Ok(auth_response)
    }
}

/// 登录预授权码服务
pub struct AuthorizationIndexService {
    config: Arc<AuthConfig>,
}

impl AuthorizationIndexService {
    /// 创建新的服务实例
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self { config }
    }

    /// 获取登录预授权码
    pub fn get_index(&self) -> AuthorizationIndexBuilder {
        AuthorizationIndexBuilder::new(self.config.clone())
            .from_config()
    }

    /// 自定义请求获取登录预授权码
    pub fn get_authorization_index(
        &self,
        app_id: &str,
        redirect_uri: &str,
    ) -> AuthorizationIndexBuilder {
        AuthorizationIndexBuilder::new(self.config.clone())
            .app_id(app_id)
            .redirect_uri(redirect_uri)
    }
}