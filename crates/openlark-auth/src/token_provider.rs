//! openlark-auth 的 TokenProvider 实现
//!
//! `openlark-core` 通过 `TokenProvider` 抽象获取 token，而不关心具体获取/刷新/缓存策略。
//! 这里提供一个带缓存的实现：缓存 token 并在过期前复用。

use async_trait::async_trait;
use openlark_core::{
    auth::{TokenProvider, TokenRequest},
    config::Config,
    constants::{AccessTokenType, AppType},
    error::configuration_error,
    SDKResult,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 缓存的 token 信息
#[derive(Debug, Clone)]
struct CachedToken {
    /// token 值
    token: String,
    /// 过期时间戳（Unix 时间戳，秒）
    expires_at: i64,
}

impl CachedToken {
    /// 创建新的缓存 token
    fn new(token: String, expires_in_seconds: i64) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        // 提前 60 秒过期，避免临界情况
        let expires_at = now + expires_in_seconds - 60;

        Self { token, expires_at }
    }

    /// 检查 token 是否已过期
    fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        now >= self.expires_at
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
    pub fn new(config: Config) -> Self {
        Self {
            config,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 生成缓存键
    fn cache_key(token_type: &AccessTokenType, app_type: &AppType) -> String {
        format!("{:?}_{:?}", token_type, app_type)
    }
}

#[async_trait]
impl TokenProvider for AuthTokenProvider {
    async fn get_token(&self, request: TokenRequest) -> SDKResult<String> {
        match request.token_type {
            AccessTokenType::App => {
                let cache_key = Self::cache_key(&AccessTokenType::App, &self.config.app_type);

                {
                    let cache = self.cache.read().await;
                    if let Some(cached) = cache.get(&cache_key) {
                        if !cached.is_expired() {
                            tracing::debug!("使用缓存的 app_access_token");
                            return Ok(cached.token.clone());
                        }
                    }
                }

                use crate::api::auth::v3::auth::AuthServiceV3;
                let auth = AuthServiceV3::new(self.config.clone());

                let (token, expires_in) = match self.config.app_type {
                    AppType::SelfBuild => {
                        let resp = auth
                            .app_access_token_internal()
                            .app_id(self.config.app_id.clone())
                            .app_secret(self.config.app_secret.clone())
                            .execute()
                            .await?;
                        (resp.data.app_access_token, resp.data.expires_in)
                    }
                    AppType::Marketplace => {
                        let resp = auth
                            .app_access_token()
                            .app_id(self.config.app_id.clone())
                            .app_secret(self.config.app_secret.clone())
                            .execute()
                            .await?;
                        (resp.data.app_access_token, resp.data.expires_in)
                    }
                };

                let cached = CachedToken::new(token.clone(), expires_in as i64);
                self.cache.write().await.insert(cache_key, cached);

                Ok(token)
            }
            AccessTokenType::Tenant => {
                let cache_key = Self::cache_key(&AccessTokenType::Tenant, &self.config.app_type);

                {
                    let cache = self.cache.read().await;
                    if let Some(cached) = cache.get(&cache_key) {
                        if !cached.is_expired() {
                            tracing::debug!("使用缓存的 tenant_access_token");
                            return Ok(cached.token.clone());
                        }
                    }
                }

                use crate::api::auth::v3::auth::AuthServiceV3;
                let auth = AuthServiceV3::new(self.config.clone());

                let (token, expires_in) = match self.config.app_type {
                    AppType::SelfBuild => {
                        let resp = auth
                            .tenant_access_token_internal()
                            .app_id(self.config.app_id.clone())
                            .app_secret(self.config.app_secret.clone())
                            .execute()
                            .await?;
                        (resp.data.tenant_access_token, resp.data.expires_in)
                    }
                    AppType::Marketplace => {
                        let app_ticket = request.app_ticket.ok_or_else(|| {
                            configuration_error(
                                "token_provider: marketplace app requires app_ticket to fetch tenant_access_token",
                            )
                        })?;

                        let resp = auth
                            .tenant_access_token()
                            .app_id(self.config.app_id.clone())
                            .app_secret(self.config.app_secret.clone())
                            .app_ticket(app_ticket)
                            .execute()
                            .await?;
                        (resp.data.tenant_access_token, resp.data.expires_in)
                    }
                };

                let cached = CachedToken::new(token.clone(), expires_in as i64);
                self.cache.write().await.insert(cache_key, cached);

                Ok(token)
            }
            AccessTokenType::User => Err(configuration_error(
                "token_provider: user token 不应由 core 自动获取，请在 RequestOption 中显式传入 user_access_token（或由上层自行实现 TokenProvider 扩展）。",
            )),
            AccessTokenType::None => Err(configuration_error(
                "token_provider: AccessTokenType::None 不应触发 token 获取",
            )),
        }
    }
}
