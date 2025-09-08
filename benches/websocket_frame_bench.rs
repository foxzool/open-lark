#![cfg(feature = "benchmarks")]
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use futures_util::future::join_all;
use lark_websocket_protobuf::pbbp2::{Frame, Header};
use open_lark::{client::ws_client::FrameHandler, event::dispatcher::EventDispatcherHandler};
use std::time::Duration;
use tokio::runtime::Builder;

// WebSocket 帧处理性能基准测试
// 生成不同类型的测试帧
fn generate_ping_frame() -> Frame {
    Frame {
        seq_id: 1,
        log_id: 1001,
        service: 1,
        method: 0, // Control frame
        headers: vec![Header {
            key: "type".to_string(),
            value: "ping".to_string(),
        }],
        payload_encoding: None,
        payload_type: None,
        payload: None,
        log_id_new: None,
    }
}

fn generate_pong_frame() -> Frame {
    let config = serde_json::json!({
        "heartbeat_interval": 30000,
        "compress": false
    });

    Frame {
        seq_id: 2,
        log_id: 1002,
        service: 1,
        method: 0, // Control frame
        headers: vec![Header {
            key: "type".to_string(),
            value: "pong".to_string(),
        }],
        payload_encoding: None,
        payload_type: None,
        payload: Some(serde_json::to_vec(&config).unwrap()),
        log_id_new: None,
    }
}

fn generate_simple_event_frame() -> Frame {
    let event = serde_json::json!({
        "ts": "1693834271.787977",
        "uuid": "bench-uuid-123",
        "token": "v=bench_token",
        "type": "event_callback",
        "event": {
            "sender": {
                "sender_id": {
                    "open_id": "ou_bench_user"
                },
                "sender_type": "user",
                "tenant_key": "tenant_bench"
            },
            "message": {
                "message_id": "om_bench_message",
                "create_time": "1693834271",
                "chat_id": "oc_bench",
                "chat_type": "group",
                "message_type": "text",
                "content": "{\"text\":\"Benchmark message\"}",
                "mentions": []
            }
        }
    });

    Frame {
        seq_id: 3,
        log_id: 1003,
        service: 1,
        method: 1, // Data frame
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "bench_msg_123".to_string(),
            },
            Header {
                key: "trace_id".to_string(),
                value: "bench_trace_456".to_string(),
            },
        ],
        payload_encoding: None,
        payload_type: None,
        payload: Some(serde_json::to_vec(&event).unwrap()),
        log_id_new: None,
    }
}

fn generate_complex_event_frame() -> Frame {
    // 创建包含大量数据的复杂事件
    let mentions: Vec<_> = (0..100)
        .map(|i| {
            serde_json::json!({
                "key": format!("user_{}", i),
                "id": {
                    "open_id": format!("ou_mention_{}", i)
                },
                "name": format!("用户{}", i)
            })
        })
        .collect();

    let event = serde_json::json!({
        "ts": "1693834271.787977",
        "uuid": "complex-bench-uuid-123456789",
        "token": "v=complex_bench_token_with_long_string",
        "type": "event_callback",
        "event": {
            "sender": {
                "sender_id": {
                    "open_id": "ou_complex_bench_user_with_very_long_id",
                    "user_id": "user_987654321",
                    "union_id": "on_union_987654321"
                },
                "sender_type": "user",
                "tenant_key": "tenant_complex_bench"
            },
            "message": {
                "message_id": "om_complex_bench_message_with_long_id",
                "root_id": "om_root_bench",
                "parent_id": "om_parent_bench",
                "create_time": "1693834271787",
                "chat_id": "oc_complex_bench_group",
                "chat_type": "group",
                "message_type": "text",
                "content": format!("{{\"text\":\"{}\"}}", "复杂基准测试消息内容".repeat(50)),
                "mentions": mentions
            }
        }
    });

    Frame {
        seq_id: 4,
        log_id: 1004,
        service: 1,
        method: 1, // Data frame
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "complex_bench_msg_123456".to_string(),
            },
            Header {
                key: "trace_id".to_string(),
                value: "complex_bench_trace_789012".to_string(),
            },
        ],
        payload_encoding: None,
        payload_type: None,
        payload: Some(serde_json::to_vec(&event).unwrap()),
        log_id_new: None,
    }
}

// 控制帧处理基准测试
fn control_frame_benchmark(c: &mut Criterion) {
    let rt = Builder::new_current_thread().enable_all().build().unwrap();
    let mut group = c.benchmark_group("control_frame_handling");

    let ping_frame = generate_ping_frame();
    let pong_frame = generate_pong_frame();

    group.bench_function("ping_frame", |b| {
        b.to_async(&rt).iter(|| async {
            let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
            let handler = EventDispatcherHandler::builder().build();

            let _result =
                FrameHandler::handle_frame(black_box(ping_frame.clone()), &handler, &tx).await;
        })
    });

    group.bench_function("pong_frame", |b| {
        b.to_async(&rt).iter(|| async {
            let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
            let handler = EventDispatcherHandler::builder().build();

            let _result =
                FrameHandler::handle_frame(black_box(pong_frame.clone()), &handler, &tx).await;
        })
    });

    group.finish();
}

