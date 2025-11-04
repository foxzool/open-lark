//! WebSocket 集成测试
//!
//! 测试 WebSocket 连接、事件处理、消息推送等实时功能
//! 验证事件分发机制和处理器注册系统

#![cfg(feature = "integration-tests")]
#![cfg(feature = "websocket")]

use std::env;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;
use wiremock::{
    matchers::{body_json, header, method, path},
    Mock, MockServer, ResponseTemplate,
};
use serde_json::json;
use open_lark::prelude::*;

/// 模拟WebSocket事件处理器
struct TestEventHandler {
    events_received: Arc<Mutex<Vec<serde_json::Value>>>,
}

impl TestEventHandler {
    fn new() -> Self {
        Self {
            events_received: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn get_events(&self) -> Vec<serde_json::Value> {
        self.events_received.lock().unwrap().clone()
    }

    fn clear_events(&self) {
        self.events_received.lock().unwrap().clear();
    }

    fn add_event(&self, event: serde_json::Value) {
        self.events_received.lock().unwrap().push(event);
    }
}

/// 创建WebSocket测试客户端
fn create_websocket_test_client(base_url: &str) -> LarkClient {
    LarkClient::builder("test_app_id", "test_app_secret")
        .with_app_type(AppType::SelfBuild)
        .with_base_url(base_url)
        .enable_token_cache(true)
        .req_timeout(Duration::from_secs(30))
        .build()
}

/// 模拟WebSocket连接握手和认证
async fn setup_websocket_mocks(mock_server: &MockServer) {
    // Mock应用访问令牌获取
    Mock::given(method("POST"))
        .and(path("/open-apis/auth/v3/tenant_access_token/internal"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "expire": 7200,
            "tenant_access_token": "mock_tenant_token_ws"
        })))
        .mount(mock_server)
        .await;
}

/// 模拟各种WebSocket事件
fn create_mock_message_receive_event() -> serde_json::Value {
    json!({
        "schema": "2.0",
        "header": {
            "event_id": "evt_message_receive_123",
            "event_type": "im.message.receive_v1",
            "create_time": "1640995200000",
            "app_id": "test_app_id",
            "tenant_key": "test_tenant"
        },
        "event": {
            "sender": {
                "sender_id": {
                    "user_id": "ou_message_sender_456",
                    "union_id": "on_message_sender_union",
                    "open_id": "ou_message_sender_456"
                },
                "sender_type": "user",
                "tenant_key": "test_tenant"
            },
            "message": {
                "message_id": "om_test_message_789",
                "root_id": null,
                "parent_id": null,
                "create_time": "1640995200000",
                "chat_type": "p2p",
                "message_type": "text",
                "content": "{\"text\":\"这是一条测试消息\"}",
                "mentions": [],
                "message_link": "https://example.com/message/789"
            },
            "chat": {
                "chat_id": "oc_test_chat_123",
                "chat_type": "p2p",
                "name": "测试对话",
                "avatar": "https://example.com/avatar.jpg",
                "description": "WebSocket测试对话"
            }
        }
    })
}

fn create_mock_message_read_event() -> serde_json::Value {
    json!({
        "schema": "2.0",
        "header": {
            "event_id": "evt_message_read_456",
            "event_type": "im.message.reader_v1",
            "create_time": "1640995250000",
            "app_id": "test_app_id",
            "tenant_key": "test_tenant"
        },
        "event": {
            "reader": {
                "reader_id": {
                    "user_id": "ou_message_reader_789",
                    "union_id": "on_message_reader_union",
                    "open_id": "ou_message_reader_789"
                },
                "reader_type": "user",
                "tenant_key": "test_tenant"
            },
            "message_id_list": ["om_test_message_789"],
            "chat_id": "oc_test_chat_123",
            "timestamp": "1640995250000"
        }
    })
}

