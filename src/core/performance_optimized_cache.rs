//! 性能优化的缓存实现
//!
//! 提供高性能、内存优化的缓存机制，支持：
//! - 自适应缓存容量
//! - LRU淘汰策略
//! - 内存使用监控
//! - 智能预热机制

use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::{Arc, RwLock};
use tokio::time::sleep;

/// 缓存配置
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// 初始容量
    pub initial_capacity: usize,
    /// 最大容量
    pub max_capacity: usize,
    /// 默认过期时间（秒）
    pub default_ttl: u64,
    /// 是否启用自适应调整
    pub enable_adaptive: bool,
    /// 内存使用阈值（字节）
    pub memory_threshold: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            initial_capacity: 1000,
            max_capacity: 10000,
            default_ttl: 300, // 5分钟
            enable_adaptive: true,
            memory_threshold: 100 * 1024 * 1024, // 100MB
        }
    }
}

/// 缓存条目
#[derive(Debug, Clone)]
struct CacheEntry<T: Clone> {
    value: T,
    created_at: Instant,
    expires_at: Instant,
    access_count: u64,
    last_accessed: Instant,
    size_bytes: usize,
}

impl<T: Clone> CacheEntry<T> {
    fn new(value: T, ttl: u64) -> Self {
        let now = Instant::now();
        let size_bytes = std::mem::size_of::<T>() + std::mem::size_of::<Self>();

        Self {
            value,
            created_at: now,
            expires_at: now + Duration::from_secs(ttl),
            access_count: 0,
            last_accessed: now,
            size_bytes,
        }
    }

    fn is_expired(&self) -> bool {
        self.expires_at <= Instant::now()
    }

    fn access(&mut self) -> &T {
        self.access_count += 1;
        self.last_accessed = Instant::now();
        &self.value
    }
}

/// 性能优化的缓存
#[derive(Debug)]
pub struct PerformanceCache<T: Clone> {
    entries: Arc<RwLock<HashMap<String, CacheEntry<T>>>>,
    config: CacheConfig,
    stats: Arc<RwLock<CacheStats>>,
    lru_list: Arc<RwLock<Vec<String>>>, // LRU跟踪
}

/// 缓存统计信息
#[derive(Debug, Default, Clone)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub sets: u64,
    pub evictions: u64,
    pub total_memory_bytes: usize,
    pub entry_count: usize,
    pub hit_rate: f64,
}

