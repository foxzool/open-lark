# Plan 2: Feature Flag 清理与依赖优化 (Issue B + D)

## TL;DR

> **Quick Summary**: 将 openlark-auth 改为 optional 依赖，清理空/废弃 feature flags，统一 cfg 检查风格。
> 
> **Deliverables**:
> - openlark-auth 在 client 中变为 optional 依赖
> - 删除 9 个空/废弃 feature flags
> - 统一 root crate 的 cfg 检查风格
> - 补充 feature 组合编译测试
> 
> **Estimated Effort**: Short
> **Parallel Execution**: YES - 3 waves
> **Critical Path**: Task 1 → Task 2 → Task 4 → Task 5
> **前置依赖**: Plan 1 (core-auth-boundary) 必须先完成

---

## Context

### Original Request
openlark-auth 是 client 的非 optional 依赖，即使用户只需要 docs 也会编译 auth。同时存在大量空 feature flags 和不一致的 cfg 风格。

### 前置条件
Plan 1 完成后：
- core auth 模块已瘦身（仅 TokenProvider trait）
- client Config 已包装 core Config
- AuthTokenProvider 注入逻辑已在 client 中

### 关键发现
- `openlark-auth = { workspace = true }` 无 `optional = true`（client Cargo.toml 第 32 行）
- `auth = []` feature 是空的，不控制编译
- 6 个空 feature：calendar, admin, approval, helpdesk, mail, application
- 3 个废弃 feature：collab, people, hire
- root src/lib.rs 混用 `#[cfg(feature = "openlark-client")]` 和 `#[cfg(feature = "workflow")]` 风格

---

## Work Objectives

### Core Objective
让 feature flags 真正控制编译，清理无效 features，统一代码风格。

### Concrete Deliverables
- `crates/openlark-client/Cargo.toml` — auth 变为 optional
- `crates/openlark-client/src/core_config.rs`（或其替代逻辑）— cfg(feature="auth") 条件编译
- `crates/openlark-client/Cargo.toml` — 删除空/废弃 features
- `src/lib.rs` — 统一 cfg 风格

### Definition of Done
- [x] `cargo check -p openlark-client --no-default-features` 编译通过（无 auth）
- [x] `cargo check -p openlark-client --features auth` 编译通过（有 auth）
- [x] 无空 feature flags 存在
- [x] root src/lib.rs 中 cfg 风格统一

### Must Have
- auth 关闭时，Client 仍可构建（使用 NoOpTokenProvider）
- auth 开启时，行为与当前完全一致
- default features 保持 `["auth", "communication"]`

### Must NOT Have (Guardrails)
- ❌ 不修改业务 crate 内部实现
- ❌ 不改变 default features 的行为（用户无感）
- ❌ 不删除有实际功能的 feature（如 communication, docs, hr）
- ❌ 不修改 core crate

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed.

### Test Decision
- **Infrastructure exists**: YES
- **Automated tests**: Tests-after
- **Framework**: cargo test + cargo check with various feature combinations

### QA Policy
- **Library/Module**: cargo check/test with different feature flag combinations

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — auth optional 化):
├── Task 1: 审计 auth feature 的所有 cfg 使用点 [quick]
├── Task 2: 将 openlark-auth 改为 optional + 条件编译 [deep]

Wave 2 (After Wave 1 — feature 清理):
├── Task 3: 删除空/废弃 feature flags [quick]
├── Task 4: 统一 root crate cfg 风格 [quick]

Wave 3 (After Wave 2 — 验证):
├── Task 5: Feature 组合编译测试 [unspecified-high]

Wave FINAL:
├── Task F1: Plan compliance audit [oracle]
├── Task F2: Code quality review [unspecified-high]
├── Task F3: Scope fidelity check [deep]

