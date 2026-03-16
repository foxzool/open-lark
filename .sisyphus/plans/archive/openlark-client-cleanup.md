# openlark-client 架构收敛与死代码清理

## TL;DR

> **Quick Summary**: 消除 openlark-client 中"极简字段直挂"与"运行时容器体系"的双重架构冲突，移除 ~4000 行死代码/未使用基础设施，修复公共导出冲突和 feature 漂移。
> 
> **Deliverables**:
> - 修复公共 API 导出冲突（ClientBuilder/ServiceMetadata/ServiceStatus 同名问题）
> - 移除未使用的 registry 子系统（service_factory, feature_flags, dependency_resolver）
> - 移除 14 个未实现 trait 中的 12 个
> - 对齐 Cargo.toml feature 依赖关系
> - 下沉 error context 能力到 openlark-core
> - 合并重复的 ConfigSummary，清理 features.rs/utils.rs
> - 更新 AGENTS.md 和相关文档
> 
> **Estimated Effort**: Medium (1-2d)
> **Parallel Execution**: YES - 4 waves
> **Critical Path**: Task 1 → Task 4 → Task 7 → Task 10 → Final

---

## Context

### Original Request
对 openlark-client crate 按最佳实践进行分析，并转化为可执行的清理重构计划。用户明确允许破坏性变更。

### Interview Summary
**Key Discussions**:
- 确认以 Client 字段直挂 + meta 链式访问为唯一范式
- registry 仅保留元信息注册（bootstrap.rs），移除 factory/flags/resolver
- traits 仅保留 LarkClient + ServiceLifecycle，移除其余未实现 trait
- 允许破坏性变更，无需 deprecated 过渡期

**Research Findings**:
- Oracle 确认双体系冲突是核心问题，建议 1-2 天工作量
- 所有业务 crate 仅依赖 openlark-core，不依赖 openlark-client，爆炸半径可控
- 唯一依赖 openlark-client 的是根 crate open-lark

### Metis Review
**Identified Gaps** (addressed):
- `task` feature 在 utils.rs 中使用但 Cargo.toml 未定义 → 纳入 feature 清理任务
- `bitable` feature 空定义 → 纳入 feature 清理任务
- 集成测试文件 ~1274 行需同步删除 → 纳入对应删除任务
- ARCHITECTURE.md 引用 registry 概念需更新 → 纳入文档更新任务
- 两套 ConfigSummary 字段差异已确认 → 纳入合并任务

---

## Work Objectives

### Core Objective
让 openlark-client 的"实际架构"和"代码表达"完全对齐：Client 字段直挂是唯一服务访问范式，registry 仅做元信息可观测，公共 API 面最小化且无冲突。

### Concrete Deliverables
- 无同名类型冲突的 crate root 导出
- 移除 ~4000 行死代码（registry 子系统 + 未实现 traits + 集成测试）
- Cargo.toml feature 依赖完整表达
- error context 能力下沉到 openlark-core
- 单一 ConfigSummary 定义
- 与实际代码一致的 AGENTS.md

### Definition of Done
- [x] `cargo check -p openlark-client --all-features` 零警告零错误
- [x] `cargo check -p openlark-client --no-default-features` 编译通过
- [x] `cargo check -p openlark-client --features docs` 编译通过
- [x] `cargo test -p openlark-client --all-features` 全部通过
- [x] `cargo check -p open-lark --all-features` 根 crate 编译通过
- [x] `cargo clippy -p openlark-client --all-features` 零警告

### Must Have
- 公共导出无同名类型冲突
- 移除 `#![allow(unexpected_cfgs)]`
- Feature 依赖在 Cargo.toml 层正确表达
- registry 子系统仅保留 bootstrap + 最小 ServiceRegistry
- 未实现 trait 全部移除
- error context 下沉到 openlark-core

### Must NOT Have (Guardrails)
- ❌ 不引入运行时服务容器/动态服务发现
- ❌ 不修改业务 crate（openlark-docs/communication/hr 等）的内部实现
- ❌ 不改变 Client 的字段直挂访问模式
- ❌ 不添加新的公共 trait
- ❌ 不在 openlark-core 引入对 openlark-client 的依赖（避免循环）
- ❌ 不修改 WebSocket 模块（ws_client/）

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: YES（cargo test）
- **Automated tests**: Tests-after（清理后补充必要测试）
- **Framework**: cargo test（Rust 内置）

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Library/Module**: Use Bash (cargo check/test/clippy) — 编译、测试、lint 验证
- **API/Backend**: Use Bash — feature 组合编译验证

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — 基础修复，无依赖):
├── Task 1: 修复公共导出冲突 + 移除 #![allow(unexpected_cfgs)] [deep]
├── Task 2: 下沉 error context 能力到 openlark-core [deep]
├── Task 3: 对齐 Cargo.toml feature 依赖 + 清理幽灵 feature [quick]

Wave 2 (After Wave 1 — 大规模删除):
├── Task 4: 移除 registry 子系统死代码（factory/flags/resolver） [unspecified-high]
├── Task 5: 移除未实现 traits + 支撑类型 [unspecified-high]
├── Task 6: 清理 features.rs + 合并 ConfigSummary + 修复 utils.rs [quick]

Wave 3 (After Wave 2 — 集成修复):
├── Task 7: 修复 lib.rs 导出 + prelude + 删除失效测试文件 [deep]
├── Task 8: 更新 AGENTS.md + README.md + docs/ [writing]

Wave 4 (After Wave 3 — 验证):
├── Task 9: 全 feature 组合编译回归 [unspecified-high]
├── Task 10: 运行完整测试套件 + clippy [unspecified-high]

Wave FINAL (After ALL tasks — independent review, 4 parallel):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Real manual QA (unspecified-high)
├── Task F4: Scope fidelity check (deep)

