//! 缓存管理器

use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tracing::{debug, info, warn};

use crate::auth::cache::{CacheConfig, CacheStats, MemoryTokenCache, TokenCache};
use crate::auth::token::TokenInfo;
use crate::error::{AuthError, AuthResult};

/// 缓存管理器
///
/// 负责令牌缓存的管理、统计和优化
pub struct CacheManager {
    cache: Arc<MemoryTokenCache>,
    config: CacheConfig,
}

impl CacheManager {
    /// 创建新的缓存管理器
    pub fn new(config: CacheConfig) -> Self {
        let cache = Arc::new(MemoryTokenCache::new(config.clone()));

        Self { cache, config }
    }

    /// 获取缓存的令牌
    pub async fn get_token(&self, key: &str) -> AuthResult<Option<TokenInfo>> {
        debug!("Getting token from cache: {}", key);
        Ok(self.cache.get(key).await)
    }

    /// 缓存令牌
    pub async fn put_token(&self, key: &str, token: TokenInfo) -> AuthResult<()> {
        debug!("Putting token into cache: {}", key);
        self.cache.put(key, token).await;
        Ok(())
    }

    /// 缓存令牌带TTL
    pub async fn put_token_with_ttl(
        &self,
        key: &str,
        token: TokenInfo,
        ttl: Duration,
    ) -> AuthResult<()> {
        debug!(
            "Putting token into cache with TTL {}: {}",
            ttl.as_secs(),
            key
        );
        self.cache.put_with_ttl(key, token, ttl).await;
        Ok(())
    }

    /// 移除缓存的令牌
    pub async fn remove_token(&self, key: &str) -> AuthResult<()> {
        debug!("Removing token from cache: {}", key);
        self.cache.remove(key).await;
        Ok(())
    }

    /// 检查令牌是否存在于缓存中
    pub async fn contains_token(&self, key: &str) -> bool {
        debug!("Checking if token exists in cache: {}", key);
        self.cache.contains(key).await
    }

    /// 清空所有缓存
    pub async fn clear_cache(&self) -> AuthResult<()> {
        debug!("Clearing all cache");
        self.cache.clear().await;
        info!("All cache cleared successfully");
        Ok(())
    }

    /// 获取缓存统计信息
    pub async fn get_cache_stats(&self) -> CacheStats {
        self.cache.stats().await
    }

    /// 清理过期令牌
    pub async fn cleanup_expired_tokens(&self) -> AuthResult<usize> {
        debug!("Cleaning up expired tokens");
        let count = self.cache.cleanup_expired().await;
        info!("Cleaned up {} expired tokens", count);
        Ok(count)
    }

    /// 获取缓存大小
    pub async fn get_cache_size(&self) -> usize {
        self.cache.size().await
    }

    /// 检查缓存是否为空
    pub async fn is_cache_empty(&self) -> bool {
        self.cache.is_empty().await
    }

    /// 批量获取令牌
    pub async fn batch_get_tokens(&self, keys: &[String]) -> Vec<AuthResult<Option<TokenInfo>>> {
        debug!("Batch getting {} tokens from cache", keys.len());
        let mut results = Vec::with_capacity(keys.len());

        for key in keys {
            let result = self.cache.get(key).await;
            results.push(Ok(result));
        }

        debug!(
            "Batch get completed: {} successes, {} failures",
            results.iter().filter(|r| r.is_ok()).count(),
            results.iter().filter(|r| r.is_err()).count()
        );

        results
    }

    /// 批量缓存令牌
    pub async fn batch_put_tokens(&self, token_pairs: Vec<(String, TokenInfo)>) -> AuthResult<()> {
        debug!("Batch putting {} tokens into cache", token_pairs.len());

        for (key, token) in token_pairs {
            self.cache.put(&key, token).await;
        }

        info!("Batch put completed");
        Ok(())
    }

    /// 获取缓存命中率
    pub async fn get_hit_rate(&self) -> f64 {
        let stats = self.get_cache_stats().await;
        if stats.hits + stats.misses == 0 {
            0.0
        } else {
            stats.hits as f64 / (stats.hits + stats.misses) as f64
        }
    }

    /// 预热缓存
    pub async fn warmup_cache(&self, tokens: Vec<(String, TokenInfo)>) -> AuthResult<()> {
        info!("Warming up cache with {} tokens", tokens.len());

        for (key, token) in tokens {
            self.cache.put(&key, token).await;
        }

        info!("Cache warmup completed");
        Ok(())
    }

    /// 获取所有缓存键
    pub async fn get_all_cache_keys(&self) -> Vec<String> {
        debug!("Getting all cache keys");
        self.cache.keys().await
    }

    /// 根据模式获取缓存键
    pub async fn get_cache_keys_by_pattern(&self, pattern: &str) -> Vec<String> {
        debug!("Getting cache keys matching pattern: {}", pattern);
        let keys = self.cache.keys().await;
        keys.into_iter()
            .filter(|key| key.contains(pattern))
            .collect()
    }

    /// 根据模式删除缓存项
    pub async fn remove_cache_by_pattern(&self, pattern: &str) -> AuthResult<usize> {
        debug!("Removing cache items matching pattern: {}", pattern);
        let keys = self.get_cache_keys_by_pattern(pattern).await;
        let mut removed_count = 0;

        for key in keys {
            self.cache.remove(&key).await;
            removed_count += 1;
        }

        info!(
            "Removed {} cache items matching pattern: {}",
            removed_count, pattern
        );
        Ok(removed_count)
    }

