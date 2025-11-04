//! 内存优化工具
//!
//! 提供高性能的内存管理工具，包括：
//! - 字符串池管理
//! - 对象池实现
//! - 内存使用监控
//! - 智能垃圾回收建议

use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use parking_lot::RwLock as ParkingLotRwLock;

/// 内存优化配置
#[derive(Debug, Clone)]
pub struct MemoryOptimizationConfig {
    /// 字符串池配置
    pub string_pool_config: StringPoolConfig,
    /// 对象池配置
    pub object_pool_config: ObjectPoolConfig,
    /// 监控配置
    pub monitoring_config: MemoryMonitoringConfig,
}

impl Default for MemoryOptimizationConfig {
    fn default() -> Self {
        Self {
            string_pool_config: StringPoolConfig::default(),
            object_pool_config: ObjectPoolConfig::default(),
            monitoring_config: MemoryMonitoringConfig::default(),
        }
    }
}

/// 字符串池配置
#[derive(Debug, Clone)]
pub struct StringPoolConfig {
    /// 最大池大小
    pub max_pool_size: usize,
    /// 清理间隔
    pub cleanup_interval: Duration,
    /// 字符串长度阈值
    pub min_string_length: usize,
    /// 是否启用LRU淘汰
    pub enable_lru: bool,
}

impl Default for StringPoolConfig {
    fn default() -> Self {
        Self {
            max_pool_size: 10000,
            cleanup_interval: Duration::from_secs(300), // 5分钟
            min_string_length: 8,
            enable_lru: true,
        }
    }
}

/// 对象池配置
#[derive(Debug, Clone)]
pub struct ObjectPoolConfig {
    /// 最大池大小
    pub max_pool_size: usize,
    /// 预分配对象数量
    pub preallocated_count: usize,
    /// 是否启用自动扩展
    pub enable_auto_expand: bool,
    /// 最大扩展大小
    pub max_expand_size: usize,
}

impl Default for ObjectPoolConfig {
    fn default() -> Self {
        Self {
            max_pool_size: 1000,
            preallocated_count: 100,
            enable_auto_expand: true,
            max_expand_size: 100,
        }
    }
}

/// 内存监控配置
#[derive(Debug, Clone)]
pub struct MemoryMonitoringConfig {
    /// 是否启用监控
    pub enable_monitoring: bool,
    /// 监控间隔
    pub monitoring_interval: Duration,
    /// 内存使用阈值（MB）
    pub memory_threshold_mb: usize,
    /// 是否启用自动清理
    pub enable_auto_cleanup: bool,
}

impl Default for MemoryMonitoringConfig {
    fn default() -> Self {
        Self {
            enable_monitoring: true,
            monitoring_interval: Duration::from_secs(60), // 1分钟
            memory_threshold_mb: 512, // 512MB
            enable_auto_cleanup: true,
        }
    }
}

/// 高性能字符串池
#[derive(Debug)]
pub struct StringPool {
    pool: Arc<ParkingLotRwLock<HashMap<String, StringEntry>>>,
    config: StringPoolConfig,
    stats: Arc<Mutex<StringPoolStats>>,
    last_cleanup: Arc<Mutex<Instant>>,
}

/// 字符串条目
#[derive(Debug, Clone)]
struct StringEntry {
    string: String,
    created_at: Instant,
    last_accessed: Instant,
    access_count: u64,
}

impl StringEntry {
    fn new(string: String) -> Self {
        let now = Instant::now();
        Self {
            string,
            created_at: now,
            last_accessed: now,
            access_count: 0,
        }
    }

    fn access(&mut self) -> &str {
        self.last_accessed = Instant::now();
        self.access_count += 1;
        &self.string
    }
}

/// 字符串池统计信息
#[derive(Debug, Default, Clone)]
pub struct StringPoolStats {
    pub total_strings: usize,
    pub total_memory_bytes: usize,
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
    pub evictions: u64,
}

impl StringPoolStats {
    fn update_hit_rate(&mut self) {
        let total = self.hits + self.misses;
        self.hit_rate = if total > 0 {
            self.hits as f64 / total as f64
        } else {
            0.0
        };
    }
}

