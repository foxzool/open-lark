#![cfg(feature = "benchmarks")]
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use open_lark::core::{
    api::ApiRequest,
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AppType,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::runtime::Runtime;

// API 调用吞吐量性能基准测试
// 测量批量 API 调用、并发处理、序列化/反序列化性能

#[derive(Debug, Serialize, Deserialize)]
struct BenchmarkRequest {
    message: String,
    count: i32,
    data: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BenchmarkResponse {
    success: bool,
    result: String,
    items: Vec<BenchmarkItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BenchmarkItem {
    id: String,
    name: String,
    timestamp: u64,
}

impl ApiResponseTrait for BenchmarkResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

async fn create_test_config() -> Config {
    let timeout = Duration::from_secs(30);
    let http_client = reqwest::Client::builder()
        .timeout(timeout)
        .build()
        .expect("failed to build bench http client");

    Config::builder()
        .app_id("bench_app_id")
        .app_secret("bench_app_secret")
        .app_type(AppType::SelfBuild)
        .req_timeout(timeout)
        .http_client(http_client)
        .build()
}

fn create_benchmark_request(size: usize) -> BenchmarkRequest {
    BenchmarkRequest {
        message: "Performance benchmark test message".to_string(),
        count: size as i32,
        data: (0..size).map(|i| format!("test_data_item_{}", i)).collect(),
    }
}

fn create_api_request(payload: &BenchmarkRequest) -> SDKResult<ApiRequest> {
    let mut request = ApiRequest::default();
    request.api_path = "/benchmark/test".to_string();
    request.query_params = Default::default();
    request.body = serde_json::to_vec(payload).unwrap();
    Ok(request)
}

fn bench_request_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("request_serialization");

    for size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("json_serialization", size),
            size,
            |b, &size| {
                let request = create_benchmark_request(size);
                b.iter(|| {
                    let _json = black_box(serde_json::to_vec(&request).unwrap());
                })
            },
        );
    }
    group.finish();
}

fn bench_response_deserialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("response_deserialization");

    for size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("json_deserialization", size),
            size,
            |b, &size| {
                // 创建测试响应数据
                let items: Vec<BenchmarkItem> = (0..size)
                    .map(|i| BenchmarkItem {
                        id: format!("item_{}", i),
                        name: format!("Benchmark Item {}", i),
                        timestamp: 1672531200 + i as u64,
                    })
                    .collect();

                let response = BenchmarkResponse {
                    success: true,
                    result: "Benchmark completed".to_string(),
                    items,
                };

                let json_bytes = serde_json::to_vec(&response).unwrap();

                b.iter(|| {
                    let _parsed: BenchmarkResponse =
                        black_box(serde_json::from_slice(&json_bytes).unwrap());
                })
            },
        );
    }
    group.finish();
}

fn bench_api_request_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("api_request_creation");

    for size in [1, 10, 100].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("request_creation", size),
            size,
            |b, &size| {
                let payload = create_benchmark_request(size);
                b.iter(|| {
                    let _req = black_box(create_api_request(&payload).unwrap());
                })
            },
        );
    }
    group.finish();
}

fn bench_concurrent_request_preparation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("concurrent_request_prep");

    for concurrency in [5, 20, 50].iter() {
        group.throughput(Throughput::Elements(*concurrency as u64));
        group.bench_with_input(
            BenchmarkId::new("concurrent_prep", concurrency),
            concurrency,
            |b, &concurrency| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut handles = Vec::new();

                        for i in 0..concurrency {
                            let handle = tokio::spawn(async move {
                                let payload = create_benchmark_request(10);
                                let _req = create_api_request(&payload).unwrap();
                                black_box(i);
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

fn bench_config_cloning(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("config_cloning", |b| {
        b.iter(|| {
            rt.block_on(async {
                let config = create_test_config().await;
                let _cloned = black_box(config.clone());
            })
        })
    });
}

fn bench_request_option_creation(c: &mut Criterion) {
    c.bench_function("request_option_creation", |b| {
        b.iter(|| {
            let _option = black_box(
                RequestOption::builder()
                    .user_access_token("test_token")
                    .build(),
            );
        })
    });
}

fn bench_memory_allocation_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation");

    // 测试字符串分配模式
    group.bench_function("string_allocations", |b| {
        b.iter(|| {
            let mut strings = Vec::new();
            for i in 0..100 {
                strings.push(black_box(format!("test_string_{}", i)));
            }
            black_box(strings);
        })
    });

    // 测试 Vec 预分配 vs 动态增长
    group.bench_function("vec_preallocation", |b| {
        b.iter(|| {
            let mut vec = Vec::with_capacity(100);
            for i in 0..100 {
                vec.push(black_box(format!("item_{}", i)));
            }
            black_box(vec);
        })
    });

    group.bench_function("vec_dynamic_growth", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..100 {
                vec.push(black_box(format!("item_{}", i)));
            }
            black_box(vec);
        })
    });

    group.finish();
}

criterion_group!(
    api_throughput_benches,
    bench_request_serialization,
    bench_response_deserialization,
    bench_api_request_creation,
    bench_concurrent_request_preparation,
    bench_config_cloning,
    bench_request_option_creation,
    bench_memory_allocation_patterns
);

criterion_main!(api_throughput_benches);
