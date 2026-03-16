# openlark-core 最佳实践重构

## TL;DR

> **Quick Summary**: 按 Rust 最佳实践激进简化 openlark-core：删除冗余错误系统模块、统一验证系统、RequestOption 改用 Option<String>、收窄公开 API 表面，并适配所有下游 crate。
> 
> **Deliverables**:
> - 精简后的错误系统（删除 ErrorKind、capabilities、ErrorKindAnalyzer、analysis、defaults）
> - 统一的验证系统（删除函数版 validate_required，只保留宏）
> - 惯用的 RequestOption（Option<String> 替代空字符串）
> - 最小化公开 API（trait_system 保留公开，其余内部模块收窄）
> - 所有 18+ 下游 crate 适配完成
> 
> **Estimated Effort**: Large
> **Parallel Execution**: YES - 4 waves
> **Critical Path**: T1-T4 → T5 → T8 → T9-T10 → F1-F4

---

## Context

### Original Request
用户要求按最佳实践修改 openlark-core，允许破坏性修改，允许修改下游 crate。

### Interview Summary
**Key Discussions**:
- 错误系统: 激进简化，删除 ErrorKind 整个模块、capabilities、ErrorKindAnalyzer、ERROR_SYSTEM_VERSION
- 验证系统: 只保留宏版本 validate_required!，删除函数版本
- RequestOption: String → Option<String>，影响所有下游
- 公开 API: 收窄到最小集（trait_system 例外保留）
- 测试: 现有测试通过即可

### Metis Review
**Identified Gaps** (addressed):
- trait_system 有 4 个下游依赖（ai, docs, meeting, hr）— 必须保留公开
- Validatable trait 保留在 lib.rs（宏路径依赖 $crate::Validatable），同时从 validation 模块重导出
- openlark-auth 中 validate_required 函数调用需适配为宏调用
- RequestOption 下游影响主要在 core 内部（21 处），下游通过 Builder 使用

---

## Work Objectives

### Core Objective
将 openlark-core 从"功能完整但冗余"升级为"精简、惯用、最小化"的 Rust SDK 核心。

### Definition of Done
- [x] `cargo check -p openlark-core --all-features` 无错误
- [x] `cargo test -p openlark-core` 全部通过
- [x] `cargo check --workspace --all-features` 无错误
- [x] `cargo test --workspace` 全部通过
- [x] `cargo clippy --workspace --all-features` 零新增警告

### Must Have
- 错误系统只保留 CoreError + ErrorCode + ErrorContext 核心三件套
- validate_required! 宏是唯一的必填验证入口
- RequestOption 使用 Option<String>
- 公开 API 最小化（trait_system 例外保留）

### Must NOT Have (Guardrails)
- ❌ 不修改 CoreError 枚举变体本身（只删除周边冗余）
- ❌ 不修改 ErrorCode 枚举（飞书错误码映射是核心功能）
- ❌ 不修改 Config/ConfigBuilder 的公开 API
- ❌ 不引入新的依赖
- ❌ 不修改 API 请求/响应的序列化行为
- ❌ 不修改 Transport 的核心请求逻辑
- ❌ 不删除任何现有测试（可以修改以适配新 API）

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed.

### Test Decision
- **Infrastructure exists**: YES
- **Automated tests**: Tests-after（现有测试通过即可）
- **Framework**: cargo test

### QA Policy
- **Compilation**: `cargo check -p openlark-core` + `cargo check --workspace`
- **Tests**: `cargo test -p openlark-core`
- **Lint**: `cargo clippy -p openlark-core`
- Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — 独立的删除/清理操作):
├── Task 1: 删除 error/kinds.rs 模块 [quick]
├── Task 2: 清理 error/mod.rs 冗余代码 [quick]
├── Task 3: AccessTokenType::Display 优化 [quick]
└── Task 4: 删除 validate_required 函数 + 适配 [quick]

Wave 2 (After Wave 1 — 核心结构变更):
├── Task 5: RequestOption String → Option<String> [deep]
├── Task 6: lib.rs 模块可见性收窄 + prelude 清理 [unspecified-high]
└── Task 7: 删除 validation 业务模块 [quick]

Wave 3 (After Wave 2 — 适配):
├── Task 8: openlark-core 内部适配 (http.rs 等) [deep]
├── Task 9: openlark-client 适配 [unspecified-high]
└── Task 10: 其余 16+ 下游 crate 批量适配 [unspecified-high]

Wave 4 (After Wave 3 — 清理):
├── Task 11: 清理命名和 deprecated 代码 [quick]
└── Task 12: 更新 AGENTS.md 知识库 [writing]

Wave FINAL (After ALL — 独立审查, 4 parallel):
├── Task F1: Plan compliance audit [oracle]
├── Task F2: Code quality review [unspecified-high]
├── Task F3: Full workspace build + test [unspecified-high]
└── Task F4: Scope fidelity check [deep]

Critical Path: T1-T4 → T5 → T8 → T10 → F1-F4
Max Concurrent: 4 (Wave 1)
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|-----------|--------|
| 1-4 | — | 5-7 |
| 5 | 1-4 | 8 |
| 6 | 1-4 | 8, 9, 10 |
| 7 | — | 9, 10 |
| 8 | 5, 6, 7 | 9, 10 |
| 9 | 8 | F1-F4 |
| 10 | 8 | F1-F4 |
| 11-12 | 8 | F1-F4 |
| F1-F4 | 9-12 | — |