    /// 获取缓存配置
    pub fn get_config(&self) -> &CacheConfig {
        &self.config
    }

    /// 更新缓存配置
    pub async fn update_config(&mut self, config: CacheConfig) -> AuthResult<()> {
        info!("Updating cache configuration");

        // 保存现有缓存数据
        let keys = self.cache.keys().await;
        let mut cached_data = Vec::new();

        for key in keys {
            if let Some(token) = self.cache.get(&key).await {
                cached_data.push((key, token));
            }
        }

        // 创建新缓存
        self.cache = Arc::new(MemoryTokenCache::new(config.clone()));
        self.config = config;

        // 恢复缓存数据
        for (key, token) in cached_data {
            self.cache.put(&key, token).await;
        }

        info!("Cache configuration updated successfully");
        Ok(())
    }

    /// 获取缓存性能指标
    pub async fn get_performance_metrics(&self) -> CachePerformanceMetrics {
        let stats = self.get_cache_stats().await;

        CachePerformanceMetrics {
            total_items: stats.current_size as u64,
            hit_count: stats.hits,
            miss_count: stats.misses,
            hit_rate: self.get_hit_rate().await,
            memory_usage: self.get_memory_usage().await,
            oldest_token_age: self.get_oldest_token_age().await,
        }
    }

    /// 估算内存使用量
    async fn get_memory_usage(&self) -> usize {
        // 简单的内存使用估算
        let size = self.cache.size().await;
        // 假设每个TokenInfo平均占用512字节
        size * 512
    }

    /// 获取最旧令牌的年龄
    async fn get_oldest_token_age(&self) -> Option<Duration> {
        // 这里需要实际实现来获取最旧令牌的创建时间
        // 目前返回None
        None
    }
}

/// 缓存性能指标
#[derive(Debug, Clone)]
pub struct CachePerformanceMetrics {
    /// 缓存项总数
    pub total_items: u64,
    /// 命中次数
    pub hit_count: u64,
    /// 未命中次数
    pub miss_count: u64,
    /// 命中率
    pub hit_rate: f64,
    /// 内存使用量（字节）
    pub memory_usage: usize,
    /// 最旧令牌年龄
    pub oldest_token_age: Option<Duration>,
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_cache_manager_creation() {
        let config = CacheConfig::default();
        let manager = CacheManager::new(config);

        let stats = manager.get_cache_stats().await;
        assert_eq!(stats.current_size, 0);
    }

    #[tokio::test]
    async fn test_cache_put_and_get() {
        let config = CacheConfig::default();
        let manager = CacheManager::new(config);

        let token = TokenInfo::new(
            "test_token".to_string(),
            crate::auth::token::TokenType::AppAccessToken,
            Duration::from_secs(3600),
            "test_app".to_string(),
        );

        // 缓存令牌
        manager.put_token("test_key", token.clone()).await.unwrap();

        // 获取令牌
        let cached_token = manager.get_token("test_key").await.unwrap();
        assert!(cached_token.is_some());
        assert_eq!(cached_token.unwrap().access_token, token.access_token);
    }

    #[tokio::test]
    async fn test_batch_operations() {
        let config = CacheConfig::default();
        let manager = CacheManager::new(config);

        let tokens = vec![
            (
                "key1".to_string(),
                TokenInfo::new(
                    "token1".to_string(),
                    crate::auth::token::TokenType::AppAccessToken,
                    Duration::from_secs(3600),
                    "test_app".to_string(),
                ),
            ),
            (
                "key2".to_string(),
                TokenInfo::new(
                    "token2".to_string(),
                    crate::auth::token::TokenType::AppAccessToken,
                    Duration::from_secs(3600),
                    "test_app".to_string(),
                ),
            ),
        ];

        // 批量缓存
        manager.batch_put_tokens(tokens.clone()).await.unwrap();

        // 批量获取
        let keys = tokens.iter().map(|(k, _)| k.clone()).collect::<Vec<_>>();
        let results = manager.batch_get_tokens(&keys).await;

        assert_eq!(results.len(), 2);
        assert!(results
            .iter()
            .all(|r| r.is_ok() && r.as_ref().unwrap().is_some()));
    }

    #[tokio::test]
    async fn test_cache_pattern_operations() {
        let config = CacheConfig::default();
        let manager = CacheManager::new(config);

        // 添加不同模式的键
        manager
            .put_token(
                "app_access_token",
                TokenInfo::new(
                    "app_token".to_string(),
                    crate::auth::token::TokenType::AppAccessToken,
                    Duration::from_secs(3600),
                    "test_app".to_string(),
                ),
            )
            .await
            .unwrap();

        manager
            .put_token(
                "tenant_access_token:tenant1",
                TokenInfo::new(
                    "tenant_token".to_string(),
                    crate::auth::token::TokenType::TenantAccessToken,
                    Duration::from_secs(3600),
                    "test_app".to_string(),
                ),
            )
            .await
            .unwrap();

        // 测试模式匹配
        let app_keys = manager.get_cache_keys_by_pattern("app_access").await;
        assert_eq!(app_keys.len(), 1);
        assert_eq!(app_keys[0], "app_access_token");

        let tenant_keys = manager.get_cache_keys_by_pattern("tenant_access").await;
        assert_eq!(tenant_keys.len(), 1);
        assert_eq!(tenant_keys[0], "tenant_access_token:tenant1");
    }
}
