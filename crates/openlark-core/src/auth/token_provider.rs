//! Token Provider Trait
//!
//! 为 `openlark-core::http::Transport` 提供“只负责注入、不负责获取逻辑”的抽象：
//! - core 只知道“需要什么 token（App/Tenant/User）”，并把必要上下文（如 app_ticket/tenant_key）交给 provider
//! - token 获取/刷新/缓存由业务 crate（例如 openlark-auth）实现

use crate::{constants::AccessTokenType, SDKResult};
use async_trait::async_trait;

/// Token 获取请求上下文
///
/// 注意：这里不绑定 `Config`，避免 core 持有“获取逻辑”，由具体实现自行决定读取哪些配置或状态。
#[derive(Debug, Clone, Default)]
pub struct TokenRequest {
    pub token_type: AccessTokenType,
    pub tenant_key: Option<String>,
    pub app_ticket: Option<String>,
}

impl TokenRequest {
    pub fn app() -> Self {
        Self {
            token_type: AccessTokenType::App,
            ..Default::default()
        }
    }

    pub fn tenant() -> Self {
        Self {
            token_type: AccessTokenType::Tenant,
            ..Default::default()
        }
    }

    pub fn user() -> Self {
        Self {
            token_type: AccessTokenType::User,
            ..Default::default()
        }
    }

    pub fn tenant_key(mut self, tenant_key: impl Into<String>) -> Self {
        self.tenant_key = Some(tenant_key.into());
        self
    }

    pub fn app_ticket(mut self, app_ticket: impl Into<String>) -> Self {
        self.app_ticket = Some(app_ticket.into());
        self
    }
}

/// Token provider trait for acquiring and refreshing access tokens
///
/// This trait allows `Transport` to obtain tokens without knowing
/// concrete implementation (in-memory cache, distributed cache, custom refresh logic).
#[async_trait]
pub trait TokenProvider: Send + Sync + std::fmt::Debug {
    /// 获取指定类型的 access token
    async fn get_token(&self, request: TokenRequest) -> SDKResult<String>;

    /// 便捷方法：获取 tenant token（可带 tenant_key）
    async fn get_tenant_token(&self, tenant_key: Option<&str>) -> SDKResult<String> {
        let mut req = TokenRequest::tenant();
        if let Some(key) = tenant_key {
            if !key.is_empty() {
                req = req.tenant_key(key);
            }
        }
        self.get_token(req).await
    }

    /// Optional: Get app access token explicitly
    async fn get_app_token(&self) -> SDKResult<String> {
        self.get_token(TokenRequest::app()).await
    }

    /// Optional: Get user access token explicitly
    async fn get_user_token(&self) -> SDKResult<String> {
        self.get_token(TokenRequest::user()).await
    }
}

/// Default implementation that does not cache tokens
///
/// This is used when caching is disabled or as a fallback.
#[derive(Debug)]
pub struct NoOpTokenProvider;

#[async_trait]
impl TokenProvider for NoOpTokenProvider {
    async fn get_token(&self, request: TokenRequest) -> SDKResult<String> {
        Err(crate::error::configuration_error(
            format!(
                "token_provider: NoOpTokenProvider 未实现 token 获取逻辑（请求：{:?}），请在 Config 中设置 TokenProvider（建议使用 openlark-auth 提供的实现）。",
                request
            ),
        ))
    }

    async fn get_tenant_token(&self, tenant_key: Option<&str>) -> SDKResult<String> {
        <Self as TokenProvider>::get_tenant_token(self, tenant_key).await
    }
}
