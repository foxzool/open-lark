# openlark-core P0+P1 问题修复

## TL;DR

> **Quick Summary**: 修复 openlark-core 代码审查中发现的 4 个 P0 和 5 个 P1 问题，涵盖错误链路、feature 清理、API 边界收紧和宏 hygiene。
> 
> **Deliverables**:
> - 网络错误 source 链完整保留
> - 43 个无用业务 feature 从 core 移除
> - req_timeout 接入请求链
> - 孤儿模块处理
> - ApiRequest 字段可见性收紧
> - 序列化错误不再静默吞掉
> - 错误分类体系收敛为 ErrorCode 主导
> - 宏 hygiene 修复（$crate）
> - send() 占位 API 移除
> 
> **Estimated Effort**: Medium
> **Parallel Execution**: YES - 3 waves
> **Critical Path**: Task 1 → Task 5 → Task 8 → Task 9 → Final

---

## Context

### Original Request
修复 openlark-core 代码审查中发现的所有 P0 和 P1 问题。

### Interview Summary
**Key Discussions**:
- req_timeout: 用户选择接入请求链（而非删除）
- 业务 feature: 迁移到聚合层 crate（根 Cargo.toml 已有完整 feature 管理）
- 错误体系: 以 ErrorCode 为单一语义源，ErrorType/ErrorKind 作为派生视图
- 测试策略: Tests-after（先修复代码，再补充/更新测试）

**Research Findings**:
- 43 个业务 feature 在 core 内零 `cfg` 落点，其他 crate 也未引用，移除安全
- `validate_required!` 宏在全仓库 965 处使用（633 文件），hygiene 修复需全量验证
- openlark-client 的 ClientBuilder 直接 mutate ConfigInner 字段 → ConfigInner pub 字段可见性降级需 descope

### Metis Review
**Identified Gaps** (addressed):
- ConfigInner 字段可见性变更会破坏 openlark-client → 从本轮 descope，仅添加 getter 方法作为迁移准备
- error/observability.rs 可能被 improved_response_handler.rs 间接引用 → 需先检查再决定删除或纳入
- 宏 hygiene 修复影响面大（965 处）→ 必须 `cargo check --workspace` 全量验证

---

## Work Objectives

### Core Objective
修复 openlark-core 的 P0+P1 问题，提升错误链路完整性、API 边界安全性和代码规范一致性。

### Concrete Deliverables
- `http.rs`: 网络错误保留 source chain
- `Cargo.toml`: 移除 43 个无用业务 feature
- `config.rs` + `request_builder/`: req_timeout 接入请求链
- `error/mod.rs`: 孤儿模块处理
- `api/mod.rs`: ApiRequest 字段收紧 + send() 移除 + 序列化错误处理
- `lib.rs`: 宏 hygiene 修复
- `error/`: 错误分类体系收敛

### Definition of Done
- [x] `cargo check -p openlark-core` 通过
- [x] `cargo check --workspace` 通过（验证无跨 crate 破坏）
- [x] `cargo test -p openlark-core` 全部通过
- [x] `cargo test --workspace` 无新增失败

### Must Have
- 网络错误 source 不丢失
- req_timeout 在请求中生效
- 宏在 crate 重命名场景下仍可用
- 序列化失败返回错误而非静默丢弃

### Must NOT Have (Guardrails)
- ❌ 不修改 openlark-client 或其他下游 crate 的代码（本轮仅 core）
- ❌ 不破坏现有公开 API 签名（方法名、参数类型不变）
- ❌ 不删除 ConfigInner 的 pub 字段（Metis 确认会破坏 openlark-client）
- ❌ 不触碰 P2 问题（prelude 收敛、命名统一、文档等）
- ❌ 不引入新的 public API（仅修复/收紧现有 API）

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: YES
- **Automated tests**: Tests-after
- **Framework**: cargo test (Rust built-in)
- **Strategy**: 修复代码后更新受影响的测试，确保全量通过

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Library/Module**: Use Bash (cargo test / cargo check) — 编译检查、测试运行、grep 验证
- **Cross-crate**: Use Bash (cargo check --workspace) — 全工作空间编译验证

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — 独立修复，互不依赖):
├── Task 1: 网络错误 source 保留 [quick]
├── Task 2: 移除 43 个无用业务 feature [quick]
├── Task 3: req_timeout 接入请求链 [unspecified-high]
├── Task 4: 孤儿模块 error/observability.rs 处理 [quick]
├── Task 5: 宏 hygiene 修复 ($crate) [quick]
└── Task 6: send() 占位 API 移除 [quick]

