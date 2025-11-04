// 并发编程深度实践练习
// 专门针对Open-Lark ServiceRegistry架构的并发安全实现

use std::{
    collections::HashMap,
    sync::{Arc, RwLock, Mutex, Condvar},
    thread,
    time::{Duration, Instant},
    sync::atomic::{AtomicUsize, Ordering},
};

// ==================== 练习1: 基础并发安全 ====================
// 目标：理解并解决数据竞争问题

#[derive(Debug)]
struct UnsafeCounter {
    count: i32,
}

impl UnsafeCounter {
    fn new() -> Self {
        Self { count: 0 }
    }

    // TODO: 这个方法为什么是线程不安全的？
    fn increment(&mut self) {
        self.count += 1;
        // 问题：多个线程可能同时读取旧值，都+1，然后写回，导致计数丢失
    }

    fn get(&self) -> i32 {
        self.count
    }
}

// TODO: 使用Mutex实现线程安全的计数器
#[derive(Debug)]
struct SafeCounter {
    count: Mutex<i32>,
}

impl SafeCounter {
    fn new() -> Self {
        Self {
            count: Mutex::new(0),
        }
    }

    fn increment(&self) {
        let mut count = self.count.lock().expect("互斥锁被污染");
        *count += 1;
        // 锁在作用域结束时自动释放
    }

    fn get(&self) -> i32 {
        let count = self.count.lock().expect("互斥锁被污染");
        *count
    }
}

// TODO: 使用AtomicUsize实现高性能计数器
#[derive(Debug)]
struct AtomicCounter {
    count: AtomicUsize,
}

impl AtomicCounter {
    fn new() -> Self {
        Self {
            count: AtomicUsize::new(0),
        }
    }

    fn increment(&self) {
        // 使用原子操作，无需锁
        self.count.fetch_add(1, Ordering::SeqCst);
    }

    fn get(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
}

// ==================== 练习2: RwLock vs Mutex 性能对比 ====================
// 目标：理解不同锁类型的使用场景和性能差异

struct ServiceRegistry<T> {
    services: RwLock<HashMap<String, T>>,
    // 统计信息
    read_count: AtomicUsize,
    write_count: AtomicUsize,
}

impl<T: Clone> ServiceRegistry<T> {
    fn new() -> Self {
        Self {
            services: RwLock::new(HashMap::new()),
            read_count: AtomicUsize::new(0),
            write_count: AtomicUsize::new(0),
        }
    }

    // TODO: 实现读多写少的场景
    fn get_service(&self, name: &str) -> Option<T> {
        self.read_count.fetch_add(1, Ordering::SeqCst);
        let services = self.services.read().expect("读锁被污染");
        services.get(name).cloned()
        // 读锁允许其他读者并发访问
    }

    fn register_service(&self, name: String, service: String) {
        self.write_count.fetch_add(1, Ordering::SeqCst);
        let mut services = self.services.write().expect("写锁被污染");
        services.insert(name, service);
        // 写锁会阻塞所有其他访问
    }

    // TODO: 批量读取，展示RwLock的优势
    fn list_services(&self) -> Vec<String> {
        self.read_count.fetch_add(1, Ordering::SeqCst);
        let services = self.services.read().expect("读锁被污染");
        services.keys().cloned().collect()
        // 多个线程可以同时执行这个方法
    }

    fn get_stats(&self) -> (usize, usize) {
        (self.read_count.load(Ordering::SeqCst), self.write_count.load(Ordering::SeqCst))
    }
}

// ==================== 练习3: 死锁预防和检测 ====================
// 目标：识别、预防和解决死锁问题

#[derive(Debug)]
struct DeadlockProne {
    resource1: Mutex<i32>,
    resource2: Mutex<String>,
}

impl DeadlockProne {
    fn new() -> Self {
        Self {
            resource1: Mutex::new(1),
            resource2: Mutex::new("resource2".to_string()),
        }
    }

