//! ServiceRegistry性能基准测试框架
//!
//! 提供全面的性能基准测试，用于验证ServiceRegistry的性能表现

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::task::JoinSet;

use crate::core::config::{Config, ConfigBuilder};
use crate::service_registry::{ServiceRegistry, ServiceError};
// 条件编译导入适配器
#[cfg(feature = "authentication")]
use crate::service_registry::adapters::AuthenticationServiceAdapter;
#[cfg(feature = "contact")]
use crate::service_registry::adapters::ContactServiceAdapter;
#[cfg(feature = "group")]
use crate::service_registry::adapters::GroupServiceAdapter;
#[cfg(feature = "im")]
use crate::service_registry::adapters::ImServiceAdapter;
#[cfg(feature = "search")]
use crate::service_registry::adapters::SearchServiceAdapter;

/// 性能基准测试结果
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// 测试名称
    pub test_name: String,
    /// 迭代次数
    pub iterations: usize,
    /// 总耗时
    pub total_duration: Duration,
    /// 平均耗时（纳秒）
    pub avg_duration_nanos: u128,
    /// 最小耗时（纳秒）
    pub min_duration_nanos: u128,
    /// 最大耗时（纳秒）
    pub max_duration_nanos: u128,
    /// 每秒操作数
    pub ops_per_second: f64,
    /// 内存使用（字节）
    pub memory_usage_bytes: Option<usize>,
}

impl BenchmarkResult {
    /// 创建新的基准测试结果
    pub fn new(
        test_name: String,
        iterations: usize,
        total_duration: Duration,
        measurements: &[Duration],
    ) -> Self {
        let avg_duration_nanos = total_duration.as_nanos() / iterations as u128;
        let min_duration_nanos = measurements.iter().map(|d| d.as_nanos()).min().unwrap_or(0);
        let max_duration_nanos = measurements.iter().map(|d| d.as_nanos()).max().unwrap_or(0);
        let ops_per_second = iterations as f64 / total_duration.as_secs_f64();

        Self {
            test_name,
            iterations,
            total_duration,
            avg_duration_nanos,
            min_duration_nanos,
            max_duration_nanos,
            ops_per_second,
            memory_usage_bytes: None,
        }
    }

    /// 设置内存使用量
    pub fn with_memory_usage(mut self, memory_bytes: usize) -> Self {
        self.memory_usage_bytes = Some(memory_bytes);
        self
    }

    /// 打印结果
    pub fn print(&self) {
        println!("📊 {}", self.test_name);
        println!("  迭代次数: {}", self.iterations);
        println!("  总耗时: {:?}", self.total_duration);
        println!("  平均耗时: {} ns ({:.2} μs)", self.avg_duration_nanos, self.avg_duration_nanos as f64 / 1000.0);
        println!("  最小耗时: {} ns", self.min_duration_nanos);
        println!("  最大耗时: {} ns", self.max_duration_nanos);
        println!("  每秒操作数: {:.0}", self.ops_per_second);

        if let Some(memory) = self.memory_usage_bytes {
            println!("  内存使用: {} bytes ({:.2} KB)", memory, memory as f64 / 1024.0);
        }
        println!();
    }

    /// 验证性能要求
    pub fn validate_performance(&self, max_avg_nanos: u128) -> Result<(), String> {
        if self.avg_duration_nanos > max_avg_nanos {
            Err(format!(
                "性能不达标: 平均耗时 {} ns 超过要求的 {} ns",
                self.avg_duration_nanos, max_avg_nanos
            ))
        } else {
            Ok(())
        }
    }
}

/// 性能基准测试套件
pub struct BenchmarkSuite {
    registry: Arc<ServiceRegistry>,
    config: Config,
}

impl BenchmarkSuite {
    /// 创建新的基准测试套件
    pub fn new() -> Self {
        let config = ConfigBuilder::default()
            .app_id("benchmark_app")
            .app_secret("benchmark_secret")
            .build();

        Self {
            registry: Arc::new(ServiceRegistry::new()),
            config,
        }
    }

    /// 使用自定义配置创建基准测试套件
    pub fn with_config(config: Config) -> Self {
        Self {
            registry: Arc::new(ServiceRegistry::new()),
            config,
        }
    }

