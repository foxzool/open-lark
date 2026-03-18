# OpenLark RC 发布与 Crate 对外策略计划

## TL;DR

> **Quick Summary**: 本计划将 `open-lark` 明确收敛为唯一默认安装入口，RC 阶段优先修正发布口径、README/根导出一致性与预发布 workflow；stable 阶段再推进 crate 分层定位与对外策略收敛。
>
> **Deliverables**:
> - `open-lark` 作为 canonical package 的对外口径
> - RC 发布规则：预发布仅 GitHub Release，不做 crates.io 全量预发布
> - 根 crate feature/导出/README 一致性修正计划
> - stable 阶段的 crate 分层与迁移路线图
>
> **Estimated Effort**: Medium
> **Parallel Execution**: YES - 4 waves
> **Critical Path**: 审计现状 → 修正 RC 入口一致性 → 收敛发布文案/流程 → stable 路线图

---

## Context

### Original Request
设计对外发布策略修改方案，判断应主推 `open-lark` 主 crate + feature，还是继续对外主推多个业务子 crate；当前目标是准备 RC 版本发布。

### Interview Summary
**Key Discussions**:
- 用户认可计划范围采用“RC + Stable 路线”。
- 对外优先策略已收敛为：主推 `open-lark`，而不是把全部业务 crate 作为一等公开入口。
- RC 版本重点是降低外部认知成本与发布摩擦，而不是立即做大规模 crate 合并。

**Research Findings**:
- workspace 分层大致为 `protocol/core -> 业务 crates -> client -> open-lark`。
- 大多数业务 crate 仅依赖 `openlark-core`，更像内部模块化边界而非必须独立营销的公开产品。
- 代码量最小且整合空间较大的 crate 包括 `openlark-protocol`、`openlark-user`、`openlark-webhook`、`openlark-analytics`。
- 根 `Cargo.toml` 已具备按业务域组织的 feature；README 也已主推 `open-lark`。
- 现有关键缺口：`README.md` 使用 `open_lark::prelude::*`，但根入口未对齐；`cardkit` feature 与根导出也有不一致风险。

### Metis Review
**Identified Gaps** (addressed):
- 缺少 RC 与 stable 范围边界 → 已拆分为“RC 最小收敛 + stable 路线图”。
- 缺少 acceptance criteria → 已补充 feature matrix、README 示例、发布 dry-run、文档一致性。
- 可能 scope creep 到 crate 重构/合并 → 已明确延后到 stable 规划，不纳入 RC 实施。
- 需要明确 guardrails → 已加入“不改 crate 名称/路径、不做大规模结构重组”的限制。

---

## Work Objectives

### Core Objective
建立统一、低认知负担的 OpenLark 对外发布叙事：RC 阶段将 `open-lark` 固化为默认安装入口并修正对外体验缺口；stable 阶段明确哪些 crate 保留独立专家定位，哪些降级为 support crates。

### Concrete Deliverables
- 根 crate 对外定位说明与 RC 发布策略文档
- `README.md` / `src/lib.rs` / 根 feature 口径一致性整改
- 预发布 workflow 规则（RC/Beta/Alpha 跳过 crates.io）
- stable crate 分层清单与迁移路线图

### Definition of Done
- [ ] `open-lark` 被文档和发布口径明确为默认入口
- [ ] README 示例与根导出行为一致
- [ ] RC 发布流程不再尝试 crates.io 预发布
- [ ] stable 路线图明确 canonical / expert-tier / support crates

### Must Have
- 保留当前 workspace 多 crate 架构，不在 RC 阶段做破坏性合并
- 形成可执行的 RC 落地计划与 stable 演进计划
- 覆盖验证：feature、README 示例、发布流程、文档口径

### Must NOT Have (Guardrails)
- 不删除 crate、不改 crate 名称、不改 crate 路径
- 不在 RC 阶段引入新的公开 API 设计重构
- 不把“crate 合并实现”混入当前 RC 发布工作
- 不把所有子 crate 立即停止发布或移除兼容入口

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — 所有验收都以 agent 可执行命令、文件检查、构建结果为准。

### Test Decision
- **Infrastructure exists**: YES
- **Automated tests**: Tests-after
- **Framework**: cargo test / cargo doc / cargo build / cargo hack（如已安装）

### QA Policy
每个任务都必须包含 agent-executed QA。RC 计划重点验证：
- feature 组合是否可编译
- README 示例是否与根 crate 一致
- release workflow 是否正确分流 prerelease 与 stable
- 文档是否把 `open-lark` 设为 canonical package

---

## Execution Strategy

### Parallel Execution Waves