### Agent Dispatch Summary

- **Wave 1**: 4 tasks — T1-T4 → `quick`
- **Wave 2**: 3 tasks — T5 → `deep`, T6 → `unspecified-high`, T7 → `quick`
- **Wave 3**: 3 tasks — T8 → `deep`, T9-T10 → `unspecified-high`
- **Wave 4**: 2 tasks — T11 → `quick`, T12 → `writing`
- **FINAL**: 4 tasks — F1 → `oracle`, F2-F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

- [x] 1. 删除 error/kinds.rs 模块 + 清理引用

  **What to do**:
  - 删除 `crates/openlark-core/src/error/kinds.rs` 文件
  - 从 `error/mod.rs` 移除 `pub mod kinds;` 和 `pub use self::kinds::ErrorKind;`
  - 搜索 workspace 中所有 `ErrorKind` 引用，替换为 `ErrorType`（功能重叠）或直接删除未使用的 import
  - 运行 `cargo check -p openlark-core` 确认

  **Must NOT do**:
  - 不修改 ErrorType 枚举变体
  - 不修改 CoreError 枚举

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3, 4)
  - **Blocks**: Tasks 5-7
  - **Blocked By**: None

  **References**:
  - `crates/openlark-core/src/error/kinds.rs` — 要删除的文件（407行），包含 ErrorKind 枚举、ErrorKindExt trait、ErrorKindAnalyzer、ErrorAnalysis
  - `crates/openlark-core/src/error/mod.rs:19` — `pub use self::kinds::ErrorKind;` 需移除
  - `crates/openlark-core/src/error/traits.rs` — ErrorType 枚举已覆盖 ErrorKind 的全部功能

  **Acceptance Criteria**:
  - [ ] `kinds.rs` 文件已删除
  - [ ] `error/mod.rs` 不再引用 kinds 模块
  - [ ] workspace 中无 `ErrorKind` 引用
  - [ ] `cargo check -p openlark-core` 通过

  **QA Scenarios**:
  ```
  Scenario: kinds.rs 删除后编译通过
    Tool: Bash
    Steps:
      1. test ! -f crates/openlark-core/src/error/kinds.rs — 文件不存在
      2. grep -r "ErrorKind" crates/openlark-core/src/ — 应无结果
      3. cargo check -p openlark-core — 应成功
    Expected Result: 零编译错误
    Evidence: .sisyphus/evidence/task-1-kinds-deleted.txt
  ```

  **Commit**: YES (groups with Wave 1)
  - Message: `refactor(core): 删除 ErrorKind 冗余模块`
  - Files: `error/kinds.rs`, `error/mod.rs`

- [x] 2. 清理 error/mod.rs 冗余代码

  **What to do**:
  - 删除 `error/mod.rs` 中的以下内容：
    - `pub const ERROR_SYSTEM_VERSION` 常量（~第43行）
    - `pub mod capabilities { ... }` 块（~第46-59行）
    - `pub mod defaults { ... }` 块（~第62-72行）
    - `pub mod analysis { ... }` 块（~第78-143行）
    - `#[deprecated] pub type LegacyCoreError = CoreError;` 别名（~第29-30行）
  - 删除 `error/mod.rs` 中引用这些模块的测试代码
  - 保留核心 re-export：CoreError, SDKResult, ErrorCode, ErrorContext 等
  - 运行 `cargo check -p openlark-core` 和 `cargo test -p openlark-core`

  **Must NOT do**:
  - 不修改核心 re-export（CoreError, SDKResult 等）
  - 不修改 LarkAPIError 别名（本任务不处理）

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3, 4)
  - **Blocks**: Tasks 5-7
  - **Blocked By**: None

  **References**:
  - `crates/openlark-core/src/error/mod.rs:43` — ERROR_SYSTEM_VERSION 常量
  - `crates/openlark-core/src/error/mod.rs:46-59` — capabilities 模块（const bool 声明，无实际用途）
  - `crates/openlark-core/src/error/mod.rs:62-72` — defaults 模块（default_retry_policy/default_error_context）
  - `crates/openlark-core/src/error/mod.rs:78-143` — analysis 模块（ErrorAnalysisResult, analyze_error 等）
  - `crates/openlark-core/src/error/mod.rs:29-30` — deprecated LegacyCoreError
  - `crates/openlark-core/src/error/mod.rs:145-1142` — 测试代码中引用 capabilities/analysis/defaults 的测试需删除

  **Acceptance Criteria**:
  - [ ] capabilities/analysis/defaults 模块已删除
  - [ ] ERROR_SYSTEM_VERSION/LegacyCoreError 已删除
  - [ ] `cargo check -p openlark-core` 通过
  - [ ] `cargo test -p openlark-core` 通过（删除引用已删模块的测试）

  **QA Scenarios**:
  ```
  Scenario: mod.rs 清理后编译和测试通过
    Tool: Bash
    Steps:
      1. grep -c "capabilities\|ERROR_SYSTEM_VERSION\|LegacyCoreError" crates/openlark-core/src/error/mod.rs — 应为 0
      2. cargo check -p openlark-core — 应成功
      3. cargo test -p openlark-core -- error — 应通过
    Expected Result: 清理完成，测试通过
    Evidence: .sisyphus/evidence/task-2-mod-cleaned.txt
  ```

  **Commit**: YES (groups with Wave 1)
  - Message: `refactor(core): 清理错误模块冗余代码`
  - Files: `error/mod.rs`

