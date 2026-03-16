# openlark-core 代码规范整改（P1 + P2）

## TL;DR

> **目标**: 修复 openlark-core 代码审查中发现的 P1/P2 级别问题，包括宏修复、内部模块重命名、死代码清理和 feature flag 整理。
>
> **交付物**:
> - 修复 `validate_required!` / `validate_required_list!` 宏的 `$crate::` 用法
> - 收窄 `#![allow(missing_docs)]` 范围
> - 重命名内部模块（improved_response_handler, utils, app_ticket_manager）
> - 移除 `send()` 占位方法
> - 清理空 feature flags
> - 修复 `api_path()` 硬编码路径
>
> **预估工作量**: Short（2-4小时）
> **并行执行**: YES - 3 waves
> **关键路径**: Task 1 → Task 7 → Task 8 → Final

---

## Context

### Original Request
基于 openlark-core 代码结构审查结果，按 P1+P2 优先级执行整改。

### Interview Summary
**关键决策**:
- 整改范围: P1 + P2（跳过 P3 的 http.rs 拆分）
- 内部模块重命名: 直接重命名（这些是 `mod` 非 `pub mod`，不影响外部 API）
- `send()` 方法: 直接移除

**研究发现**:
- `$crate::` 在 `#[macro_export]` 宏中始终指向定义宏的 crate（Rust Reference 确认）
- `app_ticket_manager` 移动风险最高（5+ 测试文件引用）
- 空 feature flags 被其他 crate 的 `Cargo.toml` 引用，需先 grep 确认

### Metis Review
**已识别风险**（已纳入计划）:
- `$crate::` 修复影响 18 个 crate ~965 个调用点，需 `cargo check --workspace` 验证
- `app_ticket_manager` 移动需更新 http.rs 和测试文件的 import
- 空 feature flags 可能被其他 crate 的 `[features]` 或 `[dependencies]` 引用
- `allow(missing_docs)` 收窄可能暴露大量警告，需渐进处理

---

## Work Objectives

### Core Objective
修复 openlark-core 中 P1/P2 级别的代码规范问题，提升代码质量和可维护性，同时保持完全向后兼容。

### Concrete Deliverables
- 修复后的宏定义（使用 `$crate::` 替代硬编码 crate 名）
- 重命名后的内部模块文件
- 清理后的 Cargo.toml feature flags
- 移除 `send()` 占位方法后的 api/mod.rs

### Definition of Done
-- [x] `cargo check --workspace --all-features` 零错误
-- [x] `cargo test -p openlark-core` 全部通过
-- [x] `cargo test --workspace` 全部通过
-- [x] `cargo clippy -p openlark-core` 零警告

### Must Have
- 所有公共 API 保持不变（零 breaking change）
- 宏在所有 18 个业务 crate 中正常工作
- 所有现有测试通过

### Must NOT Have (Guardrails)
- ❌ 不修改任何 `pub` 接口签名
- ❌ 不触碰 P3 项（http.rs 拆分）
- ❌ 不删除被其他 crate 实际引用的 feature flags
- ❌ 不引入新的 `unwrap()` 或 `expect()`
- ❌ 不修改 openlark-core 以外的业务逻辑

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed.

### Test Decision
- **Infrastructure exists**: YES
- **Automated tests**: Tests-after（验证现有测试通过即可）
- **Framework**: cargo test（Rust 内置）

### QA Policy
每个任务完成后执行 `cargo check` 和 `cargo test`，最终执行全 workspace 验证。

- **编译验证**: `cargo check --workspace --all-features`
- **测试验证**: `cargo test -p openlark-core` + `cargo test --workspace`
- **Lint 验证**: `cargo clippy -p openlark-core`

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — 独立修复，互不依赖):
├── Task 1: 修复 validate_required!/validate_required_list! 宏的 $crate:: 用法 [quick]
├── Task 2: 移除 send() 占位方法 [quick]
├── Task 3: 修复 api_path() 硬编码路径 [quick]
└── Task 4: 收窄 #![allow(missing_docs)] 范围 [quick]

Wave 2 (After Wave 1 — 内部模块重命名):
├── Task 5: 重命名 improved_response_handler.rs → response_handler.rs [quick]
├── Task 6: 审查并拆分/重命名 utils.rs [quick]
└── Task 7: 移动 app_ticket_manager.rs → auth/app_ticket.rs [quick]

