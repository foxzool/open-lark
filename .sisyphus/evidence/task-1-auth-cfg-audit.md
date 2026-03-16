# Task 1: Auth Feature cfg 审计报告

## 1. 审计目标
- 列出所有 `#[cfg(feature = "auth")]` 使用点
- 记录 `use openlark_auth::` 导入位置
- 记录 `AuthTokenProvider` 和 `AuthClient` 的使用位置

---

## 2. `#[cfg(feature = "auth")]` 使用点

### 2.1 crates/openlark-client/src/client.rs (5 处)

| 行号 | 代码内容 | 说明 |
|------|----------|------|
| 15 | `#[cfg(feature = "auth")]` | AuthClient 结构体定义 |
| 23 | `#[cfg(feature = "auth")]` | AuthClient impl 块 |
| 74 | `#[cfg(feature = "auth")]` | Client 结构体中 auth 字段 |
| 161 | `#[cfg(feature = "auth")]` | Client::new() 中初始化 AuthClient |
| 179 | `#[cfg(feature = "auth")]` | Client builder 中 auth 设置 |

**详细上下文:**
```rust
// 第15-21行: AuthClient 结构体
#[cfg(feature = "auth")]
#[derive(Debug, Clone)]
pub struct AuthClient {
    pub app: openlark_auth::AuthService,
    pub user: openlark_auth::AuthenService,
    pub oauth: openlark_auth::OAuthService,
}

// 第74-75行: Client 结构体 auth 字段
#[cfg(feature = "auth")]
pub auth: AuthClient,
```

### 2.2 crates/openlark-client/src/registry/bootstrap.rs (2 处)

| 行号 | 代码内容 | 说明 |
|------|----------|------|
| 12 | `#[cfg(feature = "auth")]` | 注册 auth 服务函数调用 |
| 45 | `#[cfg(feature = "auth")]` | register_auth 函数定义 |

**详细上下文:**
```rust
// 第12行: register_compiled_services 函数内
#[cfg(feature = "auth")]
register_auth(registry)?;

// 第45-60行: register_auth 函数
#[cfg(feature = "auth")]
fn register_auth(registry: &mut DefaultServiceRegistry) -> Result<()> {
    let metadata = ServiceMetadata {
        name: "auth".to_string(),
        // ...
    };
    register(registry, metadata)
}
```

### 2.3 crates/openlark-client/src/lib.rs (3 处)

| 行号 | 代码内容 | 说明 |
|------|----------|------|
| 105 | `#[cfg(feature = "auth")]` | 文档注释示例 |
| 290 | `#[cfg(feature = "auth")]` | AuthClient 导出 |
| 398 | `#[cfg(feature = "auth")]` | prelude 导出 |

**详细上下文:**
```rust
// 第105行: 文档注释示例
//! #[cfg(feature = "auth")]
//! let _auth = &client.auth;

// 第290行: pub use client::AuthClient
#[cfg(feature = "auth")]
pub use client::AuthClient;

// 第398行: prelude 模块导出
#[cfg(feature = "auth")]
pub use crate::AuthClient;
```

---

## 3. `use openlark_auth::` 导入位置

### 3.1 业务代码中的导入 (1 处实际使用)

| 文件 | 行号 | 导入内容 | 说明 |
|------|------|----------|------|
| crates/openlark-client/src/core_config.rs | 35 | `use openlark_auth::AuthTokenProvider` | 构建 TokenProvider |

**详细上下文:**
```rust
// core_config.rs 第35行
let provider = openlark_auth::AuthTokenProvider::new(base_core_config.clone());
```

### 3.2 示例/测试中的导入 (8 处)

| 文件 | 行号 | 导入内容 |
|------|------|----------|
| crates/openlark-auth/examples/api_demo.rs | 5 | `AuthService, AuthenService, OAuthService` |
| crates/openlark-auth/examples/api_validation_demo.rs | 5 | `AuthService, AuthenService, OAuthService` |
| crates/openlark-auth/tests/auth_validation_tests.rs | 3 | `auth::auth::v3::*` |
| tests/unit/auth/auth_validation_tests.rs | 3 | `auth::auth::v3::*` |
| crates/openlark-auth/src/lib.rs | 23 | 文档注释示例 |
| crates/openlark-auth/src/services/auth_service.rs | 19,53,80 | 文档注释示例 |
| crates/openlark-auth/src/services/oauth_service.rs | 18,53,76 | 文档注释示例 |
| crates/openlark-auth/src/services/authen_service.rs | 18,52,79 | 文档注释示例 |

---

## 4. AuthTokenProvider 使用位置