Wave 2 (After Wave 1 — 依赖 Wave 1 的编译稳定):
├── Task 7: ApiRequest 字段可见性收紧 [unspecified-high]
├── Task 8: 序列化错误处理改造 [unspecified-high]
└── Task 9: 错误分类体系收敛 [deep]

Wave 3 (After Wave 2 — 验证与测试):
└── Task 10: 全量测试更新与验证 [deep]

Wave FINAL (After ALL tasks — 独立审查，4 并行):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Real manual QA (unspecified-high)
└── Task F4: Scope fidelity check (deep)

Critical Path: Task 1 → Task 5 → Task 8 → Task 10 → F1-F4
Parallel Speedup: ~60% faster than sequential
Max Concurrent: 6 (Wave 1)
```

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|-----------|--------|------|
| 1 | — | 10 | 1 |
| 2 | — | 10 | 1 |
| 3 | — | 10 | 1 |
| 4 | — | 10 | 1 |
| 5 | — | 7, 8, 10 | 1 |
| 6 | — | 7, 10 | 1 |
| 7 | 5, 6 | 10 | 2 |
| 8 | 5 | 10 | 2 |
| 9 | — | 10 | 2 |
| 10 | 1-9 | F1-F4 | 3 |

### Agent Dispatch Summary

- **Wave 1**: 6 tasks — T1 `quick`, T2 `quick`, T3 `unspecified-high`, T4 `quick`, T5 `quick`, T6 `quick`
- **Wave 2**: 3 tasks — T7 `unspecified-high`, T8 `unspecified-high`, T9 `deep`
- **Wave 3**: 1 task — T10 `deep`
- **FINAL**: 4 tasks — F1 `oracle`, F2 `unspecified-high`, F3 `unspecified-high`, F4 `deep`

---

## TODOs

- [x] 1. 修复网络错误 source 丢失

  **What to do**:
  - 在 `http.rs:148-152` 中，将 `Err(crate::error::network_error(err.to_string()))` 改为 `Err(err.into())`
  - 已有 `From<reqwest::Error> for CoreError` 实现（`error/core.rs:784`），直接利用
  - 确保错误链中 `source()` 方法能返回原始 `reqwest::Error`

  **Must NOT do**:
  - 不改变 `LarkAPIError` 的公开类型签名
  - 不修改其他错误转换路径

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 单文件单行修改，逻辑明确
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `openlark-code-standards`: 不需要规范检查，修复点明确

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3, 4, 5, 6)
  - **Blocks**: Task 10
  - **Blocked By**: None

  **References**:

  **Pattern References**:
  - `crates/openlark-core/src/error/core.rs:784` — `From<reqwest::Error> for CoreError` 实现，确认转换路径存在
  - `crates/openlark-core/src/error/core.rs:721-731` — `CoreError::network()` 构造函数，展示如何保留 source

  **API/Type References**:
  - `crates/openlark-core/src/error/core.rs:356` — `CoreError` 枚举定义，确认 Network 变体结构
  - `crates/openlark-core/src/error/core.rs:436-448` — `NetworkError` 结构体，包含 `source` 字段

  **WHY Each Reference Matters**:
  - `core.rs:784` 的 From impl 证明 `err.into()` 可以直接工作，无需手动构造
  - `core.rs:436` 的 NetworkError 结构体确认 source 字段存在，链路不会断

  **Acceptance Criteria**:
  - [x] `cargo check -p openlark-core` 通过
  - [ ] 修改后的代码使用 `err.into()` 而非 `err.to_string()`

  **QA Scenarios:**

  ```
  Scenario: 网络错误 source chain 保留验证
    Tool: Bash (grep + cargo check)
    Preconditions: 代码已修改
    Steps:
      1. grep -n 'network_error(err.to_string())' crates/openlark-core/src/http.rs → 应无匹配
      2. grep -n 'err.into()' crates/openlark-core/src/http.rs → 应有匹配（在原 148 行附近）
      3. cargo check -p openlark-core → 编译通过
    Expected Result: grep 验证替换完成，编译通过
    Failure Indicators: grep 仍找到 network_error(err.to_string()) 或编译失败
    Evidence: .sisyphus/evidence/task-1-network-error-source.txt
  ```

  **Commit**: YES
  - Message: `fix(core): 保留网络错误 source chain`
  - Files: `crates/openlark-core/src/http.rs`
  - Pre-commit: `cargo check -p openlark-core`

- [x] 2. 移除 43 个无用业务 feature flags
  **What to do**:
  - 从 `Cargo.toml` 的 `[features]` 中删除以下 43 个业务 feature：`acs`, `admin`, `ai`, `aily`, `apass`, `application`, `approval`, `attendance`, `authentication`, `bot`, `calendar`, `cardkit`, `cloud-docs`, `contact`, `corehr`, `directory`, `ehr`, `elearning`, `group`, `helpdesk`, `hire`, `human-authentication`, `im`, `lingo`, `mail`, `mdm`, `minutes`, `moments`, `okr`, `passport`, `payroll`, `performance`, `personal-settings`, `report`, `search`, `security-and-compliance`, `task`, `tenant`, `tenant-tag`, `trust-party`, `vc`, `verification`, `workplace`
  - 保留 `default`, `websocket`, `otel`, `testing` 这 4 个有实际代码落点的 feature
  - 在保留的 feature 区域添加注释说明：业务模块 feature 已迁移到根 crate 管理
  **Must NOT do**:
  - 不修改根 Cargo.toml 或其他 crate 的 Cargo.toml
  - 不删除 `websocket`/`otel`/`testing` feature
  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 单文件删除操作，无逻辑复杂度
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3, 4, 5, 6)
  - **Blocks**: Task 10
  - **Blocked By**: None
  **References**:
  **Pattern References**:
  - `crates/openlark-core/Cargo.toml:48-103` — 当前 features 区域，需要删除 61-103 行的业务 feature
  - `Cargo.toml:240-290`（根 crate）— 根 crate 已有完整 feature 管理，确认不依赖 core 的业务 feature
  **WHY Each Reference Matters**:
  - 根 Cargo.toml 证明 feature 管理在聚合层完成，core 的业务 feature 是冗余声明
  **Acceptance Criteria**:
  - [x] `cargo check --workspace` 通过（无跨 crate 破坏）
  - [ ] `grep -c 'acs\|admin\|corehr\|hire' crates/openlark-core/Cargo.toml` 返回 0
  **QA Scenarios:**
  ```
  Scenario: 业务 feature 已全部移除
    Tool: Bash (grep + cargo check)
    Preconditions: Cargo.toml 已修改
    Steps:
      1. grep -E '^(acs|admin|ai |aily|apass|application|approval|attendance|authentication|bot|calendar|cardkit|cloud-docs|contact|corehr|directory|ehr|elearning|group|helpdesk|hire|human-authentication|im |lingo|mail |mdm|minutes|moments|okr|passport|payroll|performance|personal-settings|report|search|security-and-compliance|task |tenant|tenant-tag|trust-party|vc |verification|workplace)' crates/openlark-core/Cargo.toml → 应无匹配
      2. grep -E '^(default|websocket|otel|testing)' crates/openlark-core/Cargo.toml → 应有 4 个匹配
      3. cargo check --workspace → 编译通过
    Expected Result: 43 个业务 feature 已删除，4 个基础 feature 保留，全工作空间编译通过
    Failure Indicators: grep 仍找到业务 feature 或 cargo check 失败
    Evidence: .sisyphus/evidence/task-2-feature-cleanup.txt
  ```
  **Commit**: YES
  - Message: `refactor(core): 移除无用业务 feature flags`
  - Files: `crates/openlark-core/Cargo.toml`
  - Pre-commit: `cargo check --workspace`

- [x] 3. 将 req_timeout 接入请求链
  **What to do**:
  - 在 `request_builder/mod.rs` 的 `UnifiedRequestBuilder::build()` 中，读取 `config.req_timeout` 并调用 `req_builder.timeout(duration)`
  - 在 `http.rs` 的 `do_request()` 中，如果 `ApiRequest` 自带 timeout 则优先使用，否则 fallback 到 config 级别
  - 优先级：ApiRequest.timeout > Config.req_timeout > 无超时（当前行为）
  - 确保 `ConfigBuilder::req_timeout()` 方法的文档说明生效位置
  **Must NOT do**:
  - 不改变 Config 的公开 API 签名
  - 不引入新的 public 类型
  - 不修改默认超时行为（默认仍为无超时）
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 涉及 3 个文件的协调修改，需理解请求链路
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 4, 5, 6)
  - **Blocks**: Task 10
  - **Blocked By**: None
  **References**:
  **Pattern References**:
  - `crates/openlark-core/src/request_builder/mod.rs:23-90` — `UnifiedRequestBuilder::build()` 构建 reqwest::RequestBuilder 的完整流程
  - `crates/openlark-core/src/http.rs:100-160` — `do_request()` 发送请求的完整流程
  **API/Type References**:
  - `crates/openlark-core/src/config.rs:60` — `req_timeout: Option<Duration>` 字段定义
  - `crates/openlark-core/src/config.rs:233` — `ConfigBuilder::req_timeout()` 方法
  - `crates/openlark-core/src/api/mod.rs:224-228` — `ApiRequest::timeout()` 方法
  **WHY Each Reference Matters**:
  - `request_builder/mod.rs:23` 是构建 reqwest 请求的入口，timeout 需在此注入
  - `api/mod.rs:224` 展示 ApiRequest 已有 timeout 字段，需处理优先级
  **Acceptance Criteria**:
  - [x] `cargo check -p openlark-core` 通过
  - [ ] `cargo test -p openlark-core` 通过
  - [ ] Config.req_timeout 在请求链中被消费（grep 验证）
  **QA Scenarios:**
  ```
  Scenario: req_timeout 接入验证
    Tool: Bash (grep + cargo check)
    Preconditions: 代码已修改
    Steps:
      1. grep -n 'req_timeout' crates/openlark-core/src/request_builder/mod.rs → 应有新增引用
      2. grep -n 'timeout' crates/openlark-core/src/request_builder/mod.rs → 应有 .timeout() 调用
      3. cargo test -p openlark-core → 全部通过
    Expected Result: req_timeout 在请求构建中被消费
    Failure Indicators: grep 未找到 timeout 消费点或测试失败
    Evidence: .sisyphus/evidence/task-3-req-timeout.txt

  Scenario: 无超时时行为不变
    Tool: Bash (cargo test)
    Preconditions: Config 使用默认值（req_timeout = None）
    Steps:
      1. cargo test -p openlark-core → 现有测试全部通过（默认行为未改变）
    Expected Result: 所有现有测试通过
    Failure Indicators: 任何现有测试失败
    Evidence: .sisyphus/evidence/task-3-req-timeout-default.txt
  ```
  **Commit**: YES
  - Message: `fix(core): 将 req_timeout 接入请求链`
  - Files: `crates/openlark-core/src/config.rs`, `crates/openlark-core/src/request_builder/mod.rs`, `crates/openlark-core/src/http.rs`
  - Pre-commit: `cargo test -p openlark-core`
---

- [x] 4. 处理 error/observability.rs 孤儿模块
  **What to do**:
  - 检查 `error/observability.rs` 是否被 `improved_response_handler.rs` 或 `src/observability.rs` 间接引用
  - 如果有引用：在 `error/mod.rs` 中添加 `pub mod observability;` 纳入模块树
  - 如果无引用：删除 `error/observability.rs` 文件（死代码）
  - 无论哪种情况，确保 `cargo check -p openlark-core` 通过
  **Must NOT do**:
  - 不修改 error/observability.rs 的内部实现
  - 不改变其他模块的公开 API
  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 检查引用关系后做简单的纳入或删除
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3, 5, 6)
  - **Blocks**: Task 10
  - **Blocked By**: None
  **References**:
  **Pattern References**:
  - `crates/openlark-core/src/error/mod.rs:33-37` — 当前模块声明列表，observability 未纳入
  - `crates/openlark-core/src/error/observability.rs:1-50` — 孤儿模块头部，了解其导出内容
  **API/Type References**:
  - `crates/openlark-core/src/improved_response_handler.rs` — 可能间接引用 observability 类型
  - `crates/openlark-core/src/observability.rs` — 顶层 observability 模块，可能有概念重叠
  **WHY Each Reference Matters**:
  - 需要确认 error/observability.rs 的类型是否被其他文件 use，决定纳入还是删除
  **Acceptance Criteria**:
  - [x] `cargo check -p openlark-core` 通过
  - [ ] `error/observability.rs` 要么在模块树中，要么已删除（不再是孤儿）
  **QA Scenarios:**
  ```
  Scenario: 孤儿模块已处理
    Tool: Bash (grep + cargo check)
    Preconditions: 代码已修改
    Steps:
      1. 检查 error/observability.rs 是否存在：ls crates/openlark-core/src/error/observability.rs
      2. 如果存在：grep 'mod observability' crates/openlark-core/src/error/mod.rs → 应有声明
      3. 如果不存在：确认已删除
      4. cargo check -p openlark-core → 编译通过
    Expected Result: 孤儿状态消除
    Failure Indicators: 文件存在但未纳入模块树，或编译失败
    Evidence: .sisyphus/evidence/task-4-orphan-module.txt
  ```
  **Commit**: YES
  - Message: `fix(core): 处理 error/observability.rs 孤儿模块`
  - Files: `crates/openlark-core/src/error/mod.rs`, `crates/openlark-core/src/error/observability.rs`
  - Pre-commit: `cargo check -p openlark-core`
- [x] 5. 修复导出宏 hygiene 问题
  **What to do**:
  - 在 `lib.rs:75-104` 中，将 `validate_required!` 和 `validate_required_list!` 宏内的 `openlark_core::` 替换为 `$crate::`
  - 具体替换：`openlark_core::Validatable::is_empty_trimmed` → `$crate::Validatable::is_empty_trimmed`
  - 具体替换：`openlark_core::error::CoreError::validation_msg` → `$crate::error::CoreError::validation_msg`
  - 全仓库 965 处使用，必须 `cargo check --workspace` 全量验证
  **Must NOT do**:
  - 不改变宏的语义行为
  - 不修改宏的参数签名
  - 不修改 Validatable trait 定义
  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 3 处字符串替换，逻辑明确
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3, 4, 6)
  - **Blocks**: Tasks 7, 8, 10
  - **Blocked By**: None
  **References**:
  **Pattern References**:
  - `crates/openlark-core/src/lib.rs:75-104` — 两个宏的完整定义，需要修改的 3 处 `openlark_core::` 引用
  **External References**:
  - Rust Reference: `$crate` 在 `#[macro_export]` 宏中指向定义宏的 crate，即使被重命名也能正确解析
  **WHY Each Reference Matters**:
  - `lib.rs:77-78` 是 validate_required! 的核心路径，`lib.rs:97-101` 是 validate_required_list! 的核心路径
  **Acceptance Criteria**:
  - [x] `cargo check --workspace` 通过
  - [ ] grep 确认无残留 `openlark_core::` 在宏定义中
  **QA Scenarios:**
  ```
  Scenario: 宏 hygiene 修复验证
    Tool: Bash (grep + cargo check)
    Preconditions: 代码已修改
    Steps:
      1. grep -n 'openlark_core::' crates/openlark-core/src/lib.rs → 应无匹配（宏内不再硬编码）
      2. grep -n '\$crate::' crates/openlark-core/src/lib.rs → 应有 3 处匹配
      3. cargo check --workspace → 编译通过（965 处宏调用全部兼容）
    Expected Result: 宏使用 $crate 引用，全工作空间编译通过
    Failure Indicators: grep 仍找到 openlark_core:: 或 workspace 编译失败
    Evidence: .sisyphus/evidence/task-5-macro-hygiene.txt
  ```
  **Commit**: YES
  - Message: `fix(core): 修复导出宏 hygiene ($crate)`
  - Files: `crates/openlark-core/src/lib.rs`
  - Pre-commit: `cargo check --workspace`
