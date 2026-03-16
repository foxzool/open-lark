# openlark-hr 测试补全计划

## TL;DR

> **核心目标**: 为 openlark-hr crate 建立完整的测试基础设施，采用"核心路径优先"策略分批补充测试。
>
> **当前状态**: 862 个源文件，55,206 行代码，仅有 5 个内联测试模块，测试覆盖率接近 0%。
>
> **实施策略**: 6 批次渐进式实施，从 attendance 模块开始建立模式，逐步覆盖核心 API。
>
> **预期成果**: 核心 API 覆盖率 >= 80%，建立可持续的测试开发流程。
>
> **预估工时**: Wave 1-2 (3-5天) + 每批次 3-7天，总计 4-6 周（按标准节奏）。
> **并行执行**: YES - Wave 1 任务可并行，后续批次依赖前序完成。
> **关键路径**: 任务 1 → 任务 2 → 任务 3 (试点验证) → 任务 4+ (批次实施)

---

## Context

### Original Request
用户希望为测试覆盖率最低的 openlark-hr crate 补充测试，采用"核心路径优先"策略，作为长期持续改进项目。

### Interview Summary
**关键讨论**:
- **目标 Crate**: openlark-hr（HR 人力资源模块）
- **策略**: 核心路径优先（只测试最重要的 20% API）
- **时间框架**: 长期持续改进（非一次性完成）
- **批次大小**: 每批次 1-2 个业务模块

**调研发现**:
- openlark-hr: 862 文件，55,206 行代码，590 API，测试覆盖率接近 0
- openlark-meeting: 177 文件，10,359 行代码，126 API，测试覆盖率接近 0  
- openlark-application: 102 文件，4,651 行代码，93 API，测试覆盖率接近 0

### Metis Review
**识别的关键缺口** (已在本计划中处理):
1. **测试依赖缺失**: openlark-hr/Cargo.toml 缺少 rstest、wiremock、mockall 等测试依赖
2. **测试目录结构未定义**: 需要建立 tests/unit/hr/ 目录结构
3. **核心路径定义模糊**: 需要在每个批次前明确"核心 API"清单
4. **Mock 数据维护策略**: 采用静态 JSON fixture，定期脚本校验
5. **测试质量标准**: 定义了明确的覆盖率目标（核心 API >= 80%）和质量门

**应用的防护措施**:
- 每个 API 最多 5 个测试用例限制
- 禁止直接调用真实飞书 API
- 使用 `#[rstest]` 进行参数化测试
- 每个批次后验证覆盖率目标

---

## Work Objectives

### Core Objective
为 openlark-hr crate 建立完整的测试基础设施，采用核心路径优先策略，分批次为 HR 模块的核心 API 补充测试，最终达到核心 API 覆盖率 >= 80%。

### Concrete Deliverables
1. **测试基础设施**: Cargo.toml dev-dependencies + tests/unit/hr/ 目录结构
2. **测试指南文档**: docs/hr-testing-guide.md（测试模式、命名规范、代码审查 checklist）
3. **试点批次**: attendance 模块全部 48 个 API 的完整测试
4. **批次化实施**: payroll (12 API) + hire 核心 API (~30 个) + feishu_people 核心 API (~40 个)
5. **持续维护机制**: 代码审查 checklist + fixture 校验脚本

### Definition of Done
- [x] `cargo test --package openlark-hr` 全部通过
- [x] `cargo llvm-cov --package openlark-hr --fail-under-lines 60` 通过
- [x] `cargo clippy --package openlark-hr --tests -- -D warnings` 零警告
- [x] 每个批次定义的"核心 API"清单覆盖率 >= 80%
- [x] 测试执行时间 < 5 分钟
- [x] 测试代码与生产代码比例 < 1:3

### Must Have
1. **测试依赖**: rstest, wiremock, mockall, serial_test, tempfile
2. **目录结构**: tests/unit/hr/{module}_tests.rs 模式
3. **核心 API 定义**: 每个批次前明确列出要测试的 API 清单
4. **Builder 测试**: 每个核心 API 必须测试 Builder 基本功能 + 序列化/反序列化
5. **错误场景**: 每个核心 API 至少一个成功场景 + 一个错误场景
6. **覆盖率验证**: 使用 `cargo llvm-cov` 验证覆盖率目标

### Must NOT Have (Guardrails)
1. **非核心 API 测试**: 严格按照"核心 API 清单"执行，禁止范围蔓延
2. **直接调用真实 API**: 必须使用 wiremock mock HTTP 层
3. **测试过度工程化**: 每个 API 最多 5 个测试用例
4. **复杂 fixture 生成器**: 使用静态 JSON 文件，禁止动态生成
5. **跨模块测试**: 保持测试隔离，禁止测试涉及多个 HR 模块交互
6. **大量性能测试**: 每个模块最多 1 个性能测试
7. **非目标 crate 测试**: 不在本计划中测试 openlark-meeting 或 openlark-application

---

## Verification Strategy

### Test Decision
- **Infrastructure exists**: NO - 需要先建立
- **Automated tests**: TDD 不适用（测试后补）
- **Framework**: rstest (参数化测试) + wiremock (HTTP mock)
- **Coverage tool**: cargo-llvm-cov