Critical Path: Task 1 → Task 4 → Task 7 → Task 9 → F1-F4
Parallel Speedup: ~60% faster than sequential
Max Concurrent: 3 (Waves 1 & 2)
```

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|-----------|--------|------|
| 1 | — | 4, 5, 7 | 1 |
| 2 | — | 7 | 1 |
| 3 | — | 4, 7 | 1 |
| 4 | 1, 3 | 7 | 2 |
| 5 | 1 | 7 | 2 |
| 6 | — | 7 | 2 |
| 7 | 4, 5, 6, 2 | 9, 10 | 3 |
| 8 | 4, 5 | — | 3 |
| 9 | 7 | F1-F4 | 4 |
| 10 | 7 | F1-F4 | 4 |

### Agent Dispatch Summary

- **Wave 1**: 3 tasks — T1 → `deep`, T2 → `deep`, T3 → `quick`
- **Wave 2**: 3 tasks — T4 → `unspecified-high`, T5 → `unspecified-high`, T6 → `quick`
- **Wave 3**: 2 tasks — T7 → `deep`, T8 → `writing`
- **Wave 4**: 2 tasks — T9 → `unspecified-high`, T10 → `unspecified-high`
- **FINAL**: 4 tasks — F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

> Implementation + Test = ONE Task. Never separate.
> EVERY task MUST have: Recommended Agent Profile + Parallelization info + QA Scenarios.

- [x] 1. 修复公共导出冲突 + 移除 `#![allow(unexpected_cfgs)]`

  **What to do**:
  - 移除 `lib.rs:8` 的 `#![allow(unexpected_cfgs)]`
  - 将 `lib.rs` 中的 `pub use traits::*;` 改为显式导出：仅导出 `LarkClient` 和 `ServiceLifecycle`
  - 将 `traits/client.rs` 中的 `trait ClientBuilder` 重命名为 `trait ClientBuilderTrait`（或直接删除，见 Task 5）
  - 确认 `traits/service.rs` 中的 `ServiceMetadata`（struct）和 `ServiceStatus`（enum）与 `registry/mod.rs` 中的同名类型不会同时导出到 crate root
  - 如果 traits 中的 `ServiceMetadata`/`ServiceStatus` 在 Task 5 中被删除，此处只需确保 `pub use traits::*` 被替换
  - 移除 `#![allow(unexpected_cfgs)]` 后运行 `cargo check` 确认无 unexpected_cfgs 警告

  **Must NOT do**:
  - 不修改 Client 结构体的字段或构造逻辑
  - 不改变 prelude 中已稳定使用的类型（Client, Config, Error, Result）

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: 需要理解 crate 导出层的全局影响，涉及多文件交叉引用
  - **Skills**: [`openlark-code-standards`]
    - `openlark-code-standards`: 确保导出规范符合项目约定

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3)
  - **Blocks**: Tasks 4, 5, 7
  - **Blocked By**: None (can start immediately)

  **References**:

  **Pattern References**:
  - `crates/openlark-client/src/lib.rs:8` — `#![allow(unexpected_cfgs)]` 需移除
  - `crates/openlark-client/src/lib.rs:220` — `pub use traits::*;` 需改为显式导出
  - `crates/openlark-client/src/lib.rs:242-276` — 当前 crate root 的 re-export 区域
  - `crates/openlark-client/src/traits/mod.rs:9-10` — `pub use client::*; pub use service::*;` 全量导出
  - `crates/openlark-client/src/traits/client.rs:117` — `trait ClientBuilder` 与 `struct ClientBuilder` 冲突
  - `crates/openlark-client/src/traits/service.rs:158-175` — `struct ServiceMetadata` 与 registry 同名
  - `crates/openlark-client/src/traits/service.rs:272-286` — `enum ServiceStatus` 与 registry 同名

  **WHY Each Reference Matters**:
  - lib.rs:8 是导致 feature 漂移被掩盖的根源
  - lib.rs:220 的 `pub use traits::*` 是同名冲突的直接原因
  - traits/ 中的同名类型需要确认是删除（Task 5）还是重命名

  **Acceptance Criteria**:
- [x] `cargo check -p openlark-client --all-features 2>&1 | grep unexpected_cfgs` 无输出
- [x] `grep -r 'allow(unexpected_cfgs)' crates/openlark-client/src/` 无匹配
- [x] `grep 'pub use traits::\*' crates/openlark-client/src/lib.rs` 无匹配
- [x] `cargo check -p openlark-client --all-features` 零错误

  **QA Scenarios:**

  ```
  Scenario: 公共导出无冲突编译
    Tool: Bash
    Preconditions: Wave 1 Task 1 实现完成
    Steps:
      1. 运行 `cargo check -p openlark-client --all-features 2>&1`
      2. 检查输出中无 "ambiguous" 或 "conflict" 相关错误
      3. 检查输出中无 unexpected_cfgs 警告
    Expected Result: 编译成功，零错误零警告
    Failure Indicators: 出现 E0252 (name clash) 或 unexpected_cfgs 警告
    Evidence: .sisyphus/evidence/task-1-export-conflict-check.txt

  Scenario: prelude 导入不破坏现有用法
    Tool: Bash
    Preconditions: Task 1 完成
    Steps:
      1. 运行 `cargo test -p openlark-client --all-features -- test_prelude_reexports 2>&1`
      2. 确认测试通过
    Expected Result: test_prelude_reexports 测试通过
    Failure Indicators: 测试失败或编译错误
    Evidence: .sisyphus/evidence/task-1-prelude-test.txt
  ```

  **Commit**: YES
  - Message: `refactor(client): 修复公共导出冲突，移除 unexpected_cfgs allow`
  - Files: `crates/openlark-client/src/lib.rs`, `crates/openlark-client/src/traits/mod.rs`
  - Pre-commit: `cargo check -p openlark-client --all-features`

