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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_request_app() {
        let req = TokenRequest::app();
        assert_eq!(req.token_type, AccessTokenType::App);
        assert!(req.tenant_key.is_none());
        assert!(req.app_ticket.is_none());
    }

    #[test]
    fn test_token_request_tenant() {
        let req = TokenRequest::tenant();
        assert_eq!(req.token_type, AccessTokenType::Tenant);
    }

    #[test]
    fn test_token_request_user() {
        let req = TokenRequest::user();
        assert_eq!(req.token_type, AccessTokenType::User);
    }

    #[test]
    fn test_token_request_with_tenant_key() {
        let req = TokenRequest::tenant().tenant_key("test_tenant");
        assert_eq!(req.tenant_key, Some("test_tenant".to_string()));
    }

    #[test]
    fn test_token_request_with_app_ticket() {
        let req = TokenRequest::app().app_ticket("test_ticket");
        assert_eq!(req.app_ticket, Some("test_ticket".to_string()));
    }

    #[test]
    fn test_token_request_default() {
        let req = TokenRequest::default();
        assert_eq!(req.token_type, AccessTokenType::None);
    }

    #[test]
    fn test_token_request_debug() {
        let req = TokenRequest::app();
        let debug_str = format!("{:?}", req);
        assert!(debug_str.contains("TokenRequest"));
    }

    #[tokio::test]
    async fn test_no_op_token_provider_returns_error() {
        let provider = NoOpTokenProvider;
        let result = provider.get_token(TokenRequest::app()).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_no_op_token_provider_debug() {
        let provider = NoOpTokenProvider;
        let debug_str = format!("{:?}", provider);
        assert!(debug_str.contains("NoOpTokenProvider"));
    }
}