// 数据帧处理基准测试
fn data_frame_benchmark(c: &mut Criterion) {
    let rt = Builder::new_current_thread().enable_all().build().unwrap();
    let mut group = c.benchmark_group("data_frame_handling");

    let simple_frame = generate_simple_event_frame();
    let complex_frame = generate_complex_event_frame();

    // 简单事件帧处理
    let simple_payload_size = simple_frame.payload.as_ref().unwrap().len();
    group.throughput(Throughput::Bytes(simple_payload_size as u64));
    group.bench_with_input(
        BenchmarkId::new("simple_event", simple_payload_size),
        &simple_frame,
        |b, frame: &Frame| {
            b.to_async(&rt).iter(|| async {
                let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
                let handler = EventDispatcherHandler::builder().build();

                let _result =
                    FrameHandler::handle_frame(black_box(frame.clone()), &handler, &tx).await;
            })
        },
    );

    // 复杂事件帧处理
    let complex_payload_size = complex_frame.payload.as_ref().unwrap().len();
    group.throughput(Throughput::Bytes(complex_payload_size as u64));
    group.bench_with_input(
        BenchmarkId::new("complex_event", complex_payload_size),
        &complex_frame,
        |b, frame: &Frame| {
            b.to_async(&rt).iter(|| async {
                let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
                let handler = EventDispatcherHandler::builder().build();

                let _result =
                    FrameHandler::handle_frame(black_box(frame.clone()), &handler, &tx).await;
            })
        },
    );

    group.finish();
}

// 帧构建基准测试
fn frame_building_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_building");

    group.bench_function("build_ping_frame", |b| {
        b.iter(|| {
            let _frame = FrameHandler::build_ping_frame(black_box(1));
        })
    });

    group.bench_function("build_response_frame", |b| {
        let headers = vec![
            Header {
                key: "message_id".to_string(),
                value: "test_msg".to_string(),
            },
            Header {
                key: "trace_id".to_string(),
                value: "test_trace".to_string(),
            },
        ];
        let payload = b"test response payload".to_vec();

        b.iter(|| {
            let _frame = FrameHandler::build_response_frame(
                black_box(1),
                black_box(headers.clone()),
                black_box(payload.clone()),
            );
        })
    });

    group.finish();
}

// 头部处理基准测试
fn header_processing_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("header_processing");

    // 测试不同数量的头部处理性能
    let header_counts = [1, 5, 10, 50];

    for &count in &header_counts {
        let headers: Vec<Header> = (0..count)
            .map(|i| Header {
                key: format!("header_key_{i}"),
                value: format!("header_value_{i}"),
            })
            .collect();

        group.throughput(Throughput::Elements(count));
        group.bench_with_input(
            BenchmarkId::new("get_header_value", count),
            &headers,
            |b, headers: &Vec<Header>| {
                b.iter(|| {
                    // 模拟头部查找操作
                    let _result = headers
                        .iter()
                        .find(|h| h.key == black_box("header_key_0"))
                        .map(|h| h.value.clone());
                })
            },
        );
    }

    group.finish();
}

// 并发帧处理基准测试
fn concurrent_frame_handling_benchmark(c: &mut Criterion) {
    let rt = Builder::new_current_thread().enable_all().build().unwrap();
    let mut group = c.benchmark_group("concurrent_frame_handling");

    let frames = vec![
        generate_ping_frame(),
        generate_pong_frame(),
        generate_simple_event_frame(),
        generate_complex_event_frame(),
    ];

    group.bench_function("sequential_processing", |b| {
        b.to_async(&rt).iter(|| async {
            let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
            let handler = EventDispatcherHandler::builder().build();

            for frame in black_box(&frames) {
                let _result = FrameHandler::handle_frame(frame.clone(), &handler, &tx).await;
            }
        })
    });

    group.bench_function("concurrent_processing", |b| {
        b.to_async(&rt).iter(|| async {
            let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
            let handler = EventDispatcherHandler::builder().build();

            let futs = black_box(&frames)
                .iter()
                .map(|frame| FrameHandler::handle_frame(frame.clone(), &handler, &tx));

            let _results = join_all(futs).await;
        })
    });

    group.finish();
}

// 配置 Criterion
criterion_group!(
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(50)
        .warm_up_time(Duration::from_secs(3));
    targets = control_frame_benchmark,
              data_frame_benchmark,
              frame_building_benchmark,
              header_processing_benchmark,
              concurrent_frame_handling_benchmark
);

criterion_main!(benches);
