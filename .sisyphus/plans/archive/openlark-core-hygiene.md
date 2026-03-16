# openlark-core 代码卫生修复（P0+P1+P2）

## TL;DR

> **Quick Summary**: 修复 openlark-core 中的宏导出可见性问题（P0）、tracing-subscriber 强依赖与 debug 日志泄露（P1）、validation 日志级别与 AGENTS.md 漂移（P2），全部以最小 breaking 改动落地。
>
> **Deliverables**:
> - 移除 5 个 observability 宏的 `#[macro_export]`
> - `tracing-subscriber` 改为 optional，新增 `tracing-init` feature gate
> - HTTP debug 日志脱敏（不再打印完整 req/resp）
> - validation 日志从 `error!` 降为 `warn!`
> - AGENTS.md 与实际代码结构同步
>
> **Estimated Effort**: Short（~2-3 小时）
> **Parallel Execution**: YES - 2 waves
> **Critical Path**: Task 1 → Task 2 → Task 5（Cargo.toml 改动需先于 observability.rs feature gate）

---

## Context

### Original Request
审查 openlark-core 后发现 P0-P3 级问题，用户要求按最小 breaking 改动修复 P0+P1+P2。

### Interview Summary
**Key Discussions**:
- P0 宏导出：零下游使用，直接移除 `#[macro_export]`
- P1 tracing-subscriber：改为 optional + `tracing-init` feature gate，`otel` 隐式启用 `tracing-init`
- P1 testing 默认开启：Metis 发现 47+ 下游测试文件隐式依赖，决定本次不动
- P1 debug 日志：从 `http_req`（ApiRequest）提取结构化字段替代 `{req:?}`
- P2 validation error!：降为 `warn!`
- P2 AGENTS.md：更新至实际目录结构

**Research Findings**:
- 5 个宏仅在 observability.rs 自身测试中使用
- `init_otel_tracing()` 也依赖 tracing-subscriber，需 `otel` 隐式启用 `tracing-init`
- `req` 变量是 `reqwest::RequestBuilder`，不易提取字段；应从 `http_req: ApiRequest<R>` 取 method/path

### Metis Review
**Identified Gaps** (addressed):
- testing 默认移除会打断 47+ 下游测试 → 决定保留 default，不动
- otel + tracing-init 交互 → otel 隐式启用 tracing-init
- `RequestBuilder` 无法方便提取字段 → 改用 `http_req` 的 method/api_path
- init_tracing 测试需 feature gate → 测试加 `#[cfg(feature = "tracing-init")]`
- tracing-subscriber 需加入 dev-dependencies → 确保测试可编译

---

## Work Objectives

### Core Objective
以最小改动修复 openlark-core 的宏可见性、依赖膨胀、日志安全和文档漂移问题。

### Concrete Deliverables
- `crates/openlark-core/src/observability.rs` — 移除 5 个 `#[macro_export]`
- `crates/openlark-core/Cargo.toml` — tracing-subscriber optional + tracing-init feature
- `crates/openlark-core/src/observability.rs` — init_* 函数 feature gate
- `crates/openlark-core/src/http.rs` — debug 日志脱敏
- `crates/openlark-core/src/validation/core.rs` — error! → warn!
- `crates/openlark-core/AGENTS.md` — 更新目录结构

### Definition of Done
- [x] `cargo test -p openlark-core` 全部通过
- [x] `cargo check -p openlark-core --no-default-features` 编译成功（无 tracing-subscriber 链接）
- [x] `cargo test --workspace` 无下游破坏
- [x] `cargo clippy -p openlark-core -- -D warnings` 零警告

### Must Have
- 5 个 observability 宏不再 `#[macro_export]`
- `tracing-subscriber` 为 optional 依赖
- HTTP debug 日志不包含完整 headers/body/token
- validation 日志级别为 `warn!` 而非 `error!`

