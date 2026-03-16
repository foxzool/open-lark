# 代码组织最佳实践整改计划

## TL;DR

> **Quick Summary**: 清理 5 个核心 crate 的代码组织问题（2 P0 + 5 P1），包括重复注释、lint 抑制、导出冲突、文件过大、冗余别名和 trait 位置不当。
> 
> **Deliverables**:
> - openlark-core lib.rs 清理（重复注释 + Validatable 移位）
> - openlark-client lib.rs 瘦身（lint 清理 + utils 拆分 + 别名清理 + prelude 修正）
> - openlark-docs/meeting glob re-export 冲突修复
> 
> **Estimated Effort**: Medium
> **Parallel Execution**: YES - 3 waves
> **Critical Path**: Task 1 → Task 5 → Task 7 → Final Verification

---

## Context

### Original Request
检查 openlark-core、openlark-client、openlark-auth、openlark-communication、openlark-docs 的代码组织是否符合最佳实践，并生成整改工作计划。

### Interview Summary
**Key Discussions**:
- 用户选择聚焦 5 个特定模块进行审计
- 审计发现 10 个问题（2 P0 + 5 P1 + 3 P2）
- 用户选择为所有 P0/P1 问题生成整改工作计划

**Research Findings**:
- `validate_required!` 宏统一使用（960处/629文件）✅
- `execute_with_options` 模式广泛采用（3168处/1511文件）✅
- openlark-core 模块可见性控制良好 ✅
- 业务 crate 按 bizTag/project/version/resource 组织清晰 ✅

### Metis Review
**Identified Gaps** (addressed):
- P0-2 lint 抑制数量需验证（crate 级 vs 局部级）
- P1-7 Validatable 移动必须保留 crate root re-export（宏路径 `$crate::Validatable`）
- P1-6 类型别名需同步删除模块级和 prelude 内两处（共 6 处）
- P1-3 glob 修复范围需严格限定（只修 warning，不重写全部 140+ glob）
- `#[allow(unused_mut)]` 是合理使用，不应删除
- `SDKResult` 是有意义的 re-export，不应删除

---

## Work Objectives

### Core Objective
在不改变任何公共 API 行为的前提下，清理 5 个核心 crate 的代码组织问题，提升代码质量信号和可维护性。

### Concrete Deliverables
- openlark-core/src/lib.rs 无重复注释，Validatable 移入 validation 模块
- openlark-client/src/lib.rs < 800 行，无冗余 lint 抑制
- openlark-docs/openlark-meeting 无 ambiguous_glob_reexports 抑制

### Definition of Done
- [x] `cargo check --workspace --all-features` 零新增警告
- [x] `cargo test --workspace` 全部通过
- [x] 所有 P0/P1 问题已修复

### Must Have
- 所有 7 个 P0/P1 问题得到修复
- 每个修复后 cargo check + cargo test 通过
- 公共 API 行为不变

### Must NOT Have (Guardrails)
- ❌ 不修改 `validate_required!` 宏定义本身
- ❌ 不删除 `SDKResult` re-export
- ❌ 不删除 `#[allow(unused_mut)]`（feature 条件编译场景合理）
- ❌ 不重写 openlark-docs 中所有 140+ glob re-export（只修 warning 触发的）
- ❌ 不重构 registry/dependency_resolver 模块（只清理 allow 注解或标记 TODO）
- ❌ 不新增任何功能代码
- ❌ 不改变 prelude 中除 HashMap/Duration/冗余别名外的任何导出

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: YES
- **Automated tests**: Tests-after（验证不回归）
- **Framework**: cargo test
- **Strategy**: 每个任务完成后运行 cargo check + cargo test

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Library/Module**: Use Bash (cargo check/test) — 编译检查、测试运行、grep 验证

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — 无依赖的简单修复):
├── Task 1: P0-1 openlark-core 重复注释清理 [quick]
├── Task 2: P1-4 openlark-client prelude 清理 [quick]
├── Task 3: P1-6 openlark-client 冗余类型别名清理 [quick]
└── Task 4: P0-2 openlark-client lint 抑制审计与清理 [unspecified-high]