- [x] 6. 移除 send() 占位 API
  **What to do**:
  - 删除 `api/mod.rs:318-325` 中的 `send()` 方法（运行时必然失败的占位 API）
  - 检查是否有调用方引用 `send()`，如有则更新为 `Transport::request()`
  - 如果 `send()` 在其他 crate 中被引用，改为 `#[deprecated]` 标注而非直接删除
  **Must NOT do**:
  - 不修改 `Transport::request()` 的签名
  - 不引入新的 public 方法
  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 单方法删除，逻辑简单
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3, 4, 5)
  - **Blocks**: Tasks 7, 10
  - **Blocked By**: None
  **References**:
  **Pattern References**:
  - `crates/openlark-core/src/api/mod.rs:318-325` — `send()` 方法定义，返回 `configuration_error`
  - `crates/openlark-core/src/http.rs:39-43` — `Transport::request()` 是正确的请求入口
  **WHY Each Reference Matters**:
  - `api/mod.rs:318` 是要删除的目标代码
  - `http.rs:39` 是正确替代方案，需确认调用方已使用此路径
  **Acceptance Criteria**:
  - [x] `cargo check --workspace` 通过
  - [ ] grep 确认 `send()` 方法已移除或标记 deprecated
  **QA Scenarios:**
  ```
  Scenario: send() 占位 API 已移除
    Tool: Bash (grep + cargo check)
    Preconditions: 代码已修改
    Steps:
      1. grep -n 'fn send' crates/openlark-core/src/api/mod.rs → 应无匹配（或仅有 deprecated 标注）
      2. grep -rn '.send()' crates/ --include='*.rs' → 检查无调用方（排除 reqwest 的 send）
      3. cargo check --workspace → 编译通过
    Expected Result: send() 已移除且无编译错误
    Failure Indicators: 编译失败（有调用方依赖 send）
    Evidence: .sisyphus/evidence/task-6-send-removal.txt
  ```
  **Commit**: YES
  - Message: `fix(core): 移除 send() 占位 API`
  - Files: `crates/openlark-core/src/api/mod.rs`
  - Pre-commit: `cargo check --workspace`