- [x] 3. AccessTokenType::Display 优化（&'static str 替代临时 String）

  **What to do**:
  - 修改 `constants.rs` 中 `AccessTokenType` 的 `Display` 实现
  - 将 `String::from("...")` 改为直接写入 `&'static str`（与 `AppType` 一致）
  - 添加 `as_str(&self) -> &'static str` 方法（与 `AppType` 对齐）
  - 运行 `cargo check -p openlark-core`

  **Must NOT do**:
  - 不修改 AccessTokenType 枚举变体
  - 不修改 AppType（已经是正确实现）

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 4)
  - **Blocks**: Tasks 5-7
  - **Blocked By**: None

  **References**:
  - `crates/openlark-core/src/constants.rs:48-58` — AccessTokenType::Display 当前实现，使用 String::from 创建临时字符串
  - `crates/openlark-core/src/constants.rs:13-27` — AppType 的正确实现模式（as_str + Display 委托）

  **Acceptance Criteria**:
  - [ ] AccessTokenType::Display 不再创建临时 String
  - [ ] 新增 `as_str()` 方法返回 `&'static str`
  - [ ] `cargo check -p openlark-core` 通过
  - [ ] `cargo test -p openlark-core` 通过

  **QA Scenarios**:
  ```
  Scenario: Display 实现使用 &'static str
    Tool: Bash
    Steps:
      1. grep -n "String::from" crates/openlark-core/src/constants.rs — AccessTokenType 区域应无 String::from
      2. grep -n "as_str" crates/openlark-core/src/constants.rs — 应有 AccessTokenType::as_str 方法
      3. cargo test -p openlark-core -- constants — 应通过
    Expected Result: 零 String::from 在 AccessTokenType 中，as_str 方法存在
    Evidence: .sisyphus/evidence/task-3-display-optimized.txt
  ```

  **Commit**: YES (groups with Wave 1)
  - Message: `refactor(core): AccessTokenType Display 使用 &'static str`
  - Files: `constants.rs`

- [x] 4. 删除 validate_required 函数 + 适配 openlark-auth 测试

  **What to do**:
  - 从 `validation/core.rs` 删除 `validate_required` 函数（注意：保留 `validate_string_length`、`is_chinese_char`、`validate_required_list_length`、`ValidateBuilder`、`ValidationResult` 等其他内容）
  - 从 `validation/mod.rs` 的 re-export 中移除 `validate_required`
  - 从 `lib.rs` 移除 `pub use validation::validate_required;`（第36行）
  - 适配 `openlark-auth/tests/auth_validation_tests.rs`：将 `validate_required("...", "...")` 函数调用改为使用宏 `validate_required!(value, msg)` 或直接测试 `Validatable::is_empty_trimmed`
  - 搜索 workspace 中其他 `validation::validate_required` 函数调用并替换
  - 运行 `cargo check --workspace` 和 `cargo test -p openlark-auth`

  **Must NOT do**:
  - 不删除 `validate_required!` 宏（lib.rs 中的宏定义）
  - 不删除 `validate_required_list!` 宏
  - 不删除 validation/core.rs 中的其他函数

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3)
  - **Blocks**: Tasks 5-7
  - **Blocked By**: None

  **References**:
  - `crates/openlark-core/src/validation/core.rs` — validate_required 函数定义（在文件中搜索 `pub fn validate_required`）
  - `crates/openlark-core/src/validation/mod.rs:13-16` — re-export 列表，需移除 validate_required
  - `crates/openlark-core/src/lib.rs:36` — `pub use validation::validate_required;` 需移除
  - `crates/openlark-core/src/lib.rs:75-81` — validate_required! 宏定义（保留不动）
  - `crates/openlark-auth/tests/auth_validation_tests.rs:30-50` — 5 处函数调用需适配
  - 函数签名: `fn validate_required(value: &str, field_name: &str) -> bool`
  - 宏签名: `validate_required!($field:expr, $error_msg:expr)` — 返回 Err 而非 bool
  - 适配策略: auth 测试改为直接测试 `Validatable::is_empty_trimmed` trait 方法

  **Acceptance Criteria**:
  - [ ] `validation::validate_required` 函数不存在
  - [ ] `validate_required!` 宏仍然正常工作
  - [ ] `cargo check --workspace` 通过
  - [ ] `cargo test -p openlark-auth` 通过

  **QA Scenarios**:
  ```
  Scenario: 函数删除后宏仍工作
    Tool: Bash
    Steps:
      1. grep -rn "pub fn validate_required" crates/openlark-core/src/ — 应无结果
      2. grep -rn "validate_required!" crates/openlark-core/src/lib.rs — 宏定义应存在
      3. cargo test -p openlark-auth -- auth_validation — 应通过
      4. cargo check --workspace — 应通过
    Expected Result: 函数已删除，宏保留，所有编译和测试通过
    Evidence: .sisyphus/evidence/task-4-validate-fn-deleted.txt

  Scenario: 下游 crate 无残留函数引用
    Tool: Bash
    Steps:
      1. grep -rn "validation::validate_required[^!]" crates/ — 应无结果（排除宏调用）
      2. grep -rn "use.*validate_required[^!]" crates/ — 应无函数 import
    Expected Result: 零残留函数引用
    Evidence: .sisyphus/evidence/task-4-no-residual-refs.txt
  ```

  **Commit**: YES (groups with Wave 1)
  - Message: `refactor(core): 删除 validate_required 函数，统一使用宏`
  - Files: `validation/core.rs`, `validation/mod.rs`, `lib.rs`, `openlark-auth/tests/auth_validation_tests.rs`