Wave 2 (After Wave 1 — 需要文件移动/拆分):
├── Task 5: P1-7 openlark-core Validatable trait 移位 [deep]
├── Task 6: P1-5 openlark-client lib.rs 拆分 [unspecified-high]
└── Task 7: P1-3 openlark-docs/meeting glob re-export 修复 [unspecified-high]

Wave FINAL (After ALL tasks — 独立审查):
├── Task F1: 计划合规审计 (oracle)
├── Task F2: 代码质量审查 (unspecified-high)
└── Task F3: 范围保真检查 (deep)

Critical Path: Task 1 → Task 5 → Task 7 → F1-F3
Parallel Speedup: ~50% faster than sequential
Max Concurrent: 4 (Wave 1)
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|-----------|--------|
| 1 | — | 5 |
| 2 | — | 6 |
| 3 | — | 6 |
| 4 | — | 6 |
| 5 | 1 | F1-F3 |
| 6 | 2, 3, 4 | F1-F3 |
| 7 | — | F1-F3 |

### Agent Dispatch Summary

- **Wave 1**: 4 tasks — T1→`quick`, T2→`quick`, T3→`quick`, T4→`unspecified-high`
- **Wave 2**: 3 tasks — T5→`deep`, T6→`unspecified-high`, T7→`unspecified-high`
- **FINAL**: 3 tasks — F1→`oracle`, F2→`unspecified-high`, F3→`deep`

---

## TODOs

- [x] 1. P0-1 openlark-core 重复文档注释清理

  **What to do**:
  - 删除 `crates/openlark-core/src/lib.rs` 中的重复行：
    - 删除 lines 7-9（与 lines 1-4 重复的 doc comment）
    - 删除 line 12（与 line 6 重复的注释）
    - 删除 lines 10-11（多余空行）
  - 运行 `cargo doc -p openlark-core --no-deps` 验证文档输出正确

  **Must NOT do**:
  - 不修改任何非重复的注释或代码
  - 不改变模块声明顺序

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 单文件、< 10 行变更、无逻辑复杂度
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3, 4)
  - **Blocks**: Task 5（Validatable 移位依赖 lib.rs 先清理）
  - **Blocked By**: None

  **References**:
  - `crates/openlark-core/src/lib.rs:1-12` — 重复注释所在区域
  - `crates/openlark-core/AGENTS.md` — 公开 API 设计原则

  **Acceptance Criteria**:
  - [x] `grep -c "This crate provides the core infrastructure" crates/openlark-core/src/lib.rs` → 1（不是 2）
  - [x] `grep -c "对外稳定导出" crates/openlark-core/src/lib.rs` → 1（不是 2）
  - [x] `cargo check -p openlark-core` → 零错误
  - [x] `cargo test -p openlark-core` → 全部通过

  **QA Scenarios:**
  ```
  Scenario: 重复注释已删除
    Tool: Bash (grep)
    Steps:
      1. grep -c "This crate provides the core infrastructure" crates/openlark-core/src/lib.rs
      2. Assert output is exactly "1"
      3. grep -c "对外稳定导出" crates/openlark-core/src/lib.rs
      4. Assert output is exactly "1"
    Expected Result: 每个模式只出现 1 次
    Evidence: .sisyphus/evidence/task-1-duplicate-comments.txt

  Scenario: 编译和测试通过
    Tool: Bash (cargo)
    Steps:
      1. cargo check -p openlark-core 2>&1
      2. Assert exit code 0, no errors
      3. cargo test -p openlark-core 2>&1
      4. Assert all tests pass
    Expected Result: 零错误零失败
    Evidence: .sisyphus/evidence/task-1-build-test.txt
  ```

  **Commit**: YES
  - Message: `refactor(core): 清理 lib.rs 重复文档注释`
  - Files: `crates/openlark-core/src/lib.rs`
  - Pre-commit: `cargo check -p openlark-core`