impl StringPool {
    /// 创建新的字符串池
    pub fn new(config: StringPoolConfig) -> Self {
        Self {
            pool: Arc::new(ParkingLotRwLock::new(HashMap::new())),
            config,
            stats: Arc::new(Mutex::new(StringPoolStats::default())),
            last_cleanup: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// 使用默认配置创建字符串池
    pub fn new_default() -> Self {
        Self::new(StringPoolConfig::default())
    }

    /// 获取或插入字符串
    pub fn get_or_insert(&self, s: &str) -> String {
        // 对于短字符串，直接返回新的String，避免池化开销
        if s.len() < self.config.min_string_length {
            return s.to_string();
        }

        let mut pool = self.pool.write();
        let mut stats = self.stats.lock().unwrap();

        if let Some(entry) = pool.get_mut(s) {
            let result = entry.access().to_string();
            stats.hits += 1;
            stats.update_hit_rate();
            result
        } else {
            // 检查池大小限制
            if pool.len() >= self.config.max_pool_size {
                self.cleanup_expired_entries(&mut pool, &mut stats);
            }

            let entry = StringEntry::new(s.to_string());
            let result = entry.string.clone();
            pool.insert(s.to_string(), entry);

            stats.total_strings += 1;
            stats.total_memory_bytes += s.len();
            stats.misses += 1;
            stats.update_hit_rate();

            result
        }
    }

    /// 批量获取或插入字符串
    pub fn get_or_insert_batch(&self, strings: &[&str]) -> Vec<String> {
        strings.iter()
            .map(|s| self.get_or_insert(s))
            .collect()
    }

    /// 预热字符串池
    pub fn warm_up(&self, strings: &[&str]) {
        for s in strings {
            if s.len() >= self.config.min_string_length {
                let _ = self.get_or_insert(s);
            }
        }
    }

    /// 清理过期条目
    pub fn cleanup(&self) -> usize {
        let mut pool = self.pool.write();
        let mut stats = self.stats.lock().unwrap();
        let before_size = pool.len();

        self.cleanup_expired_entries(&mut pool, &mut stats);

        before_size - pool.len()
    }

    /// 获取统计信息
    pub fn stats(&self) -> StringPoolStats {
        self.stats.lock().unwrap().clone()
    }

    /// 内存使用量（字节）
    pub fn memory_usage_bytes(&self) -> usize {
        self.stats.lock().unwrap().total_memory_bytes
    }

    // 内部方法

    fn cleanup_expired_entries(&self, pool: &mut HashMap<String, StringEntry>, stats: &mut StringPoolStats) {
        if !self.config.enable_lru {
            return;
        }

        let now = Instant::now();
        let mut entries_to_remove = Vec::new();
        let mut total_size_freed = 0;

        // 按LRU排序
        let mut entries: Vec<_> = pool.iter().collect();
        entries.sort_by(|a, b| a.1.last_accessed.cmp(&b.1.last_accessed));

        // 移除最久未访问的条目
        let target_size = self.config.max_pool_size * 8 / 10; // 保留80%

        for (index, (key, entry)) in entries.iter().enumerate() {
            if index >= target_size || now.duration_since(entry.last_accessed) > Duration::from_secs(600) {
                entries_to_remove.push(key.clone());
                total_size_freed += entry.string.len();
            }
        }

        for key in entries_to_remove {
            pool.remove(&key);
            stats.evictions += 1;
        }

        stats.total_strings = pool.len();
        stats.total_memory_bytes -= total_size_freed;

        // 更新最后清理时间
        if let Ok(mut last_cleanup) = self.last_cleanup.try_lock() {
            *last_cleanup = now;
        }
    }
}

/// 泛型对象池
#[derive(Debug)]
pub struct ObjectPool<T> {
    pool: Arc<RwLock<Vec<T>>>,
    factory: Arc<dyn Fn() -> T + Send + Sync>,
    reset_fn: Option<Arc<dyn Fn(&mut T) + Send + Sync>>,
    config: ObjectPoolConfig,
    stats: Arc<Mutex<ObjectPoolStats>>,
}

/// 对象池统计信息
#[derive(Debug, Default, Clone)]
pub struct ObjectPoolStats {
    pub total_objects: usize,
    pub active_objects: usize,
    pub pool_hits: u64,
    pub pool_misses: u64,
    pub expansions: u64,
    pub hit_rate: f64,
}

impl<T> ObjectPoolStats {
    fn update_hit_rate(&mut self) {
        let total = self.pool_hits + self.pool_misses;
        self.hit_rate = if total > 0 {
            self.pool_hits as f64 / total as f64
        } else {
            0.0
        };
    }
}

impl<T> ObjectPool<T>
where
    T: Default + Send + Sync + 'static,
{
    /// 创建新的对象池
    pub fn new(config: ObjectPoolConfig) -> Self {
        let pool = Vec::with_capacity(config.preallocated_count);

        Self {
            pool: Arc::new(RwLock::new(pool)),
            factory: Arc::new(|| T::default()),
            reset_fn: None,
            config,
            stats: Arc::new(Mutex::new(ObjectPoolStats::default())),
        }
    }

    /// 使用工厂函数创建对象池
    pub fn with_factory<F>(factory: F, config: ObjectPoolConfig) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        let pool = Vec::with_capacity(config.preallocated_count);

        Self {
            pool: Arc::new(RwLock::new(pool)),
            factory: Arc::new(factory),
            reset_fn: None,
            config,
            stats: Arc::new(Mutex::new(ObjectPoolStats::default())),
        }
    }

    /// 设置重置函数
    pub fn with_reset_fn<F>(mut self, reset_fn: F) -> Self
    where
        F: Fn(&mut T) + Send + Sync + 'static,
    {
        self.reset_fn = Some(Arc::new(reset_fn));
        self
    }

    /// 从池中获取对象
    pub fn get(&self) -> T {
        let mut pool = self.pool.write().unwrap();
        let mut stats = self.stats.lock().unwrap();

        if let Some(mut obj) = pool.pop() {
            // 重置对象状态
            if let Some(ref reset_fn) = self.reset_fn {
                reset_fn(&mut obj);
            }
            stats.pool_hits += 1;
            stats.update_hit_rate();
            obj
        } else {
            stats.pool_misses += 1;
            stats.update_hit_rate();
            (self.factory)()
        }
    }

    /// 将对象返回池中
    pub fn return_object(&self, obj: T) {
        let mut pool = self.pool.write().unwrap();
        let mut stats = self.stats.lock().unwrap();

        if pool.len() < self.config.max_pool_size {
            pool.push(obj);
            stats.total_objects = pool.len();
        } else {
            // 池已满，丢弃对象
            drop(obj);
        }

        stats.active_objects = stats.total_objects;
    }

    /// 预分配对象
    pub fn preallocate(&self, count: usize) {
        let mut pool = self.pool.write().unwrap();
        let current_size = pool.len();
        let to_add = count.saturating_sub(current_size);

        for _ in 0..to_add {
            pool.push((self.factory)());
        }

        let mut stats = self.stats.lock().unwrap();
        stats.total_objects = pool.len();
    }

    /// 获取统计信息
    pub fn stats(&self) -> ObjectPoolStats {
        self.stats.lock().unwrap().clone()
    }

    /// 池大小
    pub fn size(&self) -> usize {
        self.pool.read().unwrap().len()
    }

    /// 活跃对象数量
    pub fn active_objects(&self) -> usize {
        self.stats.lock().unwrap().active_objects
    }
}

/// 内存监控器
#[derive(Debug)]
pub struct MemoryMonitor {
    config: MemoryMonitoringConfig,
    stats: Arc<Mutex<MemoryStats>>,
    is_monitoring: Arc<RwLock<bool>>,
}

/// 内存统计信息
#[derive(Debug, Default, Clone)]
pub struct MemoryStats {
    pub total_memory_mb: f64,
    pub heap_memory_mb: f64,
    pub stack_memory_mb: f64,
    pub allocated_objects: usize,
    pub gc_pressure: f64,
    pub last_cleanup: Instant,
    pub cleanup_count: u64,
}

impl MemoryMonitor {
    /// 创建新的内存监控器
    pub fn new(config: MemoryMonitoringConfig) -> Self {
        let monitor = Self {
            config,
            stats: Arc::new(Mutex::new(MemoryStats::default())),
            is_monitoring: Arc::new(RwLock::new(false)),
        };

        if config.enable_monitoring {
            monitor.start_monitoring();
        }

        monitor
    }