- [x] 7. 收紧 ApiRequest 字段可见性
  **What to do**:
  - 将 `api/mod.rs:95-102` 中 `ApiRequest` 的字段从 `pub` 改为 `pub(crate)`
  - 确认所有外部访问通过已有的 getter 方法（`method()`, `api_path()`, `body()` 等）
  - 如果有外部 crate 直接访问字段，补充必要的 getter 方法
  **Must NOT do**:
  - 不改变 ApiRequest 的 public 方法签名
  - 不修改构造器方法（`get()`, `post()` 等）
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 需要检查跨 crate 字段访问，影响面需评估
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 8, 9)
  - **Blocks**: Task 10
  - **Blocked By**: Tasks 5, 6
  **References**:
  **Pattern References**:
  - `crates/openlark-core/src/api/mod.rs:94-102` — `ApiRequest` 结构体定义，字段全部 `pub`
  - `crates/openlark-core/src/api/mod.rs:106-170` — 构造器方法（`get/post/put/patch/delete`），不受影响
  - `crates/openlark-core/src/api/mod.rs:244-286` — getter 方法（`method()/api_path()/to_bytes()/file()`）
  **WHY Each Reference Matters**:
  - `mod.rs:94` 是修改目标，`mod.rs:244` 证明 getter 已存在，外部应通过 getter 访问
  **Acceptance Criteria**:
  - [x] `cargo check --workspace` 通过
  - [ ] ApiRequest 字段为 `pub(crate)` 而非 `pub`
  **QA Scenarios:**
  ```
  Scenario: ApiRequest 字段已收紧
    Tool: Bash (grep + cargo check)
    Preconditions: 代码已修改
    Steps:
      1. grep -n 'pub method\|pub url\|pub query\|pub body\|pub headers' crates/openlark-core/src/api/mod.rs → 应无匹配（字段不再是 pub）
      2. grep -n 'pub(crate)' crates/openlark-core/src/api/mod.rs → 应有字段级匹配
      3. cargo check --workspace → 编译通过
    Expected Result: 字段可见性已收紧，全工作空间编译通过
    Failure Indicators: 编译失败（外部 crate 直接访问字段）
    Evidence: .sisyphus/evidence/task-7-api-request-visibility.txt
  ```
  **Commit**: YES
  - Message: `refactor(core): 收紧 ApiRequest 字段可见性`
  - Files: `crates/openlark-core/src/api/mod.rs`
  - Pre-commit: `cargo check --workspace`