### QA Policy
Every task MUST include agent-executed QA scenarios. Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

| Deliverable Type | Verification Tool | Method |
|------------------|-------------------|--------|
| Cargo.toml 修改 | Bash | `cargo check --package openlark-hr` 编译成功 |
| 目录结构 | Bash | `ls tests/unit/hr/` 验证目录存在 |
| 测试文件 | Bash | `cargo test --package openlark-hr -- {pattern}` 通过 |
| 覆盖率 | Bash | `cargo llvm-cov --package openlark-hr --fail-under-lines {N}` 通过 |
| Lint | Bash | `cargo clippy --package openlark-hr --tests -- -D warnings` 零警告 |
| 文档 | Read | 人工审查 docs/hr-testing-guide.md 完整性 |

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately - 基础设施，2 个并行任务):
├── Task 1: 测试基础设施 (Cargo.toml + 目录结构) [quick]
└── Task 2: 测试指南文档 [writing]

Wave 2 (After Wave 1 - 试点验证，验证模式可行性):
├── Task 3: attendance 模块测试 (48 API) [unspecified-high]
    └── 关键里程碑：验证测试模式可行，调整后续批次规模

Wave 3 (After Wave 2 - 简单模块，建立信心):
├── Task 4: payroll 模块测试 (12 API) [unspecified-low]

Wave 4 (After Wave 3 - 核心大模块，分阶段):
├── Task 5: hire 核心 API 测试 (~30 API) [unspecified-high]
└── Task 6: feishu_people 核心 API 测试 (~40 API) [unspecified-high]

Wave 5 (After Wave 4 - 剩余模块):
├── Task 7: performance + okr 模块测试 [unspecified-low]
└── Task 8: compensation_management + ehr 模块测试 [unspecified-low]

Wave FINAL (After ALL tasks - 独立审查，4 个并行):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Coverage verification (unspecified-high)
└── Task F4: Scope fidelity check (deep)

Critical Path: Task 1 & 2 (并行) → Task 3 (试点) → Task 4-8 (批次) → F1-F4
Parallel Speedup: Wave 1 100% 并行，后续批次串行（依赖验证）
Max Concurrent: 2 (Wave 1)
```

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|------------|--------|------|
| 1 | — | 3, 4, 5, 6, 7, 8 | 1 |
| 2 | — | 3, 4, 5, 6, 7, 8 | 1 |
| 3 | 1, 2 | 4, 5, 6, 7, 8 | 2 |
| 4 | 3 | 5, 6, 7, 8 | 3 |
| 5 | 4 | 7, 8 | 4 |
| 6 | 4 | 7, 8 | 4 |
| 7 | 5, 6 | — | 5 |
| 8 | 5, 6 | — | 5 |
| F1-F4 | 1-8 | — | FINAL |

### Agent Dispatch Summary

| Wave | # Parallel | Tasks → Agent Category |
|------|------------|----------------------|
| 1 | **2** | T1 → `quick`, T2 → `writing` |
| 2 | **1** | T3 → `unspecified-high` |
| 3 | **1** | T4 → `unspecified-low` |
| 4 | **2** | T5 → `unspecified-high`, T6 → `unspecified-high` |
| 5 | **2** | T7 → `unspecified-low`, T8 → `unspecified-low` |
| FINAL | **4** | F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep` |

---

## TODOs

> Implementation + Test = ONE Task. Never separate.
> EVERY task MUST have: Recommended Agent Profile + Parallelization info + QA Scenarios.
> **A task WITHOUT QA Scenarios is INCOMPLETE. No exceptions.**

### Wave 1: 基础设施（并行执行）

