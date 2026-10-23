//! openlark-auth 的 TokenProvider 实现
//!
//! `openlark-core` 通过 `TokenProvider` 抽象获取 token，而不关心具体获取/刷新/缓存策略。
//! 这里提供一个带缓存的实现：缓存 token 并在过期前复用。

use crate::auth::auth::v3::auth::{
    AppAccessTokenInternalRequestBuilder, AppAccessTokenRequestBuilder,
    TenantAccessTokenInternalRequestBuilder, TenantAccessTokenRequestBuilder,
};
use crate::auth::authen::v1::oidc::OidcRefreshAccessTokenRequestBuilder;
use openlark_core::{
    SDKResult,
    auth::{
        CLIENT_ASSERTION_APP_TOKEN_UNAVAILABLE, CLIENT_ASSERTION_EMPTY,
        CLIENT_ASSERTION_MARKETPLACE_UNSUPPORTED, CLIENT_ASSERTION_PROVIDER_FAILED_PREFIX,
        ClientAssertionProvider, TokenProvider, TokenRequest,
    },
    config::Config,
    constants::{AccessTokenType, AppType},
    error::configuration_error,
    req_option::RequestOption,
};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::pin::Pin;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

/// 缓存的 token 信息
#[derive(Clone)]
struct CachedToken {
    /// token 值
    token: String,
    /// 过期时间戳（Unix 时间戳，秒）
    expires_at: i64,
    /// user token 路径的轮转 refresh token（app/tenant 为 `None`）。
    /// 访问令牌过期后仍可作为下一次刷新的凭证读取。
    refresh_token: Option<String>,
}

impl std::fmt::Debug for CachedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CachedToken")
            .field("token", &"***")
            .field("expires_at", &self.expires_at)
            .field("refresh_token", &self.refresh_token.as_ref().map(|_| "***"))
            .finish()
    }
}

impl CachedToken {
    fn now_epoch_secs() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0)
    }

    /// 创建新的缓存 token
    fn new(token: String, expires_in_seconds: i64) -> Self {
        let now = Self::now_epoch_secs();
        // 提前 60 秒过期，避免临界情况（小于 60 秒则视为立即过期）
        let expires_at = now.saturating_add(expires_in_seconds.saturating_sub(60));

        Self {
            token,
            expires_at,
            refresh_token: None,
        }
    }

    /// 创建带轮转 refresh token 的缓存项（user token 路径）
    fn new_with_refresh(
        token: String,
        expires_in_seconds: i64,
        refresh_token: Option<String>,
    ) -> Self {
        Self {
            refresh_token,
            ..Self::new(token, expires_in_seconds)
        }
    }

    /// 检查 token 是否已过期
    fn is_expired(&self) -> bool {
        Self::now_epoch_secs() >= self.expires_at
    }
}

/// 基于 openlark-auth API 的 TokenProvider（带缓存）
#[derive(Debug)]
pub struct AuthTokenProvider {
    config: Config,
    /// token 缓存：key 为 token 类型字符串，value 为缓存的 token
    cache: Arc<RwLock<HashMap<String, CachedToken>>>,
}

impl Clone for AuthTokenProvider {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            cache: Arc::clone(&self.cache),
        }
    }
}