- [x] 2. 下沉 error context 能力到 openlark-core

  **What to do**:
  - 在 `openlark-core` 的 `CoreError` 上添加 `pub fn map_context(self, f: impl FnOnce(&mut ErrorContext)) -> Self` 方法
  - 该方法对每个 variant 提取 `&mut ErrorContext`（或 `&mut Box<ErrorContext>`），调用闭包，返回 self
  - 同时添加 `pub fn with_context_kv(self, key: impl Into<String>, value: impl Into<String>) -> Self` 便利方法
  - 同时添加 `pub fn with_operation(self, operation: impl Into<String>, component: impl Into<String>) -> Self` 便利方法
  - 重写 `crates/openlark-client/src/error.rs` 中的 `with_context` 函数：从 ~150 行 match 简化为调用 `err.with_context_kv(key, value)`
  - 重写 `with_operation_context` 函数：从 ~150 行 match 简化为调用 `err.with_operation(op, comp)`
  - 确保 `error.rs` 中的 `_ => err` 兜底分支不再需要（所有 variant 都被 map_context 覆盖）

  **Must NOT do**:
  - 不在 openlark-core 引入对 openlark-client 的依赖
  - 不改变 CoreError 的 variant 定义或公共 API 语义
  - 不修改 error.rs 中的错误创建便利函数（network_error, api_error 等）

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: 跨 crate 修改，需要理解 CoreError 的完整 variant 结构
  - **Skills**: [`openlark-code-standards`]
    - `openlark-code-standards`: 确保 error 处理符合项目规范

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3)
  - **Blocks**: Task 7
  - **Blocked By**: None (can start immediately)

  **References**:

  **Pattern References**:
  - `crates/openlark-client/src/error.rs:308-437` — `with_context` 函数，~130 行 match 臂需简化
  - `crates/openlark-client/src/error.rs:440-598` — `with_operation_context` 函数，~160 行 match 臂需简化

  **API/Type References**:
  - `crates/openlark-core/src/error/` — CoreError 定义所在目录，需在此添加 map_context 方法
  - `crates/openlark-core/src/error/mod.rs` 或对应文件 — ErrorContext 结构体定义

  **WHY Each Reference Matters**:
  - error.rs 的两个函数是本次简化的目标，需要理解每个 variant 的 ctx 字段位置
  - openlark-core 的 CoreError 是添加 map_context 的目标位置

  **Acceptance Criteria**:
  - [x] `crates/openlark-client/src/error.rs` 中 `with_context` 函数体 < 20 行
  - [x] `crates/openlark-client/src/error.rs` 中 `with_operation_context` 函数体 < 20 行
  - [x] `cargo test -p openlark-core` 全部通过
  - [x] `cargo test -p openlark-client --all-features -- test_context_functions test_with_operation_context` 通过

  **QA Scenarios:**

  ```
  Scenario: error context 注入功能正常
    Tool: Bash
    Preconditions: map_context 已添加到 CoreError
    Steps:
      1. 运行 `cargo test -p openlark-client --all-features -- test_context_functions 2>&1`
      2. 运行 `cargo test -p openlark-client --all-features -- test_with_operation_context 2>&1`
      3. 确认两个测试均通过
    Expected Result: 2 tests passed
    Failure Indicators: 测试失败或 context 值不匹配
    Evidence: .sisyphus/evidence/task-2-error-context.txt

  Scenario: openlark-core 无回归
    Tool: Bash
    Preconditions: map_context 已添加
    Steps:
      1. 运行 `cargo test -p openlark-core 2>&1`
      2. 确认全部通过
    Expected Result: all tests pass
    Failure Indicators: 任何测试失败
    Evidence: .sisyphus/evidence/task-2-core-regression.txt
  ```

  **Commit**: YES
  - Message: `refactor(core): 为 CoreError 添加 map_context 统一上下文注入能力`
  - Files: `crates/openlark-core/src/error/*.rs`, `crates/openlark-client/src/error.rs`
  - Pre-commit: `cargo test -p openlark-core && cargo test -p openlark-client --all-features`

- [x] 3. 对齐 Cargo.toml feature 依赖 + 清理幽灵 feature

  **What to do**:
  - 修改 `crates/openlark-client/Cargo.toml`：让 `docs`、`communication`、`meeting`、`security`、`cardkit` feature 都隐式依赖 `auth`
    - 例如 `docs = ["auth", "openlark-docs"]`，`communication = ["auth", "openlark-communication"]`
  - 移除 `bitable` feature（它只是 `docs` 的别名，无独立 `#[cfg]` 使用）
  - 移除 `utils.rs` 中的 `#[cfg(feature = "task")]` 分支（Cargo.toml 中无 `task` feature）
  - 检查并移除其他幽灵 feature 引用（`calendar`、`admin`、`approval`、`helpdesk` 在 utils.rs 的 `validate_feature_dependencies` 中被引用但 Cargo.toml 无定义）
  - 更新 `default` feature：确认 `default = ["auth", "communication"]` 是否仍合理

  **Must NOT do**:
  - 不添加新的业务模块 feature
  - 不修改业务 crate 的 Cargo.toml

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 仅修改 Cargo.toml 和 utils.rs 中的少量行
  - **Skills**: [`openlark-code-standards`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2)
  - **Blocks**: Tasks 4, 7
  - **Blocked By**: None (can start immediately)

  **References**:
  **Pattern References**:
  - `crates/openlark-client/Cargo.toml:44-87` — feature 定义区域
  - `crates/openlark-client/src/utils.rs:189-213` — `get_enabled_features()` 硬编码 auth + 幽灵 feature
  - `crates/openlark-client/src/utils.rs:218-269` — `validate_feature_dependencies()` 引用不存在的 feature
  - `crates/openlark-client/src/registry/bootstrap.rs:11-36` — 元信息中声明的依赖关系（应与 Cargo.toml 对齐）

  **WHY Each Reference Matters**:
  - Cargo.toml 是 feature 依赖的唯一权威来源
  - utils.rs 中的幽灵 feature 会在移除 `#![allow(unexpected_cfgs)]` 后导致编译警告
  - bootstrap.rs 的元信息依赖应与 Cargo.toml 一致

  **Acceptance Criteria**:
  - [x] `cargo check -p openlark-client --features docs` 隐式启用 auth，编译通过
  - [x] `cargo check -p openlark-client --no-default-features --features docs` 编译通过
  - [x] `grep 'bitable' crates/openlark-client/Cargo.toml` 无匹配
  - [x] `grep 'feature = "task"' crates/openlark-client/src/` 无匹配

  **QA Scenarios:**
  ```
  Scenario: 单 feature 编译验证
    Tool: Bash
    Preconditions: Cargo.toml feature 依赖已修改
    Steps:
      1. 运行 `cargo check -p openlark-client --no-default-features --features docs 2>&1`
      2. 运行 `cargo check -p openlark-client --no-default-features --features communication 2>&1`
      3. 运行 `cargo check -p openlark-client --no-default-features --features meeting 2>&1`
      4. 确认全部编译通过
    Expected Result: 3 次编译均成功
    Failure Indicators: 缺少 auth 相关类型的编译错误
    Evidence: .sisyphus/evidence/task-3-feature-combos.txt
  ```

  **Commit**: YES
  - Message: `fix(client): 对齐 Cargo.toml feature 依赖，移除幽灵 feature`
  - Files: `crates/openlark-client/Cargo.toml`, `crates/openlark-client/src/utils.rs`
  - Pre-commit: `cargo check -p openlark-client --all-features`