### Must NOT Have (Guardrails)
- ❌ 不动 `lib.rs` 中的 `validate_required!` 和 `validate_required_list!` 宏（它们是有意导出的）
- ❌ 不修改 `crates/openlark-core/` 以外的任何文件
- ❌ 不改变任何公开函数签名、返回类型或公共 API 面
- ❌ 不动 `default = ["testing"]`（保留现状）
- ❌ 不动 `observability.rs` 的 `#![allow(dead_code)]`
- ❌ 不动 otel feature gate 以外的 otel 代码
- ❌ 不动 http.rs 中 108/110 行以外的 debug/trace 调用

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: YES
- **Automated tests**: Tests-after（验证现有测试通过）
- **Framework**: cargo test（Rust 内置）
- **No TDD**: 这是卫生修复，不是新功能

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Library/Module**: Use Bash — `cargo test`, `cargo check`, `cargo clippy`
- **Config verification**: Use Bash — `grep` 确认改动到位

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — independent changes, MAX PARALLEL):
├── Task 1: Cargo.toml feature gate 改造 [quick]
├── Task 3: HTTP debug 日志脱敏 [quick]
├── Task 4: Validation 日志降级 [quick]
├── Task 5: AGENTS.md 更新 [quick]

Wave 2 (After Task 1 — depends on Cargo.toml changes):
├── Task 2: observability.rs 宏 + feature gate [quick]

Wave 3 (After ALL — verification):
├── Task 6: 全量验证 [quick]

Wave FINAL (After ALL tasks — independent review):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Scope fidelity check (deep)

Critical Path: Task 1 → Task 2 → Task 6 → F1-F3
Parallel Speedup: ~40% faster than sequential
Max Concurrent: 4 (Wave 1)
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|-----------|--------|
| 1 | — | 2, 6 |
| 2 | 1 | 6 |
| 3 | — | 6 |
| 4 | — | 6 |
| 5 | — | 6 |
| 6 | 1,2,3,4,5 | F1-F3 |

### Agent Dispatch Summary

- **Wave 1**: 4 tasks — T1 → `quick`, T3 → `quick`, T4 → `quick`, T5 → `writing`
- **Wave 2**: 1 task — T2 → `quick`
- **Wave 3**: 1 task — T6 → `unspecified-high`
- **FINAL**: 3 tasks — F1 → `oracle`, F2 → `unspecified-high`, F3 → `deep`

---

## TODOs

> Implementation + Test = ONE Task. Never separate.
> EVERY task MUST have: Recommended Agent Profile + Parallelization info + QA Scenarios.


- [x] 1. Cargo.toml feature gate 改造（tracing-subscriber optional + tracing-init）

  **What to do**:
  - 将 `tracing-subscriber = { workspace = true }` 改为 `tracing-subscriber = { workspace = true, optional = true }`
  - 新增 feature `tracing-init = ["tracing-subscriber"]`
  - 修改 `otel` feature 为 `otel = ["tracing-init", "opentelemetry", "opentelemetry_sdk", "opentelemetry-otlp", "tracing-opentelemetry"]`
  - 在 `[dev-dependencies]` 中添加 `tracing-subscriber = { workspace = true }`（确保测试可编译）
  - `default` 保持 `["testing"]` 不变

  **Must NOT do**:
  - 不动 `default` features
  - 不动其他依赖项
  - 不改 websocket/prost/openlark-protocol 相关配置

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: `[]`
  - `Cargo.toml` 编辑是简单文本替换，无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: YES (Wave 1)
  - **Parallel Group**: Wave 1 (with Tasks 3, 4, 5)
  - **Blocks**: Task 2, Task 6
  - **Blocked By**: None

  **References**:

  **Pattern References**:
  - `crates/openlark-core/Cargo.toml:25` — 当前 tracing-subscriber 硬依赖声明
  - `crates/openlark-core/Cargo.toml:45-55` — 当前 features 定义，参考 otel 的 optional 依赖模式
  - `crates/openlark-core/Cargo.toml:52` — 当前 otel feature 定义，需要追加 `tracing-init`
  - `crates/openlark-core/Cargo.toml:74-76` — dev-dependencies 区域，需追加 tracing-subscriber

  **Acceptance Criteria**:
  - [x]
  - [x] `cargo check -p openlark-core --features tracing-init` 编译成功
  - [x] `cargo check -p openlark-core` 默认编译成功
  - [x] `default` 仍为 `["testing"]`

  **QA Scenarios:**

  ```
  Scenario: tracing-subscriber 不在默认编译路径
    Tool: Bash
    Preconditions: Task 1 changes applied to Cargo.toml
    Steps:
      1. Run `cargo check -p openlark-core --no-default-features 2>&1`
      2. Assert exit code is 0
      3. Run `cargo tree -p openlark-core -e features --no-default-features 2>&1 | grep -c tracing-subscriber` (verify tracing-subscriber not in default dependency tree)
    Expected Result: Both commands succeed; tracing-subscriber marked as optional
    Failure Indicators: Compilation error mentioning tracing_subscriber; tracing-subscriber still non-optional
    Evidence: .sisyphus/evidence/task-1-no-default-check.txt

  Scenario: otel feature 隐式启用 tracing-init
    Tool: Bash
    Preconditions: Task 1 changes applied
    Steps:
      1. Run `grep 'otel' crates/openlark-core/Cargo.toml`
      2. Assert output contains `tracing-init`
    Expected Result: otel feature definition includes tracing-init
    Failure Indicators: otel feature does not list tracing-init
    Evidence: .sisyphus/evidence/task-1-otel-feature.txt
  ```

  **Commit**: YES (groups with Task 2)
  - Message: `refactor(core): 移除 observability 宏导出并将 tracing-subscriber 改为 optional`
  - Files: `crates/openlark-core/Cargo.toml`
  - Pre-commit: `cargo check -p openlark-core --no-default-features`

