# ServiceRegistry 审计报告

## 概述

本报告审计了 OpenLark 项目中 `ServiceRegistry` 相关类型的外部引用情况。

**审计时间**: 2026-02-28  
**审计范围**: `crates/openlark-client/src/registry/` 模块

## Registry 文件清单

| 文件 | 存在状态 | 外部使用 |
|------|----------|----------|
| `mod.rs` | ✅ 存在 | ✅ 有内部使用 |
| `bootstrap.rs` | ✅ 存在 | ✅ 有内部使用 |
| `dependency_resolver.rs` | ❌ 不存在 | N/A |
| `service_factory.rs` | ❌ 不存在 | N/A |
| `feature_flags.rs` | ❌ 不存在 | N/A |

**注意**: 注册表模块实际只包含 2 个文件：`mod.rs` 和 `bootstrap.rs`。不存在 `dependency_resolver.rs`、`service_factory.rs`、`feature_flags.rs`。

---

## 1. DefaultServiceRegistry

### 定义位置
- `crates/openlark-client/src/registry/mod.rs:143`

### 外部引用

| 文件 | 行号 | 引用类型 | 用途 |
|------|------|----------|------|
| `crates/openlark-client/src/registry/mod.rs` | 148, 154, 163, 256, 274 | 定义+实现 | Default trait 实现、ServiceRegistry trait 实现、测试 |
| `crates/openlark-client/src/registry/bootstrap.rs` | 19, 23, 52, 59, 76, 94, 112, 126, 140, 154, 172, 221 | 内部函数参数 | register_compiled_services 及各 register_* 函数 |
| `crates/openlark-client/src/lib.rs` | 273 | 公开导出 | `pub use registry::{..., DefaultServiceRegistry, ...}` |
| `crates/openlark-client/src/client.rs` | 8, 64, 123, 145 | 字段类型 | `registry: Arc<DefaultServiceRegistry>`、`registry()` getter |
| `crates/openlark-client/src/features.rs` | 5, 18 | 函数参数 | FeatureLoader 中的 registry 参数 |

### 结论
- **仅 openlark-client 内部使用**，无外部 crate 引用

---

## 2. ServiceRegistry (trait)

### 定义位置
- `crates/openlark-client/src/registry/mod.rs:113`

### 外部引用

| 文件 | 行号 | 引用类型 | 用途 |
|------|------|----------|------|
| `crates/openlark-client/src/registry/mod.rs` | 113, 163 | 定义+实现 | trait 定义和 DefaultServiceRegistry 实现 |
| `crates/openlark-client/src/registry/bootstrap.rs` | 19 | 导入 | 用于函数签名 |
| `crates/openlark-client/src/lib.rs` | 273, 384 | 公开导出 | `pub use registry::{..., ServiceRegistry, ...}` |

### 结论
- **仅 openlark-client 内部使用**，trait 已公开导出但无外部实现

---

## 3. ServiceEntry

### 定义位置
- `crates/openlark-client/src/registry/mod.rs:101`

### 外部引用

| 文件 | 行号 | 引用类型 | 用途 |
|------|------|----------|------|
| `crates/openlark-client/src/registry/mod.rs` | 101, 103, 121, 129, 145, 173, 197, 220 | 定义+内部使用 | 结构体定义、trait 方法返回类型、HashMap 存储 |
| `crates/openlark-client/src/lib.rs` | 273 | 公开导出 | `pub use registry::{..., ServiceEntry, ...}` |

### 结论
- **仅 openlark-client 内部使用**

---

## 4. ServiceMetadata

### 定义位置
- `crates/openlark-client/src/registry/mod.rs:65`

### 外部引用

| 文件 | 行号 | 引用类型 | 用途 |
|------|------|----------|------|
| `crates/openlark-client/src/registry/mod.rs` | 65, 103, 115, 164, 258, 276 | 定义+内部使用 | 结构体定义、ServiceEntry 字段、register_service 参数 |
| `crates/openlark-client/src/registry/bootstrap.rs` | 19, 52, 60, 77, 95, 113, 127, 141, 155, 173, 191, 228, 243, 252 | 创建实例 | 各 register_* 函数中创建 ServiceMetadata |
| `crates/openlark-client/src/lib.rs` | 273 | 公开导出 | `pub use registry::{..., ServiceMetadata, ...}` |

### 结论
- **仅 openlark-client 内部使用**

---

## 5. register_compiled_services()

### 定义位置
- `crates/openlark-client/src/registry/bootstrap.rs:23`

### 调用位置