- [x] 5. RequestOption String → Option<String> 全面改造

  **What to do**:
  - 修改 `req_option.rs` 中 `RequestOption` 结构体：将 `tenant_key`、`user_access_token`、`app_access_token`、`tenant_access_token`、`request_id`、`app_ticket` 从 `String` 改为 `Option<String>`
  - 保留 `need_helpdesk_auth: bool`、`file_upload: bool`、`file_download: bool`、`header: HashMap<String, String>` 不变
  - 修改 `RequestOptionBuilder`：各 setter 方法改为设置 `Some(value.to_string())`
  - 修改 `Default` 实现：String 字段默认值从 `""` 变为 `None`
  - 更新 `req_option.rs` 中的测试：`assert_eq!(option.tenant_key, "")` → `assert_eq!(option.tenant_key, None)` 等
  - 运行 `cargo check -p openlark-core` 和 `cargo test -p openlark-core`

  **Must NOT do**:
  - 不修改 `header: HashMap<String, String>` 字段
  - 不修改 `bool` 类型字段
  - 不修改 Builder 的公开方法签名（仍接受 `impl ToString`）

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: 涉及结构体定义变更 + 大量测试适配，需要仔细处理
  - **Skills**: [`openlark-code-standards`]
    - `openlark-code-standards`: 确保 Option<String> 模式符合项目规范

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 6, 7)
  - **Blocks**: Task 8
  - **Blocked By**: Tasks 1-4

  **References**:
  - `crates/openlark-core/src/req_option.rs:4-15` — RequestOption 结构体定义，6 个 String 字段需改为 Option<String>
  - `crates/openlark-core/src/req_option.rs:28-90` — RequestOptionBuilder 各 setter 方法，需改为 `self.option.field = Some(value.to_string())`
  - `crates/openlark-core/src/req_option.rs:92-466` — 测试代码，所有 `assert_eq!(option.field, "")` 需改为 `assert_eq!(option.field, None)`
  - `crates/openlark-core/src/http.rs:162,191,248` — Transport 中读取 RequestOption 字段的逻辑，需适配 Option（Task 8 处理）
  - `crates/openlark-core/src/req_translator.rs:17` — RequestTranslator 使用 RequestOption（Task 8 处理）

  **Acceptance Criteria**:
  - [ ] RequestOption 的 6 个 String 字段全部改为 Option<String>
  - [ ] Builder setter 方法设置 Some(...)
  - [ ] Default 实现中 String 字段为 None
  - [ ] `cargo check -p openlark-core` 通过
  - [ ] `cargo test -p openlark-core -- req_option` 通过

  **QA Scenarios**:
  ```
  Scenario: Option<String> 默认值正确
    Tool: Bash
    Steps:
      1. grep -n "Option<String>" crates/openlark-core/src/req_option.rs — 应有 6 处
      2. cargo test -p openlark-core -- test_request_option_default — 应通过
      3. cargo test -p openlark-core -- test_request_option_builder_chaining — 应通过
    Expected Result: 6 个字段为 Option<String>，所有测试通过
    Evidence: .sisyphus/evidence/task-5-option-string.txt
  Scenario: Builder 设置 Some 值
    Tool: Bash
    Steps:
      1. cargo test -p openlark-core -- test_request_option_builder_tenant_key — 应通过
      2. cargo test -p openlark-core -- test_request_option_builder_empty_strings — 应通过（空字符串变为 Some("")）
    Expected Result: Builder 正确设置 Option 值
    Evidence: .sisyphus/evidence/task-5-builder-some.txt
  ```

  **Commit**: YES (groups with Wave 2)
  - Message: `refactor(core): RequestOption 字段改为 Option<String>`
  - Files: `req_option.rs`