- [x] 2. observability.rs 宏移除 #[macro_export] + init 函数 feature gate

  **What to do**:
  - 移除 5 个宏的 `#[macro_export]` 属性：
    - `trace_performance!`（line 428）
    - `trace_async_performance!`（line 445）
    - `trace_auth_operation!`（line 478）
    - `trace_http_request!`（line 507）
    - `trace_response_processing!`（line 534）
  - 给 3 个 init 函数添加 feature gate：
    - `init_tracing()`（line 94）→ `#[cfg(feature = "tracing-init")]`
    - `init_tracing_with_filter()`（line 99）→ `#[cfg(feature = "tracing-init")]`
    - `init_structured_tracing()`（line 119）→ `#[cfg(feature = "tracing-init")]`
  - 将 `use tracing_subscriber::{...}` imports（lines 8-13）包裹在 `#[cfg(feature = "tracing-init")]` 中
  - 给 `init_otel_tracing()`（line 163）改为 `#[cfg(all(feature = "otel", feature = "tracing-init"))]`（原来只有 `#[cfg(feature = "otel")]`）
  - 给测试中调用 init_* 的测试函数加 `#[cfg(feature = "tracing-init")]`（line 1040 `test_init_tracing_functions`）

  **Must NOT do**:
  - 不动 `OperationTracker`、`HttpTracker`、`AuthTracker`、`ResponseTracker` 结构体及其 impl（它们只用 `tracing`，不用 `tracing-subscriber`）
  - 不动 `#![allow(dead_code)]`
  - 不动 otel 相关的 `OtelConfig`、`shutdown_otel` 等（它们已有正确的 feature gate）
  - 不动 lib.rs 中的 `validate_required!` 和 `validate_required_list!`

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: `[]`
  - 简单的属性移除和 cfg gate 添加

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2 (sequential after Task 1)
  - **Blocks**: Task 6
  - **Blocked By**: Task 1（Cargo.toml 必须先定义 tracing-init feature）

  **References**:

  **Pattern References**:
  - `crates/openlark-core/src/observability.rs:428` — `#[macro_export]` on trace_performance!
  - `crates/openlark-core/src/observability.rs:445` — `#[macro_export]` on trace_async_performance!
  - `crates/openlark-core/src/observability.rs:478` — `#[macro_export]` on trace_auth_operation!
  - `crates/openlark-core/src/observability.rs:507` — `#[macro_export]` on trace_http_request!
  - `crates/openlark-core/src/observability.rs:534` — `#[macro_export]` on trace_response_processing!
  - `crates/openlark-core/src/observability.rs:8-13` — tracing_subscriber imports 需要 feature gate
  - `crates/openlark-core/src/observability.rs:94-116` — init_tracing / init_tracing_with_filter 函数
  - `crates/openlark-core/src/observability.rs:119-133` — init_structured_tracing 函数
  - `crates/openlark-core/src/observability.rs:163-211` — init_otel_tracing 函数，当前 `#[cfg(feature = "otel")]`
  - `crates/openlark-core/src/observability.rs:1040-1055` — test_init_tracing_functions 测试

  **API/Type References**:
  - `crates/openlark-core/src/observability.rs:15-22` — otel 相关 cfg imports，参考其 feature gate 写法

  **Acceptance Criteria**:
  - [x] `grep -c 'macro_export' crates/openlark-core/src/observability.rs` 返回 0
  - [x] `cargo test -p openlark-core` 全部通过
  - [x] `cargo check -p openlark-core --no-default-features` 编译成功
  - [x] init_tracing 等函数被 `#[cfg(feature = "tracing-init")]` 包裹

  **QA Scenarios:**

  ```
  Scenario: 宏不再导出到 crate 根
    Tool: Bash
    Preconditions: Task 2 changes applied
    Steps:
      1. Run `grep -c 'macro_export' crates/openlark-core/src/observability.rs`
      2. Assert output is `0`
    Expected Result: 零个 #[macro_export] 在 observability.rs 中
    Failure Indicators: 输出大于 0
    Evidence: .sisyphus/evidence/task-2-macro-export-count.txt

  Scenario: 无 tracing-init 时 init 函数不编译
    Tool: Bash
    Preconditions: Task 1 + Task 2 changes applied
    Steps:
      1. Run `cargo check -p openlark-core --no-default-features 2>&1`
      2. Assert exit code is 0（init 函数被 cfg gate 排除，不会因缺少 tracing-subscriber 报错）
    Expected Result: 编译成功
    Failure Indicators: 编译错误提及 tracing_subscriber
    Evidence: .sisyphus/evidence/task-2-no-tracing-init.txt

  Scenario: 测试仍然通过
    Tool: Bash
    Preconditions: Task 1 + Task 2 changes applied
    Steps:
      1. Run `cargo test -p openlark-core 2>&1`
      2. Assert exit code is 0
      3. Assert output contains `test result: ok`
    Expected Result: 所有测试通过
    Failure Indicators: 任何测试失败
    Evidence: .sisyphus/evidence/task-2-tests.txt
  ```

  **Commit**: YES (groups with Task 1)
  - Message: `refactor(core): 移除 observability 宏导出并将 tracing-subscriber 改为 optional`
  - Files: `crates/openlark-core/src/observability.rs`
  - Pre-commit: `cargo test -p openlark-core`