Wave 1 (Start Immediately — audit + boundary lock):
├── Task 1: 审计根 feature 矩阵与导出一致性 [quick]
├── Task 2: 审计 README / docs / release 口径 [writing]
├── Task 3: 审计发布 workflow 与 RC/stable 行为 [quick]
└── Task 4: 审计子 crate 对外定位现状与独立价值 [deep]

Wave 2 (After Wave 1 — policy baseline):
└── Task 5: 定义 canonical package 与 crate 分层规则 [writing]

Wave 3 (After Wave 2 — RC minimum fixes plan):
├── Task 6: 设计 RC 阶段根入口一致性修正项 [quick]
└── Task 7: 设计 RC 文档与发布说明收敛项 [writing]

Wave 4 (After Wave 3 — RC verification + stable roadmap):
├── Task 8: 设计 RC 验证矩阵与发布检查单 [quick]
├── Task 9: 制定 stable crate 分层清单 [deep]
├── Task 10: 制定 stable 迁移/兼容策略 [deep]
├── Task 11: 制定 stable crates.io 发布策略与工具路线 [unspecified-high]
└── Task 12: 制定 docs.rs / README / 子 crate README 统一口径 [writing]

Wave FINAL (After ALL tasks — review):
├── Task F1: 计划完整性审计 [oracle]
├── Task F2: 文件/引用/命名一致性检查 [unspecified-high]
├── Task F3: 验证命令与 QA 场景可执行性检查 [deep]
└── Task F4: Scope fidelity check [deep]

Critical Path: 1/2/3 → 5/6/7 → 9/10/11 → F1-F4

### Dependency Matrix
- **1**: None — 5, 6
- **2**: None — 5, 7, 12
- **3**: None — 7, 8, 11
- **4**: None — 5, 9, 10
- **5**: 1, 2, 4 — 9, 10, 12
- **6**: 1, 5 — 8
- **7**: 2, 3, 5 — 12
- **8**: 3, 6 — F3
- **9**: 4, 5 — F1, F4
- **10**: 4, 5 — F1, F4
- **11**: 3 — F1, F3
- **12**: 2, 5, 7 — F2, F4

### Agent Dispatch Summary
- **Wave 1**: T1 `quick`, T2 `writing`, T3 `quick`, T4 `deep`
- **Wave 2**: T5 `writing`
- **Wave 3**: T6 `quick`, T7 `writing`
- **Wave 4**: T8 `quick`, T9 `deep`, T10 `deep`, T11 `unspecified-high`, T12 `writing`
- **FINAL**: F1 `oracle`, F2 `unspecified-high`, F3 `deep`, F4 `deep`

---

## TODOs

- [ ] 1. 审计根 feature 矩阵与导出一致性

  **What to do**:
  - 读取根 `Cargo.toml` 的 feature 定义，确认业务 feature、组合 feature、别名 feature 的当前形态。
  - 对照根 `src/lib.rs`，列出 feature 与 re-export 的一致项和缺口项。
  - 特别核对 `prelude`、`cardkit`、feature alias（如 `base`/`bitable`）等关键不一致点。

  **Must NOT do**:
  - 不修改 feature 名称。
  - 不在此任务里扩大为 crate 结构重组。

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 以文件审计和差异比对为主，改动范围集中。
  - **Skills**: [`openlark-code-standards`]
    - `openlark-code-standards`: 用于快速检查导出、一致性和模块边界。
  - **Skills Evaluated but Omitted**:
    - `openlark-naming`: 本任务先做事实审计，不先改命名。

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3, 4)
  - **Blocks**: 5, 6
  - **Blocked By**: None

  **References**:
  - `Cargo.toml:233` - 根 feature 入口与 feature 分层的事实来源。
  - `src/lib.rs:1` - 根 crate re-export 与公开入口行为的事实来源。
  - `README.md:22` - 对外默认安装方式和使用姿势的事实来源。

  **Acceptance Criteria**:
  - [ ] 输出一份 feature → root export → README usage 的对照清单。
  - [ ] 明确标出所有不一致点及其影响范围。

  **QA Scenarios**:
  ```text
  Scenario: 发现根入口不一致点
    Tool: Bash
    Preconditions: 仓库存在 Cargo.toml、src/lib.rs、README.md
    Steps:
      1. 读取根 feature 段与根导出文件
      2. 列出 README 中出现的公开入口模式
      3. 比对 feature、导出、示例是否一致
    Expected Result: 形成明确差异清单，例如 prelude 缺失、cardkit 导出缺口
    Failure Indicators: 只能给出模糊判断，无法指出具体文件位置
    Evidence: .sisyphus/evidence/task-1-feature-export-audit.txt

  Scenario: 检查 alias feature 的边界
    Tool: Bash
    Preconditions: 根 Cargo.toml 含 docs 相关 alias feature
    Steps:
      1. 查找 `base`、`bitable` 等 alias feature 定义
      2. 判断是否与公开叙事冲突
    Expected Result: 明确 alias 是否需要在 RC 文档中解释或延后处理
    Evidence: .sisyphus/evidence/task-1-alias-feature-audit.txt
  ```

