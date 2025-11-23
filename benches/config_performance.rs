//! Config 性能基准测试
//!
//! 测试当前 Config 克隆方式与 Arc<Config> 共享方式的性能差异，
//! 为性能优化提供数据支持。

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use openlark_core::config::{Config, ConfigBuilder};
use std::sync::Arc;
use std::time::Duration;

/// 创建测试用的配置对象
fn create_test_config() -> Config {
    ConfigBuilder::default()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url("https://open.feishu.cn")
        .enable_token_cache(true)
        .req_timeout(Duration::from_secs(30))
        .build()
}

/// 测试当前 Config 克隆方式的性能
fn bench_config_clone(c: &mut Criterion) {
    let config = create_test_config();
    let mut group = c.benchmark_group("config_clone");

    group.bench_function("config_clone_1000", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let _config_clone = config.clone();
                black_box(&_config_clone);
            }
        })
    });

    group.bench_function("config_clone_single", |b| {
        b.iter(|| {
            let _config_clone = config.clone();
            black_box(&_config_clone);
        })
    });

    group.finish();
}

/// 测试 Arc<Config> 共享方式的性能
fn bench_arc_config(c: &mut Criterion) {
    let config = create_test_config();
    let arc_config = Arc::new(config);
    let mut group = c.benchmark_group("arc_config");

    group.bench_function("arc_config_clone_1000", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let _arc_clone = arc_config.clone();
                black_box(&_arc_clone);
            }
        })
    });

    group.bench_function("arc_config_single", |b| {
        b.iter(|| {
            let _arc_clone = arc_config.clone();
            black_box(&_arc_clone);
        })
    });

    group.bench_function("arc_config_access", |b| {
        b.iter(|| {
            let app_id = arc_config.app_id.as_str();
            black_box(app_id);
        })
    });

    group.finish();
}

/// 测试服务创建场景中的性能
fn bench_service_creation(c: &mut Criterion) {
    let config = create_test_config();
    let arc_config = Arc::new(config.clone());

    let mut group = c.benchmark_group("service_creation");

    // 模拟当前服务创建方式（需要克隆 Config）
    group.bench_function("service_with_config_clone_100", |b| {
        b.iter(|| {
            for _ in 0..100 {
                let service_config = config.clone();
                // 模拟服务初始化
                let _service = TestService::new(service_config);
                black_box(&_service);
            }
        })
    });

    // 模拟使用 Arc<Config> 的服务创建方式
    group.bench_function("service_with_arc_config_100", |b| {
        b.iter(|| {
            for _ in 0..100 {
                let service_config = arc_config.clone();
                // 模拟服务初始化
                let _service = TestServiceArc::new(service_config);
                black_box(&_service);
            }
        })
    });

    group.finish();
}

/// 模拟服务结构体（当前方式）
#[derive(Clone)]
struct TestService {
    config: Config,
}

impl TestService {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn get_app_id(&self) -> &str {
        &self.config.app_id
    }
}

/// 模拟使用 Arc<Config> 的服务结构体
#[derive(Clone)]
struct TestServiceArc {
    config: Arc<Config>,
}

impl TestServiceArc {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    fn get_app_id(&self) -> &str {
        &self.config.app_id
    }
}

/// 内存使用对比测试
fn bench_memory_usage(c: &mut Criterion) {
    let config = create_test_config();
    let mut group = c.benchmark_group("memory_usage");

    // 测试大量 Config 克隆的内存分配
    group.bench_function("create_1000_config_clones", |b| {
        b.iter(|| {
            let configs: Vec<Config> = (0..1000).map(|_| config.clone()).collect();
            black_box(configs);
        })
    });

    // 测试大量 Arc 克隆的内存分配
    group.bench_function("create_1000_arc_clones", |b| {
        b.iter(|| {
            let arc_config = Arc::new(config.clone());
            let configs: Vec<Arc<Config>> = (0..1000).map(|_| arc_config.clone()).collect();
            black_box(configs);
        })
    });

    group.finish();
}

/// 并发访问性能测试
fn bench_concurrent_access(c: &mut Criterion) {
    let config = create_test_config();
    let arc_config = Arc::new(config.clone());

    let mut group = c.benchmark_group("concurrent_access");

    // 测试多个线程同时克隆 Config 的性能
    group.bench_function("concurrent_config_clone", |b| {
        b.iter(|| {
            use std::thread;

            let handles: Vec<_> = (0..8)
                .map(|_| {
                    let config_clone = config.clone();
                    thread::spawn(move || {
                        for _ in 0..100 {
                            let _clone = config_clone.clone();
                            black_box(&_clone);
                        }
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }
        })
    });

    // 测试多个线程同时访问 Arc<Config> 的性能
    group.bench_function("concurrent_arc_access", |b| {
        b.iter(|| {
            use std::thread;

            let handles: Vec<_> = (0..8)
                .map(|_| {
                    let arc_clone = arc_config.clone();
                    thread::spawn(move || {
                        for _ in 0..100 {
                            let _app_id = arc_clone.app_id.as_str();
                            black_box(_app_id);
                        }
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_config_clone,
    bench_arc_config,
    bench_service_creation,
    bench_memory_usage,
    bench_concurrent_access
);
criterion_main!(benches);