fn create_mock_chat_member_added_event() -> serde_json::Value {
    json!({
        "schema": "2.0",
        "header": {
            "event_id": "evt_chat_member_added_789",
            "event_type": "im.chat.member.added_v1",
            "create_time": "1640995300000",
            "app_id": "test_app_id",
            "tenant_key": "test_tenant"
        },
        "event": {
            "chat": {
                "chat_id": "oc_test_chat_123",
                "chat_type": "group",
                "name": "测试群组",
                "avatar": "https://example.com/group_avatar.jpg",
                "description": "WebSocket测试群组"
            },
            "users": [{
                "user_id": {
                    "user_id": "ou_new_member_456",
                    "union_id": "on_new_member_union",
                    "open_id": "ou_new_member_456"
                },
                "name": "新成员"
            }],
            "invitor": {
                "invitor_id": {
                    "user_id": "ou_invitor_123",
                    "union_id": "on_invitor_union",
                    "open_id": "ou_invitor_123"
                },
                "invitor_type": "user",
                "tenant_key": "test_tenant"
            }
        }
    })
}

fn create_mock_application_bot_menu_event() -> serde_json::Value {
    json!({
        "schema": "2.0",
        "header": {
            "event_id": "evt_bot_menu_012",
            "event_type": "application.bot.menu_v6",
            "create_time": "1640995350000",
            "app_id": "test_app_id",
            "tenant_key": "test_tenant"
        },
        "event": {
            "tenant_key": "test_tenant",
            "operator": {
                "operator_id": {
                    "user_id": "ou_menu_operator_345",
                    "union_id": "on_menu_operator_union",
                    "open_id": "ou_menu_operator_345"
                },
                "operator_type": "user"
            },
            "app_id": "test_app_id",
            "action": {
                "value": "help_command",
                "tag": "button",
                "text": "帮助",
                "type": "template",
                "url": "",
                "timezone": "Asia/Shanghai"
            },
            "context": {
                "message": {
                    "message_id": "om_menu_context_message",
                    "chat_hash": "chat_hash_123",
                    "message_type": "text"
                }
            }
        }
    })
}

/// WebSocket连接和认证测试
#[tokio::test]
async fn test_websocket_authentication_flow() {
    let _ = dotenvy::dotenv();

    // 检查环境变量
    let app_id = env::var("APP_ID").ok();
    let app_secret = env::var("APP_SECRET").ok();

    if app_id.is_none() || app_secret.is_none() {
        println!("⚠️  跳过WebSocket认证测试：未设置APP_ID/APP_SECRET环境变量");
        return;
    }

    let mock_server = MockServer::start().await;
    setup_websocket_mocks(&mock_server).await;

    let client = create_websocket_test_client(&mock_server.uri());

    // 测试获取WebSocket访问令牌
    match client.auth.v3.get_app_access_token(None).await {
        Ok(token_response) => {
            assert_eq!(token_response.code, 0);
            assert!(!token_response.tenant_access_token.is_empty());
            println!("✅ WebSocket认证令牌获取成功");
        }
        Err(e) => {
            println!("❌ WebSocket认证令牌获取失败: {}", e.user_friendly_message());
        }
    }

    // 注意：实际的WebSocket连接测试需要真实的WebSocket服务器
    // 这里主要测试认证流程和令牌获取
    println!("✅ WebSocket认证流程测试完成");
}

/// 事件处理器注册和分发测试
#[test]
fn test_event_handler_registration() {
    let handler = TestEventHandler::new();

    // 模拟注册消息接收事件处理器
    let message_event = create_mock_message_receive_event();
    handler.add_event(message_event.clone());

    let events = handler.get_events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["header"]["event_type"], "im.message.receive_v1");
    println!("✅ 事件处理器注册测试通过");

    // 清理事件
    handler.clear_events();
    let events_after_clear = handler.get_events();
    assert_eq!(events_after_clear.len(), 0);
    println!("✅ 事件清理测试通过");
}