- [x] 2. P1-4 openlark-client prelude 导出清理

  **What to do**:
  - 从 `crates/openlark-client/src/lib.rs` 的 prelude 模块中移除：
    - `pub use std::collections::HashMap;`（line 453）
    - `pub use std::time::Duration;`（line 454）
  - 这两个 std 类型不应通过 prelude 导出（违反 openlark-core 约定）

  **Must NOT do**:
  - 不移除 prelude 中的其他导出
  - 不修改 prelude 外的代码

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 单文件、2 行删除、无逻辑复杂度
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3, 4)
  - **Blocks**: Task 6（lib.rs 拆分依赖 prelude 先清理）
  - **Blocked By**: None

  **References**:
  - `crates/openlark-client/src/lib.rs:452-454` — prelude 中的 std 类型导出
  - `crates/openlark-core/AGENTS.md` — "不 re-export 第三方类型（serde/HashMap 等）"

  **Acceptance Criteria**:
  - [x] `grep -c "pub use std::collections::HashMap" crates/openlark-client/src/lib.rs` → 0
  - [x] `grep -c "pub use std::time::Duration" crates/openlark-client/src/lib.rs` → 0（prelude 内）
  - [x] `cargo check -p openlark-client --all-features` → 零错误
  - [x] `cargo test -p openlark-client --all-features` → 全部通过

  **QA Scenarios:**
  ```
  Scenario: std 类型已从 prelude 移除
    Tool: Bash (grep)
    Steps:
      1. grep "pub use std::collections::HashMap" crates/openlark-client/src/lib.rs
      2. Assert no output (exit code 1)
      3. grep "pub use std::time::Duration" crates/openlark-client/src/lib.rs
      4. Assert: 仅在 prelude 外的 import 中出现（如有），prelude 内无此导出
    Expected Result: prelude 模块内无 std 类型导出
    Evidence: .sisyphus/evidence/task-2-prelude-cleanup.txt

  Scenario: 编译和测试通过
    Tool: Bash (cargo)
    Steps:
      1. cargo check -p openlark-client --all-features 2>&1
      2. cargo check -p openlark-client --no-default-features 2>&1
      3. cargo test -p openlark-client --all-features 2>&1
      4. Assert all pass with zero errors
    Expected Result: 所有 feature 组合编译通过
    Evidence: .sisyphus/evidence/task-2-build-test.txt
  ```

  **Commit**: YES (groups with Task 3)
  - Message: `refactor(client): 清理 prelude 导出和冗余类型别名`
  - Files: `crates/openlark-client/src/lib.rs`
  - Pre-commit: `cargo check -p openlark-client --all-features`