Wave 3 (After Wave 2 — feature flags 清理):
└── Task 8: 清理空 feature flags [quick]

Wave FINAL (After ALL tasks — 全量验证):
├── Task F1: 计划合规审计 (oracle)
├── Task F2: 代码质量审查 (unspecified-high)
├── Task F3: 全量 QA 验证 (unspecified-high)
└── Task F4: 范围保真检查 (deep)

Critical Path: Task 1 → Task 7 → Task 8 → F1-F4
Parallel Speedup: ~50% faster than sequential
Max Concurrent: 4 (Wave 1)
```

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|-----------|--------|------|
| 1 | — | F1-F4 | 1 |
| 2 | — | F1-F4 | 1 |
| 3 | — | F1-F4 | 1 |
| 4 | — | F1-F4 | 1 |
| 5 | 1-4 | F1-F4 | 2 |
| 6 | 1-4 | 7 | 2 |
| 7 | 1-4 | 8 | 2 |
| 8 | 5-7 | F1-F4 | 3 |

### Agent Dispatch Summary

- **Wave 1**: **4** — T1-T4 → `quick`
- **Wave 2**: **3** — T5-T7 → `quick`
- **Wave 3**: **1** — T8 → `quick`
- **FINAL**: **4** — F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

> Implementation + Test = ONE Task.
> EVERY task MUST have: Recommended Agent Profile + Parallelization info + QA Scenarios.


- [x] 1. 修复 validate_required! / validate_required_list! 宏的 $crate:: 用法

  **What to do**:
  - 编辑 `crates/openlark-core/src/lib.rs`
  - 将 `validate_required!` 宏（第75-81行）中的 `openlark_core::Validatable` 改为 `$crate::Validatable`
  - 将 `openlark_core::error::CoreError` 改为 `$crate::error::CoreError`
  - 将 `validate_required_list!` 宏（第94-104行）中的 `openlark_core::error::CoreError` 改为 `$crate::error::CoreError`
  - 注意：`$crate` 在 `#[macro_export]` 宏中始终指向定义宏的 crate（已由 Rust Reference 确认安全）

  **Must NOT do**:
  - 不修改宏的语义行为
  - 不修改 Validatable trait 定义
  - 不修改错误消息文本

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3, 4)
  - **Blocks**: F1-F4
  - **Blocked By**: None

  **References**:
  - `crates/openlark-core/src/lib.rs:74-104` — 两个宏的当前实现，需将 `openlark_core::` 替换为 `$crate::`
  - `crates/openlark-core/src/lib.rs:39-71` — Validatable trait 定义（不修改，仅参考）
  - Rust Reference `$crate` 文档：https://doc.rust-lang.org/reference/macros-by-example.html#hygiene

  **Acceptance Criteria**:
  - [ ] 两个宏中不再包含硬编码的 `openlark_core::`
  - [ ] `cargo check -p openlark-core` 通过
  -- [x] `cargo check --workspace --all-features` 通过（验证 18 个 crate 的 ~965 个调用点）

  **QA Scenarios**:
  ```
  Scenario: 全 workspace 编译验证
    Tool: Bash
    Steps:
      1. 运行 `cargo check --workspace --all-features`
      2. 验证退出码为 0
    Expected Result: 零错误，零警告（与宏相关的）
    Evidence: .sisyphus/evidence/task-1-workspace-check.txt

  Scenario: 宏语义不变验证
    Tool: Bash
    Steps:
      1. 运行 `cargo test -p openlark-core -- validate`
      2. 验证所有 validate 相关测试通过
    Expected Result: 所有测试 PASS
    Evidence: .sisyphus/evidence/task-1-macro-tests.txt
  ```

  **Commit**: YES (groups with Wave 1)
  - Message: `fix(core): 将宏中硬编码 openlark_core:: 替换为 $crate::`
  - Files: `crates/openlark-core/src/lib.rs`
  - Pre-commit: `cargo check --workspace --all-features`

---