- [x] 8. 序列化错误处理改造
  **What to do**:
  - 在 `api/mod.rs:217-220` 的 `json_body()` 中，将序列化失败从静默设为 `Empty` 改为返回错误或 log warning
  - 在 `api/mod.rs:267` 的 `to_bytes()` 中，同样处理序列化失败
  - 在 `request_builder/mod.rs:71` 中，处理 body 序列化失败的情况
  - 推荐方案：序列化失败时设置 body 为 `RequestData::Json(Value::Null)` 并 `tracing::warn!` 记录，而非静默丢弃
  - 备选方案：改 `json_body` 签名为 `Result`（但会破坏 builder chain，不推荐）
  **Must NOT do**:
  - 不改变 `json_body()` 的方法签名（保持 builder chain）
  - 不引入 panic 路径
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 涉及多处修改，需理解序列化链路
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 7, 9)
  - **Blocks**: Task 10
  - **Blocked By**: Task 5
  **References**:
  **Pattern References**:
  - `crates/openlark-core/src/api/mod.rs:213-220` — `json_body()` 方法，序列化失败设为 Empty
  - `crates/openlark-core/src/api/mod.rs:265-270` — `to_bytes()` 方法，序列化失败返回空 vec
  - `crates/openlark-core/src/request_builder/mod.rs:65-75` — body 构建逻辑
  **WHY Each Reference Matters**:
  - 这三处是序列化静默失败的完整链路，需要一起修复
  **Acceptance Criteria**:
  - [x] `cargo check -p openlark-core` 通过
  - [ ] 序列化失败时有 warning 日志输出（grep 验证 tracing::warn）
  **QA Scenarios:**
  ```
  Scenario: 序列化失败不再静默
    Tool: Bash (grep + cargo check)
    Preconditions: 代码已修改
    Steps:
      1. grep -n 'RequestData::Empty' crates/openlark-core/src/api/mod.rs → 应无匹配（不再静默设为 Empty）
      2. grep -n 'warn!' crates/openlark-core/src/api/mod.rs → 应有匹配（序列化失败有日志）
      3. cargo test -p openlark-core → 全部通过
    Expected Result: 序列化失败有 warning 日志，不再静默丢弃
    Failure Indicators: 仍有 RequestData::Empty 作为序列化失败的 fallback
    Evidence: .sisyphus/evidence/task-8-serialization-error.txt
  ```
  **Commit**: YES
  - Message: `fix(core): 序列化失败返回错误而非静默丢弃`
  - Files: `crates/openlark-core/src/api/mod.rs`, `crates/openlark-core/src/request_builder/mod.rs`
  - Pre-commit: `cargo test -p openlark-core`