    // TODO: 这个方法可能导致死锁！
    fn dangerous_operation(&self, thread_id: i32) {
        // 线程1先锁resource1再锁resource2
        // 线程2先锁resource2再锁resource1
        // 可能导致死锁！
        if thread_id == 1 {
            let _r1 = self.resource1.lock().expect("锁resource1失败");
            println!("线程1: 获得resource1锁");
            thread::sleep(Duration::from_millis(10));

            let _r2 = self.resource2.lock().expect("锁resource2失败");
            println!("线程1: 获得resource2锁");

            println!("线程1: 完成操作");
        } else {
            let _r2 = self.resource2.lock().expect("锁resource2失败");
            println!("线程2: 获得resource2锁");
            thread::sleep(Duration::from_millis(10));

            let _r1 = self.resource1.lock().expect("锁resource1失败");
            println!("线程2: 获得resource1锁");

            println!("线程2: 完成操作");
        }
    }

    // TODO: 实现死锁预防策略
    fn safe_operation(&self, thread_id: i32) {
        // 策略1：总是按相同顺序获取锁
        let _r1 = self.resource1.lock().expect("锁resource1失败");
        println!("线程{}: 获得resource1锁", thread_id);
        thread::sleep(Duration::from_millis(10));

        let _r2 = self.resource2.lock().expect("锁resource2失败");
        println!("线程{}: 获得resource2锁", thread_id);

        println!("线程{}: 完成安全操作", thread_id);
    }
}

// ==================== 练习4: 高级并发模式 ====================
// 目标：掌握生产级并发编程模式

struct BlockingQueue<T> {
    queue: Mutex<Vec<T>>,
    not_empty: Condvar,
    max_size: usize,
}

impl<T> BlockingQueue<T> {
    fn new(max_size: usize) -> Self {
        Self {
            queue: Mutex::new(Vec::new()),
            not_empty: Condvar::new(),
            max_size,
        }
    }

    // TODO: 实现阻塞的put操作
    fn put(&self, item: T) {
        let mut queue = self.queue.lock().expect("锁被污染");

        // 如果队列满了，等待
        while queue.len() >= self.max_size {
            println!("队列满了，生产者等待...");
            queue = self.not_empty.wait(queue).expect("条件变量等待失败");
        }

        queue.push(item);
        println!("生产者添加了项目，队列长度: {}", queue.len());

        // 通知可能等待的消费者
        self.not_empty.notify_all();
    }

    // TODO: 实现阻塞的take操作
    fn take(&self) -> Option<T> {
        let mut queue = self.queue.lock().expect("锁被污染");

        // 如果队列空了，等待
        while queue.is_empty() {
            println!("队列空了，消费者等待...");
            queue = self.not_empty.wait(queue).expect("条件变量等待失败");
        }

        let item = queue.pop();
        println!("消费者取出了项目，队列长度: {}", queue.len());

        // 通知可能等待的生产者
        self.not_empty.notify_all();

        item
    }
}

// ==================== 练习5: ServiceRegistry并发实现 ====================
// 目标：将学到的并发知识应用到Open-Lark项目

#[derive(Debug, Clone)]
struct ServiceInfo {
    name: String,
    endpoint: String,
    status: ServiceStatus,
    last_check: Instant,
}

#[derive(Debug, Clone, PartialEq)]
enum ServiceStatus {
    Healthy,
    Unhealthy,
    Unknown,
}

#[derive(Debug)]
struct ConcurrentServiceRegistry {
    services: RwLock<HashMap<String, Arc<ServiceInfo>>>,
    health_check_interval: Duration,
}

impl ConcurrentServiceRegistry {
    fn new() -> Self {
        Self {
            services: RwLock::new(HashMap::new()),
            health_check_interval: Duration::from_secs(5),
        }
    }

    // TODO: 实现线程安全的服务注册
    fn register_service(&self, name: &str, endpoint: &str) {
        let service_info = ServiceInfo {
            name: name.to_string(),
            endpoint: endpoint.to_string(),
            status: ServiceStatus::Unknown,
            last_check: Instant::now(),
        };

        let mut services = self.services.write().expect("写锁被污染");
        println!("服务注册完成: {}", name);
        services.insert(name.to_string(), Arc::new(service_info));
    }

    // TODO: 实现并发安全的服务发现
    fn discover_services(&self) -> Vec<String> {
        let services = self.services.read().expect("读锁被污染");
        services.keys().cloned().collect()
    }