- [x] 3. HTTP debug 日志脱敏
  **What to do**:
  - 修改 `http.rs:108` 的 `debug!("Req:{req:?}")` 为结构化字段日志：
    ```rust
    debug!(
        method = %http_req.method(),
        path = %http_req.api_path(),
        "Sending request"
    );
    ```
  - 修改 `http.rs:110` 的 `debug!("Res:{resp:?}")` 为结构化字段日志：
    ```rust
    debug!(
        success = resp.is_success(),
        code = resp.raw_response.code,
        msg = %resp.raw_response.msg,
        "Received response"
    );
    ```
  - 注意：`req` 是 `reqwest::RequestBuilder`，不易提取字段。改用 `http_req`（`ApiRequest<R>`）的 method/api_path
  - 注意：`resp` 是 `Response<T>`，需要确认 `raw_response` 的字段可用性
  **Must NOT do**:
  - 不动 http.rs 中 108/110 行以外的任何 debug/trace 调用
  - 不改变函数签名或返回类型
  - 不删除日志（只替换为脱敏版本）
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: `[]`
  **Parallelization**:
  - **Can Run In Parallel**: YES (Wave 1)
  - **Parallel Group**: Wave 1 (with Tasks 1, 4, 5)
  - **Blocks**: Task 6
  - **Blocked By**: None
  **References**:
  **Pattern References**:
  - `crates/openlark-core/src/http.rs:100-117` — `do_request` 方法，包含需修改的两行 debug 日志
  - `crates/openlark-core/src/http.rs:38-98` — `request` 方法，参考其 span 中结构化字段的写法（method/path/app_id）
  - `crates/openlark-core/src/http.rs:40-41` — `http_req: ApiRequest<R>` 参数，提供 `method()` 和 `api_path()` 方法
  **API/Type References**:
  - `crates/openlark-core/src/api/mod.rs:93` — `ApiRequest<R>` 结构体定义，确认 `method()` 和 `api_path()` 方法签名
  - `crates/openlark-core/src/api/responses.rs:115` — `Response<T>` 结构体定义，确认 `raw_response` 字段和 `is_success()` 方法
  **Acceptance Criteria**:
  - [x] `grep -c 'req:?' crates/openlark-core/src/http.rs` 返回 0
  - [x] `grep -c 'resp:?' crates/openlark-core/src/http.rs` 返回 0
  - [x] `cargo check -p openlark-core` 编译成功
  - [x] debug 日志仍然存在（只是脱敏了，不是删除了）
  **QA Scenarios:**
  ```
  Scenario: 旧的 Debug 格式已移除
    Tool: Bash
    Preconditions: Task 3 changes applied
    Steps:
      1. Run `grep -c 'req:?' crates/openlark-core/src/http.rs`
      2. Assert output is `0`
      3. Run `grep -c 'resp:?' crates/openlark-core/src/http.rs`
      4. Assert output is `0`
    Expected Result: 零个旧格式 debug 日志
    Failure Indicators: 输出大于 0
    Evidence: .sisyphus/evidence/task-3-no-raw-debug.txt
  Scenario: 新的结构化日志存在
    Tool: Bash
    Preconditions: Task 3 changes applied
    Steps:
      1. Run `grep -n 'Sending request\|Received response' crates/openlark-core/src/http.rs`
      2. Assert output contains both patterns
      3. Run `cargo check -p openlark-core 2>&1`
      4. Assert exit code is 0
    Expected Result: 结构化日志存在且编译通过
    Failure Indicators: 缺少日志或编译失败
    Evidence: .sisyphus/evidence/task-3-structured-log.txt
  ```
  **Commit**: YES
  - Message: `fix(core): HTTP debug 日志脱敏，避免泄露 headers/token`
  - Files: `crates/openlark-core/src/http.rs`
  - Pre-commit: `cargo check -p openlark-core`

