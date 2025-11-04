// 智能指针深度实践练习
// 基于Open-Lark ServiceRegistry的实际应用场景

use std::{
    collections::HashMap,
    sync::{Arc, RwLock, Mutex},
    thread,
    time::{Duration, Instant},
    any::Any,
};

// ==================== 练习1: Arc基础 ====================
// 目标：理解Arc如何在多线程间共享数据

#[derive(Debug, Clone)]
struct ServiceConfig {
    name: String,
    endpoint: String,
    timeout: Duration,
}

impl ServiceConfig {
    fn new(name: &str, endpoint: &str) -> Self {
        Self {
            name: name.to_string(),
            endpoint: endpoint.to_string(),
            timeout: Duration::from_secs(30),
        }
    }
}

// TODO: 完成ArcServiceManager的实现
// 要求：
// 1. 使用Arc<ServiceConfig>存储配置
// 2. 实现多线程安全的配置访问
// 3. 提供配置更新方法

struct ArcServiceManager {
    configs: HashMap<String, Arc<ServiceConfig>>,
}

impl ArcServiceManager {
    fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    // TODO: 实现添加配置方法
    fn add_config(&mut self, service: Arc<ServiceConfig>) {
        self.configs.insert(service.name.clone(), service);
    }

    // TODO: 实现获取配置方法
    // 返回Arc<ServiceConfig>，允许共享访问
    fn get_config(&self, name: &str) -> Option<Arc<ServiceConfig>> {
        self.configs.get(name).cloned()
    }

    // TODO: 实现并发访问测试
    fn test_concurrent_access(&self) {
        let mut handles = vec![];

        // 启动10个线程并发访问配置
        for i in 0..10 {
            let configs = self.configs.clone();
            let handle = thread::spawn(move || {
                // 模拟服务访问配置
                for (name, config) in &configs {
                    println!("线程{} 访问服务: {} -> {}",
                           i, name, config.endpoint);
                    // 模拟工作负载
                    thread::sleep(Duration::from_millis(10));
                }
            });
            handles.push(handle);
        }

        // 等待所有线程完成
        for handle in handles {
            handle.join().expect("线程应该正常完成");
        }
    }
}

// ==================== 练习2: RwLock深入 ====================
// 目标：掌握读写锁在并发场景下的使用

#[derive(Debug)]
struct ThreadSafeCache<K, V> {
    data: RwLock<HashMap<K, V>>,
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone + std::cmp::PartialEq> ThreadSafeCache<K, V> {
    fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }

    // TODO: 实现读取方法
    // 使用read()获取读锁，允许多个并发读者
    fn get(&self, key: &K) -> Option<V> {
        let data = self.data.read().expect("读锁应该可用");
        data.get(key).cloned()
    }

    // TODO: 实现写入方法
    // 使用write()获取写锁，独占访问
    fn insert(&self, key: K, value: V) {
        let mut data = self.data.write().expect("写锁应该可用");
        data.insert(key, value);
    }

    // TODO: 实现批量读取
    // 演示读锁的优势：多个读者可以并发访问
    fn batch_get(&self, keys: &[K]) -> HashMap<K, V> {
        let data = self.data.read().expect("读锁应该可用");
        keys.iter()
            .filter_map(|key| data.get(key).map(|v| (key.clone(), v.clone())))
            .collect()
    }

    // TODO: 实现条件更新
    // 只有在特定条件下才获取写锁
    fn update_if<F>(&self, key: K, update_fn: F, condition: V)
    where
        F: FnOnce(&V) -> V,
    {
        // 先读取检查条件
        if let Some(current) = self.get(&key) {
            if current == condition {
                // 获取写锁进行更新
                let mut data = self.data.write().expect("写锁应该可用");
                if let Some(value) = data.get_mut(&key) {
                    *value = update_fn(value);
                }
            }
        }
    }
}

// ==================== 练习3: Arc<RwLock<>>组合模式 ====================
// 目标：掌握企业级并发编程的核心模式

#[derive(Debug)]
struct ConcurrentServiceRegistry {
    services: RwLock<HashMap<String, Arc<dyn Any + Send + Sync>>>,
}

impl ConcurrentServiceRegistry {
    fn new() -> Self {
        Self {
            services: RwLock::new(HashMap::new()),
        }
    }