Critical Path: Task 1 → Task 2 → Task 4 → Task 5 → F1-F3
Max Concurrent: 2
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|-----------|--------|
| 1 | — | 2 |
| 2 | 1 | 3, 4, 5 |
| 3 | 2 | 5 |
| 4 | 2 | 5 |
| 5 | 3, 4 | F1-F3 |

### Agent Dispatch Summary

- **Wave 1**: T1 → `quick`, T2 → `deep`
- **Wave 2**: T3 → `quick`, T4 → `quick`
- **Wave 3**: T5 → `unspecified-high`
- **FINAL**: F1 → `oracle`, F2 → `unspecified-high`, F3 → `deep`

---

## TODOs

- [x] 1. 审计 auth feature 的所有 cfg 使用点
  **What to do**:
  - 搜索 `openlark-client` 中所有 `#[cfg(feature = "auth")]` 使用点
  - 搜索 `openlark-auth` 在 client 代码中的直接引用（`use openlark_auth::`）
  - 记录 `AuthTokenProvider` 的使用位置（当前在 core_config.rs 或其替代逻辑中）
  - 记录 `AuthClient` 的定义和使用位置
  - 输出到 `.sisyphus/evidence/task-1-auth-cfg-audit.md`
  **Must NOT do**: 不修改任何代码
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Blocks**: Task 2
  - **Blocked By**: None（但需要 Plan 1 已完成）
  **References**:
  - `crates/openlark-client/src/client.rs:15-32` — AuthClient 定义和 `#[cfg(feature = "auth")]`
  - `crates/openlark-client/Cargo.toml:32` — `openlark-auth = { workspace = true }` 非 optional
  - `crates/openlark-client/Cargo.toml:51` — `auth = []` 空 feature
  **Acceptance Criteria**:
  - [ ] 审计清单文件存在
  - [ ] 清单包含所有 cfg(feature="auth") 位置
  **QA Scenarios:**
  ```
  Scenario: 审计完整性
    Tool: Bash (grep)
    Steps:
      1. grep -rn 'cfg.*feature.*auth\|use openlark_auth' crates/openlark-client/src/ --include="*.rs"
      2. 对比清单
    Expected Result: 清单覆盖所有引用
    Evidence: .sisyphus/evidence/task-1-auth-cfg-audit.md
  ```
  **Commit**: NO

- [x] 2. 将 openlark-auth 改为 optional + 条件编译
  **What to do**:
  - 修改 `crates/openlark-client/Cargo.toml`:
    - `openlark-auth = { workspace = true, optional = true }`
    - `auth = ["openlark-auth"]`（不再是空数组）
  - 修改 client 构建逻辑（client.rs 中 Client::with_config）:
    - `#[cfg(feature = "auth")]` 包裹 AuthTokenProvider 注入
    - 无 auth 时使用 NoOpTokenProvider（core 默认行为）
  - 确保 `AuthClient` 已有 `#[cfg(feature = "auth")]`（当前已有）
  - 确保 lib.rs 中 `pub use client::AuthClient` 已有 cfg gate（当前已有）
  - 验证 default features 仍包含 auth
  **Must NOT do**:
  - 不改变 default features 行为
  - 不修改 core crate
  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: [`openlark-code-standards`]
  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Blocks**: Task 3, 4, 5
  - **Blocked By**: Task 1
  **References**:
  - `crates/openlark-client/Cargo.toml:32` — 当前非 optional 依赖
  - `crates/openlark-client/Cargo.toml:45` — default features
  - `crates/openlark-client/Cargo.toml:51` — 空 auth feature
  - `crates/openlark-client/src/client.rs:161-162` — AuthClient 构建（已有 cfg gate）
  - `.sisyphus/evidence/task-1-auth-cfg-audit.md` — Task 1 产出
  **Acceptance Criteria**:
  - [ ] `cargo check -p openlark-client --no-default-features` 通过（无 auth）
  - [ ] `cargo check -p openlark-client --features auth` 通过（有 auth）
  - [ ] default features 行为不变
  **QA Scenarios:**
  ```
  Scenario: 无 auth 编译
    Tool: Bash (cargo check)
    Steps:
      1. cargo check -p openlark-client --no-default-features
    Expected Result: 编译通过，0 errors
    Evidence: .sisyphus/evidence/task-2-no-auth.txt
  Scenario: 有 auth 编译
    Tool: Bash (cargo check)
    Steps:
      1. cargo check -p openlark-client --features auth
    Expected Result: 编译通过，0 errors
    Evidence: .sisyphus/evidence/task-2-with-auth.txt
  Scenario: 默认 features 编译
    Tool: Bash (cargo check)
    Steps:
      1. cargo check -p openlark-client
    Expected Result: 编译通过（default 包含 auth）
    Evidence: .sisyphus/evidence/task-2-default.txt
  ```
  **Commit**: YES
  - Message: `refactor(client): 将 openlark-auth 改为 optional 依赖`
  - Files: `crates/openlark-client/Cargo.toml`, `crates/openlark-client/src/client.rs`
  - Pre-commit: `cargo check -p openlark-client --no-default-features && cargo check -p openlark-client`