- [x] 4. Validation 日志级别降级（error! → warn!）
  **What to do**:
  - 修改 `validation/core.rs:49` 的 `error!` 为 `warn!`（validate_string_length 中的截断日志）
  - 修改 `validation/core.rs:73` 的 `error!` 为 `warn!`（validate_required_list_length 空列表日志）
  - 修改 `validation/core.rs:76` 的 `error!` 为 `warn!`（validate_required_list_length 超长日志）
  - 修改 `validation/core.rs:100` 的 `error!` 为 `warn!`（validate_content_size 超限日志）
  - 修改 `use tracing::error;`（line 3）为 `use tracing::warn;`
  **Must NOT do**:
  - 不改变函数签名、返回类型或逻辑
  - 不改变日志消息内容（只改级别）
  - 不动测试代码
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: `[]`
  **Parallelization**:
  - **Can Run In Parallel**: YES (Wave 1)
  - **Parallel Group**: Wave 1 (with Tasks 1, 3, 5)
  - **Blocks**: Task 6
  - **Blocked By**: None
  **References**:
  **Pattern References**:
  - `crates/openlark-core/src/validation/core.rs:3` — `use tracing::error;` import 需改为 `warn`
  - `crates/openlark-core/src/validation/core.rs:47-60` — `validate_string_length` 函数，line 49 的 `error!`
  - `crates/openlark-core/src/validation/core.rs:71-86` — `validate_required_list_length` 函数，lines 73, 76 的 `error!`
  - `crates/openlark-core/src/validation/core.rs:97-108` — `validate_content_size` 函数，line 100 的 `error!`
  **Acceptance Criteria**:
  - [x] `grep -n 'error!' crates/openlark-core/src/validation/core.rs | grep -v '//' | grep -v 'test'` 返回 0 行
  - [x] `grep -c 'warn!' crates/openlark-core/src/validation/core.rs` 返回 4（替换后的 warn 调用数）
  - [x] `cargo check -p openlark-core` 编译成功
  **QA Scenarios:**
  ```
  Scenario: error! 已全部替换为 warn!
    Tool: Bash
    Preconditions: Task 4 changes applied
    Steps:
      1. Run `grep -n 'error!' crates/openlark-core/src/validation/core.rs | grep -v '//' | grep -v test | grep -v cfg`
      2. Assert output is empty (0 lines)
      3. Run `grep -c 'warn!' crates/openlark-core/src/validation/core.rs`
      4. Assert output is `4`
    Expected Result: 4 个 warn! 调用，0 个非测试 error! 调用
    Failure Indicators: 仍有 error! 或 warn! 数量不对
    Evidence: .sisyphus/evidence/task-4-log-level.txt
  Scenario: 编译通过
    Tool: Bash
    Preconditions: Task 4 changes applied
    Steps:
      1. Run `cargo check -p openlark-core 2>&1`
      2. Assert exit code is 0
    Expected Result: 编译成功
    Failure Indicators: 编译错误
    Evidence: .sisyphus/evidence/task-4-compile.txt
  ```
  **Commit**: YES
  - Message: `fix(core): validation 日志从 error 降为 warn`
  - Files: `crates/openlark-core/src/validation/core.rs`
  - Pre-commit: `cargo check -p openlark-core`

