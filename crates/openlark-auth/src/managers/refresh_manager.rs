//! 刷新管理器

use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tracing::{debug, error, info, warn};

use crate::auth::{
    cache::MemoryTokenCache,
    refresh::TokenRefresher,
    token::{TokenInfo, TokenType},
};
use crate::error::{AuthError, AuthResult};

/// 刷新管理器
///
/// 负责令牌的刷新策略、批量刷新和刷新优化
pub struct RefreshManager {
    refresher: TokenRefresher,
    cache: Arc<MemoryTokenCache>,
}

impl RefreshManager {
    /// 创建新的刷新管理器
    pub fn new(refresher: TokenRefresher, cache: Arc<MemoryTokenCache>) -> Self {
        Self { refresher, cache }
    }

    /// 刷新应用访问令牌
    pub async fn refresh_app_token(&self) -> AuthResult<TokenInfo> {
        debug!("Refreshing app access token");

        let token_info = self.refresher.refresh_app_token().await?;

        // 更新缓存
        self.cache.put("app_access_token", token_info.clone()).await;

        info!("App access token refreshed successfully");
        Ok(token_info)
    }

    /// 刷新租户访问令牌
    pub async fn refresh_tenant_token(&self, tenant_key: &str) -> AuthResult<TokenInfo> {
        debug!("Refreshing tenant access token for: {}", tenant_key);

        let token_info = self.refresher.refresh_tenant_token(tenant_key).await?;

        // 更新缓存
        let cache_key = format!("tenant_access_token:{}", tenant_key);
        self.cache.put(&cache_key, token_info.clone()).await;

        info!(
            "Tenant access token refreshed successfully for: {}",
            tenant_key
        );
        Ok(token_info)
    }

    /// 使用刷新令牌获取新的访问令牌
    pub async fn refresh_with_refresh_token(&self, refresh_token: &str) -> AuthResult<TokenInfo> {
        debug!("Refreshing token with refresh token");

        let token_info = self
            .refresher
            .refresh_with_refresh_token(refresh_token)
            .await?;

        // 更新缓存
        let cache_key = format!("user_access_token:{}", refresh_token);
        self.cache.put(&cache_key, token_info.clone()).await;

        info!("Token refreshed successfully with refresh token");
        Ok(token_info)
    }

    /// 批量刷新令牌
    pub async fn batch_refresh_tokens(
        &self,
        requests: Vec<RefreshRequest>,
    ) -> Vec<AuthResult<TokenInfo>> {
        debug!("Batch refreshing {} tokens", requests.len());

        let mut results = Vec::with_capacity(requests.len());

        for request in requests {
            let result = match request.token_type {
                TokenType::AppAccessToken => self.refresh_app_token().await,
                TokenType::TenantAccessToken => {
                    if let Some(tenant_key) = request.tenant_key {
                        self.refresh_tenant_token(&tenant_key).await
                    } else {
                        Err(AuthError::TokenError(
                            "tenant_key is required for tenant access token".to_string(),
                        ))
                    }
                }
                TokenType::UserAccessToken => {
                    if let Some(refresh_token) = request.refresh_token {
                        self.refresh_with_refresh_token(&refresh_token).await
                    } else {
                        Err(AuthError::TokenError(
                            "refresh_token is required for user access token".to_string(),
                        ))
                    }
                }
            };
            results.push(result);
        }

        let success_count = results.iter().filter(|r| r.is_ok()).count();
        info!(
            "Batch refresh completed: {}/{} successful",
            success_count,
            results.len()
        );

        results
    }

    /// 智能刷新（检查是否需要刷新）
    pub async fn smart_refresh_token(&self, key: &str) -> AuthResult<Option<TokenInfo>> {
        debug!("Smart refreshing token: {}", key);

        // 尝试从缓存获取
        if let Some(token_info) = self.cache.get(key).await {
            // 检查是否需要刷新
            if self.refresher.should_refresh(&token_info).await {
                debug!("Token needs refresh: {}", key);

                // 根据键判断令牌类型并刷新
                if key == "app_access_token" {
                    let refreshed_token = self.refresh_app_token().await?;
                    return Ok(Some(refreshed_token));
                } else if key.starts_with("tenant_access_token:") {
                    let tenant_key = key.strip_prefix("tenant_access_token:").unwrap();
                    let refreshed_token = self.refresh_tenant_token(tenant_key).await?;
                    return Ok(Some(refreshed_token));
                } else if key.starts_with("user_access_token:") {
                    // 用户令牌需要特殊处理
                    debug!("User access token requires manual refresh with refresh token");
                    return Ok(None);
                }
            } else {
                debug!("Token is still valid: {}", key);
                return Ok(Some(token_info));
            }
        }

        debug!("Token not found in cache: {}", key);
        Ok(None)
    }