- [ ] 2. 审计 README / docs / release 对外口径

  **What to do**:
  - 盘点根 README、发布说明、release workflow 所表达的对外安装路径与发布语义。
  - 确认是否已把 `open-lark` 作为默认入口，是否仍残留“多个 crate 并列主推”的表达。
  - 为 RC 和 stable 分别整理需要收敛的文案点。

  **Must NOT do**:
  - 不在本任务中直接改写全部文档。
  - 不引入新的产品分层术语，除非已在后续策略任务中确认。

  **Recommended Agent Profile**:
  - **Category**: `writing`
    - Reason: 以文档和对外口径梳理为主。
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `openlark-naming`: 当前重点是口径盘点，不是 API 命名整改。

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3, 4)
  - **Blocks**: 5, 7, 12
  - **Blocked By**: None

  **References**:
  - `README.md:11` - 当前主文档是否已主推 `open-lark`。
  - `CHANGELOG.md:8` - RC 版本对外说明的承载点。
  - `.github/workflows/release.yml:91` - prerelease/stable 发布行为的承载点。

  **Acceptance Criteria**:
  - [ ] 产出“已对齐 / 未对齐”的文案审计清单。
  - [ ] 明确 RC 该修哪些文案、stable 再补哪些文案。

  **QA Scenarios**:
  ```text
  Scenario: 识别对外入口是否唯一
    Tool: Bash
    Preconditions: README 和 release 说明文件存在
    Steps:
      1. 搜索安装示例与 crate 名称引用
      2. 判断是否统一指向 `open-lark`
    Expected Result: 可明确说明 canonical package 当前是否已被主文档统一表达
    Evidence: .sisyphus/evidence/task-2-messaging-audit.txt

  Scenario: 区分 RC 与 stable 文案需求
    Tool: Bash
    Preconditions: release workflow 与 changelog 可读取
    Steps:
      1. 查 workflow 对 prerelease/stable 的分流
      2. 查 changelog/README 是否区分 RC 和正式版语义
    Expected Result: 形成 RC/stable 文案差异表
    Evidence: .sisyphus/evidence/task-2-rc-stable-messaging.txt
  ```

- [ ] 3. 审计发布 workflow 与 RC/stable 行为

  **What to do**:
  - 检查 release workflow 现状，确认 RC/Beta/Alpha 与 stable 的行为分流是否符合策略。
  - 识别 crates.io 发布失败的真正根因是否被策略层规避，哪些问题仍需 stable 解决。
  - 明确 RC 发布的 canonical 渠道和 stable 的 crates.io 策略需求。

  **Must NOT do**:
  - 不在本任务中转成实现性修复大包。
  - 不把 stable crates.io 全量自动化发布实现混入 RC 工作。

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 以 CI/workflow 审计和策略判断为主。
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 4)
  - **Blocks**: 7, 8, 11
  - **Blocked By**: None

  **References**:
  - `.github/workflows/release.yml:91` - crates.io 发布 job 的真实行为。
  - `Cargo.toml:149` - 根 crate 作为统一发布包的配置事实。
  - `CHANGELOG.md:8` - RC 说明与版本节奏的事实来源。

  **Acceptance Criteria**:
  - [ ] 明确 RC 与 stable 的发布渠道策略。
  - [ ] 明确当前 workflow 的已完成项、待补项、故意跳过项。

  **QA Scenarios**:
  ```text
  Scenario: 验证 RC 发布不会再走错误渠道
    Tool: Bash
    Preconditions: release workflow 已存在
    Steps:
      1. 读取 release workflow 条件逻辑
      2. 验证 prerelease 标签不会触发 crates.io 发布
    Expected Result: 能明确说明 RC/Beta/Alpha 的行为与正式版的区别
    Evidence: .sisyphus/evidence/task-3-workflow-audit.txt

  Scenario: 列出 stable 仍未解决的问题
    Tool: Bash
    Preconditions: 已知 crates.io 发布失败案例
    Steps:
      1. 对照失败原因与当前策略
      2. 列出 stable 阶段必须处理的依赖/版本/发布工具问题
    Expected Result: 形成 stable 发布待办清单
    Evidence: .sisyphus/evidence/task-3-stable-publish-gaps.txt
  ```