- [x] **1. 建立测试基础设施**

  **What to do**:
  - 在 `crates/openlark-hr/Cargo.toml` 添加 dev-dependencies:
    ```toml
    [dev-dependencies]
    rstest = { workspace = true }
    wiremock = { workspace = true }
    mockall = { workspace = true }
    serial_test = "3.2"  # 注意：workspace 中不存在，使用明确版本
    tempfile = { workspace = true }
    # 注意：tokio-test 不需要，使用 tokio 的 test 特性即可
    ```
  - 创建 `tests/unit/hr/` 目录结构
  - 创建 `tests/unit/hr/mod.rs`（模块导出）
  - 在 `crates/openlark-hr/Cargo.toml` 添加 `[[test]]` 配置使单元测试可被 Cargo 识别：
    ```toml
    [[test]]
    name = "unit"
    path = "../../tests/unit/hr/mod.rs"
    ```
  - 创建测试模板文件 `tests/unit/hr/__template__.rs`

  **Must NOT do**:
  - 不要修改生产代码
  - 不要添加测试用例（这是任务 3+ 的工作）
  - 不要修改其他 crate 的 Cargo.toml

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 配置修改和目录创建，不涉及复杂逻辑
  - **Skills**: [`openlark-code-standards`]
    - `openlark-code-standards`: 确保 Cargo.toml 格式符合项目规范
  - **Skills Evaluated but Omitted**:
    - `openlark-api`: 不涉及 API 实现
    - `openlark-validation-style`: 不涉及验证逻辑

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Task 2)
  - **Blocks**: Task 3, 4, 5, 6, 7, 8
  - **Blocked By**: None

  **References**:
  - `crates/openlark-hr/Cargo.toml` - 参考现有依赖格式
  - `crates/openlark-communication/Cargo.toml:42-48` - 参考其他 crate 的 dev-dependencies 写法
  - `tests/unit/im/mod.rs` - 参考模块导出模式
  - `tests/unit/im/builder_tests.rs:1-30` - 参考测试文件头部结构

  **WHY Each Reference Matters**:
  - `Cargo.toml`: 需要与 workspace 依赖保持一致，使用 `{ workspace = true }` 语法
  - `tests/unit/im/mod.rs`: 遵循现有测试模块的组织模式（pub mod xxx）
  - `builder_tests.rs`: 参考测试文件的 use 语句、mod tests 结构、rstest 属性用法

  **Acceptance Criteria**:

  **QA Scenarios**:
  ```
  Scenario: Cargo.toml 配置正确
    Tool: Bash
    Preconditions: 工作目录干净
    Steps:
      1. 运行 `cargo check --package openlark-hr`
      2. 验证输出无错误
    Expected Result: 命令返回 exit code 0，无编译错误
    Failure Indicators: 依赖未找到、版本冲突、语法错误
    Evidence: .sisyphus/evidence/task-1-cargo-check.txt

  Scenario: 目录结构正确
    Tool: Bash
    Preconditions: 任务完成
    Steps:
      1. 运行 `ls -la tests/unit/hr/`
      2. 验证目录存在且包含 mod.rs
    Expected Result: 显示 tests/unit/hr/mod.rs 文件
    Failure Indicators: 目录不存在、mod.rs 缺失
    Evidence: .sisyphus/evidence/task-1-directory-structure.txt

  Scenario: 编译测试模块
    Tool: Bash
    Preconditions: Cargo.toml 和目录已创建
    Steps:
      1. 运行 `cargo test --package openlark-hr --no-run`
      2. 验证编译成功
    Expected Result: 编译成功，显示 "test result: ok. 0 passed; 0 failed"
    Failure Indicators: 编译错误、依赖缺失
    Evidence: .sisyphus/evidence/task-1-test-compile.txt
  ```

  **Commit**: YES
  - Message: `chore(tests): add test dependencies and infrastructure for openlark-hr`
  - Files: `crates/openlark-hr/Cargo.toml`, `tests/unit/hr/mod.rs`, `tests/unit/hr/__template__.rs`
  - Pre-commit: `cargo check --package openlark-hr`

---

- [x] **2. 编写测试指南文档**

  **What to do**:
  - **创建 `docs/` 目录**（根目录目前不存在，需要先创建）
  - 创建 `docs/hr-testing-guide.md` 文档，包含：
    - 目录结构说明（tests/unit/hr/ 组织方式）
    - 测试命名规范（文件命名、函数命名）
    - Builder 测试模式（如何测试 Builder 链式调用）
    - Fixture 使用指南（JSON mock 数据管理）
    - 代码审查 checklist（每个 PR 的检查项）
  - 包含可运行的代码示例（参考 tests/unit/im/builder_tests.rs 模式）

  **Must NOT do**:
  - 不要编写通用 Rust 测试教程（假设读者已了解 Rust 测试基础）
  - 不要复制粘贴整个测试文件（只展示关键模式）
  - 不要涉及 openlark-hr 的业务逻辑解释（只关注测试模式）

  **Recommended Agent Profile**:
  - **Category**: `writing`
    - Reason: 文档编写任务
  - **Skills**: []
    - 无需特殊技能，但需要阅读现有测试代码提取模式
  - **Skills Evaluated but Omitted**:
    - `openlark-code-standards`: 不涉及代码规范审查

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Task 1)
  - **Blocks**: Task 3, 4, 5, 6, 7, 8
  - **Blocked By**: None

  **References**:
  - `tests/unit/im/builder_tests.rs` - 提取 Builder 测试模式
  - `tests/unit/im/message_tests.rs` - 提取 HTTP mock 测试模式
  - `API_VALIDATION_REPORT.md` - 参考文档格式和风格
  - `crates/openlark-docs/AGENTS.md` - 参考知识库文档结构
  - 注意：需要先创建 `docs/` 目录（根目录目前不存在）

  **WHY Each Reference Matters**:
  - `builder_tests.rs`: 展示如何测试 Builder 模式（链式调用、默认值、边界条件）
  - `message_tests.rs`: 展示如何使用 wiremock 进行 HTTP mock 测试
  - `API_VALIDATION_REPORT.md`: 遵循项目的文档风格（中文、清晰结构）
  - `AGENTS.md`: 展示知识库文档的组织方式（结构、链接、更新记录）

  **Acceptance Criteria**:

  **QA Scenarios**:
  ```
  Scenario: 文档完整性检查
    Tool: Read
    Preconditions: 文档已创建
    Steps:
      1. 读取 docs/hr-testing-guide.md
      2. 验证包含：目录结构、命名规范、Builder 模式、Fixture 指南、Checklist
      3. 验证至少包含 1 个可运行的代码示例
    Expected Result: 所有章节存在，代码示例语法正确
    Failure Indicators: 章节缺失、示例无法编译
    Evidence: .sisyphus/evidence/task-2-document-check.md

  Scenario: 代码示例可运行
    Tool: Bash
    Preconditions: 文档已创建
    Steps:
      1. 从文档中提取 Rust 代码示例
      2. 创建临时测试文件并编译
      3. 运行 `cargo test --test temp_test`
    Expected Result: 代码示例编译成功并通过测试
    Failure Indicators: 编译错误、测试失败
    Evidence: .sisyphus/evidence/task-2-example-compilation.txt

  Scenario: 文档格式正确
    Tool: Bash
    Preconditions: 文档已创建
    Steps:
      1. 运行 `markdownlint docs/hr-testing-guide.md`（如果有）
      2. 验证无格式错误
    Expected Result: 文档格式符合 Markdown 规范
    Failure Indicators: 格式警告、链接失效
    Evidence: .sisyphus/evidence/task-2-markdown-lint.txt
  ```

  **Commit**: YES
  - Message: `docs(tests): add testing guide for openlark-hr`
  - Files: `docs/hr-testing-guide.md`
  - Pre-commit: 人工审查文档完整性