impl<T: Clone> PerformanceCache<T> {
    /// 创建新的性能优化缓存
    pub fn new(config: CacheConfig) -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::with_capacity(config.initial_capacity))),
            config,
            stats: Arc::new(RwLock::new(CacheStats::default())),
            lru_list: Arc::new(RwLock::new(Vec::with_capacity(config.initial_capacity))),
        }
    }

    /// 使用默认配置创建缓存
    pub fn new_default() -> Self {
        Self::new(CacheConfig::default())
    }

    /// 获取缓存值
    pub fn get(&self, key: &str) -> Option<T> {
        let mut entries = self.entries.write().unwrap();
        let mut lru_list = self.lru_list.write().unwrap();
        let mut stats = self.stats.write().unwrap();

        if let Some(entry) = entries.get_mut(key) {
            if entry.is_expired() {
                // 移除过期条目
                entries.remove(key);
                lru_list.retain(|k| k != key);
                stats.misses += 1;
                self.update_hit_rate(&mut stats);
                return None;
            }

            // 更新访问信息和LRU
            let value = entry.access().clone();
            self.update_lru(&mut lru_list, key);
            stats.hits += 1;
            self.update_hit_rate(&mut stats);

            Some(value)
        } else {
            stats.misses += 1;
            self.update_hit_rate(&mut stats);
            None
        }
    }

    /// 设置缓存值
    pub fn set(&self, key: &str, value: T, ttl: Option<u64>) {
        let ttl = ttl.unwrap_or(self.config.default_ttl);
        let entry = CacheEntry::new(value, ttl);

        let mut entries = self.entries.write().unwrap();
        let mut lru_list = self.lru_list.write().unwrap();
        let mut stats = self.stats.write().unwrap();

        // 检查是否需要淘汰
        if entries.len() >= self.config.max_capacity {
            self.evict_lru_entries(&mut entries, &mut lru_list, &mut stats);
        }

        // 插入新条目
        let old_size = entries.get(key).map(|e| e.size_bytes).unwrap_or(0);
        entries.insert(key.to_string(), entry);
        self.update_lru(&mut lru_list, key);

        // 更新统计信息
        stats.total_memory_bytes = stats.total_memory_bytes - old_size + entry.size_bytes;
        stats.entry_count = entries.len();
        stats.sets += 1;

        // 自适应调整（如果启用）
        if self.config.enable_adaptive {
            self.adaptive_if_needed(&entries, &stats);
        }
    }

    /// 获取值和过期信息
    pub fn get_with_expiry(&self, key: &str) -> Option<(T, Instant)> {
        let mut entries = self.entries.write().unwrap();
        let mut lru_list = self.lru_list.write().unwrap();
        let mut stats = self.stats.write().unwrap();

        if let Some(entry) = entries.get_mut(key) {
            if entry.is_expired() {
                entries.remove(key);
                lru_list.retain(|k| k != key);
                stats.misses += 1;
                self.update_hit_rate(&mut stats);
                return None;
            }

            let value = entry.access().clone();
            let expires_at = entry.expires_at;
            self.update_lru(&mut lru_list, key);
            stats.hits += 1;
            self.update_hit_rate(&mut stats);

            Some((value, expires_at))
        } else {
            stats.misses += 1;
            self.update_hit_rate(&mut stats);
            None
        }
    }

    /// 批量获取
    pub fn get_batch(&self, keys: &[&str]) -> HashMap<String, T> {
        let mut results = HashMap::new();
        let mut entries = self.entries.write().unwrap();
        let mut lru_list = self.lru_list.write().unwrap();
        let mut stats = self.stats.write().unwrap();

        for key in keys {
            if let Some(entry) = entries.get_mut(key) {
                if entry.is_expired() {
                    entries.remove(key);
                    lru_list.retain(|k| k != key);
                    continue;
                }

                let value = entry.access().clone();
                self.update_lru(&mut lru_list, key);
                results.insert(key.to_string(), value);
                stats.hits += 1;
            } else {
                stats.misses += 1;
            }
        }

        self.update_hit_rate(&mut stats);
        results
    }

    /// 批量设置
    pub fn set_batch(&self, items: &[(&str, T)], ttl: Option<u64>) {
        let ttl = ttl.unwrap_or(self.config.default_ttl);
        let mut entries = self.entries.write().unwrap();
        let mut lru_list = self.lru_list.write().unwrap();
        let mut stats = self.stats.write().unwrap();

        // 预先检查容量并淘汰
        let needed_capacity = entries.len() + items.len();
        if needed_capacity > self.config.max_capacity {
            let evict_count = needed_capacity - self.config.max_capacity +
                (self.config.max_capacity / 10); // 额外10%空间
            self.evict_lru_entries_by_count(&mut entries, &mut lru_list, &mut stats, evict_count);
        }

        let mut total_size_delta = 0;
        for (key, value) in items {
            let entry = CacheEntry::new(value.clone(), ttl);
            let old_size = entries.get(*key).map(|e| e.size_bytes).unwrap_or(0);

            entries.insert(key.to_string(), entry);
            self.update_lru(&mut lru_list, key);

            total_size_delta += entry.size_bytes - old_size;
            stats.sets += 1;
        }

        stats.total_memory_bytes += total_size_delta;
        stats.entry_count = entries.len();

        if self.config.enable_adaptive {
            self.adaptive_if_needed(&entries, &stats);
        }
    }

    /// 清除过期条目
    pub fn cleanup_expired(&self) -> usize {
        let mut entries = self.entries.write().unwrap();
        let mut lru_list = self.lru_list.write().unwrap();
        let mut stats = self.stats.write().unwrap();

        let now = Instant::now();
        let mut expired_keys = Vec::new();
        let mut total_size_freed = 0;

        for (key, entry) in entries.iter() {
            if entry.expires_at <= now {
                expired_keys.push(key.clone());
                total_size_freed += entry.size_bytes;
            }
        }

        for key in &expired_keys {
            entries.remove(key);
            lru_list.retain(|k| k != key);
        }

        stats.total_memory_bytes -= total_size_freed;
        stats.entry_count = entries.len();

        expired_keys.len()
    }

    /// 获取缓存统计信息
    pub fn stats(&self) -> CacheStats {
        self.stats.read().unwrap().clone()
    }

    /// 预热缓存
    pub async fn warm_up<F, Fut>(&self, keys: Vec<String>, loader: F)
    where
        F: Fn(String) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Option<T>> + Send + 'static,
    {
        const BATCH_SIZE: usize = 10;
        const BATCH_DELAY: Duration = Duration::from_millis(10);

        for chunk in keys.chunks(BATCH_SIZE) {
            let mut batch_results = Vec::new();

            for key in chunk {
                if let Some(result) = loader(key.clone()).await {
                    batch_results.push((key.clone(), result));
                }

                // 小延迟避免过载
                sleep(Duration::from_millis(1)).await;
            }

            // 批量插入
            if !batch_results.is_empty() {
                let refs: Vec<(&str, T)> = batch_results.iter()
                    .map(|(k, v)| (k.as_str(), v.clone()))
                    .collect();
                self.set_batch(&refs, Some(self.config.default_ttl));
            }

            // 批次间延迟
            if chunk.len() == BATCH_SIZE {
                sleep(BATCH_DELAY).await;
            }
        }
    }

    // 内部辅助方法

    fn update_lru(&self, lru_list: &mut Vec<String>, key: &str) {
        // 移除旧位置
        lru_list.retain(|k| k != key);
        // 添加到末尾（最近使用）
        lru_list.push(key.to_string());
    }

    fn evict_lru_entries(&self, entries: &mut HashMap<String, CacheEntry<T>>,
                        lru_list: &mut Vec<String>, stats: &mut CacheStats) {
        let evict_count = self.config.max_capacity / 10; // 淘汰10%

        for _ in 0..evict_count.min(lru_list.len()) {
            if let Some(key) = lru_list.first() {
                let key = key.clone();
                if let Some(entry) = entries.remove(&key) {
                    stats.total_memory_bytes -= entry.size_bytes;
                    stats.evictions += 1;
                }
                lru_list.remove(0);
            }
        }

        stats.entry_count = entries.len();
    }

    fn evict_lru_entries_by_count(&self, entries: &mut HashMap<String, CacheEntry<T>>,
                                   lru_list: &mut Vec<String>, stats: &mut CacheStats, count: usize) {
        for _ in 0..count.min(lru_list.len()) {
            if let Some(key) = lru_list.first() {
                let key = key.clone();
                if let Some(entry) = entries.remove(&key) {
                    stats.total_memory_bytes -= entry.size_bytes;
                    stats.evictions += 1;
                }
                lru_list.remove(0);
            }
        }

        stats.entry_count = entries.len();
    }

    fn update_hit_rate(&self, stats: &mut CacheStats) {
        let total = stats.hits + stats.misses;
        stats.hit_rate = if total > 0 {
            stats.hits as f64 / total as f64
        } else {
            0.0
        };
    }

    fn adaptive_if_needed(&self, entries: &HashMap<String, CacheEntry<T>>, stats: &CacheStats) {
        // 如果内存使用超过阈值，减少缓存容量
        if stats.total_memory_bytes > self.config.memory_threshold {
            // 可以触发紧急清理或调整策略
            log::warn!("Cache memory usage exceeded threshold: {} bytes", stats.total_memory_bytes);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[tokio::test]
    async fn test_performance_cache_basic() {
        let cache = PerformanceCache::new_default();

        // 测试设置和获取
        cache.set("key1", "value1".to_string(), Some(60));
        assert_eq!(cache.get("key1"), Some("value1".to_string()));

        // 测试过期
        cache.set("expire_key", "expire_value".to_string(), Some(1));
        thread::sleep(Duration::from_secs(2));
        assert_eq!(cache.get("expire_key"), None);
    }

    #[tokio::test]
    async fn test_batch_operations() {
        let cache = PerformanceCache::new_default();

        // 批量设置
        let items = vec![
            ("key1", "value1".to_string()),
            ("key2", "value2".to_string()),
            ("key3", "value3".to_string()),
        ];
        cache.set_batch(&items, Some(60));

        // 批量获取
        let keys = vec!["key1", "key2", "key3"];
        let results = cache.get_batch(&keys);
        assert_eq!(results.len(), 3);
        assert_eq!(results.get("key1"), Some(&"value1".to_string()));
    }

    #[test]
    fn test_cache_stats() {
        let cache = PerformanceCache::new_default();

        cache.set("test", "value".to_string(), Some(60));
        assert_eq!(cache.get("test"), Some("value".to_string()));
        assert_eq!(cache.get("nonexistent"), None);

        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert!(stats.hit_rate > 0.0);
    }

    #[tokio::test]
    async fn test_warm_up() {
        let cache = PerformanceCache::new_default();
        let keys = vec!["key1".to_string(), "key2".to_string(), "key3".to_string()];

        async fn loader(key: String) -> Option<String> {
            Some(format!("loaded_{}", key))
        }

        cache.warm_up(keys.clone(), loader).await;

        for key in keys {
            assert_eq!(cache.get(&key), Some(format!("loaded_{}", key)));
        }
    }
}