- [ ] 4. 审计子 crate 对外定位现状与独立价值

  **What to do**:
  - 基于代码量、依赖结构、使用场景与业务边界，将子 crate 粗分为“大型领域 crate”“基础设施 crate”“小型专用 crate”“support crates”。
  - 判断哪些 crate 值得保留独立专家定位，哪些只应保留兼容发布但不主推。
  - 输出 stable 路线图的输入清单。

  **Must NOT do**:
  - 不在此任务中直接决定 crate 合并实现。
  - 不直接下线任何子 crate。

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: 涉及架构边界、长期维护与用户心智模型判断。
  - **Skills**: [`openlark-design-review`]
    - `openlark-design-review`: 适合做公开 API/模块边界与长期架构判断。

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3)
  - **Blocks**: 5, 9, 10
  - **Blocked By**: None

  **References**:
  - `Cargo.toml:3` - workspace 成员全貌。
  - `crates/openlark-protocol/Cargo.toml:1` - 极小 crate 的代表。
  - `crates/openlark-hr/Cargo.toml:1` - 大型领域 crate 的代表。
  - `crates/openlark-webhook/Cargo.toml:1` - 小型但有独立消费场景 crate 的代表。

  **Acceptance Criteria**:
  - [ ] 形成 crate 分层表：canonical support path / expert-tier / support-only。
  - [ ] 每类 crate 都给出保留或降级的理由。

  **QA Scenarios**:
  ```text
  Scenario: 按独立消费价值给子 crate 分层
    Tool: Bash
    Preconditions: 可读取各 crate Cargo.toml 与统计数据
    Steps:
      1. 汇总 crate 大小、依赖关系、职责边界
      2. 标记独立消费价值高/中/低
    Expected Result: 每个 crate 都被归类且有证据支撑
    Evidence: .sisyphus/evidence/task-4-crate-tiering.txt

  Scenario: 避免过早结构重组
    Tool: Bash
    Preconditions: 已生成分层草案
    Steps:
      1. 检查是否有建议越过 RC 范围进入结构合并实现
      2. 把这些项转入 stable 路线图候选
    Expected Result: RC 与 stable 边界清晰
    Evidence: .sisyphus/evidence/task-4-scope-boundary.txt
  ```

- [ ] 5. 定义 canonical package 与 crate 分层规则

  **What to do**:
  - 将 `open-lark` 明确定义为唯一默认安装与文档入口。
  - 定义三层对外定位：canonical package、expert-tier crates、support crates。
  - 为每一层给出对外表达原则、文档出现位置、是否主推的规则。

  **Must NOT do**:
  - 不用模糊术语，例如“推荐但也许不是默认”。
  - 不把 support crates 完全视为内部私有实现，除非计划明确说明仍保留兼容发布。

  **Recommended Agent Profile**:
  - **Category**: `writing`
    - Reason: 这是策略定义与对外口径规范化工作。
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2
  - **Blocks**: 9, 10, 12
  - **Blocked By**: 1, 2, 4

  **References**:
  - `README.md:22` - 现有默认安装叙事。
  - `Cargo.toml:149` - 根 crate 的统一发布定位。
  - `.sisyphus/drafts/rc-release-crate-strategy.md:1` - 已收集的策略背景与约束。

  **Acceptance Criteria**:
  - [ ] 明确写出三层 crate 定位定义。
  - [ ] 明确每类 crate 的宣传级别、文档优先级与兼容承诺。

  **QA Scenarios**:
  ```text
  Scenario: 形成可执行的 crate 分层规则
    Tool: Bash
    Preconditions: Wave 1 审计结果已完成
    Steps:
      1. 汇总 RC 与 stable 的定位需求
      2. 输出 canonical / expert-tier / support 三层规则
    Expected Result: 后续所有 README、release note、workflow 调整都可引用该规则
    Evidence: .sisyphus/evidence/task-5-positioning-rules.txt

  Scenario: 检查分层定义是否互相冲突
    Tool: Bash
    Preconditions: 已写出 crate 分层规则
    Steps:
      1. 核对是否出现同一 crate 同时被定义为 canonical 与 support
      2. 修正冲突项
    Expected Result: 分层定义唯一、无冲突
    Evidence: .sisyphus/evidence/task-5-positioning-consistency.txt
  ```