- [x] 3. 删除空/废弃 feature flags
  **What to do**:
  - 从 `crates/openlark-client/Cargo.toml` 删除空 features：
    - `calendar = []`, `admin = []`, `approval = []`
    - `helpdesk = []`, `mail = []`, `application = []`
  - 删除废弃 legacy features：
    - `collab = []`, `people = []`, `hire = []`
  - 搜索代码中是否有 `#[cfg(feature = "calendar")]` 等引用，如有则一并删除
  - 检查 root Cargo.toml 是否引用了这些 features，如有则清理
  **Must NOT do**:
  - 不删除有实际功能的 feature（communication, docs, hr, security 等）
  - 不删除 `client = []`（虽然空但被其他 feature 引用）
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 4)
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 5
  - **Blocked By**: Task 2
  **References**:
  - `crates/openlark-client/Cargo.toml:69-81` — 空 features 和 legacy features
  **Acceptance Criteria**:
  - [ ] 9 个空/废弃 features 已删除
  - [ ] `cargo check -p openlark-client --all-features` 通过
  **QA Scenarios:**
  ```
  Scenario: 空 features 已清除
    Tool: Bash (grep)
    Steps:
      1. grep -E '^(calendar|admin|approval|helpdesk|mail|application|collab|people|hire)' crates/openlark-client/Cargo.toml
    Expected Result: 无匹配（exit code 1）
    Evidence: .sisyphus/evidence/task-3-empty-features.txt
  ```
  **Commit**: YES (groups with Task 4)
  - Message: `chore(client): 删除空/废弃 feature flags`
  - Files: `crates/openlark-client/Cargo.toml`
  - Pre-commit: `cargo check -p openlark-client --all-features`
- [x] 4. 统一 root crate cfg 风格
  **What to do**:
  - 修改 `src/lib.rs`：统一所有 `#[cfg(feature = "...")]` 使用短 feature 名
    - `#[cfg(feature = "openlark-client")]` → `#[cfg(feature = "client")]`
    - `#[cfg(feature = "openlark-auth")]` → `#[cfg(feature = "auth")]`
    - `#[cfg(feature = "openlark-protocol")]` → `#[cfg(feature = "protocol")]`
    - 其他已使用短名的保持不变（workflow, platform, analytics, user）
  - 验证 root Cargo.toml 中 feature 定义与 cfg 检查一致
  **Must NOT do**:
  - 不修改 feature 的实际行为
  - 不修改 Cargo.toml 中的 feature 定义
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 3)
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 5
  - **Blocked By**: Task 2
  **References**:
  - `src/lib.rs:8-54` — 当前 cfg 检查（混用 dep 名和 feature 名）
  - `Cargo.toml:240-290` — root feature 定义
  **Acceptance Criteria**:
  - [ ] src/lib.rs 中所有 cfg 使用短 feature 名
  - [ ] `cargo check --all-features` 通过
  **QA Scenarios:**
  ```
  Scenario: cfg 风格统一
    Tool: Bash (grep)
    Steps:
      1. grep -n 'cfg.*feature.*openlark-' src/lib.rs
    Expected Result: 无匹配（所有都用短名）
    Evidence: .sisyphus/evidence/task-4-cfg-style.txt
  ```
  **Commit**: YES (groups with Task 3)
  - Message: `style(root): 统一 cfg feature 检查为短名风格`
  - Files: `src/lib.rs`
  - Pre-commit: `cargo check --all-features`