- [x] 9. 错误分类体系收敛为 ErrorCode 主导
  **What to do**:
  - 以 `ErrorCode`（`error/codes.rs`）为单一语义源
  - 将 `ErrorType`（`error/traits.rs:172`）改为从 `ErrorCode` 派生的视图方法，而非独立枚举
  - 将 `ErrorKind`（`error/kinds.rs:17`）改为从 `ErrorCode` 派生，或标记 `#[deprecated]` 引导迁移
  - 统一 `CoreError` 上的 `severity()` 和 `is_retryable()` 实现，委托给 `ErrorCode` 的对应方法
  - 消除 `ErrorCode::severity()` 与 `CoreError::severity()` 的双重定义
  - `BuilderKind` 保持不变（仅用于内部构造）
  **Must NOT do**:
  - 不删除 `ErrorCode` 枚举的任何变体
  - 不改变 `CoreError` 的 public match 模式（枚举变体名不变）
  - 不修改 `ErrorRecord` 的序列化格式
  - 不触碰 `error/context.rs`（上下文系统独立且正确）
  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: 错误体系收敛涉及多文件协调，需深入理解类型关系
  - **Skills**: [`openlark-design-review`]
    - `openlark-design-review`: 需要理解错误系统的架构设计意图
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 7, 8)
  - **Blocks**: Task 10
  - **Blocked By**: None
  **References**:
  **Pattern References**:
  - `crates/openlark-core/src/error/codes.rs:14-1007` — `ErrorCode` 枚举完整定义，含 severity/retryable/category 方法
  - `crates/openlark-core/src/error/traits.rs:172-215` — `ErrorType` 枚举，与 ErrorCode 职责重叠
  - `crates/openlark-core/src/error/kinds.rs:17-177` — `ErrorKind` 枚举，与 ErrorCode/ErrorType 三重重叠
  - `crates/openlark-core/src/error/core.rs:356-700` — `CoreError` 枚举及其 severity/retryable 实现
  **WHY Each Reference Matters**:
  - `codes.rs` 是目标主分类源，其他文件需要委托到它
  - `traits.rs:172` 和 `kinds.rs:17` 是需要收敛的重叠分类
  - `core.rs:672-687` 是需要统一的 severity/retryable 双重实现
  **Acceptance Criteria**:
  - [x] `cargo check -p openlark-core` 通过
  - [ ] `cargo test -p openlark-core` 通过
  - [ ] `CoreError::severity()` 委托给 `ErrorCode::severity()`（无独立逻辑）
  - [ ] `ErrorType` 或 `ErrorKind` 至少一个标记 deprecated 或改为派生方法
  **QA Scenarios:**
  ```
  Scenario: 错误分类体系已收敛
    Tool: Bash (grep + cargo test)
    Preconditions: 代码已修改
    Steps:
      1. grep -n 'fn severity' crates/openlark-core/src/error/core.rs → 应委托给 ErrorCode
      2. grep -n 'deprecated' crates/openlark-core/src/error/kinds.rs → 应有 deprecated 标记（或文件已重构）
      3. cargo test -p openlark-core → 全部通过
    Expected Result: severity/retryable 逻辑统一到 ErrorCode
    Failure Indicators: 仍存在独立的 severity 判断逻辑
    Evidence: .sisyphus/evidence/task-9-error-convergence.txt
  ```
  **Commit**: YES
  - Message: `refactor(core): 收敛错误分类体系为 ErrorCode 主导`
  - Files: `crates/openlark-core/src/error/core.rs`, `crates/openlark-core/src/error/traits.rs`, `crates/openlark-core/src/error/kinds.rs`
  - Pre-commit: `cargo test -p openlark-core`