- [ ] 6. 设计 RC 阶段根入口一致性修正项

  **What to do**:
  - 基于 Task 1 的差异清单，定义 RC 必须修的根入口一致性问题。
  - 至少覆盖：根 `prelude` 暴露、README 示例可编译、`cardkit` feature/export 对齐。
  - 约束为“只修与主入口体验直接相关的问题”。

  **Must NOT do**:
  - 不扩展为全面 API 重构。
  - 不顺手修 unrelated 代码风格或大规模 feature redesign。

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 面向有限文件和明确缺口的 RC 修正设计。
  - **Skills**: [`openlark-naming`]
    - `openlark-naming`: 保证根入口、公开表达与导出风格一致。

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Task 7)
  - **Blocks**: 8
  - **Blocked By**: 1, 5

  **References**:
  - `src/lib.rs:1` - 根导出的实际落点。
  - `README.md:64` - `prelude` 使用示例的事实来源。
  - `Cargo.toml:250` - `cardkit` feature 的配置事实。

  **Acceptance Criteria**:
  - [ ] 形成 RC 必修入口一致性清单。
  - [ ] 每个修正项都能追溯到用户可见缺口，而不是内部偏好。

  **QA Scenarios**:
  ```text
  Scenario: 锁定 RC 必修入口问题
    Tool: Bash
    Preconditions: Task 1 已输出差异清单
    Steps:
      1. 从差异清单中挑出用户可见问题
      2. 剔除非 RC 必须项
    Expected Result: RC 修正项最小且完整
    Evidence: .sisyphus/evidence/task-6-rc-entry-fixes.txt

  Scenario: 检查 README 示例是否全部被覆盖
    Tool: Bash
    Preconditions: README 中存在根 crate 使用示例
    Steps:
      1. 提取 README 中所有 root entry 相关示例
      2. 确认对应修正项已覆盖
    Expected Result: 无遗漏的用户可见入口问题
    Evidence: .sisyphus/evidence/task-6-readme-coverage.txt
  ```

- [ ] 7. 设计 RC 文档与发布说明收敛项

  **What to do**:
  - 定义 RC 需要更新的文档触点：根 README、release note、子 crate README 顶部口径、可能的 docs.rs 描述。
  - 统一 `open-lark` 的默认入口叙事，并说明子 crate 的高级/兼容定位。
  - 明确 prerelease 的预期渠道与使用方式。

  **Must NOT do**:
  - 不要求一次性改完所有深层模块文档。
  - 不制造与 stable 目标相矛盾的新口径。

  **Recommended Agent Profile**:
  - **Category**: `writing`
    - Reason: 文档触点和发布说明的整体收敛。
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Task 6)
  - **Blocks**: 12
  - **Blocked By**: 2, 3, 5

  **References**:
  - `README.md:1` - 主 README 的首页叙事。
  - `CHANGELOG.md:8` - 版本说明入口。
  - `crates/openlark-client/README.md:1` - 子 crate README 的代表性模板。

  **Acceptance Criteria**:
  - [ ] 列出 RC 必改文档文件和每个文件的目标变化。
  - [ ] 明确子 crate README 顶部需要增加的 canonical package 提示语。

  **QA Scenarios**:
  ```text
  Scenario: 统一对外入口文案
    Tool: Bash
    Preconditions: 已有根 README、子 crate README、changelog
    Steps:
      1. 列出所有对外入口文案触点
      2. 标记需更新与无需更新项
    Expected Result: 有一份有限、可执行的文档更新清单
    Evidence: .sisyphus/evidence/task-7-doc-touchpoints.txt

  Scenario: 区分 canonical 与 expert-tier 叙事
    Tool: Bash
    Preconditions: Task 5 已形成 crate 分层规则
    Steps:
      1. 将每个 crate 套入分层规则
      2. 生成文档提示模板
    Expected Result: 文档口径不再把所有 crate 当成并列产品
    Evidence: .sisyphus/evidence/task-7-doc-tiering.txt
  ```

- [ ] 8. 设计 RC 验证矩阵与发布检查单

  **What to do**:
  - 定义 RC 阶段必须执行的命令矩阵：默认 feature、关键 feature 组合、all-features、doc、README 示例。
  - 定义发布前 checklist：版本号、changelog、workflow 条件、GitHub Release、是否跳过 crates.io。
  - 让执行代理可以无需二次猜测直接按清单验收。

  **Must NOT do**:
  - 不写含糊的“确认工作正常”式条目。
  - 不把 stable 的 crates.io 多 crate 发布流程混入 RC checklist。

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 以命令、清单、验证矩阵为核心。
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 9, 10, 11, 12)
  - **Blocks**: F3
  - **Blocked By**: 3, 6

  **References**:
  - `Justfile:1` - 现有标准命令集合。
  - `.github/workflows/release.yml:1` - 发布流程的真实入口。
  - `README.md:64` - 需要进入验证矩阵的文档示例。

  **Acceptance Criteria**:
  - [ ] 输出一份可直接执行的 RC 验证矩阵。
  - [ ] 每个检查项都有对应命令或可比对文件。

  **QA Scenarios**:
  ```text
  Scenario: 生成可执行验证矩阵
    Tool: Bash
    Preconditions: Task 3 和 Task 6 已完成
    Steps:
      1. 汇总构建、测试、文档、示例、发布检查项
      2. 为每项补齐具体命令
    Expected Result: 执行代理可照单执行且结果可二元判断
    Evidence: .sisyphus/evidence/task-8-verification-matrix.txt

  Scenario: 检查 checklist 是否误入 stable 内容
    Tool: Bash
    Preconditions: RC checklist 草案已生成
    Steps:
      1. 检查是否包含 crates.io 全量 stable 发布内容
      2. 将超范围项移入 stable 路线图
    Expected Result: RC checklist 边界清晰
    Evidence: .sisyphus/evidence/task-8-rc-boundary.txt
  ```

