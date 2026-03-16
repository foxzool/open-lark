# core-p0p1-fix 遗留问题修复

## TL;DR

> **Quick Summary**: 修复 core-p0p1-fix 计划中因 `git add -A` 意外回退或从未提交的 5 个遗留问题，涵盖 API 字段可见性、序列化错误处理、孤儿文件清理和错误分类体系收敛。
> 
> **Deliverables**:
> - `ApiRequest` 8 个字段从 `pub` 收紧为 `pub(crate)`
> - 移除 `RequestData::Empty` 变体，`json_body()` 序列化失败时 `warn!` + `Json(Null)`
> - 删除孤儿文件 `error/observability.rs`（987 行，不在模块树中）
> - `ErrorType` 实现 `From<ErrorCode>`，`CoreError::is_retryable()` 对简单变体委托给 `ErrorCode`
> 
> **Estimated Effort**: Short（每个任务 10-20 分钟）
> **Parallel Execution**: YES — 2 waves
> **Critical Path**: Task 1-3 并行 → Task 4 → 验证

---

## Context

### Original Request
修复 `core-p0p1-fix` 计划执行后的遗留问题。原计划 10 个任务中，4 个已正确提交，2 个被 commit `53313770`（`git add -A`）意外回退，4 个从未提交。

### 已完成的提交（不需要修改）
- `085475a7` fix(core): 保留网络错误 source chain ✅
- `b45399cf` fix(core): 将 req_timeout 接入请求链 ✅
- `cf631b3f` fix(core): 修复导出宏 hygiene ($crate) ✅
- `53313770` fix: 修复 clippy deprecated 警告 ✅

### Metis Review
**Identified Gaps** (addressed):
- Issue 4（43 个空 feature flags）经验证后 **DROP** — 这些 feature 被下游 crate（openlark-client、openlark-communication）通过 `#[cfg(feature = "...")]` 使用，删除会破坏编译
- Issue 6 `is_retryable()` 委托需保留 Network/Api 变体的现有行为（Network 使用 RetryPolicy，Api 使用 status 匹配）
- Issues 2+3 必须合并为原子操作（`RequestData::Empty` 在两处使用，耦合紧密）

---

## Work Objectives

### Core Objective
修复 openlark-core 中 5 个已确认的遗留问题，恢复到原计划的目标状态。

### Concrete Deliverables
- `crates/openlark-core/src/api/mod.rs` — 字段可见性 + 序列化处理
- `crates/openlark-core/src/error/observability.rs` — 删除
- `crates/openlark-core/src/error/traits.rs` — `From<ErrorCode> for ErrorType`
- `crates/openlark-core/src/error/core.rs` — `is_retryable()` 部分委托

### Definition of Done
- [x] `cargo check -p openlark-core` 通过
- [x] `cargo test -p openlark-core` 通过
- [x] `cargo clippy -p openlark-core -- -D warnings` 零警告
- [x] `cargo fmt -p openlark-core -- --check` 通过
- [x] `cargo check --workspace --all-features` 通过

### Must Have
- ApiRequest 8 个字段全部 `pub(crate)`
- `RequestData::Empty` 变体完全移除
- `json_body()` 序列化失败时使用 `tracing::warn!` 记录并返回 `Json(Value::Null)`
- `error/observability.rs` 文件删除
- `From<ErrorCode> for ErrorType` 实现
- `CoreError::is_retryable()` 对 Timeout/RateLimit/ServiceUnavailable 委托给 `ErrorCode`

### Must NOT Have (Guardrails)
- ❌ 不修改 openlark-client 或其他下游 crate 的代码
- ❌ 不破坏现有公开 API 签名（方法名、参数类型不变）
- ❌ 不删除 ConfigInner 的 pub 字段
- ❌ 不触碰 P2 问题（prelude 收敛、命名统一、文档等）
- ❌ 不引入新的 public API
- ❌ 不删除 Cargo.toml 中的 43 个 service module feature flags（下游 crate 依赖）
- ❌ 不改变 Network/Api 变体的 `is_retryable()` 行为（Network 用 RetryPolicy，Api 用 status 匹配）
- ❌ 不"改进"或重构 error 模块超出指定范围

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed.