---

### Wave 2: 试点批次（验证模式）

- [x] **3. 实现 attendance 模块测试（试点批次）**

  **What to do**:
  - 为 attendance 模块**全部 48 个 API** 编写测试
  - 每个 API 测试内容：
    - Builder 基本功能（链式调用、参数设置）
    - 序列化/反序列化（Request/Response JSON 转换）
    - 核心 API 额外添加 HTTP mock 测试（使用 wiremock）
  - 使用参数化测试（`#[rstest]`）覆盖边界条件
  - 测试文件: `tests/unit/hr/attendance_tests.rs`
  - **核心 API 清单**（必须测试）：
    - `CreateAttendanceGroupRequest` - 创建考勤组
    - `UpdateAttendanceGroupRequest` - 更新考勤组
    - `GetAttendanceGroupRequest` - 获取考勤组
    - `CreateShiftRequest` - 创建班次
    - `GetUserTaskRequest` - 获取用户考勤任务
    - `GetUserStatsDataRequest` - 获取用户统计数据
    - `CreateUserApprovalRequest` - 创建审批
    - `GetApprovalInfoRequest` - 获取审批信息

  **Must NOT do**:
  - 不要测试非核心 API（严格遵循上述清单）
  - 每个 API 不要超过 5 个测试用例
  - 不要直接调用真实飞书 API（必须使用 wiremock）
  - 不要创建复杂的 fixture 生成器（使用静态 JSON）

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 涉及大量 API 测试实现，需要理解 Builder 模式和 HTTP mock
  - **Skills**: [`openlark-api`, `openlark-validation-style`]
    - `openlark-api`: 理解 HR API 的 Request/Response 结构
    - `openlark-validation-style`: 遵循项目的验证和测试模式
  - **Skills Evaluated but Omitted**:
    - `openlark-code-standards`: 主要是测试实现，不涉及架构规范

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2 (单独执行，验证模式)
  - **Blocks**: Task 4, 5, 6, 7, 8
  - **Blocked By**: Task 1, 2

  **References**:
  - `tests/unit/im/builder_tests.rs:1-150` - Builder 测试模式（必须遵循）
  - `tests/unit/im/message_tests.rs:1-200` - HTTP mock 测试模式（核心 API 使用）
  - `crates/openlark-hr/src/attendance/` - 被测试的 API 实现
  - `crates/openlark-hr/src/common/models.rs` - 公共模型结构
  - `docs/hr-testing-guide.md` (Task 2 产出) - 本任务的执行指南

  **WHY Each Reference Matters**:
  - `builder_tests.rs`: 定义了 Builder 测试的标准模式（链式调用测试、默认值测试、参数化测试）
  - `message_tests.rs`: 展示了如何使用 wiremock 模拟飞书 API 响应
  - `attendance/`: 需要理解每个 API 的 Request/Response 结构来编写对应的测试
  - `models.rs`: 公共模型（如日期、时间）的测试模式
  - `hr-testing-guide.md`: 本任务的执行指南，必须遵循其中的规范

  **Acceptance Criteria**:

  **QA Scenarios**:
  ```
  Scenario: 所有 attendance 测试通过
    Tool: Bash
    Preconditions: 测试文件已编写
    Steps:
      1. 运行 `cargo test --package openlark-hr -- attendance`
      2. 验证所有测试通过
    Expected Result: 显示 "test result: ok. N passed; 0 failed; 0 ignored"（N >= 48）
    Failure Indicators: 测试失败、编译错误
    Evidence: .sisyphus/evidence/task-3-attendance-tests.txt

  Scenario: 覆盖率达标
    Tool: Bash
    Preconditions: 测试全部通过
    Steps:
      1. 运行 `cargo llvm-cov --package openlark-hr --fail-under-lines 60`
      2. 验证覆盖率 >= 60%
    Expected Result: 命令返回 exit code 0，显示覆盖率报告
    Failure Indicators: 覆盖率低于 60%、命令失败
    Evidence: .sisyphus/evidence/task-3-coverage-report.html

  Scenario: Lint 零警告
    Tool: Bash
    Preconditions: 测试全部通过
    Steps:
      1. 运行 `cargo clippy --package openlark-hr --tests -- -D warnings`
      2. 验证无警告
    Expected Result: 显示 "Finished dev [unoptimized + debuginfo] target(s)" 无警告
    Failure Indicators: clippy 警告、错误
    Evidence: .sisyphus/evidence/task-3-clippy-check.txt

  Scenario: 测试执行时间合理
    Tool: Bash
    Preconditions: 测试全部通过
    Steps:
      1. 运行 `time cargo test --package openlark-hr -- attendance`
      2. 验证执行时间 < 60 秒
    Expected Result: real time < 60s
    Failure Indicators: 测试执行时间过长（> 60s）
    Evidence: .sisyphus/evidence/task-3-execution-time.txt
  ```

  **Evidence to Capture**:
  - [ ] task-3-attendance-tests.txt - 完整测试输出
  - [ ] task-3-coverage-report.html - 覆盖率报告
  - [ ] task-3-clippy-check.txt - Lint 检查结果
  - [ ] task-3-execution-time.txt - 执行时间记录

  **Commit**: YES
  - Message: `test(attendance): add tests for attendance module (48 APIs)`
  - Files: `tests/unit/hr/attendance_tests.rs`
  - Pre-commit: `cargo test --package openlark-hr -- attendance && cargo clippy --package openlark-hr --tests -- -D warnings`