- [ ] 9. 制定 stable crate 分层清单

  **What to do**:
  - 在 RC 的三层规则基础上，输出 stable 阶段的 crate 名单归类。
  - 至少区分：`open-lark` canonical、基础设施/专家 crate、support-only 业务 crate。
  - 特别说明 `openlark-protocol`、`openlark-webhook`、`openlark-user`、`openlark-analytics` 的 stable 定位建议。

  **Must NOT do**:
  - 不直接做 crate 合并决策落地。
  - 不以代码量作为唯一分层依据，必须结合独立消费场景。

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: 涉及长期产品边界和公开 API 定位。
  - **Skills**: [`openlark-design-review`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 8, 10, 11, 12)
  - **Blocks**: F1, F4
  - **Blocked By**: 4, 5

  **References**:
  - `Cargo.toml:3` - workspace 成员清单。
  - `.sisyphus/drafts/rc-release-crate-strategy.md:1` - 已收集的 crate 价值判断。

  **Acceptance Criteria**:
  - [ ] 每个 crate 在 stable 阶段都有清晰定位。
  - [ ] 至少给出 3 类 crate 的准入规则和示例。

  **QA Scenarios**:
  ```text
  Scenario: 形成 stable crate 分层表
    Tool: Bash
    Preconditions: Task 5 已完成
    Steps:
      1. 读取 workspace crate 清单
      2. 按 stable 三层规则逐个归类
    Expected Result: 无未分类 crate
    Evidence: .sisyphus/evidence/task-9-stable-tier-table.txt

  Scenario: 检查小型 crate 的归类是否有明确理由
    Tool: Bash
    Preconditions: 小型 crate 已被分类
    Steps:
      1. 抽查 protocol/webhook/user/analytics
      2. 验证其保留/降级理由是否可解释
    Expected Result: 每个小型 crate 都有非拍脑袋的理由
    Evidence: .sisyphus/evidence/task-9-small-crate-rationale.txt
  ```

- [ ] 10. 制定 stable 迁移/兼容策略

  **What to do**:
  - 设计从“多 crate 并列感知”过渡到“主 crate 优先”的兼容路线。
  - 说明如何保留现有直接依赖子 crate 的用户路径，同时逐步降级其宣传级别。
  - 明确哪些变更只体现在文档与推荐路径，哪些未来可能进入 deprecation 提示。

  **Must NOT do**:
  - 不在 stable 路线图中承诺立即废弃所有子 crate。
  - 不假设没有直接使用子 crate 的存量用户。

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: 涉及迁移摩擦、兼容窗口、用户心智切换。
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 8, 9, 11, 12)
  - **Blocks**: F1, F4
  - **Blocked By**: 4, 5

  **References**:
  - `README.md:22` - 新用户默认入口。
  - `crates/openlark-client/README.md:27` - 子 crate 仍可直接使用的代表性入口。

  **Acceptance Criteria**:
  - [ ] 明确新用户路径与存量用户路径。
  - [ ] 明确“降级宣传”与“废弃 API/crate”之间的区别。

  **QA Scenarios**:
  ```text
  Scenario: 定义兼容窗口
    Tool: Bash
    Preconditions: stable crate 分层已完成
    Steps:
      1. 按 crate 分类定义兼容发布与文档保留策略
      2. 标出何时只降级宣传、何时才考虑 deprecation
    Expected Result: 无“直接切断用户路径”的激进动作
    Evidence: .sisyphus/evidence/task-10-compat-window.txt

  Scenario: 检查迁移策略是否依赖未知用户数据
    Tool: Bash
    Preconditions: 迁移策略草案已生成
    Steps:
      1. 识别其中是否假设不存在下游用户
      2. 将未验证假设改写为保守兼容策略
    Expected Result: 路线图对存量用户风险可控
    Evidence: .sisyphus/evidence/task-10-user-risk-check.txt
  ```