### Test Decision
- **Infrastructure exists**: YES（`cargo test -p openlark-core` 有 40+ 测试）
- **Automated tests**: Tests-after（仅 Issue 6 需要验证行为保持）
- **Framework**: cargo test

### QA Policy
每个任务完成后运行 `cargo check -p openlark-core && cargo test -p openlark-core`。
最终运行 `cargo check --workspace --all-features && cargo clippy -p openlark-core -- -D warnings && cargo fmt -p openlark-core -- --check`。

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — 3 independent tasks):
├── Task 1: ApiRequest 字段可见性 pub → pub(crate) [quick]
├── Task 2: 移除 RequestData::Empty + 修复 json_body() [quick]
└── Task 3: 删除孤儿 error/observability.rs [quick]

Wave 2 (After Wave 1 — depends on clean compilation):
└── Task 4: ErrorType From<ErrorCode> + is_retryable() 部分委托 [quick]

Wave FINAL (After ALL tasks):
├── Task F1: cargo check --workspace --all-features [quick]
├── Task F2: cargo clippy -p openlark-core -- -D warnings [quick]
├── Task F3: cargo fmt -p openlark-core -- --check [quick]
└── Task F4: grep 验证无残留 [quick]

Critical Path: Tasks 1-3 → Task 4 → F1-F4
Parallel Speedup: Wave 1 三任务并行
Max Concurrent: 3 (Wave 1)
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|-----------|--------|
| 1 | — | F1-F4 |
| 2 | — | F1-F4 |
| 3 | — | F1-F4 |
| 4 | 1, 2, 3 (clean compile) | F1-F4 |
| F1-F4 | 1, 2, 3, 4 | — |

### Agent Dispatch Summary

- **Wave 1**: 3 tasks — T1 → `quick`, T2 → `quick`, T3 → `quick`
- **Wave 2**: 1 task — T4 → `quick`
- **FINAL**: 4 tasks — F1-F4 → `quick`

---

## TODOs

- [x] 1. ApiRequest 字段可见性 pub → pub(crate)

  **What to do**:
  - 将 `crates/openlark-core/src/api/mod.rs:95-102` 的 8 个字段从 `pub` 改为 `pub(crate)`
  - 字段列表: `method`, `url`, `headers`, `query`, `body`, `file`, `timeout`, `_phantom`
  - 同文件内的 `#[cfg(test)]` 测试（line 375-400）直接访问字段是允许的（同 crate）

  **Must NOT do**:
  - 不修改任何方法签名（`.get()`, `.post()`, `.query()`, `.json_body()` 等）
  - 不修改下游 crate 代码

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3)
  - **Blocks**: Task 4, F1-F4
  - **Blocked By**: None

  **References**:
  - `crates/openlark-core/src/api/mod.rs:94-103` — ApiRequest 结构体定义，8 个 pub 字段
  - `crates/openlark-core/src/api/mod.rs:365-404` — 测试中直接访问字段（同 crate，pub(crate) 可见）
  - 下游 crate 仅通过方法使用 ApiRequest（已验证：openlark-workflow 等只用 `.get()/.query()/.body()`）

  **Acceptance Criteria**:
  - [ ] 8 个字段全部为 `pub(crate)`
  - [ ] `cargo check -p openlark-core` 通过
  - [ ] `cargo test -p openlark-core` 通过
  - [ ] `cargo check --workspace --all-features` 通过

  **QA Scenarios:**
  ```
  Scenario: 字段可见性验证
    Tool: Bash (grep)
    Steps:
      1. grep -n 'pub method\|pub url\|pub headers\|pub query\|pub body\|pub file\|pub timeout\|pub _phantom' crates/openlark-core/src/api/mod.rs
      2. 确认所有匹配行都包含 'pub(crate)'
    Expected Result: 8 行全部为 pub(crate)，无裸 pub
    Evidence: .sisyphus/evidence/task-1-field-visibility.txt

  Scenario: 下游编译验证
    Tool: Bash
    Steps:
      1. cargo check --workspace --all-features 2>&1
    Expected Result: exit code 0, 无 'field.*is private' 错误
    Evidence: .sisyphus/evidence/task-1-workspace-check.txt
  ```

  **Commit**: YES
  - Message: `fix(core): 收紧 ApiRequest 字段可见性为 pub(crate)`
  - Files: `crates/openlark-core/src/api/mod.rs`
  - Pre-commit: `cargo check -p openlark-core && cargo test -p openlark-core`