- [x] 10. 全量测试更新与验证
  **What to do**:
  - 运行 `cargo test -p openlark-core`，修复所有因 Task 1-9 导致的测试失败
  - 运行 `cargo test --workspace`，确认无跨 crate 破坏
  - 运行 `cargo clippy -p openlark-core`，确认无新增警告
  - 为新增行为补充测试：
    - req_timeout 生效测试（单元测试）
    - 序列化失败 warning 日志测试
    - 错误分类委托一致性测试
  **Must NOT do**:
  - 不删除现有测试（只修复和补充）
  - 不修改其他 crate 的测试文件
  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: 需要理解所有前置任务的变更并综合验证
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (sequential)
  - **Blocks**: F1-F4
  - **Blocked By**: Tasks 1-9
  **References**:
  **Pattern References**:
  - `crates/openlark-core/src/` 下所有 `#[cfg(test)]` 模块（46 个测试模块）
  - `tests/unit/error/mod.rs` — 外部测试文件（Cargo.toml 中声明）
  **WHY Each Reference Matters**:
  - 需要遍历所有测试模块确认无破坏，并为新行为补充测试
  **Acceptance Criteria**:
  - [x] `cargo test -p openlark-core` 全部通过
  - [x] `cargo test --workspace` 无新增失败
  - [ ] `cargo clippy -p openlark-core` 无新增警告
  **QA Scenarios:**
  ```
  Scenario: 全量测试通过
    Tool: Bash (cargo test)
    Preconditions: Tasks 1-9 全部完成
    Steps:
      1. cargo test -p openlark-core 2>&1 → 全部通过
      2. cargo test --workspace 2>&1 → 无新增失败
      3. cargo clippy -p openlark-core 2>&1 → 无新增警告
    Expected Result: 所有测试通过，无新增警告
    Failure Indicators: 任何测试失败或新增 clippy 警告
    Evidence: .sisyphus/evidence/task-10-full-test.txt
  ```
  **Commit**: YES
  - Message: `test(core): 更新受影响测试用例`
  - Files: `crates/openlark-core/src/**/*test*`, `tests/unit/error/mod.rs`
  - Pre-commit: `cargo test --workspace`
