//! OAuth旧版本API实现

use openlark_core::{
    config::Config,
    error::SDKResult,
};
use crate::{models::oauth::*};

// 类型别名
pub type AuthResult<T> = SDKResult<T>;

/// OAuth旧版本API服务
#[derive(Debug)]
pub struct OAuthServiceOld {
    config: Config,
}

impl OAuthServiceOld {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 授权码构建器
pub struct AuthorizationBuilder {
    config: Config,
    request: AuthorizationCodeRequest,
}

impl AuthorizationBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: AuthorizationCodeRequest {
                app_id: String::new(),
                redirect_uri: String::new(),
                scope: None,
                state: None,
                response_type: Some("code".to_string()),
            },
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.request.app_id = app_id.into();
        self
    }

    pub fn redirect_uri(mut self, uri: impl Into<String>) -> Self {
        self.request.redirect_uri = uri.into();
        self
    }

    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.request.scope = Some(scope.into());
        self
    }

    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.request.state = Some(state.into());
        self
    }

    /// 构建授权URL
    pub fn build_url(self) -> String {
        let mut url = format!(
            "{}/open-apis/authen/v1/index?app_id={}&redirect_uri={}",
            self.config.base_url,
            urlencoding::encode(&self.request.app_id),
            urlencoding::encode(&self.request.redirect_uri)
        );

        if let Some(scope) = &self.request.scope {
            url.push_str(&format!("&scope={}", urlencoding::encode(scope)));
        }

        if let Some(state) = &self.request.state {
            url.push_str(&format!("&state={}", urlencoding::encode(state)));
        }

        url.push_str(&format!("&response_type={}", self.request.response_type.unwrap_or_default()));
        url
    }

    pub async fn send(self) -> AuthResult<AuthorizationCodeResponse> {
        // OAuth授权是重定向流程，不直接返回响应
        todo!("OAuth授权需要重定向流程，使用 build_url() 获取授权链接")
    }
}

impl super::OAuthServiceOld {
    /// 获取授权码
    pub fn authorization(&self) -> AuthorizationBuilder {
        AuthorizationBuilder::new(self.config.clone())
    }
}