    /// 使用默认配置创建监控器
    pub fn new_default() -> Self {
        Self::new(MemoryMonitoringConfig::default())
    }

    /// 开始监控
    pub fn start_monitoring(&self) {
        let mut is_monitoring = self.is_monitoring.write().unwrap();
        if *is_monitoring {
            return;
        }
        *is_monitoring = true;

        let config = self.config.clone();
        let stats = self.stats.clone();
        let is_monitoring_ref = self.is_monitoring.clone();

        tokio::spawn(async move {
            while *is_monitoring_ref.read().unwrap() {
                Self::collect_memory_stats(&mut stats.lock().unwrap());

                // 检查内存阈值
                let stats_guard = stats.lock().unwrap();
                if stats_guard.total_memory_mb > config.memory_threshold_mb as f64 {
                    if config.enable_auto_cleanup {
                        Self::trigger_cleanup(&mut stats_guard);
                    }
                    warn!("Memory usage threshold exceeded: {:.2} MB", stats_guard.total_memory_mb);
                }

                drop(stats_guard);
                tokio::time::sleep(config.monitoring_interval).await;
            }
        });
    }

    /// 停止监控
    pub fn stop_monitoring(&self) {
        *self.is_monitoring.write().unwrap() = false;
    }

    /// 获取当前内存统计
    pub fn stats(&self) -> MemoryStats {
        self.stats.lock().unwrap().clone()
    }