    /// 设置核心服务
    pub fn setup_core_services(&self) -> Result<(), ServiceError> {
        #[cfg(feature = "authentication")]
        {
            let auth_service = crate::service::authentication::AuthenticationService::new(self.config.clone());
            let auth_adapter = AuthenticationServiceAdapter::new(auth_service);
            self.registry.register(auth_adapter)?;
        }

        #[cfg(feature = "im")]
        {
            let im_service = crate::service::im::ImService::new(self.config.clone());
            let im_adapter = ImServiceAdapter::new(im_service);
            self.registry.register(im_adapter)?;
        }

        #[cfg(feature = "contact")]
        {
            let contact_service = crate::service::contact::ContactService::new(self.config.clone());
            let contact_adapter = ContactServiceAdapter::new(contact_service);
            self.registry.register(contact_adapter)?;
        }

        #[cfg(feature = "group")]
        {
            let group_service = crate::service::group::GroupService::new(self.config.clone());
            let group_adapter = GroupServiceAdapter::new(group_service);
            self.registry.register(group_adapter)?;
        }

        #[cfg(feature = "search")]
        {
            let search_service = crate::service::search::SearchService::new(self.config.clone());
            let search_adapter = SearchServiceAdapter::new(search_service);
            self.registry.register(search_adapter)?;
        }

        Ok(())
    }

    /// 基准测试：服务注册性能
    pub fn benchmark_service_registration(&self, iterations: usize) -> BenchmarkResult {
        let mut measurements = Vec::with_capacity(iterations);

        for i in 0..iterations {
            let start = Instant::now();

            // 创建测试服务并注册
            let service = BenchmarkService::new(format!("bench-service-{}", i));
            let registry = ServiceRegistry::new();
            let _ = registry.register(service);

            let duration = start.elapsed();
            measurements.push(duration);
        }

        let total_duration: Duration = measurements.iter().sum();
        let result = BenchmarkResult::new(
            "服务注册性能基准测试".to_string(),
            iterations,
            total_duration,
            &measurements,
        );

        result.print();
        result
    }

    /// 基准测试：服务检索性能
    pub fn benchmark_service_retrieval(&self, iterations: usize) -> BenchmarkResult {
        // 确保有可用的服务
        if self.registry.service_count() == 0 {
            self.setup_core_services().expect("Failed to setup core services");
        }

        let mut measurements = Vec::with_capacity(iterations);

        for _ in 0..iterations {
            let start = Instant::now();

            // 随机选择一个服务进行检索
            let services = self.registry.discover_services();
            if let Some(service_name) = services.first() {
                let _: Result<Arc<dyn crate::service_registry::Service>, _> =
                    self.registry.get_by_name(service_name);
            }

            let duration = start.elapsed();
            measurements.push(duration);
        }

        let total_duration: Duration = measurements.iter().sum();
        let result = BenchmarkResult::new(
            "服务检索性能基准测试".to_string(),
            iterations,
            total_duration,
            &measurements,
        );

        result.print();
        result
    }

    /// 基准测试：服务发现性能
    pub fn benchmark_service_discovery(&self, iterations: usize) -> BenchmarkResult {
        // 确保有可用的服务
        if self.registry.service_count() == 0 {
            self.setup_core_services().expect("Failed to setup core services");
        }

        let mut measurements = Vec::with_capacity(iterations);

        for _ in 0..iterations {
            let start = Instant::now();

            let _services = self.registry.discover_services();

            let duration = start.elapsed();
            measurements.push(duration);
        }

        let total_duration: Duration = measurements.iter().sum();
        let result = BenchmarkResult::new(
            "服务发现性能基准测试".to_string(),
            iterations,
            total_duration,
            &measurements,
        );

        result.print();
        result
    }