---

## Final Verification Wave

> 4 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [x] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, run command). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in .sisyphus/evidence/. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo check --workspace` + `cargo clippy -p openlark-core` + `cargo test -p openlark-core`. Review all changed files for: `as any`, empty catches, commented-out code, unused imports. Check AI slop: excessive comments, over-abstraction, generic names.
  Output: `Build [PASS/FAIL] | Clippy [PASS/FAIL] | Tests [N pass/N fail] | Files [N clean/N issues] | VERDICT`

- [x] F3. **Real Manual QA** — `unspecified-high`
  Start from clean state. Execute EVERY QA scenario from EVERY task — follow exact steps, capture evidence. Test cross-task integration. Save to `.sisyphus/evidence/final-qa/`.
  Output: `Scenarios [N/N pass] | Integration [N/N] | Edge Cases [N tested] | VERDICT`

- [x] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (git log/diff). Verify 1:1 — everything in spec was built, nothing beyond spec was built. Check "Must NOT do" compliance. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

| Commit | Message | Files | Pre-commit |
|--------|---------|-------|------------|
| 1 | `fix(core): 保留网络错误 source chain` | `http.rs` | `cargo check -p openlark-core` |
| 2 | `refactor(core): 移除无用业务 feature flags` | `Cargo.toml` | `cargo check --workspace` |
| 3 | `fix(core): 将 req_timeout 接入请求链` | `config.rs`, `request_builder/mod.rs`, `http.rs` | `cargo test -p openlark-core` |
| 4 | `fix(core): 处理 error/observability.rs 孤儿模块` | `error/mod.rs`, `error/observability.rs` | `cargo check -p openlark-core` |
| 5 | `fix(core): 修复导出宏 hygiene ($crate)` | `lib.rs` | `cargo check --workspace` |
| 6 | `fix(core): 移除 send() 占位 API` | `api/mod.rs` | `cargo check --workspace` |
| 7 | `refactor(core): 收紧 ApiRequest 字段可见性` | `api/mod.rs` | `cargo check --workspace` |
| 8 | `fix(core): 序列化失败返回错误而非静默丢弃` | `api/mod.rs`, `request_builder/mod.rs` | `cargo test -p openlark-core` |
| 9 | `refactor(core): 收敛错误分类体系为 ErrorCode 主导` | `error/*.rs` | `cargo test -p openlark-core` |
| 10 | `test(core): 更新受影响测试用例` | `tests/`, `src/**/*test*` | `cargo test --workspace` |

---

## Success Criteria

### Verification Commands
```bash
cargo check -p openlark-core          # Expected: Finished
cargo check --workspace               # Expected: Finished (no cross-crate breakage)
cargo test -p openlark-core            # Expected: all tests pass
cargo test --workspace                 # Expected: no new failures
cargo clippy -p openlark-core          # Expected: no new warnings
```

### Final Checklist
- [x] All "Must Have" present
- [x] All "Must NOT Have" absent
- [x] All tests pass
- [x] No new clippy warnings