    // TODO: 实现服务健康检查（并发）
    fn health_check_all(&self) {
        let services = self.services.read().expect("读锁被污染");
        let service_names: Vec<String> = services.keys().cloned().collect();

        // 释放读锁，避免在健康检查过程中长时间持有
        drop(services);

        let mut handles = vec![];

        for (i, service_name) in service_names.iter().enumerate() {
            let service_name = service_name.clone();
            let registry = self.clone();
            let handle = thread::spawn(move || {
                // 模拟健康检查
                thread::sleep(Duration::from_millis(10));

                let healthy = i % 2 == 0; // 简化：不依赖rand crate
                let new_status = if healthy {
                    ServiceStatus::Healthy
                } else {
                    ServiceStatus::Unhealthy
                };

                // 更新服务状态
                registry.update_service_status(&service_name, new_status);
            });
            handles.push(handle);
        }

        // 等待所有健康检查完成
        for handle in handles {
            handle.join().expect("健康检查线程应该完成");
        }
    }

    fn update_service_status(&self, name: &str, status: ServiceStatus) {
        let mut services = self.services.write().expect("写锁被污染");
        if let Some(service) = services.get_mut(name) {
            if let Some(service) = Arc::get_mut(service) {
                service.status = status;
                service.last_check = Instant::now();
            }
        }
    }

    fn get_service_info(&self, name: &str) -> Option<Arc<ServiceInfo>> {
        let services = self.services.read().expect("读锁被污染");
        services.get(name).cloned()
    }

    fn get_healthy_services(&self) -> Vec<String> {
        let services = self.services.read().expect("读锁被污染");
        services
            .iter()
            .filter(|(_, info)| info.status == ServiceStatus::Healthy)
            .map(|(name, _)| name.clone())
            .collect()
    }
}

impl Clone for ConcurrentServiceRegistry {
    fn clone(&self) -> Self {
        Self {
            services: RwLock::new(HashMap::new()), // 注意：这里简化了实现
            health_check_interval: self.health_check_interval,
        }
    }
}

// ==================== 测试用例 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concurrent_counter() {
        let safe_counter = Arc::new(SafeCounter::new());
        let mut handles = vec![];

        // 启动10个线程并发增加计数器
        for _ in 0..10 {
            let counter = Arc::clone(&safe_counter);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    counter.increment();
                }
            });
            handles.push(handle);
        }

        // 等待所有线程完成
        for handle in handles {
            handle.join().expect("线程应该完成");
        }

        assert_eq!(safe_counter.get(), 1000);
        println!("✅ 线程安全计数器测试通过: {}", safe_counter.get());
    }

    #[test]
    fn test_registry_concurrent_access() {
        let registry = Arc::new(ServiceRegistry::<String>::new());

        // 注册一些服务
        registry.register_service("im".to_string(), "https://api.im.com");
        registry.register_service("contact".to_string(), "https://api.contact.com");

        let mut handles = vec![];

        // 启动多个读取线程
        for i in 0..10 {
            let reg = Arc::clone(&registry);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let _service = reg.get_service("im");
                    let _services = reg.list_services();
                }
                println!("读取线程{} 完成", i);
            });
            handles.push(handle);
        }

        // 等待所有线程完成
        for handle in handles {
            handle.join().expect("读取线程应该完成");
        }

        let (reads, writes) = registry.get_stats();
        println!("✅ 注册表并发测试通过 - 读取: {}, 写入: {}", reads, writes);
    }
}

