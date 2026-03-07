# TestServer 使用限制说明

## 问题

`TestServer` 目前设计为仅在同一 crate 内使用，因为它：
1. 使用 `#[cfg(test)]` 标记
2. 依赖 `wiremock`（仅在 `[dev-dependencies]` 中可用）

## 解决方案

### 方案 1：在同一 crate 内使用 TestServer

```rust
// 在 openlark-core 的测试中
use openlark_core::testing::prelude::*;

#[tokio::test]
async fn test_example() {
    let server = TestServer::new().await;
    server.mock_success("/api/v1/test", json!({"code": 0})).await;
    // ...
}
```

### 方案 2：在其他 crate 中直接使用 wiremock

```rust
// 在 openlark-webhook 的测试中
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_webhook() {
    let server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/webhook"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"code": 0})))
        .mount(&server)
        .await;
    
    // ...
}
```

### 方案 3：创建共享测试工具（推荐）

创建一个新的 crate `openlark-test-utils`：

```toml
# crates/openlark-test-utils/Cargo.toml
[package]
name = "openlark-test-utils"
version = "0.1.0"

[dependencies]
wiremock = "0.6"
serde_json = "1.0"

[lib]
name = "openlark_test_utils"
path = "src/lib.rs"
```

```rust
// crates/openlark-test-utils/src/lib.rs
pub use wiremock::{Mock, MockServer, ResponseTemplate};
pub use wiremock::matchers::*;

pub struct TestServer {
    inner: MockServer,
}

impl TestServer {
    pub async fn new() -> Self {
        Self {
            inner: MockServer::start().await,
        }
    }
    
    pub fn uri(&self) -> String {
        self.inner.uri()
    }
    
    pub async fn mock_success(&self, route: &str, body: serde_json::Value) {
        Mock::given(method("POST"))
            .and(path(route))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&self.inner)
            .await;
    }
}
```

然后在其他 crates 中使用：

```toml
[dev-dependencies]
openlark-test-utils = { path = "../openlark-test-utils" }
```

```rust
use openlark_test_utils::TestServer;
use serde_json::json;

#[tokio::test]
async fn test_example() {
    let server = TestServer::new().await;
    server.mock_success("/api/v1/test", json!({"code": 0})).await;
    // ...
}
```

## 当前建议

**短期**： 在每个 crate 中直接使用 wiremock（方案 2）

**长期**： 创建 `openlark-test-utils` crate（方案 3）

## 迁移指南

参见 [TEST_MIGRATION.md](./TEST_MIGRATION.md) 了解如何迁移现有测试。