- [x] 3. P1-6 openlark-client 冗余类型别名清理
  **What to do**:
  - 删除 `crates/openlark-client/src/lib.rs` 中 6 处冗余类型别名（模块级 + prelude 内各 3 处）：
    - 模块级：`ClientResult<T>`(line 327), `ServiceResult<T>`(line 333), `ConfigResult<T>`(line 336)
    - prelude 内：`ClientResult<T>`(line 434), `ServiceResult<T>`(line 440), `ConfigResult<T>`(line 443)
  - 保留 `SDKResult<T>` — 它是有意义的 re-export，不是冗余别名
  **Must NOT do**:
  - 不删除 `SDKResult<T>` re-export
  - 不修改 `Result<T>` 或 `Error` 类型定义
  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 单文件、6 行删除、无逻辑复杂度
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 4)
  - **Blocks**: Task 6
  - **Blocked By**: None
  **References**:
  - `crates/openlark-client/src/lib.rs:327-336` — 模块级类型别名
  - `crates/openlark-client/src/lib.rs:434-443` — prelude 内重复定义
  **Acceptance Criteria**:
  - [x] `grep -c "ClientResult" crates/openlark-client/src/lib.rs` → 0
  - [x] `grep -c "ServiceResult" crates/openlark-client/src/lib.rs` → 0
  - [x] `grep -c "ConfigResult" crates/openlark-client/src/lib.rs` → 0
  - [x] `grep -c "SDKResult" crates/openlark-client/src/lib.rs` → 保留（≥1）
  - [x] `cargo check -p openlark-client --all-features` → 零错误
  **QA Scenarios:**
  ```
  Scenario: 冗余别名已删除，SDKResult 保留
    Tool: Bash (grep)
    Steps:
      1. grep "ClientResult\|ServiceResult\|ConfigResult" crates/openlark-client/src/lib.rs
      2. Assert no output
      3. grep "SDKResult" crates/openlark-client/src/lib.rs
      4. Assert has output (SDKResult preserved)
    Expected Result: 3 个冗余别名消失，SDKResult 保留
    Evidence: .sisyphus/evidence/task-3-alias-cleanup.txt
  ```
  **Commit**: YES (groups with Task 2)
  - Message: `refactor(client): 清理 prelude 导出和冗余类型别名`
  - Files: `crates/openlark-client/src/lib.rs`

- [x] 4. P0-2 openlark-client lint 抑制审计与清理
  **What to do**:
  - 审计 `crates/openlark-client/src/lib.rs` 中的 crate 级 `#![allow(...)]`：
    - `#![allow(dead_code)]` (line 212) — 尝试移除，清理触发的 dead code
    - `#![allow(unused_imports)]` (line 213) — 尝试移除，清理未使用 import
    - `#![allow(unused_variables)]` (line 214) — 尝试移除，清理未使用变量
    - `#![allow(missing_copy_implementations)]` (line 215) — 评估是否仍需要
    - `#![allow(missing_debug_implementations)]` (line 216) — 评估是否仍需要
    - `#![allow(async_fn_in_trait)]` (line 217) — 保留（Rust 版本兼容）
    - `#![allow(mismatched_lifetime_syntaxes)]` (line 218) — 评估是否仍需要
  - 保留 `#![allow(unexpected_cfgs)]` (line 8) — 合理使用
  - 审计局部 `#[allow(...)]`：保留 `#[allow(unused_mut)]` (合理的 feature 条件编译)
  **Must NOT do**:
  - 不删除 `#![allow(unexpected_cfgs)]`
  - 不删除 `#[allow(unused_mut)]`（feature 条件编译场景合理）
  - 不重构 registry/dependency_resolver 模块（只清理 allow 注解或标记 TODO）
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 需要逐个评估 allow 并清理触发的代码，涉及多处修改
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3)
  - **Blocks**: Task 6
  - **Blocked By**: None
  **References**:
  - `crates/openlark-client/src/lib.rs:8,212-218` — crate 级 lint 抑制
  - `crates/openlark-client/src/registry/dependency_resolver.rs:46,97` — 局部 allow（dead_code + clippy::only_used_in_recursion）
  **Acceptance Criteria**:
  - [x] `#![allow(dead_code)]` 已移除或有明确理由保留
  - [x] `#![allow(unused_imports)]` 已移除
  - [x] `#![allow(unused_variables)]` 已移除
  - [x] `cargo check -p openlark-client --all-features` → 零错误零新增警告
  - [x] `cargo test -p openlark-client --all-features` → 全部通过
  **QA Scenarios:**
  ```
  Scenario: lint 抑制已清理
    Tool: Bash (grep)
    Steps:
      1. grep -n "#!\[allow(dead_code)\]" crates/openlark-client/src/lib.rs
      2. grep -n "#!\[allow(unused_imports)\]" crates/openlark-client/src/lib.rs
      3. grep -n "#!\[allow(unused_variables)\]" crates/openlark-client/src/lib.rs
      4. Assert: 以上 3 个 crate 级 allow 已移除
    Expected Result: dead_code/unused_imports/unused_variables 的 crate 级 allow 不存在
    Evidence: .sisyphus/evidence/task-4-lint-cleanup.txt
  Scenario: 编译无新增警告
    Tool: Bash (cargo)
    Steps:
      1. cargo check -p openlark-client --all-features 2>&1
      2. Assert zero errors and no new warnings
      3. cargo test -p openlark-client --all-features 2>&1
      4. Assert all tests pass
    Expected Result: 编译通过，测试通过
    Evidence: .sisyphus/evidence/task-4-build-test.txt
  ```
  **Commit**: YES
  - Message: `refactor(client): 清理不必要的 lint 抑制`
  - Files: `crates/openlark-client/src/lib.rs`, `crates/openlark-client/src/*.rs`
  - Pre-commit: `cargo check -p openlark-client --all-features`