- [ ] 11. 制定 stable crates.io 发布策略与工具路线

  **What to do**:
  - 为 stable 版本定义 crates.io 发布方式：继续手动分层发布、引入发布工具、还是 root-first + managed multi-crate publishing。
  - 明确 path+version、workspace 依赖、发布顺序、预发布与正式版的差异化策略。
  - 对比 `cargo-release`、`cargo-smart-release` 等工具是否值得纳入正式版流程。

  **Must NOT do**:
  - 不要求在 RC 阶段先把 stable 工具链全部接入。
  - 不把 prerelease 例外逻辑与正式版逻辑混淆。

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 既有工程流程，又有发布系统设计权衡。
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 8, 9, 10, 12)
  - **Blocks**: F1, F3
  - **Blocked By**: 3

  **References**:
  - `.github/workflows/release.yml:91` - 当前正式版发布流程的基线。
  - `Cargo.toml:107` - workspace 内部依赖组织方式的事实来源。
  - `.sisyphus/drafts/rc-release-crate-strategy.md:1` - 发布最佳实践调研结论。

  **Acceptance Criteria**:
  - [ ] 写清楚 stable crates.io 的推荐发布方式。
  - [ ] 指定是否采用发布工具，以及采用理由。

  **QA Scenarios**:
  ```text
  Scenario: 比较 stable 发布路线
    Tool: Bash
    Preconditions: 已知当前 workflow 和失败案例
    Steps:
      1. 列出至少两种 stable 发布路线
      2. 比较复杂度、风险、维护成本
    Expected Result: 选出一条推荐路线并给出原因
    Evidence: .sisyphus/evidence/task-11-publish-options.txt

  Scenario: 检查路线是否误伤 RC 节奏
    Tool: Bash
    Preconditions: stable 发布路线草案已生成
    Steps:
      1. 检查是否要求 RC 先完成 stable 才能发布
      2. 将会阻塞 RC 的项剥离到后续里程碑
    Expected Result: stable 规划不阻塞当前 RC
    Evidence: .sisyphus/evidence/task-11-rc-separation.txt
  ```

- [ ] 12. 制定 docs.rs / README / 子 crate README 统一口径

  **What to do**:
  - 定义根 README、子 crate README、docs.rs 顶部说明的统一模板。
  - 规定哪些场景写“新项目优先使用 `open-lark`”，哪些场景保留“直接依赖此 crate 适用于高级/低层使用”。
  - 把该模板作为 stable 持续收敛的内容资产。

  **Must NOT do**:
  - 不要求一次写完全部子 crate README 细节。
  - 不让根 README 与子 crate README 给出相互矛盾的入口建议。

  **Recommended Agent Profile**:
  - **Category**: `writing`
    - Reason: 统一文档模板和对外文案。
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 8, 9, 10, 11)
  - **Blocks**: F2, F4
  - **Blocked By**: 2, 5, 7

  **References**:
  - `README.md:1` - 根 README 模板基线。
  - `crates/openlark-client/README.md:1` - 子 crate README 的代表性模板。
  - `crates/openlark-webhook/README.md:1` - 小型专用 crate README 的代表性模板。

  **Acceptance Criteria**:
  - [ ] 输出统一文案模板与适用规则。
  - [ ] 根 README 与子 crate README 的推荐路径不冲突。

  **QA Scenarios**:
  ```text
  Scenario: 生成统一文案模板
    Tool: Bash
    Preconditions: crate 分层与 RC 文档触点已明确
    Steps:
      1. 为根 README 与子 crate README 生成模板草案
      2. 套用 canonical / expert-tier / support 规则
    Expected Result: 不同层级 crate 有一致但区分明确的提示语
    Evidence: .sisyphus/evidence/task-12-doc-template.txt

  Scenario: 检查模板是否自相矛盾
    Tool: Bash
    Preconditions: 模板草案已生成
    Steps:
      1. 比对根 README 和子 crate README 模板
      2. 检查是否同时把多个入口写成“默认入口”
    Expected Result: 只有 `open-lark` 被表达为默认入口
    Evidence: .sisyphus/evidence/task-12-template-consistency.txt
  ```

---

## Final Verification Wave

- [ ] F1. **Plan Compliance Audit** — `oracle`
  核对计划是否同时覆盖 RC 最小落地与 stable 路线；检查 canonical package、crate 分层、README/root export/release workflow、发布策略是否都在 TODO 中体现。

  **QA Scenarios**:
  ```text
  Scenario: 审计计划覆盖面
    Tool: Bash
    Preconditions: 计划文件已完整生成
    Steps:
      1. 逐节检查是否覆盖 canonical package、RC 修正、stable 路线图、验证矩阵
      2. 对照 Must Have 与 TODOs 核验是否一一落地
    Expected Result: 输出覆盖项与缺失项列表，给出 APPROVE/REJECT
    Evidence: .sisyphus/evidence/final-f1-plan-compliance.txt

  Scenario: 检查 guardrail 是否贯穿
    Tool: Bash
    Preconditions: 计划文件存在 guardrails 章节
    Steps:
      1. 搜索 crate 合并、重命名、下线等越界动作
      2. 确认 TODO 中未要求执行这些越界动作
    Expected Result: 无 scope creep，或明确列出越界点
    Evidence: .sisyphus/evidence/final-f1-guardrail-check.txt
  ```