    /// 预热缓存（批量刷新常用令牌）
    pub async fn warmup_cache(&self, tenant_keys: Vec<String>) -> AuthResult<()> {
        info!("Warming up cache for {} tenants", tenant_keys.len());

        // 刷新应用令牌
        if let Err(e) = self.refresh_app_token().await {
            warn!("Failed to warmup app token: {}", e);
        }

        // 批量刷新租户令牌
        let tenant_requests: Vec<_> = tenant_keys
            .into_iter()
            .map(|key| RefreshRequest {
                token_type: TokenType::TenantAccessToken,
                tenant_key: Some(key),
                refresh_token: None,
            })
            .collect();

        let results = self.batch_refresh_tokens(tenant_requests).await;

        let success_count = results.iter().filter(|r| r.is_ok()).count();
        info!(
            "Cache warmup completed: {}/{} tenant tokens refreshed successfully",
            success_count,
            results.len()
        );

        Ok(())
    }

    /// 检查令牌是否需要刷新
    pub async fn should_refresh_token(&self, key: &str) -> AuthResult<bool> {
        debug!("Checking if token needs refresh: {}", key);

        if let Some(token_info) = self.cache.get(key).await {
            let should_refresh = self.refresher.should_refresh(&token_info).await;
            debug!("Token {} needs refresh: {}", key, should_refresh);
            Ok(should_refresh)
        } else {
            debug!("Token not found in cache: {}", key);
            Ok(true) // 如果不存在，则需要获取（相当于刷新）
        }
    }

    /// 定时刷新令牌（后台任务）
    pub async fn scheduled_refresh(&self) -> AuthResult<RefreshResult> {
        debug!("Starting scheduled refresh");

        let mut refresh_result = RefreshResult::default();

        // 获取所有缓存键
        let cache_keys = self.cache.keys().await;

        for key in cache_keys {
            if let Some(token_info) = self.cache.get(&key).await {
                if self.refresher.should_refresh(&token_info).await {
                    debug!("Scheduled refresh for token: {}", key);

                    match self.smart_refresh_token(&key).await {
                        Ok(Some(_)) => {
                            refresh_result.successful_refreshes += 1;
                        }
                        Ok(None) => {
                            refresh_result.skipped_refreshes += 1;
                        }
                        Err(e) => {
                            refresh_result.failed_refreshes += 1;
                            warn!("Scheduled refresh failed for {}: {}", key, e);
                        }
                    }
                } else {
                    refresh_result.skipped_refreshes += 1;
                }
            }
        }

        info!("Scheduled refresh completed: {:?}", refresh_result);
        Ok(refresh_result)
    }

    /// 强制刷新所有令牌
    pub async fn force_refresh_all(&self) -> AuthResult<RefreshResult> {
        info!("Starting force refresh of all tokens");

        let mut refresh_result = RefreshResult::default();

        // 获取所有缓存键
        let cache_keys = self.cache.keys().await;

        for key in cache_keys {
            debug!("Force refresh for token: {}", key);

            let result = match key.as_str() {
                "app_access_token" => self.refresh_app_token().await,
                key if key.starts_with("tenant_access_token:") => {
                    let tenant_key = key.strip_prefix("tenant_access_token:").unwrap();
                    self.refresh_tenant_token(tenant_key).await
                }
                _ => {
                    debug!("Skipping force refresh for unsupported token type: {}", key);
                    refresh_result.skipped_refreshes += 1;
                    continue;
                }
            };

            match result {
                Ok(_) => {
                    refresh_result.successful_refreshes += 1;
                }
                Err(e) => {
                    refresh_result.failed_refreshes += 1;
                    error!("Force refresh failed for {}: {}", key, e);
                }
            }
        }

        info!("Force refresh completed: {:?}", refresh_result);
        Ok(refresh_result)
    }