- [x] 5. P1-7 openlark-core Validatable trait 移位
  **What to do**:
  - 将 `crates/openlark-core/src/lib.rs:40-72` 中的 Validatable trait 及 5 个 impl 移动到 `src/validation/` 模块
  - 在 `src/validation/mod.rs` 中添加 `pub mod validatable;`（或直接放入 `core.rs`）
  - 在 `lib.rs` 中保留 re-export：`pub use validation::Validatable;`（关键！宏路径依赖）
  - 验证 `validate_required!` 宏的 `$crate::Validatable` 路径仍然有效
  **Must NOT do**:
  - 不修改 `validate_required!` 宏定义本身
  - 不修改宏路径（`$crate::Validatable` 必须继续工作）
  - 不在 validation 模块中新增任何代码
  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: 涉及跨模块移动 + 宏路径验证，影响 960 处调用
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES（与 Task 6, 7 并行）
  - **Parallel Group**: Wave 2
  - **Blocks**: Final Verification
  - **Blocked By**: Task 1（lib.rs 先清理重复注释）
  **References**:
  - `crates/openlark-core/src/lib.rs:40-72` — Validatable trait 定义
  - `crates/openlark-core/src/lib.rs:76-82` — validate_required! 宏（使用 `$crate::Validatable`）
  - `crates/openlark-core/src/validation/mod.rs` — 目标模块
  - `crates/openlark-core/src/validation/core.rs` — 现有验证逻辑（参考组织方式）
  **Acceptance Criteria**:
  - [x] Validatable trait 不在 lib.rs 中定义（仅 re-export）
  - [x] `cargo check --workspace --all-features` → 零错误
  - [x] `cargo test --workspace` → 全部通过（验证 960 处宏调用仍工作）
  **QA Scenarios:**
  ```
  Scenario: Validatable 已移动且宏路径有效
    Tool: Bash (grep + cargo)
    Steps:
      1. grep -n "pub trait Validatable" crates/openlark-core/src/lib.rs
      2. Assert no output（trait 定义不在 lib.rs）
      3. grep -n "pub use validation::Validatable" crates/openlark-core/src/lib.rs
      4. Assert has output（re-export 存在）
      5. cargo check --workspace --all-features 2>&1
      6. Assert zero errors
    Expected Result: trait 已移动，re-export 存在，全工作区编译通过
    Evidence: .sisyphus/evidence/task-5-validatable-move.txt
  Scenario: 全工作区测试通过
    Tool: Bash (cargo test)
    Steps:
      1. cargo test --workspace 2>&1
      2. Assert all tests pass
    Expected Result: 960 处 validate_required! 调用全部正常
    Evidence: .sisyphus/evidence/task-5-workspace-test.txt
  ```
  **Commit**: YES
  - Message: `refactor(core): 移动 Validatable trait 到 validation 模块`
  - Files: `crates/openlark-core/src/lib.rs`, `crates/openlark-core/src/validation/`
  - Pre-commit: `cargo check --workspace --all-features`