- [x] 5. AGENTS.md 更新（与实际代码结构同步）
  **What to do**:
  - 读取 `crates/openlark-core/src/` 的实际目录结构
  - 更新 `crates/openlark-core/AGENTS.md` 中的 STRUCTURE 部分，使其与实际文件一致
  - 重点修正：
    - `api_endpoints.rs` 是否仍存在（AGENTS.md 中列出但可能已移除/重命名）
    - `api.rs` vs `api/` 目录结构
    - `auth/` 子目录内容（token.rs, sign.rs vs 实际文件）
    - `error/` 子目录内容
    - 内部模块列表（底部的 pub(crate) 模块）
  - 保持中文注释风格
  **Must NOT do**:
  - 不改变 AGENTS.md 的整体格式和章节结构
  - 不修改 CONVENTIONS、ANTI-PATTERNS、COMMANDS 等非结构相关章节（除非它们引用了不存在的文件）
  - 不修改 `crates/openlark-core/` 以外的 AGENTS.md
  **Recommended Agent Profile**:
  - **Category**: `writing`
  - **Skills**: `[]`
  **Parallelization**:
  - **Can Run In Parallel**: YES (Wave 1)
  - **Parallel Group**: Wave 1 (with Tasks 1, 3, 4)
  - **Blocks**: Task 6
  - **Blocked By**: None
  **References**:
  **Pattern References**:
  - `crates/openlark-core/AGENTS.md:14-30` — 当前 STRUCTURE 部分（需要与实际对比更新）
  - `crates/openlark-core/src/` — 实际目录结构（用 `ls -R` 或 glob 获取）
  - `crates/openlark-core/src/lib.rs` — 模块声明列表（pub/pub(crate)/private）
  **Acceptance Criteria**:
  - [x] AGENTS.md 的 STRUCTURE 部分与 `src/` 实际目录结构一致
  - [x] 无引用不存在的文件路径
  **QA Scenarios:**
  ```
  Scenario: STRUCTURE 与实际目录一致
    Tool: Bash
    Preconditions: Task 5 changes applied
    Steps:
      1. Run `git ls-files crates/openlark-core/src/ -- '*.rs' | sort`
      2. Read AGENTS.md STRUCTURE section
      3. Compare: every file in ls output should appear in STRUCTURE; no phantom files in STRUCTURE
    Expected Result: 1:1 match between STRUCTURE and actual files
    Failure Indicators: STRUCTURE lists files that don't exist, or misses files that do
    Evidence: .sisyphus/evidence/task-5-structure-diff.txt
  ```
  **Commit**: YES
  - Message: `docs(core): 更新 AGENTS.md 与实际代码结构同步`
  - Files: `crates/openlark-core/AGENTS.md`
  - Pre-commit: none