- [ ] F2. **Reference Integrity Review** — `unspecified-high`
  检查所有引用路径、文件、feature 名称、crate 名称是否存在且拼写正确；确认 README、`Cargo.toml`、`src/lib.rs`、workflow 文件都被纳入。

  **QA Scenarios**:
  ```text
  Scenario: 验证引用路径存在
    Tool: Bash
    Preconditions: 计划中的引用路径已写入
    Steps:
      1. 提取计划中的文件引用
      2. 逐个检查文件是否存在且可读取
    Expected Result: 所有关键引用路径存在；若不存在，列出 file:line
    Evidence: .sisyphus/evidence/final-f2-reference-exists.txt

  Scenario: 验证命名一致性
    Tool: Bash
    Preconditions: 计划中含 feature/crate 名称
    Steps:
      1. 抽查 `open-lark`、`openlark-client`、`cardkit`、`core-services` 等名称
      2. 对照实际文件确认拼写与含义一致
    Expected Result: 无错误命名或误引
    Evidence: .sisyphus/evidence/final-f2-name-integrity.txt
  ```

- [ ] F3. **Verification Command Review** — `deep`
  检查每个任务的 QA 是否能通过具体命令/文件比对执行，避免“人工看看是否正确”式验收。

  **QA Scenarios**:
  ```text
  Scenario: 审查任务 QA 可执行性
    Tool: Bash
    Preconditions: Task 1-12 均含 QA Scenarios
    Steps:
      1. 检查每个任务是否包含 Tool、Steps、Expected Result、Evidence
      2. 标记任何模糊或不可二元判断的项
    Expected Result: 所有任务 QA 可被 agent 执行，或列出需要修正的任务号
    Evidence: .sisyphus/evidence/final-f3-qa-executability.txt

  Scenario: 审查命令矩阵边界
    Tool: Bash
    Preconditions: Success Criteria 与 Task 8 已定义命令矩阵
    Steps:
      1. 检查命令是否覆盖默认 feature、核心 feature、all-features、doc、test
      2. 检查是否混入 stable-only 发布步骤
    Expected Result: 命令矩阵完整且边界清晰
    Evidence: .sisyphus/evidence/final-f3-command-matrix.txt
  ```

- [ ] F4. **Scope Fidelity Check** — `deep`
  确认计划没有越界到 crate 合并实现、公共 API 大改、子 crate 下线等超范围工作。

  **QA Scenarios**:
  ```text
  Scenario: 检查 RC 与 stable 边界
    Tool: Bash
    Preconditions: 计划中同时有 RC 与 stable 内容
    Steps:
      1. 逐项检查 RC 任务是否只聚焦入口一致性、文档口径、workflow、验证矩阵
      2. 检查 crate 合并、废弃、重组是否都被留在 stable 规划层
    Expected Result: RC 无越界实现项，stable 无阻塞 RC 的前置要求
    Evidence: .sisyphus/evidence/final-f4-scope-boundary.txt

  Scenario: 检查与用户原始目标的一致性
    Tool: Bash
    Preconditions: 原始目标为 RC 发布下的 crate 对外策略修改计划
    Steps:
      1. 对照 Original Request、Core Objective、TODOs
      2. 检查是否遗漏“主 crate + feature vs 多 crate”的核心决策
    Expected Result: 计划完整回应用户目标，没有跑偏到无关重构
    Evidence: .sisyphus/evidence/final-f4-user-intent-check.txt
  ```

---

## Commit Strategy

- RC 实施阶段建议拆分为 2-4 个提交：
  - `ci(release): skip prerelease crates.io publishing`
  - `docs(readme): position open-lark as canonical package`
  - `fix(root): align prelude and feature re-exports`
  - `docs(strategy): add stable crate positioning roadmap`

---

## Success Criteria

### Verification Commands
```bash
cargo build
cargo build --no-default-features
cargo build --features "core-services"
cargo build --all-features
cargo test --lib --all-features
cargo doc --no-deps --all-features --lib
```

### Final Checklist
- [ ] `open-lark` 被定义为唯一默认安装入口
- [ ] README 示例与根导出一致
- [ ] RC 版本不会触发 crates.io 预发布失败
- [ ] stable 路线图明确 expert-tier / support crate 策略
- [ ] 没有引入 crate 结构破坏性变更