impl AuthTokenProvider {
    /// 创建基于 openlark-auth API 的 TokenProvider 实例
    ///
    /// # 参数
    /// - `config`: SDK 配置信息
    pub fn new(config: Config) -> Self {
        Self {
            config,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 生成缓存键
    fn cache_key_component(value: &str) -> String {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }

    fn cache_key(
        token_type: &AccessTokenType,
        app_type: &AppType,
        request: &TokenRequest,
    ) -> String {
        match token_type {
            AccessTokenType::Tenant => {
                // 将租户与 app_ticket 一起纳入缓存上下文，避免票据轮换后复用旧 token。
                // 哈希 Option 元组以区分缺失值和字面值，并避免在 Debug 中泄露上下文。
                let mut hasher = DefaultHasher::new();
                (request.tenant_key.as_deref(), request.app_ticket.as_deref()).hash(&mut hasher);
                format!("{token_type:?}_{app_type:?}_ctx_{:016x}", hasher.finish())
            }
            AccessTokenType::App if app_type == &AppType::Marketplace => {
                let app_ticket = request.app_ticket.as_deref().unwrap_or("default");
                let app_ticket_key = Self::cache_key_component(app_ticket);
                format!("{token_type:?}_{app_type:?}_{app_ticket_key}")
            }
            _ => format!("{token_type:?}_{app_type:?}"),
        }
    }

    async fn get_cached(&self, cache_key: &str) -> Option<String> {
        let cache = self.cache.read().await;
        cache
            .get(cache_key)
            .filter(|cached| !cached.is_expired())
            .map(|cached| cached.token.clone())
    }

    async fn set_cached(&self, cache_key: String, token: String, expires_in_seconds: i64) {
        let cached = CachedToken::new(token, expires_in_seconds);
        self.cache.write().await.insert(cache_key, cached);
    }

    /// user token 路径写回：访问令牌 + 轮转 refresh token 成对入缓存
    async fn set_cached_with_refresh(
        &self,
        cache_key: String,
        token: String,
        expires_in_seconds: i64,
        refresh_token: Option<String>,
    ) {
        let cached = CachedToken::new_with_refresh(token, expires_in_seconds, refresh_token);
        self.cache.write().await.insert(cache_key, cached);
    }

    /// user token 路径：取轮转 refresh token（访问令牌过期仍可读取）
    async fn cached_refresh_token(&self, cache_key: &str) -> Option<String> {
        let cache = self.cache.read().await;
        cache.get(cache_key).and_then(|c| c.refresh_token.clone())
    }

    async fn get_or_fetch<F, Fut>(&self, cache_key: String, fetch: F) -> SDKResult<String>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = SDKResult<(String, i64)>>,
    {
        if let Some(token) = self.get_cached(&cache_key).await {
            return Ok(token);
        }

        let (token, expires_in_seconds) = fetch().await?;
        self.set_cached(cache_key, token.clone(), expires_in_seconds)
            .await;
        Ok(token)
    }

    async fn fetch_tenant_by_client_assertion(
        &self,
        provider: Arc<dyn ClientAssertionProvider>,
    ) -> SDKResult<String> {
        let endpoint = client_assertion::resolve_oauth_endpoint(
            self.config.base_url(),
            self.config.oauth_base_url(),
        )?;
        let cache_key = client_assertion::assertion_cache_key(&endpoint.aud);
        let aud = endpoint.aud;
        let token_url = endpoint.token_url;
        self.get_or_fetch(cache_key, || async move {
            let assertion = match provider.retrieve_assertion(&aud).await {
                Ok(value) => value,
                Err(err) => {
                    return Err(configuration_error(format!(
                        "{CLIENT_ASSERTION_PROVIDER_FAILED_PREFIX}{err}"
                    )));
                }
            };
            let assertion = assertion.trim();
            if assertion.is_empty() {
                return Err(configuration_error(CLIENT_ASSERTION_EMPTY));
            }
            client_assertion::exchange_tenant_token(&self.config, &token_url, assertion).await
        })
        .await
    }

    /// user access token：命中缓存直接复用；否则用轮转/种子 refresh token
    /// 走现有刷新接口，并把新的 access + refresh 成对写回缓存（#693）。
    ///
    /// 刷新失败不写缓存（与 app/tenant 路径同语义）。
    async fn get_or_refresh_user_token(&self, cache_key: String) -> SDKResult<String> {
        if let Some(token) = self.get_cached(&cache_key).await {
            return Ok(token);
        }

        // refresh token：优先缓存里轮转写回的，其次配置的种子
        let refresh_token = self
            .cached_refresh_token(&cache_key)
            .await
            .or_else(|| self.config.user_refresh_token().map(str::to_string));
        let Some(refresh_token) = refresh_token else {
            return Err(configuration_error(
                "token_provider: user token 缓存未命中且未配置 user_refresh_token；请在 Config 设置种子 refresh token，或在 RequestOption 显式传入 user_access_token",
            ));
        };

        // 刷新接口走 App 鉴权：显式注入本 provider 缓存的 app token，
        // 不依赖 config.token_provider 回装自身（独立构造 AuthTokenProvider 也可用）。
        let app_token = self.get_token(TokenRequest::app()).await?;
        let refresh_option = RequestOption::builder().app_access_token(app_token).build();
        let data = OidcRefreshAccessTokenRequestBuilder::new(self.config.clone())
            .refresh_token(refresh_token.clone())
            .client_id(self.config.app_id())
            .client_secret(self.config.app_secret())
            .execute_with_options(refresh_option)
            .await?
            .data;
        // 轮转：响应给新 refresh token 则写回新的；未给则沿用本次使用的
        let new_refresh = data.refresh_token.clone().or(Some(refresh_token));
        self.set_cached_with_refresh(
            cache_key,
            data.user_access_token.clone(),
            data.expires_in as i64,
            new_refresh,
        )
        .await;
        Ok(data.user_access_token)
    }
}

impl TokenProvider for AuthTokenProvider {
    fn get_token(
        &self,
        request: TokenRequest,
    ) -> Pin<Box<dyn Future<Output = SDKResult<String>> + Send + '_>> {
        Box::pin(async move {
            match request.token_type {
                AccessTokenType::App => {
                    if self.config.client_assertion_provider().is_some() {
                        return Err(configuration_error(CLIENT_ASSERTION_APP_TOKEN_UNAVAILABLE));
                    }
                    let cache_key =
                        Self::cache_key(&AccessTokenType::App, &self.config.app_type(), &request);
                    self.get_or_fetch(cache_key, || async {
                        let (token, expires_in) = match self.config.app_type() {
                            AppType::SelfBuild => {
                                let resp = AppAccessTokenInternalRequestBuilder::new(
                                    self.config.clone(),
                                )
                                .app_id(self.config.app_id())
                                .app_secret(self.config.app_secret())
                                .execute()
                                .await?;
                                (resp.data.app_access_token, resp.data.expires_in as i64)
                            }
                            AppType::Marketplace => {
                                let app_ticket = request.app_ticket.clone().ok_or_else(|| {
                                    configuration_error(
                                        "token_provider: marketplace app requires app_ticket to fetch app_access_token",
                                    )
                                })?;
                                let resp =
                                    AppAccessTokenRequestBuilder::new(self.config.clone())
                                        .app_id(self.config.app_id())
                                        .app_secret(self.config.app_secret())
                                        .app_ticket(app_ticket)
                                        .execute()
                                        .await?;
                                (resp.data.app_access_token, resp.data.expires_in as i64)
                            }
                        };
                        Ok((token, expires_in))
                    })
                    .await
                }
                AccessTokenType::Tenant => {
                    if let Some(provider) = self.config.client_assertion_provider().cloned() {
                        return match self.config.app_type() {
                            AppType::Marketplace => Err(configuration_error(
                                CLIENT_ASSERTION_MARKETPLACE_UNSUPPORTED,
                            )),
                            AppType::SelfBuild => {
                                self.fetch_tenant_by_client_assertion(provider).await
                            }
                        };
                    }
                    let cache_key = Self::cache_key(
                        &AccessTokenType::Tenant,
                        &self.config.app_type(),
                        &request,
                    );
                    self.get_or_fetch(cache_key, || async {
                        let (token, expires_in) = match self.config.app_type() {
                            AppType::SelfBuild => {
                                let resp = TenantAccessTokenInternalRequestBuilder::new(
                                    self.config.clone(),
                                )
                                .app_id(self.config.app_id())
                                .app_secret(self.config.app_secret())
                                .execute()
                                .await?;
                                (resp.data.tenant_access_token, resp.data.expires_in as i64)
                            }
                            AppType::Marketplace => {
                                let tenant_key = request.tenant_key.clone().ok_or_else(|| {
                                    configuration_error(
                                        "token_provider: marketplace app requires tenant_key to fetch tenant_access_token",
                                    )
                                })?;
                                let app_ticket = request.app_ticket.clone().ok_or_else(|| {
                                    configuration_error(
                                        "token_provider: marketplace app requires app_ticket to fetch tenant_access_token",
                                    )
                                })?;
                                let app_access_token =
                                    self.get_token(TokenRequest::app().app_ticket(app_ticket))
                                        .await?;

                                let resp =
                                    TenantAccessTokenRequestBuilder::new(self.config.clone())
                                        .app_access_token(app_access_token)
                                        .tenant_key(tenant_key)
                                        .execute()
                                        .await?;
                                (resp.data.tenant_access_token, resp.data.expires_in as i64)
                            }
                        };
                        Ok((token, expires_in))
                    })
                    .await
                }
                AccessTokenType::User => {
                    let cache_key =
                        Self::cache_key(&AccessTokenType::User, &self.config.app_type(), &request);
                    self.get_or_refresh_user_token(cache_key).await
                }
                AccessTokenType::None => Err(configuration_error(
                    "token_provider: AccessTokenType::None 不应触发 token 获取",
                )),
            }
        })
    }
}

mod client_assertion;

pub use client_assertion::ensure_client_assertion_endpoint;

#[cfg(test)]
mod tests;