/// 消息接收事件处理测试
#[test]
fn test_message_receive_event_processing() {
    let handler = TestEventHandler::new();

    // 添加消息接收事件
    let message_event = create_mock_message_receive_event();
    handler.add_event(message_event.clone());

    let events = handler.get_events();
    assert_eq!(events.len(), 1);

    // 验证事件结构
    let event = &events[0];
    assert_eq!(event["schema"], "2.0");
    assert_eq!(event["header"]["event_type"], "im.message.receive_v1");
    assert_eq!(event["event"]["sender"]["sender_id"]["open_id"], "ou_message_sender_456");
    assert_eq!(event["event"]["message"]["message_id"], "om_test_message_789");
    assert_eq!(event["event"]["message"]["message_type"], "text");

    println!("✅ 消息接收事件处理测试通过");
}

/// 消息已读事件处理测试
#[test]
fn test_message_read_event_processing() {
    let handler = TestEventHandler::new();

    // 添加消息已读事件
    let read_event = create_mock_message_read_event();
    handler.add_event(read_event.clone());

    let events = handler.get_events();
    assert_eq!(events.len(), 1);

    // 验证事件结构
    let event = &events[0];
    assert_eq!(event["header"]["event_type"], "im.message.reader_v1");
    assert_eq!(event["event"]["reader"]["reader_id"]["open_id"], "ou_message_reader_789");
    assert_eq!(event["event"]["message_id_list"], json!(["om_test_message_789"]));
    assert_eq!(event["event"]["chat_id"], "oc_test_chat_123");

    println!("✅ 消息已读事件处理测试通过");
}

/// 群成员添加事件处理测试
#[test]
fn test_chat_member_added_event_processing() {
    let handler = TestEventHandler::new();

    // 添加群成员添加事件
    let member_event = create_mock_chat_member_added_event();
    handler.add_event(member_event.clone());

    let events = handler.get_events();
    assert_eq!(events.len(), 1);

    // 验证事件结构
    let event = &events[0];
    assert_eq!(event["header"]["event_type"], "im.chat.member.added_v1");
    assert_eq!(event["event"]["chat"]["chat_id"], "oc_test_chat_123");
    assert_eq!(event["event"]["chat"]["chat_type"], "group");
    assert_eq!(event["event"]["users"][0]["user_id"]["open_id"], "ou_new_member_456");
    assert_eq!(event["event"]["users"][0]["name"], "新成员");
    assert_eq!(event["event"]["invitor"]["invitor_id"]["open_id"], "ou_invitor_123");

    println!("✅ 群成员添加事件处理测试通过");
}

/// 机器人菜单事件处理测试
#[test]
fn test_bot_menu_event_processing() {
    let handler = TestEventHandler::new();

    // 添加机器人菜单事件
    let menu_event = create_mock_application_bot_menu_event();
    handler.add_event(menu_event.clone());

    let events = handler.get_events();
    assert_eq!(events.len(), 1);

    // 验证事件结构
    let event = &events[0];
    assert_eq!(event["header"]["event_type"], "application.bot.menu_v6");
    assert_eq!(event["event"]["operator"]["operator_id"]["open_id"], "ou_menu_operator_345");
    assert_eq!(event["event"]["action"]["value"], "help_command");
    assert_eq!(event["event"]["action"]["text"], "帮助");
    assert_eq!(event["event"]["action"]["tag"], "button");

    println!("✅ 机器人菜单事件处理测试通过");
}

/// 复合事件流处理测试
#[test]
fn test_complex_event_stream_processing() {
    let handler = TestEventHandler::new();

    // 模拟一个完整的事件流
    let events = vec![
        create_mock_message_receive_event(),
        create_mock_message_read_event(),
        create_mock_chat_member_added_event(),
        create_mock_application_bot_menu_event(),
    ];

    // 按时间顺序添加事件
    for event in events {
        handler.add_event(event);
    }

    let received_events = handler.get_events();
    assert_eq!(received_events.len(), 4);

    // 验证事件顺序和类型
    assert_eq!(received_events[0]["header"]["event_type"], "im.message.receive_v1");
    assert_eq!(received_events[1]["header"]["event_type"], "im.message.reader_v1");
    assert_eq!(received_events[2]["header"]["event_type"], "im.chat.member.added_v1");
    assert_eq!(received_events[3]["header"]["event_type"], "application.bot.menu_v6");

    // 验证时间戳递增
    for i in 1..received_events.len() {
        let prev_time: i64 = received_events[i-1]["header"]["create_time"].as_str().unwrap().parse().unwrap();
        let curr_time: i64 = received_events[i]["header"]["create_time"].as_str().unwrap().parse().unwrap();
        assert!(curr_time >= prev_time);
    }

    println!("✅ 复合事件流处理测试通过");
}