| 文件 | 行号 | 使用方式 |
|------|------|----------|
| crates/openlark-auth/src/lib.rs | 69 | `pub use token_provider::AuthTokenProvider` (导出定义) |
| crates/openlark-auth/src/lib.rs | 73 | 二次导出 |
| crates/openlark-auth/src/token_provider.rs | 55 | `pub struct AuthTokenProvider` (结构体定义) |
| crates/openlark-auth/src/token_provider.rs | 61 | `impl Clone for AuthTokenProvider` |
| crates/openlark-auth/src/token_provider.rs | 70 | `impl AuthTokenProvider` 方法实现 |
| crates/openlark-auth/src/token_provider.rs | 164 | `impl TokenProvider for AuthTokenProvider` |
| crates/openlark-auth/src/token_provider.rs | 250 | `use super::AuthTokenProvider` (内部使用) |
| crates/openlark-auth/src/token_provider.rs | 264 | `let provider = AuthTokenProvider::new(config)` |
| **crates/openlark-client/src/core_config.rs** | **35** | **`openlark_auth::AuthTokenProvider::new(...)`** |

---

## 5. AuthClient 使用位置

| 文件 | 行号 | 使用方式 |
|------|------|----------|
| crates/openlark-client/src/client.rs | 17 | `pub struct AuthClient` (结构体定义) |
| crates/openlark-client/src/client.rs | 24 | `impl AuthClient` (impl 块) |
| crates/openlark-client/src/client.rs | 75 | `pub auth: AuthClient` (Client 字段) |
| crates/openlark-client/src/client.rs | 162 | `let auth = AuthClient::new(...)` (初始化) |
| crates/openlark-client/src/lib.rs | 291 | `pub use client::AuthClient` (导出) |
| crates/openlark-client/src/lib.rs | 399 | `pub use crate::AuthClient` (prelude 导出) |

---

## 6. 审计总结

### 6.1 auth feature 依赖关系

```
openlark-client (crate)
    │
    ├── client.rs: AuthClient 结构体 + impl (5 cfg)
    │       └── 依赖 openlark_auth::AuthService/AuthenService/OAuthService
    │
    ├── core_config.rs: 构建 AuthTokenProvider
    │       └── 依赖 openlark_auth::AuthTokenProvider
    │
    ├── registry/bootstrap.rs: 注册 auth 服务 (2 cfg)
    │
    └── lib.rs: 导出 AuthClient (3 cfg)
```

### 6.2 关键发现

1. **auth 不是 optional**: 当前 Cargo.toml 中 auth 是默认启用的 feature
2. **强耦合点**: `core_config.rs` 第35行硬依赖 `openlark_auth::AuthTokenProvider`
3. **影响范围**: 
   - 若将 auth 改为 optional，需要处理 `core_config.rs` 的条件编译
   - 需要为 `Client::new()` 和 builder 添加条件逻辑
   - 需要处理 ServiceRegistry 中的依赖关系

### 6.3 验证命令

```bash
# 验证所有 cfg(feature = "auth") 使用点
grep -rn '#\[cfg(feature = "auth")\]' crates/openlark-client/src/

# 验证所有 use openlark_auth 导入
grep -rn 'use openlark_auth::' . --include='*.rs'

# 验证 AuthTokenProvider 使用
grep -rn 'AuthTokenProvider' crates/openlark-client/

# 验证 AuthClient 使用
grep -rn 'AuthClient' crates/openlark-client/
```

---

**审计完成**: 2026-02-27


## 5. 补充发现 (2026-02-28)

### 5.1 新增 config.rs 使用点

本次审计新增发现 `crates/openlark-client/src/config.rs` 中的 2 处使用点:

| 行号 | 代码内容 | 说明 |
|------|----------|------|
| 300 | `#[cfg(feature = "auth")]` | build_core_config_with_token_provider 方法 |
| 317 | `#[cfg(feature = "auth")]` | get_or_build_core_config_with_token_provider 方法 |

**详细上下文:**
```rust
// 第300-306行
#[cfg(feature = "auth")]
pub fn build_core_config_with_token_provider(&self) -> CoreConfig {
    use openlark_auth::AuthTokenProvider;
    let base_config = self.build_core_config();
    let provider = AuthTokenProvider::new(base_config.clone());
    base_config.with_token_provider(provider)
}

// 第317-323行
#[cfg(feature = "auth")]
pub fn get_or_build_core_config_with_token_provider(&self) -> CoreConfig {
    if let Some(ref core_config) = self.core_config {
        return core_config.clone();
    }
    self.build_core_config_with_token_provider()
}
```

### 5.2 更新后的统计

| 搜索模式 | 原报告 | 本次审计 | 差异 |
|----------|--------|----------|------|
| `#[cfg(feature = "auth")]` | 10 处 | 13 处 | +3 (config.rs) |
| `#[cfg(all(feature = "auth", ...))]` | 0 | 0 | - |
| `#[cfg(any(feature = "auth", ...))]` | 0 | 0 | - |
| `cfg!(...auth...)` | 0 | 0 | - |

### 5.3 完整文件清单

| 文件 | 数量 |
|------|------|
| client.rs | 6 |
| config.rs | 2 |
| registry/bootstrap.rs | 2 |
| lib.rs | 3 |
| **总计** | **13** |

**补充审计完成**: 2026-02-28