- [x] 6. lib.rs 模块可见性收窄 + prelude 清理
  **What to do**:
  - 将以下 `pub mod` 改为 `pub(crate) mod` 或 `mod`：
    - `observability` → `pub(crate) mod`
    - `query_params` → `pub(crate) mod`
    - `request_builder` → `pub(crate) mod`
  - 保留 `pub mod`：`api`、`auth`、`config`、`constants`、`error`、`http`、`req_option`、`trait_system`、`validation`、`testing`（feature-gated）
  - 清理 prelude 中的第三方 re-export：
    - 删除 `pub use serde::{Deserialize, Serialize};`（第121行）
    - 删除 `pub use serde_json::Value;`（第122行）
    - 删除 `pub use std::collections::HashMap;`（第123行）
  - 运行 `cargo check --workspace` 确认无下游破坏
  **Must NOT do**:
  - 不修改 `trait_system` 可见性（必须保持 pub）
  - 不修改 `api`、`auth`、`config`、`constants`、`error`、`http` 可见性
  - 不删除 prelude 中的 openlark 自身类型 re-export
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 涉及可见性变更和下游影响评估，需要仔细处理
  - **Skills**: [`openlark-code-standards`, `openlark-naming`]
    - `openlark-code-standards`: 确保公开 API 最小化符合规范
    - `openlark-naming`: 确保模块导出和 prelude 命名规范
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 7)
  - **Blocks**: Tasks 8, 9, 10
  - **Blocked By**: Tasks 1-4
  **References**:
  - `crates/openlark-core/src/lib.rs:9-22` — 当前模块声明，需逐一评估可见性
  - `crates/openlark-core/src/lib.rs:107-124` — prelude 模块，需删除第三方 re-export
  - `crates/openlark-core/src/lib.rs:16` — `pub mod observability` → 检查下游是否使用
  - `crates/openlark-core/src/lib.rs:17` — `pub mod query_params` → 检查下游是否使用
  - `crates/openlark-core/src/lib.rs:18` — `pub mod request_builder` → 检查下游是否使用
  - 下游 crate 可能通过 `openlark_core::prelude::HashMap` 等路径引用第三方类型，需搜索确认
  **Acceptance Criteria**:
  - [ ] `observability`、`query_params`、`request_builder` 不再 pub
  - [ ] prelude 不再 re-export serde/serde_json/HashMap
  - [ ] `cargo check --workspace --all-features` 通过
  **QA Scenarios**:
  ```
  Scenario: 模块可见性收窄后编译通过
    Tool: Bash
    Steps:
      1. grep -n "pub mod observability" crates/openlark-core/src/lib.rs — 应无结果（已改为 pub(crate) 或 mod）
      2. grep -n "pub use serde" crates/openlark-core/src/lib.rs — 应无结果
      3. grep -n "pub use std::collections::HashMap" crates/openlark-core/src/lib.rs — 应无结果
      4. cargo check --workspace --all-features — 应通过
    Expected Result: 可见性收窄，第三方 re-export 已删除，编译通过
    Evidence: .sisyphus/evidence/task-6-visibility-narrowed.txt
  ```
  **Commit**: YES (groups with Wave 2)
  - Message: `refactor(core): 收窄模块可见性，清理 prelude 第三方 re-export`
  - Files: `lib.rs`
- [x] 7. 删除 validation 业务模块（file/pagination/password/uuid）
  **What to do**:
  - 删除以下文件：
    - `crates/openlark-core/src/validation/file.rs`
    - `crates/openlark-core/src/validation/pagination.rs`
    - `crates/openlark-core/src/validation/password.rs`
    - `crates/openlark-core/src/validation/uuid.rs`
  - 更新 `validation/mod.rs`：移除对应的 `pub mod` 和 `pub use` 声明
  - 搜索 workspace 确认无下游使用这些模块
  - 运行 `cargo check --workspace`
  **Must NOT do**:
  - 不删除 `validation/core.rs`（保留 validate_string_length 等通用函数）
  - 不删除 `validation/mod.rs`
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 6)
  - **Blocks**: Tasks 9, 10
  - **Blocked By**: None（独立操作，但归入 Wave 2 以简化依赖）
  **References**:
  - `crates/openlark-core/src/validation/mod.rs:7-10` — `pub mod file/pagination/password/uuid` 声明
  - `crates/openlark-core/src/validation/mod.rs:17-21` — 对应的 `pub use` re-export
  - `crates/openlark-core/src/validation/file.rs` — 文件验证（validate_file_extension 等）
  - `crates/openlark-core/src/validation/pagination.rs` — 分页验证
  - `crates/openlark-core/src/validation/password.rs` — 密码验证
  - `crates/openlark-core/src/validation/uuid.rs` — UUID 验证
  - 这些是业务逻辑，不属于 core 基础设施
  **Acceptance Criteria**:
  - [ ] 4 个业务验证文件已删除
  - [ ] `validation/mod.rs` 只保留 `pub mod core;` 和对应 re-export
  - [ ] `cargo check --workspace` 通过
  **QA Scenarios**:
  ```
  Scenario: 业务验证模块已删除
    Tool: Bash
    Steps:
      1. test ! -f crates/openlark-core/src/validation/file.rs — 文件不存在
      2. test ! -f crates/openlark-core/src/validation/pagination.rs — 文件不存在
      3. test ! -f crates/openlark-core/src/validation/password.rs — 文件不存在
      4. test ! -f crates/openlark-core/src/validation/uuid.rs — 文件不存在
      5. cargo check --workspace — 应通过
    Expected Result: 4 个文件已删除，编译通过
    Evidence: .sisyphus/evidence/task-7-validation-modules-deleted.txt
  ```
  **Commit**: YES (groups with Wave 2)
  - Message: `refactor(core): 删除 validation 业务模块，保留核心验证`
  - Files: `validation/file.rs`, `validation/pagination.rs`, `validation/password.rs`, `validation/uuid.rs`, `validation/mod.rs`