    /// 获取刷新统计信息
    pub async fn get_refresh_stats(&self) -> RefreshStats {
        let cache_keys = self.cache.keys().await;
        let mut stats = RefreshStats::default();

        for key in cache_keys {
            if let Some(token_info) = self.cache.get(&key).await {
                stats.total_tokens += 1;

                if self.refresher.should_refresh(&token_info).await {
                    stats.tokens_needing_refresh += 1;
                }

                // 统计令牌类型
                match token_info.token_type {
                    TokenType::AppAccessToken => stats.app_tokens += 1,
                    TokenType::TenantAccessToken => stats.tenant_tokens += 1,
                    TokenType::UserAccessToken => stats.user_tokens += 1,
                }
            }
        }

        stats
    }
}

/// 刷新请求
#[derive(Debug, Clone)]
pub struct RefreshRequest {
    /// 令牌类型
    pub token_type: TokenType,
    /// 租户密钥（租户令牌使用）
    pub tenant_key: Option<String>,
    /// 刷新令牌（用户令牌使用）
    pub refresh_token: Option<String>,
}

/// 刷新结果
#[derive(Debug, Clone, Default)]
pub struct RefreshResult {
    /// 成功刷新的令牌数量
    pub successful_refreshes: u32,
    /// 失败刷新的令牌数量
    pub failed_refreshes: u32,
    /// 跳过刷新的令牌数量
    pub skipped_refreshes: u32,
}

/// 刷新统计信息
#[derive(Debug, Clone, Default)]
pub struct RefreshStats {
    /// 总令牌数量
    pub total_tokens: u32,
    /// 需要刷新的令牌数量
    pub tokens_needing_refresh: u32,
    /// 应用令牌数量
    pub app_tokens: u32,
    /// 租户令牌数量
    pub tenant_tokens: u32,
    /// 用户令牌数量
    pub user_tokens: u32,
}

#[cfg(test)]
mod tests {
    use crate::auth::cache::CacheConfig;

    #[tokio::test]
    async fn test_refresh_manager_creation() {
        let cache = Arc::new(MemoryTokenCache::new(CacheConfig::default()));
        let refresher = TokenRefresher::new(
            "test_app_id".to_string(),
            "test_app_secret".to_string(),
            "https://open.feishu.cn".to_string(),
            cache.clone(),
        );

        let manager = RefreshManager::new(refresher, cache);
        let stats = manager.get_refresh_stats().await;

        assert_eq!(stats.total_tokens, 0);
        assert_eq!(stats.tokens_needing_refresh, 0);
    }

    #[tokio::test]
    async fn test_batch_refresh_tokens() {
        let cache = Arc::new(MemoryTokenCache::new(CacheConfig::default()));
        let refresher = TokenRefresher::new(
            "test_app_id".to_string(),
            "test_app_secret".to_string(),
            "https://open.feishu.cn".to_string(),
            cache.clone(),
        );

        let manager = RefreshManager::new(refresher, cache);

        let requests = vec![
            RefreshRequest {
                token_type: TokenType::AppAccessToken,
                tenant_key: None,
                refresh_token: None,
            },
            RefreshRequest {
                token_type: TokenType::AppAccessToken,
                tenant_key: None,
                refresh_token: None,
            },
        ];

        let results = manager.batch_refresh_tokens(requests).await;
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.is_ok()));
    }

    #[tokio::test]
    async fn test_warmup_cache() {
        let cache = Arc::new(MemoryTokenCache::new(CacheConfig::default()));
        let refresher = TokenRefresher::new(
            "test_app_id".to_string(),
            "test_app_secret".to_string(),
            "https://open.feishu.cn".to_string(),
            cache.clone(),
        );

        let manager = RefreshManager::new(refresher, cache);

        let tenant_keys = vec![
            "tenant1".to_string(),
            "tenant2".to_string(),
            "tenant3".to_string(),
        ];

        let result = manager.warmup_cache(tenant_keys).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_smart_refresh_token_not_found() {
        let cache = Arc::new(MemoryTokenCache::new(CacheConfig::default()));
        let refresher = TokenRefresher::new(
            "test_app_id".to_string(),
            "test_app_secret".to_string(),
            "https://open.feishu.cn".to_string(),
            cache.clone(),
        );

        let manager = RefreshManager::new(refresher, cache);

        let result = manager
            .smart_refresh_token("non_existent_key")
            .await
            .unwrap();
        assert!(result.is_none());
    }
}