    // TODO: 实现服务注册
    // 使用Arc<Any>允许存储不同类型的服务
    fn register<T>(&self, name: String, service: T)
    where
        T: Send + Sync + 'static,
    {
        let mut services = self.services.write().expect("写锁应该可用");
        services.insert(name, Arc::new(service));
    }

    // TODO: 实现服务获取
    // 安全的类型向下转换
    fn get<T>(&self, name: &str) -> Option<Arc<T>>
    where
        T: Send + Sync + 'static,
    {
        let services = self.services.read().expect("读锁应该可用");
        services.get(name)
            .and_then(|any| any.clone().downcast::<T>().ok())
    }

    // TODO: 实现服务发现
    // 返回所有已注册的服务名称
    fn discover(&self) -> Vec<String> {
        let services = self.services.read().expect("读锁应该可用");
        services.keys().cloned().collect()
    }

    // TODO: 实现性能测试
    // 测试并发访问性能
    fn performance_test(&self, iterations: usize) -> Duration {
        let start = Instant::now();

        // 启动多个线程进行并发访问
        let mut handles = vec![];
        for i in 0..10 {
            let registry = self.clone();
            let handle = thread::spawn(move || {
                for _j in 0..iterations {
                    // 模拟服务查找
                    let _services = registry.discover();
                    // 模拟工作
                    thread::sleep(Duration::from_micros(1));
                }
                println!("线程{} 完成 {} 次迭代", i, iterations);
            });
            handles.push(handle);
        }

        // 等待所有线程完成
        for handle in handles {
            handle.join().expect("线程应该正常完成");
        }

        start.elapsed()
    }
}

// 需要为ConcurrentServiceRegistry实现Clone
impl Clone for ConcurrentServiceRegistry {
    fn clone(&self) -> Self {
        // 注意：这里只克隆结构体，不克隆内部数据
        // 实际应用中可能需要更复杂的逻辑
        Self {
            services: RwLock::new(HashMap::new()),
        }
    }
}

// ==================== 练习4: 内存管理和性能优化 ====================
// 目标：理解智能指针的内存开销和优化策略

#[derive(Debug)]
struct OptimizedServiceManager {
    // 使用不同的智能指针组合来优化性能
    read_heavy_data: Arc<RwLock<HashMap<String, String>>>,  // 读多写少
    write_heavy_data: Arc<Mutex<HashMap<String, i32>>>,    // 写多读少
    static_data: Arc<HashMap<String, bool>>,               // 只读数据
}

impl OptimizedServiceManager {
    fn new() -> Self {
        Self {
            read_heavy_data: Arc::new(RwLock::new(HashMap::new())),
            write_heavy_data: Arc::new(Mutex::new(HashMap::new())),
            static_data: Arc::new(HashMap::new()),
        }
    }

    // TODO: 为不同类型的数据选择合适的访问模式

    // 读多写少数据 - 使用RwLock
    fn get_read_heavy(&self, key: &str) -> Option<String> {
        let data = self.read_heavy_data.read().expect("读锁应该可用");
        data.get(key).cloned()
    }

    fn set_read_heavy(&self, key: String, value: String) {
        let mut data = self.read_heavy_data.write().expect("写锁应该可用");
        data.insert(key, value);
    }

    // 写多读少数据 - 使用Mutex
    fn increment_write_heavy(&self, key: String) -> i32 {
        let mut data = self.write_heavy_data.lock().expect("互斥锁应该可用");
        let counter = data.entry(key).or_insert(0);
        *counter += 1;
        *counter
    }

    // 只读数据 - 直接使用Arc
    fn get_static(&self, key: &str) -> Option<bool> {
        self.static_data.get(key).cloned()
    }

    // TODO: 实现内存使用分析
    fn analyze_memory_usage(&self) {
        println!("📊 内存使用分析:");
        println!("  read_heavy_data: {} bytes",
                std::mem::size_of_val(&*self.read_heavy_data));
        println!("  write_heavy_data: {} bytes",
                std::mem::size_of_val(&*self.write_heavy_data));
        println!("  static_data: {} bytes",
                std::mem::size_of_val(&*self.static_data));

        // Arc的引用计数
        println!("  read_heavy_data 引用计数: {}",
                Arc::strong_count(&self.read_heavy_data));
        println!("  write_heavy_data 引用计数: {}",
                Arc::strong_count(&self.write_heavy_data));
        println!("  static_data 引用计数: {}",
                Arc::strong_count(&self.static_data));
    }
}