- [x] 8. openlark-core 内部适配（http.rs / req_translator.rs / improved_response_handler.rs）
  **What to do**:
  - 适配 `http.rs` 中所有读取 RequestOption String 字段的逻辑：
    - `option.tenant_key` → `option.tenant_key.as_deref().unwrap_or_default()` 或 `.is_some()` 判断
    - `option.user_access_token` / `app_access_token` / `tenant_access_token` 同理
    - `option.request_id` / `app_ticket` 同理
    - 重点关注 `validate` 函数（~第162行）和 `build_request` 逻辑（~第191行）
  - 适配 `req_translator.rs` 中 RequestTranslator 对 RequestOption 字段的读取
  - 适配 `improved_response_handler.rs` 中可能的 RequestOption 引用
  - 更新 `http.rs` 中的测试代码：所有 `option.field = "value".to_string()` 改为 `option.field = Some("value".to_string())`
  - 运行 `cargo check -p openlark-core` 和 `cargo test -p openlark-core`
  **Must NOT do**:
  - 不修改 Transport 的核心请求逻辑（只适配字段读取方式）
  - 不修改 API 请求/响应的序列化行为
  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: http.rs 有 58 处 RequestOption 引用（含大量测试），需仔细逐一适配
  - **Skills**: [`openlark-code-standards`]
  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (sequential start, then T9/T10 parallel)
  - **Blocks**: Tasks 9, 10
  - **Blocked By**: Tasks 5, 6, 7
  **References**:
  - `crates/openlark-core/src/http.rs:15` — `use req_option::RequestOption`
  - `crates/openlark-core/src/http.rs:42` — `option: Option<RequestOption>` 在 Transport 结构体
  - `crates/openlark-core/src/http.rs:104` — `option: RequestOption` 在方法参数
  - `crates/openlark-core/src/http.rs:162,191,248` — 读取 option 字段的核心逻辑
  - `crates/openlark-core/src/http.rs:319-985` — 测试代码，大量 `RequestOption::default()` 和字段赋值
  - `crates/openlark-core/src/req_translator.rs:7,17` — RequestTranslator 使用 RequestOption
  - `crates/openlark-core/src/req_translator.rs:47-153` — 测试代码中的 RequestOption 使用
  - 适配模式: `option.field.is_empty()` → `option.field.as_deref().map_or(true, |s| s.is_empty())`
  - 或更简洁: `option.field.as_deref().unwrap_or_default()`
  **Acceptance Criteria**:
  - [ ] `http.rs` 中所有 RequestOption 字段读取已适配 Option<String>
  - [ ] `req_translator.rs` 已适配
  - [ ] `cargo check -p openlark-core` 通过
  - [ ] `cargo test -p openlark-core` 全部通过
  **QA Scenarios**:
  ```
  Scenario: http.rs 适配后测试全部通过
    Tool: Bash
    Steps:
      1. cargo test -p openlark-core -- http — 应全部通过
      2. cargo test -p openlark-core -- req_translator — 应全部通过
      3. cargo check -p openlark-core — 应通过
    Expected Result: 所有内部模块适配完成，测试通过
    Evidence: .sisyphus/evidence/task-8-internal-adapted.txt
  ```
  **Commit**: YES (groups with Wave 3)
  - Message: `refactor(core): 适配内部模块使用 Option<String> RequestOption`
  - Files: `http.rs`, `req_translator.rs`, `improved_response_handler.rs`
- [x] 9. openlark-client 适配
  **What to do**:
  - 适配 `openlark-client` 中对 openlark-core 变更的引用：
    - 移除对已删除模块的 re-export（ErrorKind、capabilities、analysis、defaults、LegacyCoreError、ERROR_SYSTEM_VERSION）
    - 适配 RequestOption Option<String> 变更（如有直接字段访问）
    - 适配 prelude 变更（如有使用 `openlark_core::prelude::HashMap` 等路径）
    - 适配 validation 模块变更（如有使用已删除的 file/pagination/password/uuid 函数）
  - 运行 `cargo check -p openlark-client` 和 `cargo test -p openlark-client`
  **Must NOT do**:
  - 不修改 openlark-client 自身的公开 API 设计
  - 不修改 ServiceRegistry 模式
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 需要理解 client 对 core 的依赖关系并逐一适配
  - **Skills**: [`openlark-code-standards`]
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Task 10, after Task 8)
  - **Blocks**: F1-F4
  - **Blocked By**: Task 8
  **References**:
  - `crates/openlark-client/src/` — 整个 client crate 源码
  - `crates/openlark-client/Cargo.toml` — 对 openlark-core 的依赖声明
  - 已知 re-export: CoreError, ErrorCode, ErrorSeverity, ErrorTrait, ErrorType（来自之前的研究）
  - 搜索 `use openlark_core::` 找到所有引用点
  **Acceptance Criteria**:
  - [ ] `cargo check -p openlark-client` 通过
  - [ ] `cargo test -p openlark-client` 通过
  - [ ] 无对已删除模块的引用
  **QA Scenarios**:
  ```
  Scenario: client crate 编译和测试通过
    Tool: Bash
    Steps:
      1. grep -rn "ErrorKind\|LegacyCoreError\|ERROR_SYSTEM_VERSION" crates/openlark-client/src/ — 应无结果
      2. cargo check -p openlark-client — 应通过
      3. cargo test -p openlark-client — 应通过
    Expected Result: client 适配完成，零残留引用
    Evidence: .sisyphus/evidence/task-9-client-adapted.txt
  ```
  **Commit**: YES (groups with Wave 3)
  - Message: `refactor(client): 适配 openlark-core 重构变更`
  - Files: `openlark-client/src/*`