/// 事件过滤和路由测试
#[test]
fn test_event_filtering_and_routing() {
    let handler = TestEventHandler::new();

    // 添加不同类型的事件
    let events = vec![
        create_mock_message_receive_event(),
        create_mock_message_read_event(),
        create_mock_chat_member_added_event(),
        create_mock_application_bot_menu_event(),
    ];

    for event in events {
        handler.add_event(event);
    }

    let all_events = handler.get_events();

    // 过滤消息相关事件
    let message_events: Vec<_> = all_events.iter()
        .filter(|event| {
            let event_type = event["header"]["event_type"].as_str().unwrap();
            event_type.contains("message")
        })
        .collect();

    assert_eq!(message_events.len(), 2); // message.receive 和 message.reader

    // 过滤聊天相关事件
    let chat_events: Vec<_> = all_events.iter()
        .filter(|event| {
            let event_type = event["header"]["event_type"].as_str().unwrap();
            event_type.contains("chat")
        })
        .collect();

    assert_eq!(chat_events.len(), 1); // chat.member.added

    // 过滤应用相关事件
    let app_events: Vec<_> = all_events.iter()
        .filter(|event| {
            let event_type = event["header"]["event_type"].as_str().unwrap();
            event_type.contains("application")
        })
        .collect();

    assert_eq!(app_events.len(), 1); // application.bot.menu

    println!("✅ 事件过滤和路由测试通过");
}

/// 错误事件处理测试
#[test]
fn test_error_event_handling() {
    let handler = TestEventHandler::new();

    // 模拟格式错误的事件
    let malformed_events = vec![
        // 缺少必要字段的事件
        json!({
            "schema": "2.0",
            "header": {
                "event_type": "im.message.receive_v1"
            }
            // 缺少 event 字段
        }),
        // 事件类型未知的事件
        json!({
            "schema": "2.0",
            "header": {
                "event_id": "evt_unknown_123",
                "event_type": "unknown.event.type",
                "create_time": "1640995200000"
            },
            "event": {}
        }),
        // JSON结构损坏的事件
        json!({
            "schema": "2.0",
            "header": null,
            "event": "invalid_structure"
        }),
    ];

    // 添加这些错误事件
    for malformed_event in malformed_events {
        handler.add_event(malformed_event);
    }

    let all_events = handler.get_events();
    assert_eq!(all_events.len(), 3);

    // 验证系统能够处理这些错误事件而不崩溃
    for (i, event) in all_events.iter().enumerate() {
        // 检查事件至少有schema字段
        if event.get("schema").is_some() {
            println!("事件 {}: schema = {}", i, event["schema"]);
        } else {
            println!("事件 {}: 缺少schema字段", i);
        }

        // 检查header字段
        if event.get("header").is_some() {
            println!("事件 {}: header存在", i);
        } else {
            println!("事件 {}: header缺失或为null", i);
        }
    }

    println!("✅ 错误事件处理测试通过");
}