// ==================== 测试用例 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arc_service_manager() {
        let mut manager = ArcServiceManager::new();
        let config = Arc::new(ServiceConfig::new("im", "https://api.example.com/im"));

        manager.add_config(config);

        let retrieved = manager.get_config("im");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().endpoint, "https://api.example.com/im");
    }

    #[test]
    fn test_thread_safe_cache() {
        let cache = Arc::new(ThreadSafeCache::new());

        // 测试写入
        cache.insert("key1".to_string(), "value1".to_string());
        cache.insert("key2".to_string(), "value2".to_string());

        // 测试读取
        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));

        // 测试批量读取
        let keys = vec!["key1".to_string(), "key2".to_string()];
        let results = cache.batch_get(&keys);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_concurrent_service_registry() {
        let registry = Arc::new(ConcurrentServiceRegistry::new());

        // 注册不同类型的服务
        registry.register("string_service".to_string(), "Hello World".to_string());
        registry.register("number_service".to_string(), 42i32);

        // 获取服务
        let string_service: Option<Arc<String>> = registry.get("string_service");
        assert!(string_service.is_some());
        assert_eq!(*string_service.unwrap(), "Hello World");

        let number_service: Option<Arc<i32>> = registry.get("number_service");
        assert!(number_service.is_some());
        assert_eq!(*number_service.unwrap(), 42);
    }

    #[test]
    fn test_optimized_service_manager() {
        let manager = OptimizedServiceManager::new();

        // 测试不同数据类型的访问
        manager.set_read_heavy("config".to_string(), "value".to_string());
        assert_eq!(manager.get_read_heavy("config"), Some("value".to_string()));

        let counter = manager.increment_write_heavy("counter".to_string());
        assert_eq!(counter, 1);

        manager.analyze_memory_usage();
    }
}

fn main() {
    println!("🧠 智能指针深度实践练习");

    // 演示ArcServiceManager
    println!("\n📚 练习1: Arc基础应用");
    let mut manager = ArcServiceManager::new();
    let im_config = Arc::new(ServiceConfig::new("im", "https://api.larksuite.com/im"));
    let contact_config = Arc::new(ServiceConfig::new("contact", "https://api.larksuite.com/contact"));

    manager.add_config(im_config);
    manager.add_config(contact_config);

    println!("配置添加完成，开始并发访问测试...");
    manager.test_concurrent_access();

    // 演示ThreadSafeCache
    println!("\n📚 练习2: RwLock缓存应用");
    let cache = Arc::new(ThreadSafeCache::new());

    // 添加一些缓存数据
    cache.insert("user:123".to_string(), "Alice".to_string());
    cache.insert("user:456".to_string(), "Bob".to_string());

    // 批量读取测试
    let keys = vec!["user:123".to_string(), "user:456".to_string()];
    let results = cache.batch_get(&keys);
    println!("批量读取结果: {:?}", results);

    // 演示ConcurrentServiceRegistry
    println!("\n📚 练习3: Arc<RwLock<>>组合模式");
    let registry = Arc::new(ConcurrentServiceRegistry::new());

    registry.register("database".to_string(), "MySQL".to_string());
    registry.register("cache".to_string(), "Redis".to_string());
    registry.register("queue".to_string(), "RabbitMQ".to_string());

    println!("已注册服务: {:?}", registry.discover());

    // 性能测试
    let duration = registry.performance_test(100);
    println!("性能测试完成，耗时: {:?}", duration);

    // 演示OptimizedServiceManager
    println!("\n📚 练习4: 内存管理优化");
    let manager = OptimizedServiceManager::new();

    manager.set_read_heavy("api_endpoint".to_string(), "https://api.larksuite.com".to_string());
    let counter = manager.increment_write_heavy("request_count".to_string());

    println!("计数器值: {}", counter);
    manager.analyze_memory_usage();

    println!("\n🎉 智能指针深度练习完成！");
}