- [x] 2. 移除 RequestData::Empty + 修复 json_body() 序列化错误处理

  **What to do**:
  - 从 `RequestData` 枚举中移除 `Empty` 变体（line 52）
  - 修改 `json_body()` 方法（line 219）：`Err(_) => self.body = Some(RequestData::Empty)` 改为 `Err(e) => { tracing::warn!(error = %e, "json_body 序列化失败"); self.body = Some(RequestData::Json(serde_json::Value::Null)); }`
  - 修改 `to_body_bytes()` 中的 match arm（line 276）：`Some(RequestData::Empty) => vec![]` 改为移除该分支（Empty 不再存在）
  - `tracing` 已在 Cargo.toml 依赖中（line 25: `tracing = { workspace = true }`）

  **Must NOT do**:
  - 不修改其他 RequestData 变体的行为
  - 不改变 `json_body()` 的方法签名

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3)
  - **Blocks**: Task 4, F1-F4
  - **Blocked By**: None

  **References**:
  - `crates/openlark-core/src/api/mod.rs:47-53` — RequestData 枚举定义，Empty 在 line 52
  - `crates/openlark-core/src/api/mod.rs:213-222` — json_body() 方法，Err 分支在 line 219
  - `crates/openlark-core/src/api/mod.rs:260-280` — to_body_bytes() 方法，Empty match arm 在 line 276
  - `crates/openlark-core/Cargo.toml:25` — tracing 依赖已存在

  **Acceptance Criteria**:
  - [ ] `RequestData::Empty` 变体不存在
  - [ ] `json_body()` 序列化失败时调用 `tracing::warn!` 并设置 `Json(Value::Null)`
  - [ ] `grep -r 'RequestData::Empty' crates/openlark-core/src/` 返回空
  - [ ] `cargo check -p openlark-core` 通过
  - [ ] `cargo test -p openlark-core` 通过

  **QA Scenarios:**
  ```
  Scenario: Empty 变体完全移除
    Tool: Bash (grep)
    Steps:
      1. grep -rn 'RequestData::Empty' crates/openlark-core/src/
      2. grep -rn 'Empty' crates/openlark-core/src/api/mod.rs
    Expected Result: 两个命令均无输出
    Evidence: .sisyphus/evidence/task-2-no-empty.txt

  Scenario: json_body 使用 warn 而非静默
    Tool: Bash (grep)
    Steps:
      1. grep -A2 'Err' crates/openlark-core/src/api/mod.rs | grep -E 'warn|tracing'
    Expected Result: 包含 tracing::warn 或 warn! 调用
    Evidence: .sisyphus/evidence/task-2-warn-check.txt
  ```

  **Commit**: YES
  - Message: `fix(core): 移除 RequestData::Empty，序列化失败 warn + Json(Null)`
  - Files: `crates/openlark-core/src/api/mod.rs`
  - Pre-commit: `cargo check -p openlark-core && cargo test -p openlark-core`

- [x] 3. 删除孤儿 error/observability.rs

  **What to do**:
  - 删除 `crates/openlark-core/src/error/observability.rs`（987 行）
  - 该文件不在模块树中（`error/mod.rs` 无 `mod observability;`）
  - 注意：`src/observability.rs`（顶层）是正常模块，不要删除

  **Must NOT do**:
  - 不删除 `src/observability.rs`（顶层模块，在 lib.rs 中声明）
  - 不修改 `error/mod.rs`（不需要添加任何声明）

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2)
  - **Blocks**: Task 4, F1-F4
  - **Blocked By**: None

  **References**:
  - `crates/openlark-core/src/error/observability.rs` — 987 行孤儿文件，使用已废弃的 ErrorKind
  - `crates/openlark-core/src/error/mod.rs` — 无 `mod observability;` 声明
  - `crates/openlark-core/src/observability.rs` — 顶层模块（不要删除！）

  **Acceptance Criteria**:
  - [ ] `crates/openlark-core/src/error/observability.rs` 文件不存在
  - [ ] `cargo check -p openlark-core` 通过
  - [ ] `src/observability.rs`（顶层）仍然存在

  **QA Scenarios:**
  ```
  Scenario: 孤儿文件已删除
    Tool: Bash
    Steps:
      1. test ! -f crates/openlark-core/src/error/observability.rs && echo DELETED
      2. test -f crates/openlark-core/src/observability.rs && echo TOP_LEVEL_EXISTS
    Expected Result: 输出 DELETED 和 TOP_LEVEL_EXISTS
    Evidence: .sisyphus/evidence/task-3-orphan-deleted.txt
  ```

  **Commit**: YES
  - Message: `refactor(core): 删除孤儿 error/observability.rs`
  - Files: `crates/openlark-core/src/error/observability.rs`（git rm）
  - Pre-commit: `cargo check -p openlark-core`