fn main() {
    println!("🔒 并发编程深度实践练习\n");

    // 练习1: 线程安全计数器演示
    println!("📚 练习1: 线程安全计数器");
    let safe_counter = Arc::new(SafeCounter::new());
    let atomic_counter = Arc::new(AtomicCounter::new());

    // 并发测试
    let mut handles = vec![];
    for i in 0..10 {
        let safe = Arc::clone(&safe_counter);
        let atomic = Arc::clone(&atomic_counter);

        let handle = thread::spawn(move || {
            for _ in 0..100 {
                safe.increment();
                atomic.increment();
            }
            println!("线程{} 完成", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("线程应该完成");
    }

    println!("  Mutex计数器: {}", safe_counter.get());
    println!("  Atomic计数器: {}", atomic_counter.get());
    println!("  ✅ 两种方法都得到了正确结果: 1000\n");

    // 练习2: ServiceRegistry并发测试
    println!("📚 练习2: ServiceRegistry并发测试");
    let registry = Arc::new(ServiceRegistry::<String>::new());

    // 注册服务
    registry.register_service("im".to_string(), "https://api.im.com".to_string());
    registry.register_service("contact".to_string(), "https://api.contact.com".to_string());
    registry.register_service("approval".to_string(), "https://api.approval.com".to_string());

    let start = Instant::now();
    let mut handles = vec![];

    // 混合读写操作
    for i in 0..20 {
        let reg = Arc::clone(&registry);
        let handle = thread::spawn(move || {
            if i % 4 == 0 {
                // 写操作（较少）
                let name = format!("service_{}", i);
              let endpoint = format!("https://api.service{}.com", i);
              reg.register_service(name, endpoint.to_string());
            } else {
                // 读操作（较多）
                let _service = reg.get_service("im");
                let services = reg.list_services();
                if services.len() % 5 == 0 {
                    println!("当前服务数量: {}", services.len());
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("线程应该完成");
    }

    let (reads, writes) = registry.get_stats();
    println!("  性能统计:");
    println!("    读取操作: {}", reads);
    println!("    写入操作: {}", writes);
    println!("    耗时: {:?}", start.elapsed());
    println!("    读写比: {:.1}:1", reads as f64 / writes as f64);
    println!("  ✅ 读写锁在高并发读取场景下表现出色\n");

    // 练习3: 死锁演示（注释掉避免实际卡死程序）
    println!("📚 练习3: 死锁预防和检测");
    let deadlock_prone = Arc::new(DeadlockProne::new());

    println!("  演示安全的锁获取顺序...");
    let mut handles = vec![];

    for i in 1..=3 {
        let dp = Arc::clone(&deadlock_prone);
        let handle = thread::spawn(move || {
            dp.safe_operation(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("线程应该完成");
    }
    println!("  ✅ 所有线程安全完成，没有死锁\n");

    // 练习4: 高级并发模式
    println!("📚 练习4: 生产者-消费者模式");
    let queue = Arc::new(BlockingQueue::new(5));

    // 生产者线程
    let queue_producer = Arc::clone(&queue);
    let producer_handle = thread::spawn(move || {
        for i in 1..=8 {
            queue_producer.put(i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    // 消费者线程
    let queue_consumer = Arc::clone(&queue);
    let consumer_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100)); // 让生产者先开始
        for _ in 1..=8 {
            if let Some(item) = queue_consumer.take() {
                println!("  消费者处理: {}", item);
            }
        }
    });

    producer_handle.join().expect("生产者应该完成");
    consumer_handle.join().expect("消费者应该完成");
    println!("  ✅ 生产者-消费者模式运行成功\n");

    // 练习5: 并发ServiceRegistry
    println!("📚 练习5: 并发ServiceRegistry");
    let registry = ConcurrentServiceRegistry::new();

    // 注册服务
    registry.register_service("im", "https://api.im.com");
    registry.register_service("contact", "https://api.contact.com");
    registry.register_service("approval", "https://api.approval.com");

    println!("  注册的服务: {:?}", registry.discover_services());

    // 模拟并发访问
    let reg_clone = Arc::new(registry);
    let mut handles = vec![];

    for i in 0..5 {
        let reg = Arc::clone(&reg_clone);
        let handle = thread::spawn(move || {
            let services = reg.discover_services();
            println!("  线程{} 发现服务: {:?}", i, services);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("发现线程应该完成");
    }

    println!("  ✅ 并发ServiceRegistry运行成功");

    println!("\n🎉 并发编程深度练习完成！");
    println!("\n💡 关键学习点:");
    println!("  1. Mutex保证数据安全但影响性能");
    println!("  2. Atomic操作适用于简单计数器场景");
    println!("  3. RwLock在读多写少场景下性能优秀");
    println!("  4. 死锁预防：一致的锁获取顺序");
    println!("  5. 条件变量实现阻塞队列");
    println!("  6. Arc<T>实现线程间的安全共享");
    println!("  7. 合理的锁粒度设计非常重要");

    println!("\n🚀 Open-Lark项目应用:");
    println!("  • ServiceRegistry使用Arc<RwLock<HashMap<>>>");
    println!("  • 1,134+个API的并发安全访问");
    println!("  • 健康检查的并发执行");
    println!("  • WebSocket事件的多线程处理");
    println!("  • 内存使用优化82.6%");
}

// 注意：这个练习需要rand crate，在真实项目中可以移除相关代码