- [x] 6. 全量验证（所有改动集成后）
  **What to do**:
  - 运行完整验证命令序列，确认所有改动协同工作：
    1. `cargo check -p openlark-core --no-default-features`
    2. `cargo check -p openlark-core`
    3. `cargo test -p openlark-core`
    4. `cargo test --workspace`
    5. `cargo clippy -p openlark-core -- -D warnings`
  - 如果任何命令失败，定位问题并报告（不自行修复，交由对应 task owner 处理）
  **Must NOT do**:
  - 不修改任何代码文件
  - 不自行修复问题（只报告）
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: `[]`
  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (sequential, after all implementation tasks)
  - **Blocks**: F1, F2, F3
  - **Blocked By**: Tasks 1, 2, 3, 4, 5
  **References**:
  **Pattern References**:
  - 本计划的 Success Criteria 部分 — 完整验证命令列表
  **Acceptance Criteria**:
  - [x] 所有 5 个验证命令均返回 exit code 0
  - [x] `cargo test --workspace` 无下游 crate 测试失败
  **QA Scenarios:**
  ```
  Scenario: 完整验证通过
    Tool: Bash
    Preconditions: Tasks 1-5 all completed
    Steps:
      1. Run `cargo check -p openlark-core --no-default-features 2>&1`
      2. Assert exit code is 0
      3. Run `cargo check -p openlark-core 2>&1`
      4. Assert exit code is 0
      5. Run `cargo test -p openlark-core 2>&1`
      6. Assert exit code is 0 and output contains `test result: ok`
      7. Run `cargo test --workspace 2>&1`
      8. Assert exit code is 0
      9. Run `cargo clippy -p openlark-core -- -D warnings 2>&1`
      10. Assert exit code is 0
    Expected Result: 全部通过
    Failure Indicators: 任何命令返回非零 exit code
    Evidence: .sisyphus/evidence/task-6-full-verification.txt
  Scenario: 无下游破坏
    Tool: Bash
    Preconditions: Tasks 1-5 all completed
    Steps:
      1. Run `cargo test --workspace 2>&1 | tail -20`
      2. Assert output contains `test result: ok` for all crates
      3. Assert no `FAILED` in output
    Expected Result: 所有 workspace crate 测试通过
    Failure Indicators: 任何 crate 测试失败
    Evidence: .sisyphus/evidence/task-6-workspace-tests.txt
  ```
  **Commit**: NO（纯验证任务）
---

## Final Verification Wave (MANDATORY — after ALL implementation tasks)

> 3 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [x] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, run command). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in .sisyphus/evidence/. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo clippy -p openlark-core -- -D warnings` + `cargo test -p openlark-core`. Review all changed files for: `as any`/`@ts-ignore` equivalents, empty catches, commented-out code, unused imports. Check AI slop: excessive comments, over-abstraction, generic names.
  Output: `Build [PASS/FAIL] | Clippy [PASS/FAIL] | Tests [N pass/N fail] | Files [N clean/N issues] | VERDICT`

- [x] F3. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (git log/diff). Verify 1:1 — everything in spec was built (no missing), nothing beyond spec was built (no creep). Check "Must NOT do" compliance. Detect cross-task contamination. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

- **Commit 1** (Task 1+2): `refactor(core): 移除 observability 宏导出并将 tracing-subscriber 改为 optional` — Cargo.toml, observability.rs
- **Commit 2** (Task 3): `fix(core): HTTP debug 日志脱敏，避免泄露 headers/token` — http.rs
- **Commit 3** (Task 4): `fix(core): validation 日志从 error 降为 warn` — validation/core.rs
- **Commit 4** (Task 5): `docs(core): 更新 AGENTS.md 与实际代码结构同步` — AGENTS.md

---

## Success Criteria

### Verification Commands
```bash
cargo check -p openlark-core --no-default-features  # Expected: success, no tracing-subscriber linked
cargo check -p openlark-core                         # Expected: success with defaults
cargo test -p openlark-core                          # Expected: all tests pass
cargo test --workspace                               # Expected: no downstream breakage
cargo clippy -p openlark-core -- -D warnings         # Expected: zero warnings
grep -c 'macro_export' crates/openlark-core/src/observability.rs  # Expected: 0
grep -c 'req:?' crates/openlark-core/src/http.rs     # Expected: 0
grep -n 'error!' crates/openlark-core/src/validation/core.rs | grep -v '//' | grep -v '#\[cfg(test)\]'  # Expected: 0 lines
```

### Final Checklist
- [x] All "Must Have" present
- [x] All "Must NOT Have" absent
- [x] All tests pass
- [x] Zero clippy warnings