- [x] 2. 移除 send() 占位方法

  **What to do**:
  - 编辑 `crates/openlark-core/src/api/mod.rs`
  - 删除 `ApiRequest<R>` 的 `send()` 方法（第317-327行附近）
  - 该方法始终返回 `Err(configuration_error(...))`，是无用的占位符
  - 删除前先 grep 确认无实际调用：`grep -r '\.send()' crates/ --include='*.rs'`（排除 reqwest 的 send）

  **Must NOT do**:
  - 不修改 `ApiRequest` 的其他方法
  - 不修改 `Transport::request()` 或 `Transport::do_send()`

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: `[]`

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3, 4)
  - **Blocks**: F1-F4
  - **Blocked By**: None

  **References**:
  - `crates/openlark-core/src/api/mod.rs:317-327` — send() 占位方法（始终返回错误）
  - `crates/openlark-core/src/http.rs:38-98` — Transport::request() 是实际的请求发送入口（不修改）

  **Acceptance Criteria**:
  - [ ] `ApiRequest` 上不再有 `send()` 方法
  -- [x] `cargo check --workspace --all-features` 通过（确认无调用方）

  **QA Scenarios**:
  ```
  Scenario: 确认无调用方
    Tool: Bash
    Steps:
      1. 运行 `grep -rn '\.send()\.' crates/ --include='*.rs' | grep -v 'reqwest' | grep -v 'test' | grep -v '/target/'`
      2. 确认无 ApiRequest.send() 的实际调用
    Expected Result: 无匹配结果（或仅 reqwest 相关）
    Evidence: .sisyphus/evidence/task-2-send-grep.txt

  Scenario: 编译验证
    Tool: Bash
    Steps:
      1. 运行 `cargo check --workspace --all-features`
    Expected Result: 零错误
    Evidence: .sisyphus/evidence/task-2-check.txt
  ```

  **Commit**: YES (groups with Wave 1)
  - Message: `refactor(core): 移除 ApiRequest::send() 占位方法`
  - Files: `crates/openlark-core/src/api/mod.rs`
  - Pre-commit: `cargo check --workspace --all-features`

---
- [x] 3. 修复 api_path() 硬编码路径
  **What to do**:
  - 编辑 `crates/openlark-core/src/api/mod.rs`
  - 在 `api_path()` 方法（第248-255行）中，将硬编码的 `"/open-apis/"` 提取为常量
  - 在 `crates/openlark-core/src/constants.rs` 中添加 `pub const OPEN_API_PATH_PREFIX: &str = "/open-apis/";`
  - 在 `api_path()` 中引用该常量：`self.url.find(crate::constants::OPEN_API_PATH_PREFIX)`
  **Must NOT do**:
  - 不修改 `api_path()` 的返回值语义
  - 不修改 fallback 逻辑（找不到时返回完整 URL）
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: `[]`
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 4)
  - **Blocks**: F1-F4
  - **Blocked By**: None
  **References**:
  - `crates/openlark-core/src/api/mod.rs:248-255` — api_path() 当前实现，硬编码 `"/open-apis/"`
  - `crates/openlark-core/src/constants.rs:60-66` — 现有常量定义位置，新常量应放在此处
  **Acceptance Criteria**:
  - [ ] `api_path()` 中不再包含硬编码字符串 `"/open-apis/"`
  - [ ] `constants.rs` 中新增 `OPEN_API_PATH_PREFIX` 常量
  -- [x] `cargo test -p openlark-core` 通过
  **QA Scenarios**:
  ```
  Scenario: 硬编码消除验证
    Tool: Bash
    Steps:
      1. 运行 `grep -n '"/open-apis/"' crates/openlark-core/src/api/mod.rs`
      2. 确认无硬编码匹配（仅常量引用）
    Expected Result: 无直接硬编码匹配
    Evidence: .sisyphus/evidence/task-3-hardcode-grep.txt
  Scenario: 功能不变验证
    Tool: Bash
    Steps:
      1. 运行 `cargo test -p openlark-core -- api`
    Expected Result: 所有 api 相关测试 PASS
    Evidence: .sisyphus/evidence/task-3-api-tests.txt
  ```
  **Commit**: YES (groups with Wave 1)
  - Files: `crates/openlark-core/src/api/mod.rs`, `crates/openlark-core/src/constants.rs`

