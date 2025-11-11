#![cfg(feature = "benchmarks")]
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use openlark_auth::{PreheatingConfig, TokenManager};
use std::sync::Arc;
use tokio::runtime::Runtime;

// Token 管理性能基准测试
// 测量 Token 获取、缓存、刷新的性能

fn bench_token_manager_creation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("token_manager_creation", |b| {
        b.iter(|| {
            rt.block_on(async {
                let _token_manager = Arc::new(TokenManager::new());
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
                        let token_manager = Arc::new(TokenManager::new());

                        // 模拟并发缓存访问
                        let mut handles = Vec::new();

                        for i in 0..size {
                            let tm = Arc::clone(&token_manager);
                            let handle = tokio::spawn(async move {
                                // 访问性能指标触发读操作，模拟真实并发读取
                                let metrics = tm.metrics().clone();
                                black_box(metrics);

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
                        let token_manager = Arc::new(TokenManager::new());

                        // 模拟多租户场景下的内存占用
                        let tenant_keys: Vec<String> =
                            (0..count).map(|i| format!("tenant_key_{}", i)).collect();

                        // 读取一次指标，模拟监控场景
                        let metrics = token_manager.metrics().clone();

                        black_box((tenant_keys, metrics));
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