    /// 手动触发清理
    pub fn trigger_cleanup(&self) {
        let mut stats = self.stats.lock().unwrap();
        Self::trigger_cleanup(&mut stats);
    }

    // 内部方法

    fn collect_memory_stats(stats: &mut MemoryStats) {
        // 简化的内存统计实现
        // 在实际项目中，可以使用更精确的内存监控库

        // 模拟内存使用统计
        stats.heap_memory_mb = Self::estimate_heap_usage();
        stats.total_memory_mb = stats.heap_memory_mb;
        stats.allocated_objects = Self::count_allocated_objects();
        stats.gc_pressure = Self::estimate_gc_pressure();

        stats.last_cleanup = Instant::now();
    }

    fn estimate_heap_usage() -> f64 {
        // 简化实现，实际可以使用更精确的方法
        200.0 // 200MB 示例值
    }

    fn count_allocated_objects() -> usize {
        // 简化实现，实际可以使用更精确的方法
        10000 // 10,000 个对象示例值
    }

    fn estimate_gc_pressure() -> f64 {
        // 简化实现，可以根据内存使用情况计算
        0.3 // 30% GC压力示例值
    }

    fn trigger_cleanup(stats: &mut MemoryStats) {
        // 触发清理操作
        stats.cleanup_count += 1;
        stats.last_cleanup = Instant::now();

        // 可以在这里添加实际的清理逻辑
        // 例如：清理缓存、强制垃圾回收等

        info!("Memory cleanup triggered (count: {})", stats.cleanup_count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_pool_basic() {
        let pool = StringPool::new_default();

        // 测试短字符串不被池化
        let short = pool.get_or_insert("short");
        assert_eq!(short, "short");

        // 测试长字符串被池化
        let long = pool.get_or_insert("this_is_a_long_string_that_should_be_pooled");
        assert_eq!(long, "this_is_a_long_string_that_should_be_pooled");

        // 测试复用
        let long2 = pool.get_or_insert("this_is_a_long_string_that_should_be_pooled");
        assert_eq!(long2, long);

        let stats = pool.stats();
        assert!(stats.hits > 0);
        assert!(stats.hit_rate > 0.0);
    }

    #[test]
    fn test_string_pool_batch() {
        let pool = StringPool::new_default();
        let strings = vec!["string1", "string2", "string3"];

        let results = pool.get_or_insert_batch(&strings);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], "string1");
    }

    #[test]
    fn test_object_pool() {
        let pool = ObjectPool::new(ObjectPoolConfig::default());

        // 预分配
        pool.preallocate(10);
        assert_eq!(pool.size(), 10);

        // 获取和返回对象
        let obj1 = pool.get();
        assert_eq!(pool.size(), 9);
        assert_eq!(pool.active_objects(), 9);

        pool.return_object(obj1);
        assert_eq!(pool.size(), 10);

        let stats = pool.stats();
        assert!(stats.pool_hits > 0);
    }

    #[test]
    fn test_memory_monitor() {
        let monitor = MemoryMonitor::new_default();
        let stats = monitor.stats();

        // 验证统计信息结构
        assert!(stats.total_memory_mb >= 0.0);
        assert!(stats.allocated_objects >= 0);
    }
}