---
- [x] 4. 收窄 #![allow(missing_docs)] 范围
  **What to do**:
  - 编辑 `crates/openlark-core/src/lib.rs`
  - 移除第6行的 `#![allow(missing_docs)]` crate 级屏蔽
  - 运行 `cargo check -p openlark-core 2>&1 | grep 'missing_docs'` 查看暴露的警告
  - 对每个报警的模块/项，添加最小范围的 `#[allow(missing_docs)]`
  - 优先在模块级别添加（如 `mod xxx` 前），而非每个函数
  - 目标：从 crate 级屏蔽改为模块级屏蔽，不修复警告本身
  **Must NOT do**:
  - 不为所有缺失文档的项补写文档（这是单独任务）
  - 不引入新的编译警告
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: `[]`
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3)
  - **Blocks**: F1-F4
  - **Blocked By**: None
  **References**:
  - `crates/openlark-core/src/lib.rs:6` — 当前的 `#![allow(missing_docs)]`
  - `crates/openlark-core/src/lib.rs:24-30` — 内部模块声明（`mod xxx;`），这些是添加 `#[allow(missing_docs)]` 的主要位置
  **Acceptance Criteria**:
  - [ ] `lib.rs` 中不再有 `#![allow(missing_docs)]`
  - [ ] `cargo check -p openlark-core` 零警告（通过模块级 allow 屏蔽）
  **QA Scenarios**:
  ```
  Scenario: crate 级屏蔽已移除
    Tool: Bash
    Steps:
      1. 运行 `grep -n 'allow(missing_docs)' crates/openlark-core/src/lib.rs`
      2. 确认无 `#![allow(missing_docs)]`（crate 级），仅有模块级 `#[allow(missing_docs)]`
    Expected Result: 无 `#!` 前缀的 allow(missing_docs)
    Evidence: .sisyphus/evidence/task-4-allow-grep.txt
  Scenario: 编译无警告
    Tool: Bash
    Steps:
      1. 运行 `cargo check -p openlark-core 2>&1`
      2. 确认无 missing_docs 警告
    Expected Result: 零警告
    Evidence: .sisyphus/evidence/task-4-check.txt
  ```
  **Commit**: YES (groups with Wave 1)
  - Files: `crates/openlark-core/src/lib.rs`
---
- [x] 5. 重命名 improved_response_handler.rs → response_handler.rs
  **What to do**:
  - 将 `crates/openlark-core/src/improved_response_handler.rs` 重命名为 `response_handler.rs`
  - 更新 `crates/openlark-core/src/lib.rs` 中的 `mod improved_response_handler;` → `mod response_handler;`
  - 更新 `crates/openlark-core/src/http.rs` 中的 `use crate::improved_response_handler::` → `use crate::response_handler::`
  - 先 grep 确认所有引用点：`grep -rn 'improved_response_handler' crates/openlark-core/`
  **Must NOT do**:
  - 不修改 `ImprovedResponseHandler` 结构体名称（仅改文件名和模块名）
  - 不修改该模块的任何逻辑
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: `[]`
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 6, 7)
  - **Blocks**: F1-F4
  - **Blocked By**: Tasks 1-4 (Wave 1)
  **References**:
  - `crates/openlark-core/src/improved_response_handler.rs` — 待重命名的文件
  - `crates/openlark-core/src/lib.rs:27` — `mod improved_response_handler;` 声明
  - `crates/openlark-core/src/http.rs:14` — `use crate::improved_response_handler::ImprovedResponseHandler;`
  **Acceptance Criteria**:
  - [ ] 文件已重命名为 `response_handler.rs`
  - [ ] `improved_response_handler.rs` 不再存在
  - [ ] `cargo check -p openlark-core` 通过
  **QA Scenarios**:
  ```
  Scenario: 文件重命名验证
    Tool: Bash
    Steps:
      1. 运行 `ls crates/openlark-core/src/response_handler.rs`
      2. 运行 `ls crates/openlark-core/src/improved_response_handler.rs` (应失败)
      3. 运行 `grep -rn 'improved_response_handler' crates/openlark-core/src/`
    Expected Result: 新文件存在，旧文件不存在，无残留引用
    Evidence: .sisyphus/evidence/task-5-rename.txt
  ```
  **Commit**: YES (groups with Wave 2)
  - Files: `crates/openlark-core/src/response_handler.rs`, `lib.rs`, `http.rs`
---
- [x] 6. 审查并拆分/重命名 utils.rs
  **What to do**:
  - 先读取 `crates/openlark-core/src/utils.rs` 内容，了解其中包含哪些工具函数
  - 如果内容单一（如仅 ID 生成），直接重命名为语义化名称（如 `id_generator.rs`）
  - 如果内容混杂，按功能拆分为多个文件
  - 更新 `lib.rs` 中的 `mod utils;` 为新模块名
  - 更新所有 `crate::utils::` 引用为新路径
  - 先 grep 确认引用点：`grep -rn 'crate::utils' crates/openlark-core/`
  **Must NOT do**:
  - 不修改工具函数的逻辑
  - 不修改公共 API（utils 是 `mod` 非 `pub mod`）
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: `[]`
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 7)
  - **Blocks**: Task 7, F1-F4
  - **Blocked By**: Tasks 1-4 (Wave 1)
  **References**:
  - `crates/openlark-core/src/utils.rs` — 待审查和重命名的文件
  - `crates/openlark-core/src/lib.rs:30` — `mod utils;` 声明
  **Acceptance Criteria**:
  - [ ] `utils.rs` 已重命名或拆分为语义化文件名
  - [ ] `cargo check -p openlark-core` 通过
  **QA Scenarios**:
  ```
  Scenario: 旧文件已消除
    Tool: Bash
    Steps:
      1. 运行 `ls crates/openlark-core/src/utils.rs` (应失败)
      2. 运行 `grep -rn 'mod utils' crates/openlark-core/src/lib.rs` (应无匹配)
    Expected Result: utils.rs 不存在，无 mod utils 声明
    Evidence: .sisyphus/evidence/task-6-utils-rename.txt
  ```
  **Commit**: YES (groups with Wave 2)
  - Files: 重命名后的文件, `lib.rs`
---
- [x] 7. 移动 app_ticket_manager.rs → auth/app_ticket.rs
  **What to do**:
  - 将 `crates/openlark-core/src/app_ticket_manager.rs` 移动到 `crates/openlark-core/src/auth/app_ticket.rs`
  - 更新 `crates/openlark-core/src/lib.rs`：移除 `mod app_ticket_manager;`
  - 更新 `crates/openlark-core/src/auth/mod.rs`：添加 `mod app_ticket;`（注意是私有模块）
  - 更新 `crates/openlark-core/src/http.rs` 中的 import：`use crate::app_ticket_manager::` → `use crate::auth::app_ticket::`
  - 先 grep 确认所有引用点：`grep -rn 'app_ticket_manager' crates/openlark-core/`
  - **高风险**：此模块被 http.rs 的 `apply_app_ticket` 函数引用，需仔细更新
  **Must NOT do**:
  - 不修改 `apply_app_ticket()` 函数的逻辑
  - 不将模块改为 `pub mod`（保持私有）
  - 不修改 auth/mod.rs 的现有公共导出
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: `[]`
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 6)
  - **Blocks**: Task 8, F1-F4
  - **Blocked By**: Tasks 1-4 (Wave 1)
  **References**:
  - `crates/openlark-core/src/app_ticket_manager.rs` — 待移动的文件
  - `crates/openlark-core/src/lib.rs:25` — `mod app_ticket_manager;` 声明
  - `crates/openlark-core/src/http.rs:10` — `use crate::app_ticket_manager::apply_app_ticket;`
  - `crates/openlark-core/src/auth/mod.rs` — 目标目录的模块声明文件
  **Acceptance Criteria**:
  - [ ] `app_ticket_manager.rs` 不再存在于 `src/` 根目录
  - [ ] `auth/app_ticket.rs` 存在且内容正确
  - [ ] `cargo check -p openlark-core` 通过
  -- [x] `cargo test -p openlark-core` 通过
  **QA Scenarios**:
  ```
  Scenario: 文件移动验证
    Tool: Bash
    Steps:
      1. 运行 `ls crates/openlark-core/src/auth/app_ticket.rs`
      2. 运行 `ls crates/openlark-core/src/app_ticket_manager.rs` (应失败)
      3. 运行 `grep -rn 'app_ticket_manager' crates/openlark-core/src/`
    Expected Result: 新文件存在，旧文件不存在，无残留引用
    Evidence: .sisyphus/evidence/task-7-move.txt
  Scenario: 功能不变验证
    Tool: Bash
    Steps:
      1. 运行 `cargo test -p openlark-core`
    Expected Result: 所有测试 PASS
    Evidence: .sisyphus/evidence/task-7-tests.txt
  ```
  **Commit**: YES (groups with Wave 2)
  - Files: `auth/app_ticket.rs`, `lib.rs`, `auth/mod.rs`, `http.rs`
---
- [x] 8. 清理空 feature flags
  **What to do**:
  - 先执行 grep 审查哪些空 feature flags 被其他 crate 引用：
    `grep -rn 'openlark-core' crates/*/Cargo.toml | grep -E '(acs|admin|ai|aily|apass|application|approval|attendance|authentication|bot|calendar|cardkit|cloud-docs|contact|corehr|directory|ehr|elearning|group|helpdesk|hire|human-authentication|im|lingo|mail|mdm|minutes|moments|okr|passport|payroll|performance|personal-settings|report|search|security-and-compliance|task|tenant|tenant-tag|trust-party|vc|verification|workplace)'`
  - 对于**被引用的** feature flags：保留，添加注释说明用途
  - 对于**未被引用的** feature flags：从 `Cargo.toml` 中移除
  - 为保留的 feature flags 添加分组注释（按业务域）
  **Must NOT do**:
  - 不删除被任何其他 crate 的 Cargo.toml 引用的 feature flag
  - 不修改 feature flag 的名称
  - 不添加新的 feature flag
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: `[]`
  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (sequential)
  - **Blocks**: F1-F4
  - **Blocked By**: Tasks 5-7 (Wave 2)
  **References**:
  - `crates/openlark-core/Cargo.toml:61-103` — 当前的空 feature flags 列表
  - `crates/*/Cargo.toml` — 其他 crate 的依赖声明，需 grep 确认引用关系
  **Acceptance Criteria**:
  - [ ] 未被引用的空 feature flags 已移除
  - [ ] 被引用的 feature flags 保留且有注释
  -- [x] `cargo check --workspace --all-features` 通过
  **QA Scenarios**:
  ```
  Scenario: 编译验证
    Tool: Bash
    Steps:
      1. 运行 `cargo check --workspace --all-features`
    Expected Result: 零错误
    Evidence: .sisyphus/evidence/task-8-check.txt
  Scenario: 无误删验证
    Tool: Bash
    Steps:
      1. 运行 `cargo check --workspace` (默认 features)
    Expected Result: 零错误
    Evidence: .sisyphus/evidence/task-8-default-check.txt
  ```
  **Commit**: YES
  - Message: `chore(core): 清理未使用的空 feature flags，添加分组注释`
  - Files: `crates/openlark-core/Cargo.toml`
  - Pre-commit: `cargo check --workspace --all-features`
## Final Verification Wave (MANDATORY — after ALL implementation tasks)

> 4 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [x] F1. **计划合规审计** — `oracle`
  读取计划全文。对每个 "Must Have"：验证实现存在。对每个 "Must NOT Have"：搜索代码库中的禁止模式。检查 evidence 文件。对比交付物与计划。
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **代码质量审查** — `unspecified-high`
  运行 `cargo check --workspace --all-features` + `cargo clippy -p openlark-core` + `cargo test -p openlark-core`。审查所有变更文件：检查 `unwrap()`、空 catch、注释掉的代码、未使用的 import。
  Output: `Build [PASS/FAIL] | Lint [PASS/FAIL] | Tests [N pass/N fail] | VERDICT`

- [x] F3. **全量 QA 验证** — `unspecified-high`
  从干净状态开始。执行每个任务的 QA 场景。测试跨任务集成（宏修复 + 模块重命名后的编译）。测试边界情况：全 features 编译、单 crate 编译。
  Output: `Scenarios [N/N pass] | Integration [N/N] | Edge Cases [N tested] | VERDICT`

- [x] F4. **范围保真检查** — `deep`
  对每个任务：读取 "What to do"，读取实际 diff。验证 1:1——规格中的所有内容都已构建，规格之外的内容未构建。检查 "Must NOT do" 合规性。
  Output: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

- **Wave 1 完成后**: `refactor(core): 修复宏 $crate 用法、移除 send() 占位方法、修复硬编码路径、收窄 missing_docs`
- **Wave 2 完成后**: `refactor(core): 重命名内部模块 improved_response_handler/utils/app_ticket_manager`
- **Wave 3 完成后**: `chore(core): 清理空 feature flags`
- **Pre-commit**: `cargo test -p openlark-core && cargo check --workspace --all-features`

---

## Success Criteria

### Verification Commands
```bash
cargo check --workspace --all-features  # Expected: 零错误
cargo test -p openlark-core             # Expected: 全部通过
cargo test --workspace                  # Expected: 全部通过
cargo clippy -p openlark-core           # Expected: 零警告
```

### Final Checklist
- [x] 所有 "Must Have" 已实现
- [x] 所有 "Must NOT Have" 未出现
- [x] 所有测试通过
- [x] 零 breaking change