- [x] 5. Feature 组合编译测试
  **What to do**:
  - 创建 CI 风格的 feature 组合验证脚本或在测试中验证：
    - `--no-default-features`（最小编译）
    - `--features auth`（仅 auth）
    - `--features communication`（仅 communication）
    - `--features docs`（仅 docs）
    - `--features "auth,communication"`（default 组合）
    - `--all-features`（全量）
  - 对 root crate 也做同样的 feature 组合检查
  - 记录每个组合的编译结果
  **Must NOT do**: 不修改生产代码
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Blocks**: F1-F3
  - **Blocked By**: Task 3, Task 4
  **References**:
  - `crates/openlark-client/Cargo.toml` — feature 定义
  - `Cargo.toml:240-290` — root feature 定义
  **Acceptance Criteria**:
  - [ ] 所有 6 个 feature 组合编译通过
  - [ ] root crate 的 feature 组合也通过
  **QA Scenarios:**
  ```
  Scenario: Feature 组合编译
    Tool: Bash (cargo check)
    Steps:
      1. cargo check -p openlark-client --no-default-features
      2. cargo check -p openlark-client --features auth
      3. cargo check -p openlark-client --features communication
      4. cargo check -p openlark-client --features docs
      5. cargo check -p openlark-client --features "auth,communication"
      6. cargo check -p openlark-client --all-features
      7. cargo check --no-default-features
      8. cargo check --all-features
    Expected Result: 全部 8 个组合编译通过
    Evidence: .sisyphus/evidence/task-5-feature-matrix.txt
  ```
  **Commit**: YES
  - Message: `test(client): 验证 feature 组合编译正确性`
  - Pre-commit: `cargo check --workspace --all-features`
## Final Verification Wave (MANDATORY)

- [x] F1. **Plan Compliance Audit** — `oracle`
  Verify all "Must Have" and "Must NOT Have". Check feature combinations compile correctly.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | VERDICT`

- [x] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo check --workspace --all-features` + `cargo clippy` + `cargo test --workspace`.
  Output: `Build [PASS/FAIL] | Clippy [PASS/FAIL] | Tests [N/N] | VERDICT`

- [x] F3. **Scope Fidelity Check** — `deep`
  Verify each task's diff matches spec. No unaccounted changes.
  Output: `Tasks [N/N compliant] | VERDICT`

---

## Commit Strategy

- **Wave 1**: `refactor(client): 将 openlark-auth 改为 optional 依赖，支持无 auth 编译`
- **Wave 2**: `chore(client): 删除空/废弃 feature flags，统一 cfg 风格`
- **Wave 3**: `test(client): 补充 feature 组合编译验证测试`

---

## Success Criteria

### Verification Commands
```bash
cargo check -p openlark-client --no-default-features          # Expected: pass (no auth)
cargo check -p openlark-client --features auth                 # Expected: pass (with auth)
cargo check -p openlark-client --features "auth,communication" # Expected: pass (default combo)
cargo check --workspace --all-features                         # Expected: pass
cargo test --workspace                                         # Expected: all pass
```

### Final Checklist
- [x] All "Must Have" present
- [x] All "Must NOT Have" absent
- [x] All tests pass