- [x] 4. 移除 registry 子系统死代码（factory/flags/resolver）

  **What to do**:
  - 删除 `crates/openlark-client/src/registry/service_factory.rs`（796 行）
  - 删除 `crates/openlark-client/src/registry/feature_flags.rs`（551 行）
  - 删除 `crates/openlark-client/src/registry/dependency_resolver.rs`（474 行）
  - 简化 `crates/openlark-client/src/registry/mod.rs`：
    - 移除 `pub mod dependency_resolver;`、`pub mod feature_flags;`、`pub mod service_factory;`
    - 移除 `pub use dependency_resolver::*;`、`pub use feature_flags::*;`、`pub use service_factory::*;`
    - 移除 `DefaultServiceRegistry` 中的 `feature_flags: Arc<RwLock<FeatureFlagManager>>` 和 `dependency_resolver: Arc<DependencyResolver>` 字段
    - 移除 `initialize_services()`、`start_services()` 方法（它们依赖 resolver）
    - 保留：`ServiceMetadata`、`ServiceStatus`、`ServiceEntry`、`ServiceRegistry` trait、`DefaultServiceRegistry`（简化版）、`RegistryError`（简化版）
  - 删除 `crates/openlark-client/tests/registry_tests.rs`（782 行）
  - 检查并删除 `tests/unit/client/registry_tests.rs`（492 行，如果存在）
  - 更新 `crates/openlark-client/src/error.rs`：移除 `From<FeatureFlagError>` 和 `From<DependencyError>` impl
  - 更新 `crates/openlark-client/src/lib.rs`：移除对已删除类型的 re-export

  **Must NOT do**:
  - 不删除 `registry/bootstrap.rs`（元信息注册仍需保留）
  - 不删除 `registry/mod.rs`（简化但保留）
  - 不修改 Client 结构体的 registry 字段（仍保留 `Arc<DefaultServiceRegistry>`）

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 大规模文件删除 + 多文件级联修改，需要仔细处理引用
  - **Skills**: [`openlark-code-standards`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 6)
  - **Blocks**: Task 7
  - **Blocked By**: Tasks 1, 3

  **References**:
  **Pattern References**:
  - `crates/openlark-client/src/registry/service_factory.rs` — 整文件删除（796 行）
  - `crates/openlark-client/src/registry/feature_flags.rs` — 整文件删除（551 行）
  - `crates/openlark-client/src/registry/dependency_resolver.rs` — 整文件删除（474 行）
  - `crates/openlark-client/src/registry/mod.rs:7-19` — 模块声明和 re-export 需清理
  - `crates/openlark-client/src/registry/mod.rs:163-172` — `DefaultServiceRegistry` 字段需简化
  - `crates/openlark-client/src/registry/mod.rs:210-246` — `initialize_services`/`start_services` 需移除
  - `crates/openlark-client/src/error.rs:274-284` — `From<FeatureFlagError>` 和 `From<DependencyError>` 需移除
  - `crates/openlark-client/src/lib.rs:274-276` — re-export 需更新
  - `crates/openlark-client/tests/registry_tests.rs` — 整文件删除

  **WHY Each Reference Matters**:
  - 三个被删除的文件是本次最大的代码削减
  - mod.rs 是级联修改的核心，需要同时简化结构体和移除方法
  - error.rs 的 From impl 引用了被删除的类型

  **Acceptance Criteria**:
  - [x] `ls crates/openlark-client/src/registry/` 仅包含 `mod.rs` 和 `bootstrap.rs`
  - [x] `ls crates/openlark-client/tests/` 不包含 `registry_tests.rs`
  - [x] `cargo check -p openlark-client --all-features` 编译通过
  - [x] `grep -r 'FeatureFlagManager\|DependencyResolver\|ServiceFactory\|PlaceholderService' crates/openlark-client/src/` 无匹配

  **QA Scenarios:**
  ```
  Scenario: registry 子系统精简后编译通过
    Tool: Bash
    Preconditions: 三个文件已删除，mod.rs 已简化
    Steps:
      1. 运行 `cargo check -p openlark-client --all-features 2>&1`
      2. 确认无编译错误
      3. 运行 `ls crates/openlark-client/src/registry/`
      4. 确认仅包含 mod.rs 和 bootstrap.rs
    Expected Result: 编译成功，registry 目录仅 2 个文件
    Failure Indicators: 编译错误（未解析的引用）
    Evidence: .sisyphus/evidence/task-4-registry-cleanup.txt
  Scenario: 已删除类型无残留引用
    Tool: Bash
    Preconditions: 删除完成
    Steps:
      1. 运行 `grep -r 'FeatureFlagManager\|DependencyResolver\|ServiceFactory\|PlaceholderService' crates/openlark-client/src/ 2>&1`
      2. 确认无匹配
    Expected Result: 无输出
    Failure Indicators: 有残留引用
    Evidence: .sisyphus/evidence/task-4-no-residual-refs.txt
  ```

  **Commit**: YES
  - Message: `refactor(client): 移除未使用的 registry 子系统（factory/flags/resolver）`
  - Files: 删除 3 文件 + 修改 registry/mod.rs, error.rs, lib.rs, 删除 tests/registry_tests.rs
  - Pre-commit: `cargo check -p openlark-client --all-features`

