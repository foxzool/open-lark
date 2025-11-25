//! OAuth授权服务 (Resource: authorization)
//!
//! 提供向后兼容的OAuth授权功能。

use std::sync::Arc;

use crate::models::AuthConfig;

/// OAuth授权服务
#[derive(Debug)]
pub struct AuthorizationService {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
}

impl AuthorizationService {
    /// 创建新的OAuth授权服务
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// 获取登录预授权码
    pub fn get_index(&self) -> AuthorizationBuilder {
        AuthorizationBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            app_id: self.config.app_id.clone(),
            redirect_uri: String::new(),
            state: String::new(),
        }
    }
}

/// OAuth授权构建器
#[derive(Debug)]
pub struct AuthorizationBuilder {
    #[allow(dead_code)] // 保留用于未来扩展
    config: Arc<AuthConfig>,
    #[allow(dead_code)] // 保留用于未来扩展
    client: reqwest::Client,
    app_id: String,
    redirect_uri: String,
    state: String,
}

impl AuthorizationBuilder {
    /// 设置重定向URI
    pub fn redirect_uri(mut self, redirect_uri: impl Into<String>) -> Self {
        self.redirect_uri = redirect_uri.into();
        self
    }

    /// 设置状态参数
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = state.into();
        self
    }

    /// 构建授权URL
    pub fn build_url(self) -> String {
        format!(
            "https://open.feishu.cn/open-apis/authen/v1/authorize?app_id={}&redirect_uri={}&state={}",
            self.app_id, self.redirect_uri, self.state
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorization_service_creation() {
        let config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = AuthorizationService::new(std::sync::Arc::new(config));

        // 测试构建器创建
        let _builder = service.get_index();
    }

    #[test]
    fn test_authorization_builder() {
        let config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = AuthorizationService::new(std::sync::Arc::new(config));

        let url = service
            .get_index()
            .redirect_uri("https://example.com/callback")
            .state("random_state")
            .build_url();

        assert!(url.contains("app_id=test_app_id"));
        assert!(url.contains("redirect_uri=https://example.com/callback"));
        assert!(url.contains("state=random_state"));
    }
}