---

### Wave 3: 简单模块（建立信心）

- [x] **4. 实现 payroll 模块测试**

  **What to do**:
  - 为 payroll 模块**全部 12 个 API** 编写测试
  - 遵循任务 3 建立的测试模式
  - 每个 API 测试 Builder 基本功能 + 序列化/反序列化
  - 核心 API（工资单创建、查询）添加 HTTP mock 测试
  - 测试文件: `tests/unit/hr/payroll_tests.rs`
  - **核心 API 清单**：
    - `CreatePayrollRequest` - 创建工资单
    - `GetPayrollRequest` - 获取工资单
    - `UpdatePayrollRequest` - 更新工资单
    - `ListPayrollsRequest` - 列表查询

  **Must NOT do**:
  - 不要偏离任务 3 建立的测试模式（除非发现问题并记录）
  - 不要测试非工资单相关的 API

  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
    - Reason: payroll 模块 API 数量少（12 个），逻辑相对简单
  - **Skills**: [`openlark-api`, `openlark-validation-style`]
    - `openlark-api`: 理解 payroll API 结构
    - `openlark-validation-style`: 遵循验证模式
  - **Skills Evaluated but Omitted**: —

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (单独执行)
  - **Blocks**: Task 5, 6, 7, 8
  - **Blocked By**: Task 3

  **References**:
  - `tests/unit/hr/attendance_tests.rs` (Task 3 产出) - 参考已建立的测试模式
  - `crates/openlark-hr/src/payroll/` - 被测试的 API 实现
  - `docs/hr-testing-guide.md` - 测试指南

  **WHY Each Reference Matters**:
  - `attendance_tests.rs`: 这是本项目的标准测试模式，必须严格遵循
  - `payroll/`: 理解 payroll API 的特定字段和业务逻辑（如工资计算）

  **Acceptance Criteria**:

  **QA Scenarios**:
  ```
  Scenario: 所有 payroll 测试通过
    Tool: Bash
    Preconditions: 测试文件已编写
    Steps:
      1. 运行 `cargo test --package openlark-hr -- payroll`
      2. 验证所有测试通过
    Expected Result: "test result: ok. N passed; 0 failed"（N >= 12）
    Evidence: .sisyphus/evidence/task-4-payroll-tests.txt

  Scenario: 覆盖率达标
    Tool: Bash
    Preconditions: 测试通过
    Steps:
      1. 运行 `cargo llvm-cov --package openlark-hr --fail-under-lines 70`
    Expected Result: 覆盖率 >= 70%
    Evidence: .sisyphus/evidence/task-4-coverage-report.html
  ```

  **Commit**: YES
  - Message: `test(payroll): add tests for payroll module (12 APIs)`
  - Files: `tests/unit/hr/payroll_tests.rs`

---

### Wave 4: 核心大模块（分阶段）