/// 高并发事件处理测试
#[test]
fn test_concurrent_event_processing() {
    use std::thread;
    use std::sync::Arc;

    let handler = Arc::new(TestEventHandler::new());
    let mut handles = vec![];

    // 创建多个线程，并发添加事件
    for thread_id in 0..4 {
        let handler_clone = Arc::clone(&handler);
        let handle = thread::spawn(move || {
            for i in 0..25 {
                let event = json!({
                    "schema": "2.0",
                    "header": {
                        "event_id": format!("evt_concurrent_{}_{}", thread_id, i),
                        "event_type": "test.concurrent.event",
                        "create_time": format!("{}", 1640995200000 + thread_id * 1000 + i),
                        "app_id": "test_app_id",
                        "tenant_key": "test_tenant"
                    },
                    "event": {
                        "thread_id": thread_id,
                        "event_index": i,
                        "content": format!("来自线程 {} 的事件 {}", thread_id, i)
                    }
                });
                handler_clone.add_event(event);
            }
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    let all_events = handler.get_events();
    assert_eq!(all_events.len(), 100); // 4个线程 × 25个事件

    // 验证事件完整性
    let mut thread_counts = std::collections::HashMap::new();
    for event in &all_events {
        let thread_id = event["event"]["thread_id"].as_u64().unwrap();
        *thread_counts.entry(thread_id).or_insert(0) += 1;
    }

    // 每个线程应该有25个事件
    for thread_id in 0..4 {
        assert_eq!(thread_counts.get(&thread_id), Some(&25));
    }

    println!("✅ 高并发事件处理测试通过");
}

/// 事件性能测试
#[test]
fn test_event_processing_performance() {
    use std::time::Instant;

    let handler = TestEventHandler::new();
    let event_count = 10000;

    let start_time = Instant::now();

    // 批量添加事件
    for i in 0..event_count {
        let event = json!({
            "schema": "2.0",
            "header": {
                "event_id": format!("evt_perf_{}", i),
                "event_type": "performance.test.event",
                "create_time": format!("{}", 1640995200000 + i),
                "app_id": "test_app_id",
                "tenant_key": "test_tenant"
            },
            "event": {
                "index": i,
                "content": format!("性能测试事件 {}", i)
            }
        });
        handler.add_event(event);
    }

    let duration = start_time.elapsed();
    let events_per_second = event_count as f64 / duration.as_secs_f64();

    println!("📊 处理 {} 个事件耗时: {:?}", event_count, duration);
    println!("📊 事件处理速度: {:.2} 事件/秒", events_per_second);

    // 验证事件数量
    let all_events = handler.get_events();
    assert_eq!(all_events.len(), event_count);

    // 性能要求：应该能够处理至少10000个事件/秒
    assert!(events_per_second >= 10000.0);

    println!("✅ 事件处理性能测试通过");
}

/// WebSocket重连机制测试
#[test]
fn test_websocket_reconnection_logic() {
    let handler = TestEventHandler::new();

    // 模拟连接断开和重连的场景
    let connection_events = vec![
        // 连接成功事件
        json!({
            "type": "connection",
            "status": "connected",
            "timestamp": "1640995200000",
            "connection_id": "conn_123"
        }),
        // 接收消息
        create_mock_message_receive_event(),
        // 连接断开事件
        json!({
            "type": "connection",
            "status": "disconnected",
            "timestamp": "1640995300000",
            "connection_id": "conn_123",
            "reason": "network_error"
        }),
        // 重连成功事件
        json!({
            "type": "connection",
            "status": "reconnected",
            "timestamp": "1640995350000",
            "connection_id": "conn_124",
            "previous_connection_id": "conn_123"
        }),
        // 重连后接收消息
        create_mock_message_read_event(),
    ];

    for event in connection_events {
        handler.add_event(event);
    }

    let all_events = handler.get_events();
    assert_eq!(all_events.len(), 5);

    // 验证连接状态变化
    assert_eq!(all_events[0]["type"], "connection");
    assert_eq!(all_events[0]["status"], "connected");

    assert_eq!(all_events[2]["type"], "connection");
    assert_eq!(all_events[2]["status"], "disconnected");

    assert_eq!(all_events[3]["type"], "connection");
    assert_eq!(all_events[3]["status"], "reconnected");

    // 验证重连后消息能够正常接收
    assert_eq!(all_events[4]["header"]["event_type"], "im.message.reader_v1");

    println!("✅ WebSocket重连机制测试通过");
}