- [x] 4. ErrorType From<ErrorCode> + is_retryable() 部分委托
  **What to do**:
  - 在 `crates/openlark-core/src/error/traits.rs` 中为 `ErrorType` 实现 `From<ErrorCode>`
  - 映射规则（基于 ErrorCode 的语义分组）：
    - Network 类: `NetworkTimeout`, `NetworkConnectionFailed`, `DnsResolutionFailed`, `SslCertificateError`, `ConnectionRefused` → `ErrorType::Network`
    - Auth 类: `Unauthorized`, `AccessTokenInvalid`, `AppAccessTokenInvalid`, `TenantAccessTokenInvalid`, `AuthenticationFailed`, `TokenExpired`, `PermissionDenied`, `AccessDenied` → `ErrorType::Authentication`
    - Validation 类: `BadRequest`, `ValidationError`, `MissingRequiredParameter`, `InvalidParameterFormat`, `ParameterOutOfRange` → `ErrorType::Validation`
    - Api 类: `NotFound`, `MethodNotAllowed`, `Conflict`, `Forbidden` → `ErrorType::Api`
    - RateLimit 类: `TooManyRequests`, `RateLimitExceeded` → `ErrorType::RateLimit`
    - Timeout 类: `GatewayTimeout` → `ErrorType::Timeout`
    - ServiceUnavailable 类: `ServiceUnavailable`, `BadGateway`, `CacheServiceUnavailable` → `ErrorType::ServiceUnavailable`
    - Config 类: `ConfigurationError` → `ErrorType::Configuration`
    - Success: `Success` → `ErrorType::Api`（成功不是错误，但需要兜底）
    - 其余: `_ => ErrorType::Internal`
  - 在 `crates/openlark-core/src/error/core.rs` 中修改 `CoreError::is_retryable()`：
    - **保留** Network 变体的现有行为：`Self::Network(net) => net.policy.is_retryable()`
    - **保留** Api 变体的现有行为：`Self::Api(api) => matches!(api.status, 429 | 500..=599)`
    - **委托** 简单变体给 ErrorCode：对 Timeout/RateLimit/ServiceUnavailable/Internal 使用 `self.code().is_retryable()`（注意：Timeout 变体无 code 字段，需通过 `self.code()` 获取 ErrorCode 再调用 `is_retryable()`，不要为 Timeout 增加字段）
  **Must NOT do**:
  - 不改变 Network 变体的 is_retryable 行为（使用 RetryPolicy）
  - 不改变 Api 变体的 is_retryable 行为（使用 status 匹配）
  - 不重构 ErrorTrait 或添加新方法
  - 不修改 severity()（已经正确委托）
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: `[]`
  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2 (sequential after Wave 1)
  - **Blocks**: F1-F4
  - **Blocked By**: Tasks 1, 2, 3（需要 clean compile 基础）
  **References**:
  - `crates/openlark-core/src/error/traits.rs:170-195` — ErrorType 枚举定义（11 个变体）
  - `crates/openlark-core/src/error/codes.rs:602-617` — ErrorCode::is_retryable() 实现
  - `crates/openlark-core/src/error/codes.rs:863-904` — ErrorCode::severity() 实现（From 映射参考）
  - `crates/openlark-core/src/error/core.rs:672-685` — CoreError::severity() 已委托 + is_retryable() 待修改
  - `crates/openlark-core/src/error/codes.rs:1-100` — ErrorCode 枚举变体定义
  **Acceptance Criteria**:
  - [ ] `From<ErrorCode> for ErrorType` 实现存在且覆盖所有 ErrorCode 变体
  - [ ] `CoreError::is_retryable()` 对 Timeout/RateLimit/ServiceUnavailable/Internal 委托给 `code.is_retryable()`
  - [ ] Network 变体仍使用 `net.policy.is_retryable()`
  - [ ] Api 变体仍使用 `matches!(api.status, 429 | 500..=599)`
  - [ ] `cargo test -p openlark-core` 全部通过（零回归）
  **QA Scenarios:**
  ```
  Scenario: From<ErrorCode> 实现存在
    Tool: Bash (grep)
    Steps:
      1. grep -n 'impl From<ErrorCode> for ErrorType' crates/openlark-core/src/error/traits.rs
    Expected Result: 至少一行匹配
    Evidence: .sisyphus/evidence/task-4-from-impl.txt
  Scenario: is_retryable Network/Api 行为保留
    Tool: Bash (grep)
    Steps:
      1. grep -A1 'Network(net)' crates/openlark-core/src/error/core.rs | grep 'policy.is_retryable'
      2. grep -A1 'Api(api)' crates/openlark-core/src/error/core.rs | grep 'matches.*429'
    Expected Result: 两个 grep 均有输出
    Evidence: .sisyphus/evidence/task-4-retryable-preserved.txt
  Scenario: 简单变体委托给 ErrorCode
    Tool: Bash (grep)
    Steps:
      1. grep -A1 'Timeout.*code' crates/openlark-core/src/error/core.rs | grep 'code.is_retryable'
    Expected Result: 有输出，确认委托
    Evidence: .sisyphus/evidence/task-4-delegation.txt
  ```
  **Commit**: YES
  - Message: `fix(core): ErrorType From<ErrorCode> + is_retryable 部分委托`
  - Files: `crates/openlark-core/src/error/traits.rs`, `crates/openlark-core/src/error/core.rs`
  - Pre-commit: `cargo check -p openlark-core && cargo test -p openlark-core`
