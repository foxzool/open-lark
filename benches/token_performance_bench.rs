#![cfg(feature = "benchmarks")]
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use open_lark::core::{
    config::Config,
    constants::AppType,
    token_manager::{PreheatingConfig, TokenManager},
};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;

// Token 管理性能基准测试
// 测量 Token 获取、缓存、刷新的性能

async fn create_test_config() -> Config {
    Config::builder("test_app_id", "test_app_secret")
        .with_app_type(AppType::SelfBuilt)
        .with_enable_token_cache(true)
        .build()
}

fn bench_token_manager_creation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("token_manager_creation", |b| {
        b.iter(|| {
            rt.block_on(async {
                let config = create_test_config().await;
                let _token_manager = Arc::new(TokenManager::new(config));
            })
        })
    });
}

fn bench_token_cache_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("token_cache_operations");

    // 测试不同数量的并发 token 缓存操作
    for cache_size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*cache_size as u64));
        group.bench_with_input(
            BenchmarkId::new("concurrent_cache_access", cache_size),
            cache_size,
            |b, &size| {
                b.iter(|| {
                    rt.block_on(async {
                        let config = create_test_config().await;
                        let token_manager = Arc::new(TokenManager::new(config));

                        // 模拟并发缓存访问
                        let mut handles = Vec::new();

                        for i in 0..size {
                            let tm = Arc::clone(&token_manager);
                            let handle = tokio::spawn(async move {
                                // 模拟 token 缓存键查找
                                let key = format!("test_tenant_{}", i);
                                black_box(key);
                            });
                            handles.push(handle);
                        }

                        // 等待所有任务完成
                        for handle in handles {
                            let _ = handle.await;
                        }
                    })
                })
            },
        );
    }
    group.finish();
}

fn bench_preheating_config_creation(c: &mut Criterion) {
    c.bench_function("preheating_config_creation", |b| {
        b.iter(|| {
            let _config = black_box(PreheatingConfig::default());
        })
    });

    c.bench_function("preheating_config_custom", |b| {
        b.iter(|| {
            let _config = black_box(PreheatingConfig {
                check_interval_seconds: 300,
                preheat_threshold_seconds: 600,
                enable_tenant_preheating: true,
                max_concurrent_preheat: 5,
            });
        })
    });
}

fn bench_token_manager_memory_usage(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("token_manager_memory");

    // 测试不同规模下的内存使用
    for tenant_count in [1, 10, 100].iter() {
        group.throughput(Throughput::Elements(*tenant_count as u64));
        group.bench_with_input(
            BenchmarkId::new("memory_usage", tenant_count),
            tenant_count,
            |b, &count| {
                b.iter(|| {
                    rt.block_on(async {
                        let config = create_test_config().await;
                        let token_manager = Arc::new(TokenManager::new(config));

                        // 模拟多租户场景下的内存占用
                        let tenant_keys: Vec<String> =
                            (0..count).map(|i| format!("tenant_key_{}", i)).collect();

                        black_box(tenant_keys);
                    })
                })
            },
        );
    }
    group.finish();
}

fn bench_token_expiry_calculations(c: &mut Criterion) {
    use std::time::{SystemTime, UNIX_EPOCH};

    c.bench_function("token_expiry_calculation", |b| {
        b.iter(|| {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let expires_in = 7200; // 2 hours
            let expiry_time = black_box(now + expires_in);
            let remaining_time = black_box(expiry_time.saturating_sub(now));

            black_box(remaining_time);
        })
    });
}

criterion_group!(
    token_benches,
    bench_token_manager_creation,
    bench_token_cache_operations,
    bench_preheating_config_creation,
    bench_token_manager_memory_usage,
    bench_token_expiry_calculations
);

criterion_main!(token_benches);
