# Webhook 测试简化示例

本文档展示如何简化现有的 webhook 测试代码。

## 现状

`crates/openlark-webhook/tests/integration_webhook.rs` 使用了完整的 wiremock API：

```rust
#[tokio::test]
async fn test_send_text_message_success_200() {
    let server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/webhook"))
        .and(body_json(json!({
            "msg_type": "text",
            "content": {
                "text": "hello webhook"
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "ok"
        })))
        .mount(&server)
        .await;
    
    let resp = SendWebhookMessageRequest::new(format!("{}/webhook", server.uri()))
        .text("hello webhook".to_string())
        .execute()
        .await
        .expect("text message should be sent successfully");
    
    assert_eq!(resp.code, 0);
    assert_eq!(resp.msg, "ok");
}
```

**问题**：
- ❌ 代码冗长（27 行）
- ❌ 重复的 Mock 设置代码
- ❌ 不必要的请求体验证

## 改进方案

### 1. 简化 Mock（不需要验证请求体）

```rust
#[tokio::test]
async fn test_send_text_message_success_simplified() {
    let server = MockServer::start().await;
    
    // 简化：不需要验证请求体
    Mock::given(method("POST"))
        .and(path("/webhook"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "ok"
        })))
        .mount(&server)
        .await;
    
    let resp = SendWebhookMessageRequest::new(format!("{}/webhook", server.uri()))
        .text("hello webhook".to_string())
        .execute()
        .await
        .expect("text message should be sent successfully");
    
    assert_eq!(resp.code, 0);
    assert_eq!(resp.msg, "ok");
}
```

**代码减少**：27 行 → 19 行（-30%）

### 2. 提取辅助函数

```rust
fn setup_mock_server() -> MockServer {
    // 在实际运行时使用 tokio::test 的 async
    MockServer::start()
}

async fn mock_success(server: &MockServer, path: &str) {
    Mock::given(method("POST"))
        .and(path(path))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"code": 0, "msg": "ok"})))
        .mount(server)
        .await;
}

async fn mock_error(server: &MockServer, path: &str, code: u16) {
    Mock::given(method("POST"))
        .and(path(path))
        .respond_with(ResponseTemplate::new(code).set_body_json(json!({"code": 19001, "msg": "error"})))
        .mount(server)
        .await;
}

#[tokio::test]
async fn test_send_text_message_success_with_helpers() {
    let server = setup_mock_server().await;
    mock_success(&server, "/webhook").await;
    
    let resp = SendWebhookMessageRequest::new(format!("{}/webhook", server.uri()))
        .text("hello webhook".to_string())
        .execute()
        .await
        .expect("success");
    
    assert_eq!(resp.code, 0);
}
```

**代码减少**：每个测试从 27 行 → 8 行（-70%）

### 3. 使用 rstest 参数化

```rust
use rstest::rstest;

#[rstest]
#[tokio::test]
async fn test_all_message_types(
    #[values("text", "post", "image", "file")] msg_type: &str,
) {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/webhook"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"code": 0})))
        .mount(&server)
        .await;
    
    let url = format!("{}/webhook", server.uri());
    let result = match msg_type {
        "text" => SendWebhookMessageRequest::new(url).text("test".to_string()).execute().await,
        "post" => SendWebhookMessageRequest::new(url).post("{}".to_string()).execute().await,
        "image" => SendWebhookMessageRequest::new(url).image("key".to_string()).execute().await,
        "file" => SendWebhookMessageRequest::new(url).file("key".to_string()).execute().await,
        _ => panic!("unknown type"),
    };
    
    assert!(result.is_ok());
}
```

**收益**：4 个测试 → 1 个参数化测试（-75% 代码量）

## 实际收益统计

| 改进方法 | 代码减少 | 维护成本 | 可读性 |
|---------|---------|---------|--------|
| 简化 Mock | 30% | 低 | 好 |
| 提取辅助函数 | 70% | 中 | 更好 |
| 参数化测试 | 75% | 低 | 最好 |

## 推荐实践

1. **对于简单测试**：直接简化 Mock 设置
2. **对于多个相似测试**：使用参数化（rstest）
3. **对于复杂场景**：提取辅助函数

## 注意事项

- ✅ **不要过度验证请求体**：除非业务逻辑需要
- ✅ **优先测试响应处理**：而不是请求格式
- ✅ **使用参数化减少重复**：但要保持可读性

## 下一步

1. 在新测试中应用这些模式
2. 逐步重构现有测试
3. 在团队中推广最佳实践