---

## Final Verification Wave

- [x] F1. **Workspace 编译检查** — `quick`
  运行 `cargo check --workspace --all-features`，确认零错误。

- [x] F2. **Clippy 零警告** — `quick`
  运行 `cargo clippy -p openlark-core -- -D warnings`，确认零警告。

- [x] F3. **格式检查** — `quick`
  运行 `cargo fmt -p openlark-core -- --check`，确认无格式问题。

- [x] F4. **残留检查** — `quick`
  - `grep -r "RequestData::Empty" crates/openlark-core/src/` 返回空
  - `test ! -f crates/openlark-core/src/error/observability.rs` 通过
  - 确认 `ApiRequest` 结构体无 `pub` 字段（仅 `pub(crate)`）

---

## Commit Strategy

- **Commit 1**: `fix(core): 收紧 ApiRequest 字段可见性为 pub(crate)` — api/mod.rs
- **Commit 2**: `fix(core): 移除 RequestData::Empty，序列化失败 warn + Json(Null)` — api/mod.rs
- **Commit 3**: `refactor(core): 删除孤儿 error/observability.rs` — error/observability.rs
- **Commit 4**: `fix(core): ErrorType From<ErrorCode> + is_retryable 部分委托` — error/traits.rs, error/core.rs

⚠️ **关键**: 每次 commit 前只 `git add` 指定文件，**绝对不要使用 `git add -A`**（这是上次回退的根因）。

---

## Success Criteria

### Verification Commands
```bash
cargo check -p openlark-core          # Expected: 零错误
cargo test -p openlark-core           # Expected: 全部通过
cargo clippy -p openlark-core -- -D warnings  # Expected: 零警告
cargo fmt -p openlark-core -- --check # Expected: 零格式问题
cargo check --workspace --all-features # Expected: 零错误
grep -r "RequestData::Empty" crates/openlark-core/src/  # Expected: 无输出
test ! -f crates/openlark-core/src/error/observability.rs  # Expected: 通过
```

### Final Checklist
- [x] All "Must Have" present
- [x] All "Must NOT Have" absent
- [x] All tests pass
- [x] 无 `git add -A` 使用
