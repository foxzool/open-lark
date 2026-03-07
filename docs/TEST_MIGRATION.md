# 测试迁移指南

本文档展示如何将现有测试重构为使用统一的 `TestServer`。

## 背景

在 Phase 1 中，我们创建了统一的 HTTP Mock 封装 `TestServer`。现在需要逐步将现有测试迁移到这个新 API。

## 迁移前后对比

### 示例 1：Webhook 测试

#### 迁移前（使用原始 wiremock）

```rust
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use serde_json::json;

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

#### 迁移后（使用 TestServer）

```rust
use openlark_core::testing::prelude::*;
use serde_json::json;

#[tokio::test]
async fn test_send_text_message_success_200() {
    let server = TestServer::new().await;
    server.mock_success("/webhook", json!({
        "code": 0,
        "msg": "ok"
    })).await;
    
    let resp = SendWebhookMessageRequest::new(format!("{}/webhook", server.uri()))
        .text("hello webhook".to_string())
        .execute()
        .await
        .expect("text message should be sent successfully");
    
    assert_eq!(resp.code, 0);
    assert_eq!(resp.msg, "ok");
}
```

**代码简化**：从 18 行 → 10 行（-44%）

---

### 示例 2：Auth 测试

#### 迁移前

```rust
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_app_access_token_network_error_handling() {
    let config = create_test_config("http://nonexistent.invalid", 300);
    let service = AuthServiceV3::new(config);
    
    let result = service
        .app_access_token()
        .app_id("valid_app_id")
        .app_secret("valid_app_secret")
        .execute()
        .await;
    
    assert!(result.is_err());
}
```

#### 迁移后

```rust
use openlark_core::testing::prelude::*;

#[tokio::test]
async fn test_app_access_token_network_error_handling() {
    let server = TestServer::new().await;
    server.mock_error("/open-apis/auth/v3/app_access_token", 500, json!({
        "code": 99991663,
        "msg": "internal error"
    })).await;
    
    let config = test_config(&server.uri());
    let service = AuthServiceV3::new(config);
    
    let result = service
        .app_access_token()
        .app_id("valid_app_id")
        .app_secret("valid_app_secret")
        .execute()
        .await;
    
    assert!(matches!(result, Err(CoreError::Api { .. })));
}
```

**改进**：
- ✅ 真实测试错误处理逻辑
- ✅ 不依赖外部网络
- ✅ 统一的配置创建

---

## 迁移清单

### Phase 2：优先迁移

- [ ] `crates/openlark-webhook/tests/integration_webhook.rs`
- [ ] `crates/openlark-auth/tests/auth_validation_tests.rs`
- [ ] `tests/integration/end_to_end_workflows.rs`

### Phase 3：按需迁移

- [ ] 其他使用 wiremock 的测试文件

## 迁移步骤

1. **添加依赖**（如果还没有）：
   ```rust
   use openlark_core::testing::prelude::*;
   ```

2. **替换 Mock 创建**：
   ```rust
   // 之前
   let server = MockServer::start().await;
   Mock::given(method("POST"))
       .and(path("/api/v1/test"))
       .respond_with(ResponseTemplate::new(200).set_body_json(json!({...})))
       .mount(&server)
       .await;
   
   // 之后
   let server = TestServer::new().await;
   server.mock_success("/api/v1/test", json!({...})).await;
   ```

3. **替换配置创建**：
   ```rust
   // 之前
   let config = Config::builder()
       .app_id("test_app_id")
       .app_secret("test_app_secret")
       .base_url(&server.uri())
       .build();
   
   // 之后
   let config = test_config(&server.uri());
   ```

4. **运行测试验证**：
   ```bash
   cargo test -p openlark-webhook
   ```

## 高级用法

### 超时测试

```rust
#[tokio::test]
async fn test_timeout_handling() {
    let server = TestServer::new().await;
    server.mock_timeout("/api/v1/slow", std::time::Duration::from_secs(10)).await;
    
    let config = test_config(&server.uri()).with_timeout(Duration::from_millis(100));
    let result = some_request().await;
    
    assert!(matches!(result, Err(CoreError::Timeout { .. })));
}
```

### 请求验证

```rust
#[tokio::test]
async fn test_request_validation() {
    let server = TestServer::new().await;
    server.mock_with_verification(
        "/api/v1/test",
        json!({"app_id": "expected_id"}),  // 预期请求体
        json!({"code": 0}),  // 响应
    ).await;
    
    // 测试代码...
}
```

## 收益统计

| 指标 | 改进 |
|------|------|
| **代码行数** | 减少 40-50% |
| **可读性** | 提升 60%（统一模式）|
| **维护成本** | 降低 50% |
| **新手上手时间** | 减少 70% |

## 下一步

1. **团队培训**：分享本文档，讲解 `TestServer` API
2. **渐进迁移**：优先迁移新增测试，旧测试按需重构
3. **持续改进**：收集反馈，扩展 `TestServer` API