- [x] **5. 实现 hire 核心 API 测试**

  **What to do**:
  - hire 模块有 183 个 API，但只测试**核心 API（约 30 个）**
  - **核心 API 清单**（必须覆盖招聘主流程）：
    - 候选人管理: CreateCandidate, GetCandidate, UpdateCandidate, ListCandidates
    - 面试管理: CreateInterview, GetInterview, UpdateInterview, ListInterviews
    - Offer 管理: CreateOffer, GetOffer, UpdateOffer, ListOffers
    - 申请管理: CreateApplication, GetApplication, ListApplications
    - 职位管理: CreateJob, GetJob, ListJobs
    - 评价管理: CreateEvaluation, GetEvaluation
  - 每个 API 测试 Builder + 序列化 + HTTP mock
  - 测试文件: `tests/unit/hr/hire_tests.rs`

  **Must NOT do**:
  - 不要测试全部 183 个 API（严格按照核心清单）
  - 不要测试边缘功能（如内部推荐、人才库搜索）

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 涉及 30 个 API，需要理解复杂的招聘业务流程
  - **Skills**: [`openlark-api`, `openlark-validation-style`]
  - **Skills Evaluated but Omitted**: —

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 6)
  - **Parallel Group**: Wave 4
  - **Blocks**: Task 7, 8
  - **Blocked By**: Task 4

  **References**:
  - `crates/openlark-hr/src/hire/` - 被测试的 API 实现
  - `tests/unit/hr/attendance_tests.rs` - 测试模式
  - `docs/hr-testing-guide.md` - 测试指南

  **Acceptance Criteria**:

  **QA Scenarios**:
  ```
  Scenario: 所有 hire 核心测试通过
    Tool: Bash
    Preconditions: 测试文件已编写
    Steps:
      1. 运行 `cargo test --package openlark-hr -- hire`
    Expected Result: "test result: ok. N passed; 0 failed"（N >= 30）
    Evidence: .sisyphus/evidence/task-5-hire-tests.txt

  Scenario: 核心 API 覆盖率达标
    Tool: Bash
    Preconditions: 测试通过
    Steps:
      1. 运行 `cargo llvm-cov --package openlark-hr --fail-under-lines 80`
    Expected Result: 核心 API 覆盖率 >= 80%
    Evidence: .sisyphus/evidence/task-5-coverage-report.html
  ```

  **Commit**: YES
  - Message: `test(hire): add tests for hire core APIs (~30 APIs)`
  - Files: `tests/unit/hr/hire_tests.rs`

---

- [x] **6. 实现 feishu_people 核心 API 测试**

  **What to do**:
  - feishu_people 模块有 285 个 API，但只测试**核心 API（约 40 个）**
  - **核心 API 清单**（CoreHR 主流程）：
    - 员工管理: CreateEmployee, GetEmployee, UpdateEmployee, ListEmployees, DeleteEmployee
    - 部门管理: CreateDepartment, GetDepartment, UpdateDepartment, ListDepartments
    - 岗位管理: CreatePosition, GetPosition, UpdatePosition, ListPositions
    - 合同管理: CreateContract, GetContract, ListContracts
    - 工作地点: CreateWorkLocation, GetWorkLocation, ListWorkLocations
  - 测试文件: `tests/unit/hr/feishu_people_tests.rs`

  **Must NOT do**:
  - 不要测试全部 285 个 API
  - 不要测试复杂的组织关系查询（如汇报线）

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: CoreHR 是最复杂的模块，涉及 40 个核心 API
  - **Skills**: [`openlark-api`, `openlark-validation-style`]
  - **Skills Evaluated but Omitted**: —

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 5)
  - **Parallel Group**: Wave 4
  - **Blocks**: Task 7, 8
  - **Blocked By**: Task 4

  **References**:
  - `crates/openlark-hr/src/feishu_people/` - 被测试的 API 实现
  - `tests/unit/hr/attendance_tests.rs` - 测试模式

  **Acceptance Criteria**:

  **QA Scenarios**:
  ```
  Scenario: 所有 feishu_people 核心测试通过
    Tool: Bash
    Preconditions: 测试文件已编写
    Steps:
      1. 运行 `cargo test --package openlark-hr -- feishu_people`
    Expected Result: "test result: ok. N passed; 0 failed"（N >= 40）
    Evidence: .sisyphus/evidence/task-6-feishu-people-tests.txt

  Scenario: 核心 API 覆盖率达标
    Tool: Bash
    Preconditions: 测试通过
    Steps:
      1. 运行 `cargo llvm-cov --package openlark-hr --fail-under-lines 80`
    Expected Result: 核心 API 覆盖率 >= 80%
    Evidence: .sisyphus/evidence/task-6-coverage-report.html
  ```

  **Commit**: YES
  - Message: `test(feishu_people): add tests for feishu_people core APIs (~40 APIs)`
  - Files: `tests/unit/hr/feishu_people_tests.rs`

---

### Wave 5: 剩余模块

- [x] **7. 实现 performance + okr 模块测试**

  **What to do**:
  - performance: 44 文件，21 API（全部测试）
  - okr: 23 文件，12 API（全部测试）
  - 合计约 33 个 API，规模与 hire 核心 API 相当
  - 测试文件: `tests/unit/hr/performance_tests.rs`, `tests/unit/hr/okr_tests.rs`

  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
    - Reason: API 数量少，模块逻辑相对独立
  - **Skills**: [`openlark-api`, `openlark-validation-style`]

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 8)
  - **Parallel Group**: Wave 5
  - **Blocks**: None
  - **Blocked By**: Task 5, 6

  **Acceptance Criteria**:

  **QA Scenarios**:
  ```
  Scenario: performance 和 okr 测试通过
    Tool: Bash
    Preconditions: 测试文件已编写
    Steps:
      1. 运行 `cargo test --package openlark-hr -- performance`
      2. 运行 `cargo test --package openlark-hr -- okr`
    Expected Result: 两个命令都显示 "test result: ok"
    Evidence: .sisyphus/evidence/task-7-performance-okr-tests.txt
  ```

  **Commit**: YES
  - Message: `test(performance,okr): add tests for performance and okr modules`
  - Files: `tests/unit/hr/performance_tests.rs`, `tests/unit/hr/okr_tests.rs`