- [x] 10. 其余 16+ 下游 crate 批量适配
  **What to do**:
  - 逐一适配以下 crate 对 openlark-core 变更的引用：
    - `openlark-auth`: 已在 Task 4 处理测试，此处检查 src/ 中的 import
    - `openlark-security`: 修复 `error::ApiError` / `error::api_error` 引用
    - `openlark-communication`, `openlark-docs`, `openlark-hr`, `openlark-meeting`
    - `openlark-workflow`, `openlark-platform`, `openlark-application`
    - `openlark-mail`, `openlark-helpdesk`, `openlark-ai`, `openlark-analytics`
    - `openlark-cardkit`, `openlark-user`, `openlark-protocol`
  - 主要适配点：
    - 移除对已删除模块的 import（ErrorKind, capabilities, analysis, defaults）
    - 适配 prelude 变更（如有使用 `openlark_core::prelude::HashMap` 等路径，改为直接 `use std::collections::HashMap`）
    - 适配 validation 变更（如有使用已删除的 file/pagination/password/uuid 函数）
    - RequestOption 变更通常不影响下游（下游通过 Builder 使用）
  - 运行 `cargo check --workspace --all-features` 和 `cargo test --workspace`
  **Must NOT do**:
  - 不修改各 crate 自身的公开 API
  - 不修改各 crate 的业务逻辑
  - 只做最小化适配（修复编译错误）
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 涉及 16+ crate 的批量适配，工作量大但模式统一
  - **Skills**: [`openlark-code-standards`]
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Task 9, after Task 8)
  - **Blocks**: F1-F4
  - **Blocked By**: Task 8
  **References**:
  - `crates/openlark-security/src/` — 已知使用 `error::ApiError`
  - `crates/openlark-ai/src/`, `crates/openlark-docs/src/`, `crates/openlark-meeting/src/`, `crates/openlark-hr/src/` — 使用 trait_system（不受影响）
  - 所有下游 crate 的 `Cargo.toml` — 确认对 openlark-core 的依赖
  - 搜索策略: `grep -rn "use openlark_core::" crates/` 找到所有引用点
  - 搜索策略: `grep -rn "openlark_core::prelude::HashMap\|openlark_core::prelude::Serialize" crates/` 找第三方 re-export 使用
  **Acceptance Criteria**:
  - [ ] `cargo check --workspace --all-features` 通过
  - [ ] `cargo test --workspace` 通过
  - [ ] 无对已删除模块的引用
  **QA Scenarios**:
  ```
  Scenario: 全 workspace 编译和测试通过
    Tool: Bash
    Steps:
      1. cargo check --workspace --all-features — 应通过
      2. cargo test --workspace — 应全部通过
      3. grep -rn "ErrorKind\|LegacyCoreError\|ERROR_SYSTEM_VERSION" crates/ --include="*.rs" | grep -v target — 应无结果
    Expected Result: 所有 crate 适配完成，零编译错误
    Evidence: .sisyphus/evidence/task-10-workspace-adapted.txt
  Scenario: 无残留第三方 prelude 引用
    Tool: Bash
    Steps:
      1. grep -rn "openlark_core::prelude::HashMap\|openlark_core::prelude::Serialize\|openlark_core::prelude::Value" crates/ --include="*.rs" | grep -v target — 应无结果
    Expected Result: 零残留第三方 re-export 引用
    Evidence: .sisyphus/evidence/task-10-no-prelude-residual.txt
  ```
  **Commit**: YES (groups with Wave 3)
  - Message: `refactor: 适配所有下游 crate 至 openlark-core 新 API`
  - Files: `crates/openlark-*/src/*`
- [x] 11. 清理命名和 deprecated 代码
  **What to do**:
  - 重命名 `improved_response_handler.rs` 为 `response_handler.rs`（去掉 improved_ 前缀）
  - 更新 `lib.rs` 中的 `mod improved_response_handler;` 为 `mod response_handler;`
  - 删除 `error/mod.rs` 中的 `pub type LarkAPIError = CoreError;` 别名（如 Task 2 未处理）
  - 如果 prelude 中有 `LarkAPIError` re-export，也一并删除
  - 搜索 workspace 中所有 `LarkAPIError` 引用，替换为 `CoreError`
  - 运行 `cargo check --workspace` 和 `cargo test --workspace`
  **Must NOT do**:
  - 不修改 CoreError 枚举本身
  - 不修改 ErrorCode 枚举
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`openlark-naming`]
    - `openlark-naming`: 确保命名符合项目规范
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Task 12)
  - **Blocks**: F1-F4
  - **Blocked By**: Task 8
  **References**:
  - `crates/openlark-core/src/improved_response_handler.rs` — 需重命名为 `response_handler.rs`
  - `crates/openlark-core/src/lib.rs:27` — `mod improved_response_handler;` 需更新
  - `crates/openlark-core/src/lib.rs:114` — prelude 中 `LarkAPIError` re-export
  - 搜索: `grep -rn "LarkAPIError" crates/` 找到所有引用点
  **Acceptance Criteria**:
  - [ ] `improved_response_handler.rs` 已重命名为 `response_handler.rs`
  - [ ] workspace 中无 `LarkAPIError` 引用（全部替换为 `CoreError`）
  - [ ] `cargo check --workspace` 通过
  - [ ] `cargo test --workspace` 通过
  **QA Scenarios**:
  ```
  Scenario: 命名清理完成
    Tool: Bash
    Steps:
      1. test ! -f crates/openlark-core/src/improved_response_handler.rs — 旧文件不存在
      2. test -f crates/openlark-core/src/response_handler.rs — 新文件存在
      3. grep -rn "LarkAPIError" crates/ --include="*.rs" | grep -v target — 应无结果
      4. cargo check --workspace — 应通过
    Expected Result: 命名清理完成，零残留
    Evidence: .sisyphus/evidence/task-11-naming-cleaned.txt
  ```
  **Commit**: YES (groups with Wave 4)
  - Message: `refactor(core): 清理命名（重命名 response_handler，删除 LarkAPIError 别名）`
  - Files: `response_handler.rs`, `lib.rs`, 下游引用文件
