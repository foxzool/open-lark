//! 认证缓存实现

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tokio::time::{interval, sleep};
use tracing::{debug, warn};

use super::token::TokenInfo;

/// 缓存配置
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// 缓存容量
    pub max_size: usize,
    /// 缓存TTL（生存时间）
    pub ttl: Duration,
    /// 清理间隔
    pub cleanup_interval: Duration,
    /// 是否启用统计信息
    pub enable_stats: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size: 1000,
            ttl: Duration::from_secs(3600),             // 1小时
            cleanup_interval: Duration::from_secs(300), // 5分钟
            enable_stats: true,
        }
    }
}

/// 缓存统计信息
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// 缓存命中次数
    pub hits: u64,
    /// 缓存未命中次数
    pub misses: u64,
    /// 缓存清理次数
    pub cleanups: u64,
    /// 当前缓存大小
    pub current_size: usize,
}

/// 缓存条目
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct CacheEntry {
    token: TokenInfo,
    created_at: SystemTime,
    expires_at: SystemTime,
}

impl CacheEntry {
    fn new(token: TokenInfo, ttl: Duration) -> Self {
        let now = SystemTime::now();
        let expires_at = now + ttl;

        Self {
            token,
            created_at: now,
            expires_at,
        }
    }

    fn is_expired(&self) -> bool {
        SystemTime::now() >= self.expires_at
    }
}

/// 内存令牌缓存
pub struct MemoryTokenCache {
    config: CacheConfig,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    stats: Arc<RwLock<CacheStats>>,
    cleanup_handle: Option<tokio::task::JoinHandle<()>>,
}

