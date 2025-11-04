// ServiceRegistry POC 实现
// 基于Codex评估修正后的版本

use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, RwLock},
};

/// 统一的服务特征
pub trait Service: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn is_available(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any;
}

/// 辅助 trait，避免调用方需要硬编码字符串
pub trait NamedService: Service + Sized {
    fn name_static() -> Option<&'static str> { Some(Self::NAME) }
    const NAME: &'static str;
    fn clone_owned(&self) -> Self;
}

/// 线程安全的服务注册表
pub struct ServiceRegistry {
    services: RwLock<HashMap<&'static str, Arc<dyn Service>>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: RwLock::new(HashMap::new()),
        }
    }

    /// 注册新服务
    pub fn register<S>(&self, service: S)
    where
        S: Service,
    {
        let mut map = self.services.write().expect("poisoned lock");
        map.insert(service.name(), Arc::new(service));
    }

    /// 获取服务实例（类型安全）
    pub fn get<S>(&self) -> Option<Arc<S>>
    where
        S: Service + NamedService,
    {
        let map = self.services.read().ok()?;
        let entry = map.get(S::name_static()?)?;
        let concrete = entry
            .as_any()
            .downcast_ref::<S>()
            .expect("duplicate name with different type");
        Some(Arc::new(concrete.clone_owned()))
    }

    /// 动态服务发现
    pub fn discover_services(&self) -> Vec<&'static str> {
        let map = self.services.read().expect("poisoned lock");
        map.keys().cloned().collect()
    }
}

impl<T> Service for T
where
    T: NamedService,
{
    fn name(&self) -> &'static str { Self::NAME }
    fn version(&self) -> &'static str { "1.0" }
    fn as_any(&self) -> &dyn Any { self }
}

// 示例服务实现
#[derive(Debug, Clone)]
pub struct MockImService {
    name: &'static str,
}

impl MockImService {
    pub fn new() -> Self {
        Self { name: "im" }
    }
}

impl NamedService for MockImService {
    const NAME: &'static str = "im";

    fn clone_owned(&self) -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct MockContactService {
    name: &'static str,
}

impl MockContactService {
    pub fn new() -> Self {
        Self { name: "contact" }
    }
}

impl NamedService for MockContactService {
    const NAME: &'static str = "contact";

    fn clone_owned(&self) -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_service_registry_basic_functionality() {
        let registry = ServiceRegistry::new();

        // 注册服务
        let im_service = MockImService::new();
        let contact_service = MockContactService::new();

        registry.register(im_service);
        registry.register(contact_service);

        // 验证服务发现
        let services = registry.discover_services();
        assert_eq!(services.len(), 2);
        assert!(services.contains(&"im"));
        assert!(services.contains(&"contact"));

        // 验证类型安全获取
        let im: Option<Arc<MockImService>> = registry.get();
        assert!(im.is_some());

        let contact: Option<Arc<MockContactService>> = registry.get();
        assert!(contact.is_some());
    }

    #[test]
    fn test_concurrent_access() {
        let registry = Arc::new(ServiceRegistry::new());

        // 注册服务
        registry.register(MockImService::new());
        registry.register(MockContactService::new());

        let mut handles = vec![];

        // 启动多个线程并发访问
        for i in 0..10 {
            let reg_clone = Arc::clone(&registry);
            let handle = thread::spawn(move || {
                // 随机获取服务
                if i % 2 == 0 {
                    let _im: Option<Arc<MockImService>> = reg_clone.get();
                } else {
                    let _contact: Option<Arc<MockContactService>> = reg_clone.get();
                }

                // 服务发现
                let _services = reg_clone.discover_services();
            });
            handles.push(handle);
        }

        // 等待所有线程完成
        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }
    }

    #[test]
    fn test_performance_baseline() {
        let registry = ServiceRegistry::new();
        registry.register(MockImService::new());
        registry.register(MockContactService::new());

        let start = std::time::Instant::now();

        // 执行大量操作
        for _ in 0..10000 {
            let _im: Option<Arc<MockImService>> = registry.get();
            let _contact: Option<Arc<MockContactService>> = registry.get();
            let _services = registry.discover_services();
        }

        let duration = start.elapsed();
        println!("10K 次操作耗时: {:?}", duration);

        // 基本性能断言（应该很快）
        assert!(duration.as_millis() < 1000); // 小于1秒
    }
}

fn main() {
    println!("ServiceRegistry POC 测试");

    let registry = ServiceRegistry::new();
    registry.register(MockImService::new());
    registry.register(MockContactService::new());

    println!("已注册服务: {:?}", registry.discover_services());

    // 测试类型安全获取
    if let Some(im_service) = registry.get::<MockImService>() {
        println!("成功获取 IM 服务: {:?}", im_service.name());
    }

    if let Some(contact_service) = registry.get::<MockContactService>() {
        println!("成功获取 Contact 服务: {:?}", contact_service.name());
    }
}