- [x] 12. 更新 AGENTS.md 知识库
  **What to do**:
  - 更新 `crates/openlark-core/AGENTS.md`：
    - 移除对已删除模块的引用（kinds.rs、capabilities、analysis、defaults、file.rs、pagination.rs、password.rs、uuid.rs）
    - 更新 STRUCTURE 部分反映新的目录结构
    - 更新 WHERE TO LOOK 表格
    - 更新错误处理示例（移除 ErrorKind 引用）
    - 更新验证示例（只展示宏用法）
    - 添加 RequestOption Option<String> 的说明
  - 更新根目录 `AGENTS.md`（如有对 core 内部模块的引用）
  **Must NOT do**:
  - 不修改代码文件
  - 不添加与本次重构无关的内容
  **Recommended Agent Profile**:
  - **Category**: `writing`
    - Reason: 纯文档更新任务
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Task 11)
  - **Blocks**: F1-F4
  - **Blocked By**: Task 8
  **References**:
  - `crates/openlark-core/AGENTS.md` — 当前知识库文件
  - `AGENTS.md` — 根目录知识库
  - 本计划文件中的所有变更描述 — 作为更新内容的来源
  **Acceptance Criteria**:
  - [ ] `crates/openlark-core/AGENTS.md` 已更新，无对已删除模块的引用
  - [ ] STRUCTURE 部分反映实际目录结构
  **QA Scenarios**:
  ```
  Scenario: AGENTS.md 无过时引用
    Tool: Bash
    Steps:
      1. grep -n "kinds.rs\|ErrorKind\|capabilities\|file.rs\|pagination.rs\|password.rs\|uuid.rs" crates/openlark-core/AGENTS.md — 应无结果
      2. grep -n "validate_required" crates/openlark-core/AGENTS.md — 应只有宏用法
    Expected Result: 知识库与代码一致
    Evidence: .sisyphus/evidence/task-12-agents-updated.txt
  ```
  **Commit**: YES (groups with Wave 4)
  - Message: `docs(core): 更新 AGENTS.md 反映重构后的模块结构`
  - Files: `crates/openlark-core/AGENTS.md`, `AGENTS.md`
---

## Final Verification Wave

> 4 review agents run in PARALLEL. ALL must APPROVE.

- [x] F1. **Plan Compliance Audit** — `oracle`
  Read plan. For each "Must Have": verify exists. For each "Must NOT Have": search for forbidden patterns. Output: `APPROVE/REJECT`

- [x] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo clippy --workspace --all-features`. Review changed files for unwrap()/expect(), dead code, unused imports. Output: `APPROVE/REJECT`

- [x] F3. **Full Workspace Build + Test** — `unspecified-high`
  Run `cargo check --workspace --all-features` and `cargo test --workspace`. Output: `APPROVE/REJECT`

- [x] F4. **Scope Fidelity Check** — `deep`
  For each task: verify actual changes match spec. Check "Must NOT do" compliance. Output: `APPROVE/REJECT`

---

## Commit Strategy

- **Wave 1**: `refactor(core): 删除错误系统冗余模块和统一验证` — error/kinds.rs, error/mod.rs, validation/core.rs, constants.rs
- **Wave 2**: `refactor(core): RequestOption 改用 Option + 收窄公开 API` — req_option.rs, lib.rs
- **Wave 3**: `refactor: 适配 core 内部和下游 crate` — http.rs, 下游 crate
- **Wave 4**: `refactor(core): 清理命名和 deprecated 代码` — misc

---

## Success Criteria

### Verification Commands
```bash
cargo check -p openlark-core --all-features  # Expected: 0 errors
cargo test -p openlark-core                   # Expected: all pass
cargo check --workspace --all-features        # Expected: 0 errors
cargo test --workspace                        # Expected: all pass
cargo clippy --workspace --all-features       # Expected: 0 new warnings
```

### Final Checklist
- [x] ErrorKind 模块已删除
- [x] capabilities/analysis/defaults 模块已删除
- [x] ERROR_SYSTEM_VERSION/LegacyCoreError 已删除
- [x] validate_required 函数已删除（只保留宏）
- [x] RequestOption 使用 Option<String>
- [x] 公开 API 最小化
- [x] 所有下游 crate 编译通过
- [x] 所有现有测试通过