- [x] 5. 移除未实现 traits + 支撑类型

  **What to do**:
  - 删除 `traits/client.rs` 中以下未实现 trait 及其支撑类型：
    - `trait ClientBuilder`（与 struct ClientBuilder 冲突）
    - `trait AuthenticatedClient`
    - `trait RequestClient`
    - `trait MonitorableClient` + `struct ClientStats` + `struct ClientHealth`
    - `trait ConfigurableClient`
    - `trait RestartableClient`
    - `trait InfoClient` + `struct ClientMetadata`
  - 保留 `traits/client.rs` 中的 `trait LarkClient`（唯一被 Client 实现的 trait）
  - 删除 `traits/service.rs` 中以下未实现 trait 及其支撑类型：
    - `trait RestartableService`
    - `trait MonitorableService`
    - `trait ConfigurableService`
    - `trait LoggableService` + `enum LogLevel`
    - `trait DiscoverableService` + `struct ServiceEndpoint`
    - `struct ServiceMetadata`（与 registry 同名，且仅在测试中使用）
    - `enum ServiceStatus`（与 registry 同名）
    - `struct ServiceStats`
  - 保留 `traits/service.rs` 中的 `trait ServiceTrait` 和 `trait ServiceLifecycle`
  - 更新 `traits/mod.rs`：从 `pub use client::*; pub use service::*;` 改为显式导出保留的 trait
  - 更新 `lib.rs` 中的 `pub use traits::*;` 为显式导出（与 Task 1 协调）
  - 删除 `traits/client.rs` 和 `traits/service.rs` 中对应的测试代码

  **Must NOT do**:
  - 不删除 `LarkClient`、`ServiceTrait`、`ServiceLifecycle`
  - 不添加新 trait

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 大量代码删除 + 需要确认每个 trait 的引用情况
  - **Skills**: [`openlark-code-standards`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4, 6)
  - **Blocks**: Task 7
  - **Blocked By**: Task 1

  **References**:

  **Pattern References**:
  - `crates/openlark-client/src/traits/client.rs:63-141` — 未实现 trait 定义区域
  - `crates/openlark-client/src/traits/client.rs:156-301` — 支撑类型（ClientStats, ClientHealth, ClientMetadata）
  - `crates/openlark-client/src/traits/service.rs:60-157` — 未实现 trait 定义区域
  - `crates/openlark-client/src/traits/service.rs:158-360` — 支撑类型（ServiceMetadata, ServiceStatus, ServiceEndpoint 等）
  - `crates/openlark-client/src/traits/mod.rs:9-10` — 全量 re-export 需改为显式
  - `crates/openlark-client/src/client.rs:200-209` — Client 仅实现 LarkClient（确认保留）

  **WHY Each Reference Matters**:
  - traits/client.rs 和 traits/service.rs 是删除的主要目标
  - mod.rs 的 re-export 决定了哪些类型暴露到 crate root
  - client.rs 的 impl 确认了 LarkClient 是唯一需要保留的 trait

  **Acceptance Criteria**:
  - [x] `grep -c 'trait ' crates/openlark-client/src/traits/client.rs` 输出 1（仅 LarkClient）
  - [x] `grep -c 'trait ' crates/openlark-client/src/traits/service.rs` 输出 2（ServiceTrait + ServiceLifecycle）
  - [x] `cargo check -p openlark-client --all-features` 编译通过

  **QA Scenarios:**
  ```
  Scenario: 保留的 trait 仍可正常使用
    Tool: Bash
    Preconditions: 未实现 trait 已删除
    Steps:
      1. 运行 `cargo check -p openlark-client --all-features 2>&1`
      2. 运行 `cargo test -p openlark-client --all-features -- test_lark_client 2>&1`
      3. 确认编译和测试通过
    Expected Result: 编译成功，LarkClient 相关测试通过
    Failure Indicators: 未解析的引用或测试失败
    Evidence: .sisyphus/evidence/task-5-traits-cleanup.txt
  ```

  **Commit**: YES
  - Message: `refactor(client): 移除未实现的 traits 和支撑类型`
  - Files: `crates/openlark-client/src/traits/client.rs`, `traits/service.rs`, `traits/mod.rs`
  - Pre-commit: `cargo check -p openlark-client --all-features`