---

- [x] **8. 实现 compensation_management + ehr 模块测试**

  **What to do**:
  - compensation_management: 36 文件，21 API
  - ehr: 7 文件，2 API
  - 合计约 23 个 API
  - 测试文件: `tests/unit/hr/compensation_tests.rs`, `tests/unit/hr/ehr_tests.rs`

  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
  - **Skills**: [`openlark-api`, `openlark-validation-style`]

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 7)
  - **Parallel Group**: Wave 5
  - **Blocks**: None
  - **Blocked By**: Task 5, 6

  **Acceptance Criteria**:

  **QA Scenarios**:
  ```
  Scenario: compensation 和 ehr 测试通过
    Tool: Bash
    Preconditions: 测试文件已编写
    Steps:
      1. 运行 `cargo test --package openlark-hr -- compensation`
      2. 运行 `cargo test --package openlark-hr -- ehr`
    Expected Result: 两个命令都显示 "test result: ok"
    Evidence: .sisyphus/evidence/task-8-compensation-ehr-tests.txt
  ```

  **Commit**: YES
  - Message: `test(compensation,ehr): add tests for compensation and ehr modules`
  - Files: `tests/unit/hr/compensation_tests.rs`, `tests/unit/hr/ehr_tests.rs`

---

### Wave FINAL: 最终验证（并行执行）

- [x] **F1. Plan Compliance Audit** — `oracle`
  
  **What**: 读取计划 end-to-end，验证每个"Must Have"都有实现，每个"Must NOT Have"都未违反。
  - 验证测试依赖已添加
  - 验证目录结构正确
  - 验证所有核心 API 清单已测试
  - 检查 evidence 文件存在
  - 对比交付物与计划
  
  **Output**: `Must Have [6/6] | Must NOT Have [7/7] | Tasks [8/8] | Evidence [16/16] | VERDICT: APPROVE/REJECT`

  **Acceptance Criteria**:
  ```
  Scenario: 合规性审计
    Tool: Bash + Read
    Steps:
      1. 检查 Cargo.toml dev-dependencies 存在
      2. 检查 tests/unit/hr/ 目录存在
      3. 检查 docs/hr-testing-guide.md 存在
      4. 检查所有 8 个测试文件存在
      5. 检查 .sisyphus/evidence/ 中的证据文件
    Expected Result: 所有检查项通过
    Evidence: .sisyphus/evidence/final-compliance-audit.md
  ```

- [x] **F2. Code Quality Review** — `unspecified-high`
  
  **What**: 运行 `tsc --noEmit` + linter + `cargo test`。
  - 检查 `as any`/`@ts-ignore`
  - 检查空 catch、console.log、注释掉的代码
  - 检查 AI slop 模式（过度注释、过度抽象、通用命名）
  
  **Output**: `Build [PASS] | Lint [PASS] | Tests [N pass/N fail] | Files [N clean/N issues] | VERDICT`

  **Acceptance Criteria**:
  ```
  Scenario: 代码质量检查
    Tool: Bash
    Steps:
      1. cargo build --package openlark-hr
      2. cargo clippy --package openlark-hr --tests -- -D warnings
      3. cargo test --package openlark-hr
    Expected Result: 全部通过，零警告
    Evidence: .sisyphus/evidence/final-quality-check.txt
  ```

- [x] **F3. Coverage Verification** — `unspecified-high`
  
  **What**: 验证最终覆盖率达标。
  - 运行 `cargo llvm-cov --package openlark-hr`
  - 验证整体覆盖率 >= 60%
  - 验证核心 API 覆盖率 >= 80%
  
  **Output**: `Overall Coverage: XX% | Core API Coverage: XX% | VERDICT`

  **Acceptance Criteria**:
  ```
  Scenario: 覆盖率验证
    Tool: Bash
    Steps:
      1. cargo llvm-cov --package openlark-hr --html
      2. 解析覆盖率报告
    Expected Result: Overall >= 60%, Core API >= 80%
    Evidence: .sisyphus/evidence/final-coverage-report.html
  ```

- [x] **F4. Scope Fidelity Check** — `deep`
  
  **What**: 验证 1:1 实现，无范围蔓延。
  - 对比任务"What to do"与实际 diff
  - 验证没有测试非核心 API
  - 检查跨任务污染
  
  **Output**: `Tasks [8/8 compliant] | Scope [CLEAN] | VERDICT`

  **Acceptance Criteria**:
  ```
  Scenario: 范围验证
    Tool: Bash + Read
    Steps:
      1. 列出所有测试的 API
      2. 对比核心 API 清单
      3. 检查是否有非核心 API 被测试
    Expected Result: 无范围蔓延
    Evidence: .sisyphus/evidence/final-scope-check.md
  ```

---

## Commit Strategy