impl MemoryTokenCache {
    /// 创建新的内存令牌缓存
    pub fn new(config: CacheConfig) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(CacheStats::default())),
            config,
            cleanup_handle: None,
        }
    }

    /// 启动清理任务
    pub async fn start_cleanup_task(&mut self) {
        if self.cleanup_handle.is_some() {
            warn!("Cleanup task already started");
            return;
        }

        let cache = self.cache.clone();
        let stats = self.stats.clone();
        let cleanup_interval = self.config.cleanup_interval;

        let handle = tokio::spawn(async move {
            let mut interval = interval(cleanup_interval);
            loop {
                interval.tick().await;
                Self::cleanup_expired_entries(&cache, &stats).await;
            }
        });

        self.cleanup_handle = Some(handle);
    }

    /// 清理过期条目
    async fn cleanup_expired_entries(
        cache: &Arc<RwLock<HashMap<String, CacheEntry>>>,
        stats: &Arc<RwLock<CacheStats>>,
    ) {
        let mut cache_guard = cache.write().await;
        let before_size = cache_guard.len();

        // 移除过期条目
        cache_guard.retain(|_, entry| !entry.is_expired());

        let after_size = cache_guard.len();

        // 更新统计信息
        if before_size != after_size {
            let mut stats_guard = stats.write().await;
            stats_guard.cleanups += 1;
            stats_guard.current_size = after_size;
        }

        debug!(
            "Cleaned up {} expired cache entries, current size: {}",
            before_size - after_size,
            after_size
        );
    }

    /// 添加令牌到缓存
    pub async fn put(&self, key: &str, token: TokenInfo) {
        let cache = self.cache.clone();
        let stats = self.stats.clone();
        let key_owned = key.to_string();

        // 直接执行而不是spawn，确保操作立即完成（修复测试问题）
        {
            let mut cache_guard = cache.write().await;
            let mut stats_guard = stats.write().await;

            // 检查容量限制
            if cache_guard.len() >= self.config.max_size {
                // 简单策略：移除最旧的条目
                if let Some(oldest_key) = cache_guard
                    .iter()
                    .min_by_key(|(_, entry)| entry.created_at)
                    .map(|(k, _)| k.clone())
                {
                    cache_guard.remove(&oldest_key);
                    stats_guard.current_size = cache_guard.len();
                }
            }

            // 添加新条目
            let entry = CacheEntry::new(token, self.config.ttl);
            cache_guard.insert(key_owned, entry.clone());

            // 更新统计
            stats_guard.current_size = cache_guard.len();

            debug!("Token cached with key: {}", key);
        }
    }

    /// 带TTL添加令牌到缓存
    pub async fn put_with_ttl(&self, key: &str, token: TokenInfo, ttl: Duration) {
        let cache = self.cache.clone();
        let stats = self.stats.clone();
        let key_owned = key.to_string();

        {
            let mut cache_guard = cache.write().await;
            let mut stats_guard = stats.write().await;

            // 检查容量限制
            if cache_guard.len() >= self.config.max_size {
                // 简单策略：移除最旧的条目
                if let Some(oldest_key) = cache_guard
                    .iter()
                    .min_by_key(|(_, entry)| entry.created_at)
                    .map(|(k, _)| k.clone())
                {
                    cache_guard.remove(&oldest_key);
                    stats_guard.current_size = cache_guard.len();
                }
            }

            // 添加新条目
            let entry = CacheEntry::new(token, ttl);
            cache_guard.insert(key_owned, entry.clone());

            // 更新统计
            stats_guard.current_size = cache_guard.len();

            debug!(
                "Token cached with key: {} (custom TTL: {}s)",
                key,
                ttl.as_secs()
            );
        }
    }

    /// 从缓存获取令牌
    pub async fn get(&self, key: &str) -> Option<TokenInfo> {
        let cache = self.cache.clone();
        let stats = self.stats.clone();

        {
            let cache_guard = cache.read().await;
            let mut stats_guard = stats.write().await;

            if let Some(entry) = cache_guard.get(key) {
                if entry.is_expired() {
                    // 移除过期条目
                    drop(cache_guard);
                    let mut cache_guard = cache.write().await;
                    cache_guard.remove(key);
                    stats_guard.current_size = cache_guard.len();
                    stats_guard.misses += 1;
                    debug!("Cache miss (expired) for key: {}", key);
                    None
                } else {
                    stats_guard.hits += 1;
                    debug!("Cache hit for key: {}", key);
                    Some(entry.token.clone())
                }
            } else {
                stats_guard.misses += 1;
                debug!("Cache miss for key: {}", key);
                None
            }
        }
    }

    /// 移除缓存条目
    pub async fn remove(&self, key: &str) -> Option<TokenInfo> {
        let cache = self.cache.clone();
        let stats = self.stats.clone();

        {
            let mut cache_guard = cache.write().await;
            let removed = cache_guard.remove(key);

            if removed.is_some() {
                let mut stats_guard = stats.write().await;
                stats_guard.current_size = cache_guard.len();
            }

            debug!("Cache removed for key: {}", key);
            removed.map(|entry| entry.token)
        }
    }

    /// 清空所有缓存
    pub async fn clear(&self) {
        let cache = self.cache.clone();
        let stats = self.stats.clone();

        {
            let mut cache_guard = cache.write().await;
            let before_size = cache_guard.len();
            cache_guard.clear();

            let mut stats_guard = stats.write().await;
            stats_guard.current_size = 0;

            debug!("Cache cleared, removed {} entries", before_size);
        }
    }

    /// 获取缓存大小
    pub async fn size(&self) -> usize {
        let cache = self.cache.clone();
        let cache_guard = cache.read().await;
        cache_guard.len()
    }

    /// 检查缓存是否为空
    pub async fn is_empty(&self) -> bool {
        let cache = self.cache.clone();
        let cache_guard = cache.read().await;
        cache_guard.is_empty()
    }

    /// 检查是否包含指定key
    pub async fn contains(&self, key: &str) -> bool {
        let cache = self.cache.clone();
        let cache_guard = cache.read().await;
        cache_guard.contains_key(key)
    }

    /// 获取所有缓存键
    pub async fn keys(&self) -> Vec<String> {
        let cache = self.cache.read().await;
        cache.keys().cloned().collect()
    }

    /// 获取缓存统计信息
    pub async fn stats(&self) -> CacheStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// 清理过期条目
    pub async fn cleanup_expired(&self) -> usize {
        let mut cache_guard = self.cache.write().await;
        let before_size = cache_guard.len();

        // 移除过期条目
        let expired_keys: Vec<String> = cache_guard
            .iter()
            .filter(|(_, entry)| entry.is_expired())
            .map(|(k, _)| k.clone())
            .collect();

        for key in expired_keys {
            cache_guard.remove(&key);
        }

        let after_size = cache_guard.len();
        before_size - after_size
    }

    /// 验证缓存一致性
    pub async fn validate(&self) -> Result<usize, String> {
        let cache = self.cache.clone();
        let cache_guard = cache.read().await;
        let size = cache_guard.len();

        // 检查是否有过期条目
        let expired_count = cache_guard
            .values()
            .filter(|entry| entry.is_expired())
            .count();

        if expired_count > 0 {
            return Err(format!("Found {} expired cache entries", expired_count));
        }

        Ok(size)
    }
}

impl Drop for MemoryTokenCache {
    fn drop(&mut self) {
        if let Some(handle) = self.cleanup_handle.take() {
            handle.abort();
        }
    }
}