- [x] 6. 清理 features.rs + 合并 ConfigSummary + 修复 utils.rs

  **What to do**:
  - **features.rs 清理**：
    - 删除 `FeatureSet` 和 `FeatureStats`（未被使用）
    - 保留 `FeatureLoader`（仍被 lib.rs 引用，但简化为仅包含 `load_services` 方法）
    - 或者：如果 `FeatureLoader` 也无实际消费者，直接删除整个 features.rs，将 `load_services` 逻辑内联到 client.rs
  - **ConfigSummary 合并**：
    - 删除 `utils.rs` 中的 `ConfigSummary`（`utils.rs:155-166`）
    - 保留 `config.rs` 中的 `ConfigSummary`（`config.rs:399-420`，字段更完整）
    - 更新 `utils::get_config_summary()` 函数：改为返回 `config::ConfigSummary`（调用 `config.summary()`）
    - 删除 `utils::ConfigSummary` 的 `friendly_description()` 方法（或迁移到 config.rs 版本）
  - **utils.rs 修复**：
    - `get_enabled_features()`：将硬编码的 `vec!["auth"]` 改为 `#[cfg(feature = "auth")] features.push("auth");`
    - 移除 `validate_feature_dependencies()` 中引用不存在 feature 的分支（calendar, admin, approval, task）
    - 修复 `business_error(_code, message)` 中被忽略的 `_code` 参数（或删除该参数）
    - 修复 `internal_error()` 使用 `api_error(500, ...)` 的问题：改用 `CoreError::Internal` 构造

  **Must NOT do**:
  - 不修改 Config 结构体本身
  - 不改变 `check_env_config()` 和 `create_config_from_env()` 的行为

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 多处小修改，逻辑简单
  - **Skills**: [`openlark-code-standards`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4, 5)
  - **Blocks**: Task 7
  - **Blocked By**: None

  **References**:

  **Pattern References**:
  - `crates/openlark-client/src/features.rs:29-81` — FeatureSet/FeatureStats 定义（删除目标）
  - `crates/openlark-client/src/config.rs:399-420` — 权威 ConfigSummary（保留）
  - `crates/openlark-client/src/utils.rs:154-183` — 重复 ConfigSummary（删除目标）
  - `crates/openlark-client/src/utils.rs:188-213` — get_enabled_features() 硬编码问题
  - `crates/openlark-client/src/utils.rs:218-269` — validate_feature_dependencies() 幽灵 feature
  - `crates/openlark-client/src/error.rs:120-122` — business_error 忽略 _code
  - `crates/openlark-client/src/error.rs:146-149` — internal_error 使用 api_error

  **WHY Each Reference Matters**:
  - 两套 ConfigSummary 会导致用户混淆
  - get_enabled_features 硬编码会在 --no-default-features 下误报
  - error 便利函数的语义不一致影响错误分类

  **Acceptance Criteria**:
  - [x] `grep -c 'struct ConfigSummary' crates/openlark-client/src/` 输出 1（仅 config.rs 中）
  - [x] `grep 'FeatureSet\|FeatureStats' crates/openlark-client/src/lib.rs` 无匹配（或已移除导出）
  - [x] `cargo test -p openlark-client --all-features` 全部通过

  **QA Scenarios:**
  ```
  Scenario: ConfigSummary 统一后功能正常
    Tool: Bash
    Preconditions: ConfigSummary 已合并
    Steps:
      1. 运行 `cargo test -p openlark-client --all-features -- test_config_summary test_get_config_summary 2>&1`
      2. 确认测试通过
    Expected Result: 配置摘要相关测试通过
    Failure Indicators: 类型不匹配或字段缺失
    Evidence: .sisyphus/evidence/task-6-config-summary.txt
  Scenario: get_enabled_features 不再硬编码
    Tool: Bash
    Preconditions: utils.rs 已修复
    Steps:
      1. 运行 `grep 'vec!\["auth"\]' crates/openlark-client/src/utils.rs`
      2. 确认无匹配
    Expected Result: 无硬编码 auth
    Failure Indicators: 仍有硬编码
    Evidence: .sisyphus/evidence/task-6-no-hardcode.txt
  ```

  **Commit**: YES
  - Message: `refactor(client): 合并 ConfigSummary，清理 features.rs 和 utils.rs`
  - Files: `features.rs`, `config.rs`, `utils.rs`, `error.rs`, `lib.rs`
  - Pre-commit: `cargo test -p openlark-client --all-features`

- [x] 7. 修复 lib.rs 导出 + prelude + 删除失效测试文件
  **What to do**:
  - 全面审查 `lib.rs` 的 re-export 区域，移除所有引用已删除类型的 `pub use`
  - 移除 `pub use features::{FeatureSet, FeatureStats};`（如 Task 6 已删除）
  - 移除 `pub use registry::{FeatureFlagManager, FlagMetadata, FlagValue, ...}` 等已删除类型
  - 更新 `prelude` 模块：仅导出 Client, ClientBuilder, Config, Error, Result, LarkClient, ServiceTrait, ServiceLifecycle, ServiceRegistry
  - 移除 prelude 中对已删除类型的引用（FeatureLoader, FeatureSet 等）
  - 确认所有 `#[cfg(feature = "...")]` 条件导出仍然正确（AuthClient, DocsClient 等）
  - 删除 `lib.rs` 中引用已删除类型的测试用例
  - 确保 `cargo doc -p openlark-client --all-features` 能正常生成文档
  **Must NOT do**:
  - 不改变 Client/Config/Error/Result 的导出
  - 不改变 feature-gated 的业务 client 导出（DocsClient, CommunicationClient 等）
  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: 需要理解所有前序任务的删除结果，综合调整导出层
  - **Skills**: [`openlark-code-standards`]
  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (with Task 8)
  - **Blocks**: Tasks 9, 10
  - **Blocked By**: Tasks 2, 4, 5, 6
  **References**:
  **Pattern References**:
  - `crates/openlark-client/src/lib.rs:213-280` — 核心 re-export 区域
  - `crates/openlark-client/src/lib.rs:336-425` — prelude 模块
  - `crates/openlark-client/src/lib.rs:442-771` — 测试代码（部分可能引用已删除类型）
  **WHY Each Reference Matters**:
  - lib.rs 是 crate 的公共面定义，所有前序删除都需要在此反映
  - prelude 是用户最常用的导入路径，必须干净无冲突
  **Acceptance Criteria**:
  - [x] `cargo check -p openlark-client --all-features` 零错误零警告
  - [x] `cargo doc -p openlark-client --all-features --no-deps` 成功生成
  - [x] `cargo test -p openlark-client --all-features` 全部通过
  **QA Scenarios:**
  ```
  Scenario: lib.rs 导出层完整性
    Tool: Bash
    Preconditions: 所有 Wave 1-2 任务完成
    Steps:
      1. 运行 `cargo check -p openlark-client --all-features 2>&1`
      2. 运行 `cargo doc -p openlark-client --all-features --no-deps 2>&1`
      3. 确认两者均无错误
    Expected Result: 编译和文档生成均成功
    Failure Indicators: 未解析的导入或文档生成失败
    Evidence: .sisyphus/evidence/task-7-lib-exports.txt
  ```
  **Commit**: YES
  - Message: `refactor(client): 更新 lib.rs 导出和 prelude，移除失效引用`
  - Files: `crates/openlark-client/src/lib.rs`
  - Pre-commit: `cargo test -p openlark-client --all-features`
