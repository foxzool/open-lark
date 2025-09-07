#![cfg(feature = "benchmarks")]
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use open_lark::service::im::v1::p2_im_message_receive_v1::P2ImMessageReceiveV1Data;
use serde_json::{json, Value};
use std::time::Duration;

// JSON 解析性能基准测试
// 测量不同大小和复杂度的 JSON 解析性能

// 生成不同复杂度的测试数据
fn generate_simple_message() -> Value {
    json!({
        "ts": "1693834271.787977",
        "uuid": "c9b62180-3c4b-477e-9f50-1e1bf7bcc0b3",
        "token": "v=",
        "type": "event_callback",
        "event": {
            "sender": {
                "sender_id": {
                    "open_id": "ou_simple_test"
                },
                "sender_type": "user",
                "tenant_key": "tenant_test"
            },
            "message": {
                "message_id": "om_simple_test",
                "create_time": "1693834271",
                "chat_id": "oc_test",
                "chat_type": "group",
                "message_type": "text",
                "content": "{\"text\":\"Hello\"}",
                "mentions": []
            }
        }
    })
}

fn generate_complex_message() -> Value {
    let mentions: Vec<Value> = (0..50)
        .map(|i| {
            json!({
                "key": format!("user_{}", i),
                "id": {
                    "open_id": format!("ou_mention_{}", i),
                    "user_id": format!("user_id_{}", i)
                },
                "name": format!("用户{}", i),
                "tenant_key": "tenant_test"
            })
        })
        .collect();

    json!({
        "ts": "1693834271.787977",
        "uuid": "c9b62180-3c4b-477e-9f50-1e1bf7bcc0b3",
        "token": "v=verification_token_very_long_string_12345",
        "type": "event_callback",
        "event": {
            "sender": {
                "sender_id": {
                    "open_id": "ou_complex_test_user_with_long_id",
                    "user_id": "user_123456789",
                    "union_id": "on_union_123456789"
                },
                "sender_type": "user",
                "tenant_key": "tenant_test_complex"
            },
            "message": {
                "message_id": "om_complex_message_with_very_long_id",
                "root_id": "om_root_message_id",
                "parent_id": "om_parent_message_id",
                "create_time": "1693834271787",
                "update_time": "1693834271787",
                "chat_id": "oc_complex_group_chat",
                "chat_type": "group",
                "message_type": "text",
                "content": format!("{{\"text\":\"{}\"}}", "很长的消息内容".repeat(100)),
                "mentions": mentions
            }
        }
    })
}

fn generate_large_message() -> Value {
    // 生成大量数据的消息
    let large_content = "大".repeat(10000);
    let large_mentions: Vec<Value> = (0..1000)
        .map(|i| {
            json!({
                "key": format!("user_{}", i),
                "id": {
                    "open_id": format!("ou_large_mention_{}", i)
                },
                "name": format!("用户{}", i)
            })
        })
        .collect();

    json!({
        "ts": "1693834271.787977",
        "uuid": "c9b62180-3c4b-477e-9f50-1e1bf7bcc0b3",
        "token": "v=",
        "type": "event_callback",
        "event": {
            "sender": {
                "sender_id": {
                    "open_id": "ou_large_test"
                },
                "sender_type": "user",
                "tenant_key": "tenant_test"
            },
            "message": {
                "message_id": "om_large_test",
                "create_time": "1693834271",
                "chat_id": "oc_test",
                "chat_type": "group",
                "message_type": "text",
                "content": format!("{{\"text\":\"{}\"}}", large_content),
                "mentions": large_mentions
            }
        }
    })
}

// 基准测试函数
fn json_parsing_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_parsing");

    // 简单消息解析测试
    let simple_json = serde_json::to_string(&generate_simple_message()).unwrap();
    group.throughput(Throughput::Bytes(simple_json.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("simple_message", simple_json.len()),
        &simple_json,
        |b, json_str| {
            b.iter(|| {
                let _: P2ImMessageReceiveV1Data =
                    serde_json::from_str(black_box(json_str)).unwrap();
            })
        },
    );

    // 复杂消息解析测试
    let complex_json = serde_json::to_string(&generate_complex_message()).unwrap();
    group.throughput(Throughput::Bytes(complex_json.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("complex_message", complex_json.len()),
        &complex_json,
        |b, json_str| {
            b.iter(|| {
                let _: P2ImMessageReceiveV1Data =
                    serde_json::from_str(black_box(json_str)).unwrap();
            })
        },
    );

    // 大消息解析测试
    let large_json = serde_json::to_string(&generate_large_message()).unwrap();
    group.throughput(Throughput::Bytes(large_json.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("large_message", large_json.len()),
        &large_json,
        |b, json_str| {
            b.iter(|| {
                let _: P2ImMessageReceiveV1Data =
                    serde_json::from_str(black_box(json_str)).unwrap();
            })
        },
    );

    group.finish();
}

// 通用 JSON 值解析测试
fn generic_json_parsing_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("generic_json_parsing");

    let medium_json = json!({
        "users": (0..100).map(|i| json!({
            "id": i,
            "name": format!("User {}", i),
            "active": i % 2 == 0
        })).collect::<Vec<_>>()
    })
    .to_string();

    let nested_json = json!({
        "level1": {
            "level2": {
                "level3": {
                    "level4": {
                        "data": "deeply_nested_value"
                    }
                }
            }
        }
    })
    .to_string();

    let test_cases: Vec<(&str, String)> = vec![
        ("small", "{\"key\": \"value\"}".to_string()),
        ("medium", medium_json),
        ("nested", nested_json),
    ];

    for (name, json_owned) in &test_cases {
        group.throughput(Throughput::Bytes(json_owned.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("parse_to_value", name),
            json_owned,
            |b, input| {
                b.iter(|| {
                    let _: Value = serde_json::from_str::<Value>(black_box(input)).unwrap();
                })
            },
        );
    }

    group.finish();
}

// JSON 序列化基准测试
fn json_serialization_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_serialization");

    let simple_data = generate_simple_message();
    let complex_data = generate_complex_message();
    let large_data = generate_large_message();

    group.bench_function("serialize_simple", |b| {
        b.iter(|| {
            let _json_str = serde_json::to_string(black_box(&simple_data)).unwrap();
        })
    });

    group.bench_function("serialize_complex", |b| {
        b.iter(|| {
            let _json_str = serde_json::to_string(black_box(&complex_data)).unwrap();
        })
    });

    group.bench_function("serialize_large", |b| {
        b.iter(|| {
            let _json_str = serde_json::to_string(black_box(&large_data)).unwrap();
        })
    });

    // 二进制序列化对比
    group.bench_function("serialize_to_vec_simple", |b| {
        b.iter(|| {
            let _json_bytes = serde_json::to_vec(black_box(&simple_data)).unwrap();
        })
    });

    group.bench_function("serialize_to_vec_complex", |b| {
        b.iter(|| {
            let _json_bytes = serde_json::to_vec(black_box(&complex_data)).unwrap();
        })
    });

    group.finish();
}

// 配置 Criterion
criterion_group!(
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(100)
        .warm_up_time(Duration::from_secs(3));
    targets = json_parsing_benchmark,
              generic_json_parsing_benchmark,
              json_serialization_benchmark
);

criterion_main!(benches);
