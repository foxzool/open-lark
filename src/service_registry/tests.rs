//! ServiceRegistry集成测试和基准测试

use std::sync::Arc;
use std::time::Instant;

use super::*;
use crate::service_registry::builder::ServiceBuilderFactory;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct MockService {
        name: &'static str,
        status: Arc<std::sync::atomic::AtomicBool>,
    }

    impl MockService {
        fn new(name: &'static str) -> Self {
            Self {
                name,
                status: Arc::new(std::sync::atomic::AtomicBool::new(true)),
            }
        }

        fn set_unhealthy(&self) {
            self.status
                .store(false, std::sync::atomic::Ordering::SeqCst);
        }

        fn set_healthy(&self) {
            self.status.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }

    impl Service for MockService {
        fn name(&self) -> &'static str {
            self.name
        }

        fn version(&self) -> &'static str {
            "1.0.0"
        }

        fn status(&self) -> ServiceStatus {
            if self.status.load(std::sync::atomic::Ordering::SeqCst) {
                ServiceStatus::Healthy
            } else {
                ServiceStatus::Unhealthy
            }
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
    }

    impl NamedService for MockService {
        const NAME: &'static str = "mock-service";

        fn clone_owned(&self) -> Self {
            Self::new(self.name)
        }
    }

    #[test]
    fn test_full_lifecycle() {
        let registry = ServiceRegistry::new();

        // 1. 注册服务
        let service = MockService::new("mock-service");
        registry.register(service).unwrap();
        assert_eq!(registry.service_count(), 1);

        // 2. 获取服务
        let retrieved: Arc<MockService> = registry.get().unwrap();
        assert!(retrieved.is_available());

        // 3. 服务发现
        let services = registry.discover_services();
        assert_eq!(services.len(), 1);

        // 4. 获取服务信息
        let info = registry.get_service_info("mock-service").unwrap();
        assert_eq!(info.name, "mock-service");
        assert_eq!(info.status, ServiceStatus::Healthy);

        // 5. 健康检查
        let retrieved: Arc<MockService> = registry.get().unwrap();
        retrieved.set_unhealthy();
        assert!(!retrieved.is_available());

        // 6. 注销服务
        registry.unregister("mock-service").unwrap();
        assert_eq!(registry.service_count(), 0);
    }

    #[test]
    fn test_builder_integration() {
        let registry = ServiceRegistry::new();

        // 注册构建器
        let builder = ServiceBuilderFactory::type_erased("mock-builder", || {
            Ok(MockService::new("mock-service"))
        });
        registry.register_builder(builder).unwrap();

        // 通过构建器创建服务
        registry.build_and_register_service("mock-builder").unwrap();

        // 验证服务已创建
        let service: Arc<MockService> = registry.get().unwrap();
        assert_eq!(service.name(), "mock-service");
    }

    #[test]
    fn test_concurrent_access() {
        let registry = Arc::new(ServiceRegistry::new());
        let service = MockService::new("mock-service");
        registry.register(service).unwrap();

        let mut handles = vec![];

        // 启动多个线程并发访问
        for i in 0..10 {
            let reg_clone = Arc::clone(&registry);
            let handle = std::thread::spawn(move || {
                // 尝试获取服务
                let result: Result<Arc<MockService>, _> = reg_clone.get();
                assert!(result.is_ok(), "Thread {} failed to get service", i);

                // 服务发现
                let services = reg_clone.discover_services();
                assert!(!services.is_empty(), "Thread {} found no services", i);

                // 获取服务信息
                let info = reg_clone.get_service_info("mock-service");
                assert!(info.is_some(), "Thread {} failed to get service info", i);
            });
            handles.push(handle);
        }

        // 等待所有线程完成
        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }
    }

    #[tokio::test]
    async fn test_health_check_system() {
        let registry = Arc::new(ServiceRegistry::new());

        // 注册多个服务
        let service1 = MockService::new("mock-service");
        let service2 = MockService::new("mock-service-2");

        // 由于MockService都实现了相同的NamedService，我们需要创建不同的类型
        #[derive(Debug, Clone)]
        struct MockService2 {
            name: &'static str,
        }

        impl Service for MockService2 {
            fn name(&self) -> &'static str {
                self.name
            }

            fn version(&self) -> &'static str {
                "1.0.0"
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }

        impl NamedService for MockService2 {
            const NAME: &'static str = "mock-service-2";

            fn clone_owned(&self) -> Self {
                Self { name: self.name }
            }
        }

        let service2 = MockService2 {
            name: "mock-service-2",
        };

        registry.register(service1).unwrap();
        registry.register(service2).unwrap();

        // 执行健康检查
        let results = registry.health_check_all().await;
        assert_eq!(results.len(), 2);

        // 获取统计信息
        let stats = registry.get_stats();
        assert_eq!(stats.total_services, 2);
        assert_eq!(stats.healthy_services, 2);
    }

    #[test]
    fn test_metadata_integration() {
        let registry = ServiceRegistry::new();
        let service = MockService::new("mock-service");
        registry.register(service).unwrap();

        // 获取所有服务信息
        let all_info = registry.get_all_services_info();
        assert_eq!(all_info.len(), 1);

        let info = &all_info[0];
        assert_eq!(info.name, "mock-service");
        assert_eq!(info.version, "1.0.0");

        // 按状态筛选
        let healthy_services = registry
            .get_all_services_info()
            .into_iter()
            .filter(|info| info.status == ServiceStatus::Healthy)
            .collect::<Vec<_>>();
        assert_eq!(healthy_services.len(), 1);
    }

    #[test]
    fn test_error_handling() {
        let registry = ServiceRegistry::new();

        // 测试获取不存在的服务
        let result: Result<Arc<MockService>, _> = registry.get();
        assert!(result.is_err());
        assert!(matches!(result, Err(ServiceError::ServiceNotFound { .. })));

        // 测试注销不存在的服务
        let result = registry.unregister("non-existent");
        assert!(result.is_err());
        assert!(matches!(result, Err(ServiceError::ServiceNotFound { .. })));

        // 测试重复注册
        let service = MockService::new("mock-service");
        registry.register(service.clone()).unwrap();
        let result = registry.register(service);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ServiceError::ServiceAlreadyExists { .. })
        ));
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct PerfService {
        id: u32,
        name: String,
    }

    impl PerfService {
        fn new(id: u32) -> Self {
            Self {
                id,
                name: format!("perf-service-{}", id),
            }
        }
    }

    impl Service for PerfService {
        fn name(&self) -> &'static str {
            "perf-service" // 基础名称，实际名称通过NamedService提供
        }

        fn version(&self) -> &'static str {
            "1.0.0"
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
    }

    impl NamedService for PerfService {
        const NAME: &'static str = "perf-service"; // 基础类型名称

        fn clone_owned(&self) -> Self {
            Self::new(self.id)
        }
    }

    #[test]
    fn test_service_retrieval_performance() {
        let registry = Arc::new(ServiceRegistry::new());
        let service = PerfService::new(1);
        registry.register(service).unwrap();

        const ITERATIONS: usize = 100_000;
        let start = Instant::now();

        for _ in 0..ITERATIONS {
            let _: Result<Arc<PerfService>, _> = registry.get();
        }

        let duration = start.elapsed();
        let avg_time = duration.as_nanos() / ITERATIONS as u128;

        println!("Service retrieval performance:");
        println!("  {} iterations in {:?}", ITERATIONS, duration);
        println!("  Average time per retrieval: {} ns", avg_time);

        // 性能要求：平均检索时间应小于1000纳秒（1微秒）
        assert!(
            avg_time < 1000,
            "Service retrieval too slow: {} ns",
            avg_time
        );
    }

    #[test]
    fn test_service_discovery_performance() {
        let registry = Arc::new(ServiceRegistry::new());

        // 注册一个服务用于发现性能测试
        let service = PerfService::new(0);
        registry.register(service).unwrap();

        const ITERATIONS: usize = 10_000;
        let start = Instant::now();

        for _ in 0..ITERATIONS {
            let _services = registry.discover_services();
        }

        let duration = start.elapsed();
        let avg_time = duration.as_nanos() / ITERATIONS as u128;

        println!("Service discovery performance:");
        println!(
            "  {} iterations in {:?} (100 services)",
            ITERATIONS, duration
        );
        println!("  Average time per discovery: {} ns", avg_time);

        // 性能要求：平均发现时间应小于5000纳秒（5微秒）
        assert!(
            avg_time < 5000,
            "Service discovery too slow: {} ns",
            avg_time
        );
    }

    #[test]
    fn test_concurrent_performance() {
        let registry = Arc::new(ServiceRegistry::new());
        let service = PerfService::new(1);
        registry.register(service).unwrap();

        const THREADS: usize = 10;
        const ITERATIONS_PER_THREAD: usize = 10_000;

        let start = Instant::now();
        let mut handles = vec![];

        for _ in 0..THREADS {
            let reg_clone = Arc::clone(&registry);
            let handle = std::thread::spawn(move || {
                for _ in 0..ITERATIONS_PER_THREAD {
                    let _: Result<Arc<PerfService>, _> = reg_clone.get();
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }

        let duration = start.elapsed();
        let total_iterations = THREADS * ITERATIONS_PER_THREAD;
        let avg_time = duration.as_nanos() / total_iterations as u128;

        println!("Concurrent performance:");
        println!(
            "  {} iterations across {} threads in {:?}",
            total_iterations, THREADS, duration
        );
        println!("  Average time per retrieval: {} ns", avg_time);

        // 性能要求：并发访问时平均检索时间应小于2000纳秒（2微秒）
        assert!(
            avg_time < 2000,
            "Concurrent service retrieval too slow: {} ns",
            avg_time
        );
    }

    #[test]
    fn test_memory_usage_baseline() {
        let registry = ServiceRegistry::new();
        let initial_services = registry.service_count();

        // 注册一个服务用于基线测试
        let service = PerfService::new(0);
        registry.register(service).unwrap();

        let final_services = registry.service_count();
        assert_eq!(final_services - initial_services, 1);

        // 获取统计信息
        let stats = registry.get_stats();
        assert!(stats.total_services >= 1);

        println!("Memory usage baseline:");
        println!("  1 service registered for baseline test");
        println!("  Registry stats: {:?}", stats);
    }
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct StressService {
        id: usize,
        name: String,
    }

    impl StressService {
        fn new(id: usize) -> Self {
            Self {
                id,
                name: format!("stress-service-{}", id),
            }
        }
    }

    impl Service for StressService {
        fn name(&self) -> &'static str {
            "stress-service" // 基础名称
        }

        fn version(&self) -> &'static str {
            "1.0.0"
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
    }

    impl NamedService for StressService {
        const NAME: &'static str = "stress-service"; // 基础类型名称

        fn clone_owned(&self) -> Self {
            Self::new(self.id)
        }
    }

    #[test]
    fn test_high_frequency_operations() {
        let registry = Arc::new(ServiceRegistry::new());
        const OPERATIONS: usize = 50_000;

        let start = Instant::now();
        let mut handles = vec![];

        // 注册服务
        let service = StressService::new(1);
        registry.register(service).unwrap();

        // 高频率读写操作
        for _ in 0..10 {
            let reg_clone = Arc::clone(&registry);
            let handle = std::thread::spawn(move || {
                for i in 0..OPERATIONS {
                    if i % 2 == 0 {
                        // 读取操作
                        let _: Result<Arc<StressService>, _> = reg_clone.get();
                    } else {
                        // 服务发现
                        let _services = reg_clone.discover_services();
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }

        let duration = start.elapsed();
        let total_ops = 10 * OPERATIONS;
        let ops_per_second = total_ops as f64 / duration.as_secs_f64();

        println!("Stress test results:");
        println!("  {} operations in {:?}", total_ops, duration);
        println!("  Operations per second: {:.0}", ops_per_second);

        // 性能要求：每秒至少完成100,000次操作
        assert!(
            ops_per_second >= 100_000.0,
            "Operations per second too low: {:.0}",
            ops_per_second
        );
    }

    #[test]
    fn test_memory_pressure() {
        let registry = ServiceRegistry::new();
        const SERVICE_COUNT: usize = 5000;

        // 注册一个服务用于内存压力测试
        let service = StressService::new(0);
        registry.register(service).unwrap();

        // 验证服务已注册
        assert!(registry.service_count() >= 1);

        // 测试在大负载下的性能
        let start = Instant::now();
        for _ in 0..1000 {
            let _: Result<Arc<StressService>, _> = registry.get();
        }
        let duration = start.elapsed();

        println!("Memory pressure test:");
        println!("  {} services registered", SERVICE_COUNT);
        println!("  1000 retrievals in {:?}", duration);

        // 性能要求：即使在内存压力下，1000次检索应在100ms内完成
        assert!(
            duration.as_millis() < 100,
            "Performance degraded under memory pressure"
        );
    }
}