- [x] 8. 更新 AGENTS.md + README.md + docs/
  **What to do**:
  - 更新 `crates/openlark-client/AGENTS.md`：
    - 修正 STRUCTURE 部分：移除 `builder.rs`、`services/`、`compat/`、`prelude.rs` 等不存在的条目
    - 添加实际存在的文件：`core_config.rs`、`features.rs`（如保留）、`test_utils.rs`
    - 更新 WHERE TO LOOK 表格：移除 ServiceFactory/FeatureFlags 相关条目
    - 更新 CONVENTIONS 部分：确认 meta 调用链示例仍然正确
    - 更新 NOTES 部分：修正默认 Features 描述
  - 更新 `crates/openlark-client/README.md`：
    - 移除服务工厂/功能标志管理器相关描述
    - 简化架构图：移除 ServiceFactory/FeatureFlags 节点
    - 确认快速开始示例仍然可编译
  - 检查 `crates/openlark-client/docs/meta-api-style.md` 是否需要更新
  **Must NOT do**:
  - 不修改根目录的 README.md（除非直接引用了被删除的类型）
  **Recommended Agent Profile**:
  - **Category**: `writing`
    - Reason: 纯文档更新任务
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Task 7)
  - **Blocks**: None
  - **Blocked By**: Tasks 4, 5
  **References**:
  **Pattern References**:
  - `crates/openlark-client/AGENTS.md:10-26` — STRUCTURE 部分（与实际不符）
  - `crates/openlark-client/AGENTS.md:28-36` — WHERE TO LOOK 表格
  - `crates/openlark-client/AGENTS.md:72-76` — NOTES 部分
  - `crates/openlark-client/README.md` — 架构图和功能描述
  **Acceptance Criteria**:
  - [x] AGENTS.md 的 STRUCTURE 与 `ls crates/openlark-client/src/` 输出一致
  - [x] README.md 中无 ServiceFactory/FeatureFlagManager/DependencyResolver 引用
  **QA Scenarios:**
  ```
  Scenario: 文档与代码结构一致
    Tool: Bash
    Preconditions: 所有代码修改完成
    Steps:
      1. 运行 `ls crates/openlark-client/src/`
      2. 对比 AGENTS.md 中的 STRUCTURE 部分
      3. 运行 `grep -i 'ServiceFactory\|FeatureFlagManager\|DependencyResolver' crates/openlark-client/README.md`
      4. 确认无匹配
    Expected Result: 文档与实际结构一致，无已删除概念的引用
    Failure Indicators: 结构不匹配或有残留引用
    Evidence: .sisyphus/evidence/task-8-docs-consistency.txt
  ```
  **Commit**: YES
  - Message: `docs(client): 更新 AGENTS.md 和 README.md 匹配新架构`
  - Files: `AGENTS.md`, `README.md`
  - Pre-commit: none
- [x] 9. 全 feature 组合编译回归
  **What to do**:
  - 逐一验证以下 feature 组合能编译通过：
    - `--no-default-features`
    - `--no-default-features --features auth`
    - `--no-default-features --features docs`
    - `--no-default-features --features communication`
    - `--no-default-features --features meeting`
    - `--no-default-features --features cardkit`
    - `--no-default-features --features security`
    - `--no-default-features --features hr`
    - `--no-default-features --features ai`
    - `--no-default-features --features websocket`
    - `--features p0-services`
    - `--all-features`
  - 同时验证根 crate `open-lark` 的 `--all-features` 编译
  - 如有编译失败，定位并修复（通常是缺少条件编译守卫）
  **Must NOT do**:
  - 不添加新 feature
  - 不修改业务 crate 代码
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 需要系统性验证多种组合，可能需要修复编译问题
  - **Skills**: [`openlark-code-standards`]
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Task 10)
  - **Blocks**: F1-F4
  - **Blocked By**: Task 7
  **References**:
  **Pattern References**:
  - `crates/openlark-client/Cargo.toml:44-87` — 所有 feature 定义
  - `crates/openlark-client/src/client.rs:69-88` — feature-gated 字段
  - `crates/openlark-client/src/client.rs:155-187` — feature-gated 初始化
  **WHY Each Reference Matters**:
  - Cargo.toml 定义了所有可能的 feature 组合
  - client.rs 的条件编译是最容易出问题的地方
  **Acceptance Criteria**:
  - [x] 上述 12 种 feature 组合全部 `cargo check` 通过
  - [x] `cargo check -p open-lark --all-features` 通过
  **QA Scenarios:**
  ```
  Scenario: 全 feature 组合编译
    Tool: Bash
    Preconditions: 所有代码修改完成
    Steps:
      1. 依次运行 12 种 feature 组合的 cargo check
      2. 运行 `cargo check -p open-lark --all-features`
      3. 记录每种组合的结果
    Expected Result: 13 次编译全部成功
    Failure Indicators: 任何一种组合编译失败
    Evidence: .sisyphus/evidence/task-9-feature-matrix.txt
  ```
  **Commit**: YES (if fixes needed)
  - Message: `fix(client): 修复 feature 组合编译问题`
  - Files: 视修复内容而定
  - Pre-commit: `cargo check -p openlark-client --all-features`