/// 缓存存储特征
pub trait TokenStorage: Send + Sync {
    /// 存储令牌
    async fn store_token(&self, key: &str, token: TokenInfo) -> Result<(), String>;

    /// 获取令牌
    async fn get_token(&self, key: &str) -> Option<TokenInfo>;

    /// 移除令牌
    async fn remove_token(&self, key: &str) -> Option<TokenInfo>;

    /// 清空所有令牌
    async fn clear_tokens(&self) -> Result<(), String>;

    /// 获取缓存大小
    async fn size(&self) -> usize;
}

/// 令牌缓存特征
pub trait TokenCache: TokenStorage {
    /// 带TTL存储令牌
    async fn store_token_with_ttl(
        &self,
        key: &str,
        token: TokenInfo,
        ttl: Duration,
    ) -> Result<(), String>;
}

impl TokenStorage for MemoryTokenCache {
    async fn store_token(&self, key: &str, token: TokenInfo) -> Result<(), String> {
        self.put(key, token).await;
        Ok(())
    }

    async fn get_token(&self, key: &str) -> Option<TokenInfo> {
        self.get(key).await
    }

    async fn remove_token(&self, key: &str) -> Option<TokenInfo> {
        self.remove(key).await
    }

    async fn clear_tokens(&self) -> Result<(), String> {
        self.clear().await;
        Ok(())
    }

    async fn size(&self) -> usize {
        self.size().await
    }
}

impl TokenCache for MemoryTokenCache {
    async fn store_token_with_ttl(
        &self,
        key: &str,
        token: TokenInfo,
        ttl: Duration,
    ) -> Result<(), String> {
        self.put_with_ttl(key, token, ttl).await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn create_test_token() -> TokenInfo {
        TokenInfo::new(
            "test_token".to_string(),
            crate::auth::token::TokenType::AppAccessToken,
            Duration::from_secs(3600),
            "test_app".to_string(),
        )
    }

    #[tokio::test]
    async fn test_cache_basic_operations() {
        let config = CacheConfig::default();
        let cache = MemoryTokenCache::new(config);

        // 测试空缓存
        assert_eq!(cache.size().await, 0);
        assert!(cache.is_empty().await);
        assert!(!cache.contains("test").await);

        // 测试存储和获取
        let token = create_test_token();
        cache.put("test", token.clone()).await;

        assert_eq!(cache.size().await, 1);
        assert!(!cache.is_empty().await);
        assert!(cache.contains("test").await);

        let retrieved = cache.get("test").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().access_token, token.access_token);

        // 测试移除
        let removed = cache.remove("test").await;
        assert!(removed.is_some());
        assert_eq!(cache.size().await, 0);

        // 测试统计信息
        let stats = cache.stats().await;
        assert_eq!(stats.current_size, 0);
    }

    #[tokio::test]
    async fn test_cache_ttl() {
        let config = CacheConfig {
            ttl: Duration::from_millis(100), // 很短的TTL用于测试
            ..Default::default()
        };
        let cache = MemoryTokenCache::new(config);

        let token = create_test_token();
        cache.put("test", token.clone()).await;

        // 立即获取应该成功
        let retrieved1 = cache.get("test").await;
        assert!(retrieved1.is_some());

        // 等待过期
        sleep(Duration::from_millis(150)).await;

        // 现在应该返回None（过期）
        let retrieved2 = cache.get("test").await;
        assert!(retrieved2.is_none());
    }

    #[tokio::test]
    async fn test_cache_max_size() {
        let config = CacheConfig {
            max_size: 2,
            ..Default::default()
        };
        let cache = MemoryTokenCache::new(config);

        let token1 = create_test_token();
        let token2 = create_test_token();
        let token3 = create_test_token();

        cache.put("test1", token1).await;
        cache.put("test2", token2).await;
        assert_eq!(cache.size().await, 2);

        // 添加第三个token，应该移除最旧的
        cache.put("test3", token3).await;
        assert_eq!(cache.size().await, 2);

        // test1应该被移除（最旧的）
        assert!(!cache.contains("test1").await);
        assert!(cache.contains("test2").await);
        assert!(cache.contains("test3").await);
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let config = CacheConfig::default();
        let cache = MemoryTokenCache::new(config);

        // 初始统计
        let stats = cache.stats().await;
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.current_size, 0);

        let token = create_test_token();
        cache.put("test", token).await;

        // 第一次获取（命中）
        let _ = cache.get("test").await;
        let stats = cache.stats().await;
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 0);

        // 获取不存在的key（未命中）
        let _ = cache.get("nonexistent").await;
        let stats = cache.stats().await;
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
    }
}
