# Root Crate Re-exports 审计报告

**审计目标**: 检查 Cargo.toml (lines 186-208) 中 23 个重导出依赖是否在 SDK 公共 API 中使用

**审计方法**: 
- 搜索各依赖的类型在公开结构体字段、函数签名、特 trait 定义中的使用
- 重点关注用户需要直接引用的类型

---

## 审计结果汇总

| # | 依赖 | 分类 | 理由 |
|---|------|------|------|
| 1 | chrono | **保留** | `DateTime<Utc>` 出现在多个公开结构体字段中 (openlark-client, openlark-auth, openlark-docs) |
| 2 | serde | **保留** | `Serialize`/`Deserialize` trait 被大量公开结构体 derive |
| 3 | serde_json | **保留** | JSON 序列化是 SDK 核心功能 |
| 4 | serde_repr | **保留** | 在 openlark-docs 的 bitable/lingo 模块中用于公开结构体 |
| 5 | reqwest | **移除** | 仅内部使用，未在公开 API 签名中暴露 |
| 6 | tokio | **移除** | 主要用于 `#[tokio::test]` 测试代码，非公开 API |
| 7 | async-trait | **移除** | 代码库中完全未使用 |
| 8 | anyhow | **移除** | 代码库中完全未使用 |
| 9 | thiserror | **移除** | 内部错误定义，未暴露给用户 |
| 10 | hmac | **移除** | 代码库中完全未使用 |
| 11 | sha2 | **移除** | 代码库中完全未使用 |
| 12 | base64 | **移除** | 代码库中完全未使用 |
| 13 | urlencoding | **移除** | 仅内部 URL 编码使用 |
| 14 | url | **移除** | 仅内部使用，未在公开 API 中暴露 |
| 15 | uuid | **移除** | 仅内部生成 request_id |
| 16 | rand | **移除** | 仅内部随机数生成 |
| 17 | strum | **移除** | 代码库中完全未使用 |
| 18 | strum_macros | **移除** | 代码库中完全未使用 |
| 19 | log | **移除** | 内部日志记录 |
| 20 | tracing | **移除** | 内部追踪 |
| 21 | tracing-subscriber | **移除** | 内部使用 |
| 22 | lark-websocket-protobuf | **移除** | WebSocket 内部实现 |

## Dev Dependencies (不计入重导出)

| # | 依赖 | 用途 |
|---|------|------|
| 23 | colored | Examples 彩色输出 |
| 24 | clap | Examples 命令行参数解析 |

---

## 详细分析

### 需要保留的依赖 (API 表面需要)

#### 1. chrono
- **使用位置**: 公开结构体字段
  - `openlark-client/src/types/auth.rs`: `created_at: DateTime<Utc>`
  - `openlark-auth/src/models/authen/mod.rs`: `hire_date`, `expires_at`, `created_at`
  - `openlark-auth/src/models/auth/mod.rs`: `expires_at`, `created_at`
  - `openlark-docs/src/models.rs`: `create_time`, `modify_time`, `expire_time`
- **结论**: 用户在处理 API 响应时需要使用 `DateTime` 类型

#### 2. serde
- **使用位置**: 大量公开结构体 derive
  - `#[derive(serde::Serialize, serde::Deserialize)]` 应用于几乎所有 API 请求/响应结构体
- **结论**: 用户自定义类型需要实现 Serialize/Deserialize

#### 3. serde_json
- **使用位置**: API 参数和响应序列化
- **结论**: 用户需要构造 JSON 请求体

#### 4. serde_repr
- **使用位置**: 
  - `openlark-docs/src/base/bitable/v1/app/table/field/create.rs`
  - `openlark-docs/src/baike/lingo/v1/models.rs`
- **结论**: 某些数字类型的序列化需要

### 可移除的依赖 (仅内部使用)

#### 5. reqwest
- **使用位置**: `openlark-core/src/performance/mod.rs`, `openlark-core/src/request_builder/`
- **结论**: 内部 HTTP 客户端实现，用户不直接使用

#### 6-22. 其他依赖
- 均未在公开 API 签名中出现
- 主要用于内部实现、测试、或完全未使用

---

## 建议

**可移除依赖数量**: 18 个 (78%)
**应保留依赖数量**: 4 个 (22%)

如果目标是精简依赖:
- **优先移除**: async-trait, anyhow, hmac, sha2, base64, strum, strum_macros (完全未使用)
- **可移除但需评估**: log, tracing, tracing-subscriber (如果用户不需要日志集成)

---

*审计日期: 2026-02-28*
*审计工具: grep 代码搜索*