- [x] 6. P1-5 openlark-client lib.rs 拆分
  **What to do**:
  - 将 `crates/openlark-client/src/lib.rs` 中的 `utils` 模块（~200行代码 + ConfigSummary/SystemDiagnostics 等类型）拆分到 `src/utils.rs`
  - 将 `utils` 模块内的测试移到 `tests/utils_tests.rs` 或保留在 `utils.rs` 底部
  - 在 lib.rs 中改为 `pub mod utils;`
  - 目标：lib.rs < 800 行
  **Must NOT do**:
  - 不重组 prelude/error 模块
  - 不修改 utils 模块的公共 API
  - 不移动 info 模块（太小，不值得拆分）
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 文件拆分涉及 import 路径调整和测试迁移
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES（与 Task 5, 7 并行）
  - **Parallel Group**: Wave 2
  - **Blocks**: Final Verification
  - **Blocked By**: Tasks 2, 3, 4（prelude/别名/lint 先清理）
  **References**:
  - `crates/openlark-client/src/lib.rs:470-859` — utils 模块（含 ConfigSummary, SystemDiagnostics, check_env_config 等）
  - `crates/openlark-client/src/lib.rs:471` — `use super::*` 需改为显式 import
  **Acceptance Criteria**:
  - [x] `wc -l crates/openlark-client/src/lib.rs` → < 800
  - [x] `test -f crates/openlark-client/src/utils.rs` → 文件存在
  - [x] `cargo check -p openlark-client --all-features` → 零错误
  - [x] `cargo test -p openlark-client --all-features` → 全部通过
  **QA Scenarios:**
  ```
  Scenario: lib.rs 已瘦身
    Tool: Bash (wc)
    Steps:
      1. wc -l crates/openlark-client/src/lib.rs
      2. Assert line count < 800
      3. test -f crates/openlark-client/src/utils.rs
      4. Assert file exists
    Expected Result: lib.rs < 800 行，utils.rs 存在
    Evidence: .sisyphus/evidence/task-6-lib-split.txt
  Scenario: 编译和测试通过
    Tool: Bash (cargo)
    Steps:
      1. cargo check -p openlark-client --all-features 2>&1
      2. cargo check -p openlark-client --no-default-features 2>&1
      3. cargo test -p openlark-client --all-features 2>&1
      4. Assert all pass
    Expected Result: 所有 feature 组合编译通过
    Evidence: .sisyphus/evidence/task-6-build-test.txt
  ```
  **Commit**: YES
  - Message: `refactor(client): 拆分 utils 模块到独立文件`
  - Files: `crates/openlark-client/src/lib.rs`, `crates/openlark-client/src/utils.rs`
  - Pre-commit: `cargo check -p openlark-client --all-features`
