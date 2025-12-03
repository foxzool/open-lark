//! 获取登录预授权码 API
//!
//! 对应CSV记录: 6907569744329932801
//! 应用请求用户身份验证时，需构造登录链接，并引导用户跳转至此链接。
//! 用户登录成功后会生成登录预授权码 code，并作为参数追加到重定向URL。

use openlark_core::{
    config::Config,
    error::SDKResult,
};
use crate::models::oauth::*;

/// 授权码构建器
#[derive(Debug)]
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

    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.request.app_id = app_id.into();
        self
    }

    /// 设置重定向URI
    pub fn redirect_uri(mut self, uri: impl Into<String>) -> Self {
        self.request.redirect_uri = uri.into();
        self
    }

    /// 设置权限范围
    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.request.scope = Some(scope.into());
        self
    }

    /// 设置状态参数，用于防止CSRF攻击
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.request.state = Some(state.into());
        self
    }

    /// 构建授权URL
    ///
    /// 返回一个完整的OAuth授权URL，用户需要访问此URL进行身份验证。
    /// 验证成功后，飞书会将用户重定向到指定的redirect_uri，
    /// 并附带授权码code参数。
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

    /// 获取授权码（OAuth流程）
    ///
    /// 注意：OAuth授权是重定向流程，不直接返回响应。
    /// 实际使用时应该：
    /// 1. 调用 build_url() 获取授权链接
    /// 2. 引导用户访问该链接
    /// 3. 用户授权后，飞书重定向到redirect_uri并附带code参数
    /// 4. 使用获取到的code调用user_access_token接口获取访问令牌
    pub async fn send(self) -> SDKResult<AuthorizationCodeResponse> {
        // OAuth授权是重定向流程，不直接返回响应
        // 实际应用中需要处理重定向流程
        Err(openlark_core::error::CoreError::Configuration {
            message: "OAuth授权需要重定向流程，请使用 build_url() 获取授权链接".to_string(),
            code: openlark_core::error::ErrorCode::ValidationError,
            ctx: {
                let mut ctx = openlark_core::error::ErrorContext::new();
                ctx.add_context("endpoint", "/open-apis/authen/v1/index");
                ctx.add_context("flow", "oauth_redirect");
                ctx
            },
        })
    }
}

/// OAuth旧版本API服务
#[derive(Debug)]
pub struct OAuthServiceOld {
    config: Config,
}

impl OAuthServiceOld {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取授权码
    pub fn authorization(&self) -> AuthorizationBuilder {
        AuthorizationBuilder::new(self.config.clone())
    }

    /// 获取登录预授权码（index方法别名）
    pub fn index(&self) -> AuthorizationBuilder {
        self.authorization()
    }
}