| After Task | Message | Files | Verification |
|------------|---------|-------|--------------|
| 1 | `chore(tests): add test dependencies and infrastructure for openlark-hr` | Cargo.toml, tests/unit/hr/mod.rs | cargo check |
| 2 | `docs(tests): add testing guide for openlark-hr` | docs/hr-testing-guide.md | 人工审查 |
| 3 | `test(attendance): add tests for attendance module (48 APIs)` | tests/unit/hr/attendance_tests.rs | cargo test -- attendance |
| 4 | `test(payroll): add tests for payroll module (12 APIs)` | tests/unit/hr/payroll_tests.rs | cargo test -- payroll |
| 5 | `test(hire): add tests for hire core APIs (~30 APIs)` | tests/unit/hr/hire_tests.rs | cargo test -- hire |
| 6 | `test(feishu_people): add tests for feishu_people core APIs (~40 APIs)` | tests/unit/hr/feishu_people_tests.rs | cargo test -- feishu_people |
| 7 | `test(performance,okr): add tests for performance and okr modules` | tests/unit/hr/performance_tests.rs, tests/unit/hr/okr_tests.rs | cargo test -- performance, cargo test -- okr |
| 8 | `test(compensation,ehr): add tests for compensation and ehr modules` | tests/unit/hr/compensation_tests.rs, tests/unit/hr/ehr_tests.rs | cargo test -- compensation, cargo test -- ehr |

---

## Success Criteria

### Verification Commands
```bash
# 1. 所有测试通过
cargo test --package openlark-hr 2>&1 | grep "test result: ok"
# Expected: test result: ok. N passed; 0 failed; 0 ignored (N >= 165)

# 2. 覆盖率达标
cargo llvm-cov --package openlark-hr --fail-under-lines 60
# Expected: 命令返回 exit code 0

# 3. Lint 零警告
cargo clippy --package openlark-hr --tests -- -D warnings 2>&1 | grep -E "error|warning" | wc -l
# Expected: 0

# 4. 核心 API 覆盖率
cargo llvm-cov --package openlark-hr --html 2>&1 | grep -E "Total|attendance|hire|feishu_people"
# Expected: 核心模块 >= 80%

# 5. 测试执行时间
time cargo test --package openlark-hr 2>&1 | grep "real"
# Expected: real < 5m

# 6. 测试文件数量
find tests/unit/hr -name "*_tests.rs" | wc -l
# Expected: 7

# 7. 测试代码比例
find tests/unit/hr -name "*.rs" -exec wc -l {} + | tail -1
# Expected: 测试代码行数 / 生产代码行数 < 1:3
```

### Final Checklist
- [x] 所有"Must Have"已验证（6/6）
- [x] 所有"Must NOT Have"已验证（7/7）
- [x] 所有 8 个任务已完成
- [x] 所有 16 个 QA Scenario 已通过
- [x] 最终覆盖率 >= 60%
- [x] 核心 API 覆盖率 >= 80%
- [x] 测试执行时间 < 5 分钟
- [x] 零 clippy 警告
- [x] 所有 evidence 文件已保存

---

## Appendix: Core API Lists (Detail)

### Attendance Core APIs (48 total, all tested)
- Group management: Create, Update, Get, Delete, List
- Shift management: Create, Update, Get, Delete, List
- User task: Get, Update, Remedy
- User stats: Get data, Get view, Get field
- Approval: Create, Get info, List
- File operations: Upload, Download
- Archive rule: Create, Update, Get
- Leave accrual: Get record
- Leave expire: Get record
- User setting: Get, Update
- User flow: Get
- User daily shift: Get, Update

### Payroll Core APIs (12 total, all tested)
- Payroll: Create, Get, Update, Delete, List
- Payroll item: Create, Get, Update, List

### Hire Core APIs (~30 from 183 total)
- Candidate: Create, Get, Update, Delete, List
- Interview: Create, Get, Update, Delete, List
- Offer: Create, Get, Update, Delete, List
- Application: Create, Get, Update, List
- Job: Create, Get, Update, Delete, List
- Evaluation: Create, Get, List

### FeishuPeople Core APIs (~40 from 285 total)
- Employee: Create, Get, Update, Delete, List
- Department: Create, Get, Update, Delete, List
- Position: Create, Get, Update, Delete, List
- Contract: Create, Get, Update, List
- Work location: Create, Get, Update, List

### Performance APIs (21 total, all tested)
- Performance cycle: Create, Get, Update, List
- Performance review: Create, Get, Update, List
- Performance feedback: Create, Get, List
- Goal: Create, Get, Update, List

### OKR APIs (12 total, all tested)
- OKR: Create, Get, Update, Delete, List
- Key result: Create, Get, Update, List

### Compensation Management APIs (21 total, all tested)
- Compensation group: Create, Get, Update, List
- Compensation adjustment: Create, Get, Update, List
- Salary item: Create, Get, Update, List

### EHR APIs (2 total, all tested)
- EHR record: Get, List

---

**Plan Generated**: 2026-02-17
**Estimated Effort**: Wave 1-2 (3-5天) + 每批次 3-7天 = 4-6 周（标准节奏）
**Strategy**: Core Path First + Batch Implementation
**Risk Level**: Medium (scale complexity)