- [x] 7. P1-3 openlark-docs/meeting glob re-export 修复
  **What to do**:
  - 先运行 `cargo check -p openlark-docs --features full 2>&1 | grep "ambiguous"` 确认具体哪些 glob 触发警告
  - 对触发 ambiguous_glob_reexports 警告的模块，改用显式 re-export 替代 `pub use *`
  - 对 openlark-meeting 做同样处理：`cargo check -p openlark-meeting --all-features 2>&1 | grep "ambiguous"`
  - 修复后尝试移除 `#![allow(ambiguous_glob_reexports)]` 和 `#![allow(hidden_glob_reexports)]`
  - 如果仍有残余警告，保留 allow 并记录剩余问题
  **Must NOT do**:
  - 不重写所有 140+ glob re-export（只修 warning 触发的）
  - 不改变模块的公共 API 语义
  - 不修改非冲突的 `pub use *` 语句
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 需要诊断具体冲突源并针对性修复，涉及多个子模块
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES（与 Task 5, 6 并行）
  - **Parallel Group**: Wave 2
  - **Blocks**: Final Verification
  - **Blocked By**: None
  **References**:
  - `crates/openlark-docs/src/lib.rs:4-5` — crate 级 allow 指令
  - `crates/openlark-meeting/src/lib.rs:2-3` — crate 级 allow 指令
  - `crates/openlark-docs/src/` — 搜索所有 `pub use *` 语句定位冲突源
  **Acceptance Criteria**:
  - [x] `#![allow(ambiguous_glob_reexports)]` 已移除或有明确理由保留
  - [x] `cargo check -p openlark-docs --features full` → 零 ambiguous 警告
  - [x] `cargo check -p openlark-meeting --all-features` → 零 ambiguous 警告
  - [x] `cargo test -p openlark-docs --features full` → 全部通过
  **QA Scenarios:**
  ```
  Scenario: glob 冲突已修复
    Tool: Bash (cargo check + grep)
    Steps:
      1. cargo check -p openlark-docs --features full 2>&1 | grep -i "ambiguous"
      2. Assert no output（无 ambiguous 警告）
      3. cargo check -p openlark-meeting --all-features 2>&1 | grep -i "ambiguous"
      4. Assert no output
    Expected Result: 两个 crate 均无 ambiguous glob 警告
    Evidence: .sisyphus/evidence/task-7-glob-fix.txt
  Scenario: allow 指令已移除
    Tool: Bash (grep)
    Steps:
      1. grep "ambiguous_glob_reexports" crates/openlark-docs/src/lib.rs
      2. Assert no output（或有注释说明保留理由）
      3. grep "ambiguous_glob_reexports" crates/openlark-meeting/src/lib.rs
      4. Assert no output（或有注释说明保留理由）
    Expected Result: allow 指令已移除
    Evidence: .sisyphus/evidence/task-7-allow-removed.txt
  ```
  **Commit**: YES
  - Message: `refactor(docs,meeting): 修复 glob re-export 冲突`
  - Files: `crates/openlark-docs/src/lib.rs`, `crates/openlark-docs/src/**/*.rs`, `crates/openlark-meeting/src/lib.rs`
  - Pre-commit: `cargo check -p openlark-docs --features full && cargo check -p openlark-meeting --all-features`
---

## Final Verification Wave

- [x] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists. For each "Must NOT Have": search codebase for forbidden patterns. Check evidence files exist. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo check --workspace --all-features` + `cargo test --workspace`. Review all changed files for: unnecessary allow attributes, dead code, unused imports. Check no new warnings introduced.
  Output: `Build [PASS/FAIL] | Tests [N pass/N fail] | Warnings [N new] | VERDICT`

- [x] F3. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (git log/diff). Verify 1:1 — everything in spec was built, nothing beyond spec was built. Check "Must NOT do" compliance. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

- **Wave 1**: `refactor(core): 清理 lib.rs 重复文档注释` — openlark-core/src/lib.rs
- **Wave 1**: `refactor(client): 清理 prelude 导出和冗余类型别名` — openlark-client/src/lib.rs
- **Wave 1**: `refactor(client): 清理不必要的 lint 抑制` — openlark-client/src/*.rs
- **Wave 2**: `refactor(core): 移动 Validatable trait 到 validation 模块` — openlark-core/src/lib.rs, openlark-core/src/validation/
- **Wave 2**: `refactor(client): 拆分 utils 模块到独立文件` — openlark-client/src/lib.rs, openlark-client/src/utils.rs
- **Wave 2**: `refactor(docs,meeting): 修复 glob re-export 冲突` — openlark-docs/src/lib.rs, openlark-meeting/src/lib.rs

---

## Success Criteria

### Verification Commands
```bash
cargo check --workspace --all-features  # Expected: 零新增警告
cargo test --workspace                   # Expected: 全部通过
grep -c "This crate provides the core infrastructure" crates/openlark-core/src/lib.rs  # Expected: 1
wc -l crates/openlark-client/src/lib.rs  # Expected: < 800
```

### Final Checklist
- [x] All "Must Have" present
- [x] All "Must NOT Have" absent
- [x] All tests pass
- [x] No new warnings