- [x] 10. 完整测试套件 + Clippy 零警告验证

  **What to do**:
  - 运行 `cargo test -p openlark-client --all-features` 确认所有现有测试通过
  - 运行 `cargo clippy -p openlark-client --all-features -- -D warnings` 确认零警告
  - 运行 `cargo doc -p openlark-client --all-features --no-deps` 确认文档生成无错误
  - 如果有测试失败，修复测试代码（不修复业务逻辑）
  - 如果有 clippy 警告，修复警告
  - 确认 `cargo test -p open-lark --all-features` 根 crate 测试也通过

  **Must NOT do**:
  - 不修改业务 crate 的测试
  - 不添加新测试（本任务只验证现有测试通过）
  - 不修改 ws_client/ 相关代码
  **Recommended Agent Profile**:
  > 测试验证和 lint 修复，需要 Rust 编译能力
  - **Category**: `unspecified-high`
    - Reason: 需要运行多个编译/测试命令并根据结果修复问题
  - **Skills**: []
    - 无需额外 skill，标准 Rust 工具链即可
  - **Skills Evaluated but Omitted**:
    - `openlark-code-standards`: 本任务是验证而非新代码编写

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 9)
  - **Parallel Group**: Wave 4 (with Task 9)
  - **Blocks**: F1, F2, F3, F4 (Final Verification Wave)
  - **Blocked By**: Tasks 7, 8 (Wave 3 完成后)
  **References**:
  **Pattern References**:
  - `crates/openlark-client/tests/registry_tests.rs` — 此文件应在 Task 4 中已删除，验证不存在
  - `crates/openlark-client/src/test_utils.rs` — 测试工具模块，确认仍可编译
  **API/Type References**:
  - `crates/openlark-client/src/lib.rs` — 公开导出，确认 `cargo doc` 能正确生成
  **External References**:
  - `cargo clippy` 文档: https://doc.rust-lang.org/clippy/
  **WHY Each Reference Matters**:
  - `registry_tests.rs` 在 Task 4 中被删除，需确认不会导致 `cargo test` 报错
  - `test_utils.rs` 可能被其他测试引用，需确认删除 registry 后仍可编译
  - `lib.rs` 的导出变更（Task 7）可能影响文档生成
  **Acceptance Criteria**:
  **QA Scenarios (MANDATORY):**
  ```
  Scenario: 全量测试通过
    Tool: Bash
    Preconditions: Wave 3 所有任务完成
    Steps:
      1. 运行 `cargo test -p openlark-client --all-features 2>&1`
      2. 检查输出中 `test result: ok` 且 failures = 0
      3. 运行 `cargo test -p open-lark --all-features 2>&1`
      4. 检查输出中 `test result: ok` 且 failures = 0
    Expected Result: 两次测试全部通过，0 failures
    Failure Indicators: 输出包含 `FAILED` 或 `failures: [1-9]`
    Evidence: .sisyphus/evidence/task-10-test-suite.txt
  ```
  Scenario: Clippy 零警告
    Tool: Bash
    Preconditions: 同上
    Steps:
      1. 运行 `cargo clippy -p openlark-client --all-features -- -D warnings 2>&1`
      2. 检查退出码为 0
      3. 检查输出不包含 `warning:` 或 `error:`
    Expected Result: 退出码 0，无警告无错误
    Failure Indicators: 退出码非 0，或输出包含 `warning:` / `error:`
    Evidence: .sisyphus/evidence/task-10-clippy.txt
  ```
  Scenario: 文档生成无错误
    Tool: Bash
    Preconditions: 同上
    Steps:
      1. 运行 `cargo doc -p openlark-client --all-features --no-deps 2>&1`
      2. 检查退出码为 0
      3. 检查输出不包含 `error`
    Expected Result: 文档生成成功
    Failure Indicators: 退出码非 0 或包含 error
    Evidence: .sisyphus/evidence/task-10-doc.txt
  ```
  **Evidence to Capture:**
  - [x] task-10-test-suite.txt — 完整测试输出
  - [x] task-10-clippy.txt — clippy 输出
  - [x] task-10-doc.txt — cargo doc 输出
  **Commit**: YES (if fixes needed)
  - Message: `fix(client): 修复测试和 clippy 警告`
  - Files: 视修复内容而定
  - Pre-commit: `cargo test -p openlark-client --all-features && cargo clippy -p openlark-client --all-features -- -D warnings`
---
## Final Verification Wave (MANDATORY — after ALL implementation tasks)

> 4 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [x] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, run command). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in .sisyphus/evidence/. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo clippy -p openlark-client --all-features`. Review all changed files for: `as any`/`@ts-ignore`, empty catches, commented-out code, unused imports. Check AI slop: excessive comments, over-abstraction, generic names.
  Output: `Build [PASS/FAIL] | Clippy [PASS/FAIL] | Tests [N pass/N fail] | Files [N clean/N issues] | VERDICT`

- [x] F3. **Real Manual QA** — `unspecified-high`
  Start from clean state. Execute EVERY QA scenario from EVERY task — follow exact steps, capture evidence. Test cross-task integration (features working together). Test edge cases: `--no-default-features`, single feature combos. Save to `.sisyphus/evidence/final-qa/`.
  Output: `Scenarios [N/N pass] | Integration [N/N] | Edge Cases [N tested] | VERDICT`

- [x] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (git log/diff). Verify 1:1 — everything in spec was built (no missing), nothing beyond spec was built (no creep). Check "Must NOT do" compliance. Detect cross-task contamination. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

- **Wave 1**: `refactor(client): fix public export conflicts and remove unexpected_cfgs allow` — lib.rs, traits/
- **Wave 1**: `refactor(core): add map_context to CoreError for context injection` — openlark-core/src/error/
- **Wave 1**: `fix(client): align Cargo.toml feature dependencies` — Cargo.toml
- **Wave 2**: `refactor(client): remove unused registry subsystem (factory/flags/resolver)` — registry/
- **Wave 2**: `refactor(client): remove unimplemented traits` — traits/
- **Wave 2**: `refactor(client): consolidate ConfigSummary and clean features.rs` — config.rs, utils.rs, features.rs
- **Wave 3**: `refactor(client): update lib.rs exports and remove stale tests` — lib.rs, tests/
- **Wave 3**: `docs(client): update AGENTS.md and README to match new architecture` — AGENTS.md, README.md

---

## Success Criteria

### Verification Commands
```bash
cargo check -p openlark-client --all-features          # Expected: 0 errors, 0 warnings
cargo check -p openlark-client --no-default-features    # Expected: 0 errors
cargo check -p openlark-client --features docs          # Expected: 0 errors
cargo check -p openlark-client --features communication # Expected: 0 errors
cargo check -p openlark-client --features meeting       # Expected: 0 errors
cargo test -p openlark-client --all-features            # Expected: all pass
cargo clippy -p openlark-client --all-features          # Expected: 0 warnings
cargo check -p open-lark --all-features                 # Expected: 0 errors (root crate)
```

### Final Checklist
- [x] All "Must Have" present
- [x] All "Must NOT Have" absent
- [x] All tests pass
- [x] ~4000 lines of dead code removed
- [x] Zero public export name conflicts
- [x] Feature dependencies correctly expressed in Cargo.toml
- [x] AGENTS.md matches actual src/ structure