    /// 基准测试：并发访问性能
    pub async fn benchmark_concurrent_access(&self, iterations: usize, concurrency: usize) -> BenchmarkResult {
        // 确保有可用的服务
        if self.registry.service_count() == 0 {
            self.setup_core_services().expect("Failed to setup core services");
        }

        let mut measurements = Vec::with_capacity(iterations);
        let mut set = JoinSet::new();

        let start = Instant::now();

        for i in 0..iterations {
            let registry = Arc::clone(&self.registry);
            let services = registry.discover_services();
            let service_name = services[i % services.len()].to_string();

            set.spawn(async move {
                let op_start = Instant::now();
                let _: Result<Arc<dyn crate::service_registry::Service>, _> =
                    registry.get_by_name(&service_name);
                op_start.elapsed()
            });
        }

        while let Some(result) = set.join_next().await {
            if let Ok(duration) = result {
                measurements.push(duration);
            }
        }

        let total_duration = start.elapsed();
        let benchmark_result = BenchmarkResult::new(
            format!("并发访问性能基准测试 (并发度: {})", concurrency),
            iterations,
            total_duration,
            &measurements,
        );

        benchmark_result.print();
        benchmark_result
    }

    /// 运行完整的基准测试套件
    pub async fn run_full_benchmark_suite(&self) -> Vec<BenchmarkResult> {
        println!("🚀 开始ServiceRegistry性能基准测试套件");
        println!("{}", "=".repeat(60));
        println!();

        let mut results = Vec::new();

        // 1. 服务注册性能测试
        println!("📝 测试1: 服务注册性能");
        let result1 = self.benchmark_service_registration(1000);
        if let Err(e) = result1.validate_performance(100_000) { // 100微秒
            println!("⚠️  警告: {}", e);
        }
        results.push(result1);

        // 2. 服务检索性能测试
        println!("🔍 测试2: 服务检索性能");
        let result2 = self.benchmark_service_retrieval(10_000);
        if let Err(e) = result2.validate_performance(1_000) { // 1微秒
            println!("⚠️  警告: {}", e);
        }
        results.push(result2);

        // 3. 服务发现性能测试
        println!("🔎 测试3: 服务发现性能");
        let result3 = self.benchmark_service_discovery(5_000);
        if let Err(e) = result3.validate_performance(5_000) { // 5微秒
            println!("⚠️  警告: {}", e);
        }
        results.push(result3);

        // 4. 并发访问性能测试
        println!("⚡ 测试4: 并发访问性能");
        let result4 = self.benchmark_concurrent_access(1_000, 10).await;
        if let Err(e) = result4.validate_performance(2_000) { // 2微秒
            println!("⚠️  警告: {}", e);
        }
        results.push(result4);

        // 打印总结
        println!("{}", "=".repeat(60));
        println!("📊 基准测试总结:");
        for (i, result) in results.iter().enumerate() {
            println!("  {}. {}: {:.0} ops/sec (平均: {} ns)",
                i + 1, result.test_name, result.ops_per_second, result.avg_duration_nanos);
        }
        println!();

        results
    }
}

/// 用于基准测试的简单服务
#[derive(Debug, Clone)]
pub struct BenchmarkService {
    name: String,
}

impl BenchmarkService {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl crate::service_registry::Service for BenchmarkService {
    fn name(&self) -> &'static str {
        // 注意：这里简化处理，实际应用中可能需要更复杂的名称管理
        "benchmark-service"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn status(&self) -> crate::service_registry::ServiceStatus {
        crate::service_registry::ServiceStatus::Healthy
    }

    fn description(&self) -> &'static str {
        "Benchmark service for performance testing"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_benchmark_suite() {
        let suite = BenchmarkSuite::new();
        suite.setup_core_services().expect("Failed to setup services");

        let results = suite.run_full_benchmark_suite().await;

        // 验证所有测试都完成了
        assert_eq!(results.len(), 4);

        // 验证基本性能要求
        for result in &results {
            assert!(result.avg_duration_nanos > 0, "Average duration should be positive");
            assert!(result.ops_per_second > 0.0, "Ops per second should be positive");
        }
    }

    #[test]
    fn test_benchmark_result() {
        let measurements = vec![
            Duration::from_nanos(100),
            Duration::from_nanos(200),
            Duration::from_nanos(300),
        ];
        let total = Duration::from_nanos(600);

        let result = BenchmarkResult::new(
            "test".to_string(),
            3,
            total,
            &measurements,
        );

        assert_eq!(result.avg_duration_nanos, 200);
        assert_eq!(result.min_duration_nanos, 100);
        assert_eq!(result.max_duration_nanos, 300);
        assert_eq!(result.ops_per_second, 3.0 / total.as_secs_f64());

        // 测试性能验证
        assert!(result.validate_performance(300).is_ok());
        assert!(result.validate_performance(150).is_err());
    }
}