| 文件 | 行号 | 调用方式 | 上下文 |
|------|------|----------|--------|
| `crates/openlark-client/src/client.rs` | 148 | `crate::registry::bootstrap::register_compiled_services(&mut registry)` | Client 初始化时调用 |
| `crates/openlark-client/src/features.rs` | 21 | `crate::registry::bootstrap::register_compiled_services(registry)?` | FeatureLoader 中调用 |
| `crates/openlark-client/src/registry/bootstrap.rs` | 222 | 测试中调用 | `test_register_compiled_services` 测试 |

### 结论
- **仅 openlark-client 内部使用**（client.rs 和 features.rs）
- 无 examples/ 或 tests/ 目录中的外部调用

---

## 6. has_service()

### 定义位置
- `crates/openlark-client/src/registry/mod.rs:132` (trait 方法)
- `crates/openlark-client/src/registry/mod.rs:224` (DefaultServiceRegistry 实现)

### 使用位置

| 文件 | 行号 | 使用方式 | 上下文 |
|------|------|----------|--------|
| `crates/openlark-client/src/registry/mod.rs` | 269 | 内部测试 | `assert!(registry.has_service("test-service"))` |
| `crates/openlark-client/src/lib.rs` | 125 | 文档注释示例 | `//! if registry.has_service("communication") {` |
| `crates/openlark-client/AGENTS.md` | 67 | 文档示例 | `if client.registry().has_service("docs") { ... }` |
| `crates/openlark-client/README.md` | 135 | 文档示例 | `if client.registry().has_service("docs") {` |

### 结论
- **仅文档和内部测试使用**，无实际外部调用

---

## 7. list_services()

### 定义位置
- `crates/openlark-client/src/registry/mod.rs:129` (trait 方法)
- `crates/openlark-client/src/registry/mod.rs:220` (DefaultServiceRegistry 实现)

### 使用位置

| 文件 | 行号 | 使用方式 | 上下文 |
|------|------|----------|--------|
| `crates/openlark-client/src/lib.rs` | 122 | 文档注释示例 | `//! println!("可用服务: {:?}", registry.list_services());` |
| `crates/openlark-client/AGENTS.md` | 70 | 文档示例 | `for entry in client.registry().list_services() { ... }` |
| `crates/openlark-client/README.md` | 130 | 文档示例 | `for entry in client.registry().list_services() {` |

### 结论
- **仅文档使用**，无实际外部调用

---

## 8. dependency_resolver.rs 和 service_factory.rs

### 审计结果

| 文件 | 存在状态 | 外部引用 |
|------|----------|----------|
| `dependency_resolver.rs` | ❌ 不存在 | N/A |
| `service_factory.rs` | ❌ 不存在 | N/A |

**结论**: 这两个文件不存在于 registry 模块中，因此无外部使用。

---

## 总结

### 外部使用情况汇总

| 类型/函数 | 定义文件 | openlark-client 内部使用 | examples/ | tests/ | 其他 crates |
|-----------|----------|--------------------------|-----------|--------|-------------|
| `DefaultServiceRegistry` | mod.rs:143 | ✅ 是 | ❌ | ❌ | ❌ |
| `ServiceRegistry` (trait) | mod.rs:113 | ✅ 是 | ❌ | ❌ | ❌ |
| `ServiceEntry` | mod.rs:101 | ✅ 是 | ❌ | ❌ | ❌ |
| `ServiceMetadata` | mod.rs:65 | ✅ 是 | ❌ | ❌ | ❌ |
| `register_compiled_services()` | bootstrap.rs:23 | ✅ 是 (client.rs, features.rs) | ❌ | ❌ | ❌ |
| `has_service()` | mod.rs:132 | ✅ 仅文档/测试 | ❌ | ❌ | ❌ |
| `list_services()` | mod.rs:129 | ✅ 仅文档 | ❌ | ❌ | ❌ |

### 关键发现

1. **所有 ServiceRegistry 相关类型仅在 openlark-client crate 内部使用**
2. **无外部 crate（examples/、tests/、或其他业务 crate）实际调用 registry API**
3. **has_service() 和 list_services() 仅在文档注释和示例中出现**
4. **register_compiled_services() 只能在 client.rs 和 features.rs 中被调用**
5. **不存在 dependency_resolver.rs 和 service_factory.rs 文件**

### 公开 API 导出

通过 `crates/openlark-client/src/lib.rs:272-274` 公开导出：
```rust
pub use registry::{
    DefaultServiceRegistry, ServiceEntry, ServiceMetadata, ServiceRegistry, ServiceStatus,
};
```

但这些导出目前**未被外部使用**（无 external crate 依赖）。
