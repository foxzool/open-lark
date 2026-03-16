# Webhook 自定义机器人实现计划

## TL;DR

> **Quick Summary**: 创建独立的 `openlark-webhook` crate，实现通过 webhook URL 发送消息的自定义机器人功能。
>
> **Deliverables**:
> - 新 crate: `crates/openlark-webhook/`
> - 支持 5 种消息类型（文本、卡片、图片、文件、富文本）
> - 可选卡片支持（通过 feature flag，接受 JSON 格式）
> - 可选签名验证（timestamp + secret，通过 feature flag 控制）
> - TDD 测试覆盖
> - 使用示例
>
> **Estimated Effort**: Medium
> **Parallel Execution**: YES - 3 waves
> **Critical Path**: 基础设施 → 消息类型 → 集成测试

---

## Context

### Original Request
参考 https://open.feishu.cn/document/feishu-cards/quick-start/send-message-cards-with-custom-bot 实现 webhook 自定义机器人，通过 webhook URL 发送消息卡片，不需要 app_id/app_secret 认证。

### Interview Summary
**Key Discussions**:
- **模块位置**: 创建独立 crate `openlark-webhook`（职责清晰，按需引入）
- **功能范围**: 仅发送（outgoing），不包含事件接收服务器
- **HTTP 客户端**: 直接使用 `reqwest`，不依赖 openlark-core 的 Transport
- **Card 集成**: 可选依赖 `openlark-cardkit`（通过 feature flag）
- **签名验证**: 是，通过 feature flag 控制
- **测试策略**: TDD

**Research Findings**:
- 项目使用 Builder 模式，链式调用
- `validate_required!` 宏用于参数验证
- 使用 `CoreError` 统一错误处理
- Feature flags 控制模块编译

### Metis Review
**Identified Gaps** (addressed):
- **HTTP 客户端方案**: 直接使用 reqwest（Transport 强依赖 Config）
- **签名算法**: HMAC-SHA256(timestamp + "\n" + secret)
- **消息类型**: 5 种（text, card, image, file, post）
- **错误处理**: 使用 CoreError，无自动重试
- **范围边界**: 仅发送，不包含事件接收

---

## Work Objectives

### Core Objective
实现独立、轻量的 webhook 机器人发送消息功能，无需 app_id/app_secret 认证，直接 POST 到 webhook URL。

### Concrete Deliverables
- `crates/openlark-webhook/Cargo.toml` - 模块配置
- `crates/openlark-webhook/src/lib.rs` - 模块入口
- `crates/openlark-webhook/src/robot/` - 机器人功能
- `crates/openlark-webhook/src/common/` - 共享工具
- `crates/openlark-webhook/examples/` - 使用示例
- 测试文件（TDD）

### Definition of Done
- [x] `cargo check -p openlark-webhook --all-features` 零警告零错误
- [x] `cargo test -p openlark-webhook` 所有测试通过
- [x] 签名生成与飞书官方示例一致
- [x] 所有消息类型有测试覆盖
- [x] 文档生成成功

### Must Have
- ✅ 基础文本消息发送
- ✅ 可选签名验证（timestamp + secret）
- ✅ Builder 模式 API
- ✅ 类型安全的消息构建
- ✅ 完整错误处理
- ✅ TDD 测试覆盖

### Must NOT Have (Guardrails from Metis)
- ❌ app_id/app_secret 作为必需参数
- ❌ 事件接收服务器（incoming webhooks）
- ❌ 消息模板系统
- ❌ 自动重试逻辑
- ❌ 修改现有 crate 的公开 API
- ❌ 依赖 openlark-core 的 Transport（使用 Config）
- ❌ 同步（阻塞）API
- ❌ 批量发送功能

---

## Verification Strategy (MANDATORY)

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: NO
- **Automated tests**: TDD
- **Framework**: cargo test (built-in)
- **TDD Workflow**: RED (failing test) → GREEN (minimal impl) → REFACTOR

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Unit Tests**: Use `cargo test` — Assert function behavior, validate signatures
- **Integration Tests**: Mock HTTP requests with wiremock — Test actual network calls
- **Example Verification**: Run example code — Verify it compiles and works

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — infrastructure + scaffolding):
├── Task 1: Create crate structure and Cargo.toml [quick]
├── Task 2: Setup module entry (lib.rs) and prelude [quick]
├── Task 3: Implement common utilities (error, validation) [quick]
└── Task 4: Define message types and models [quick]

Wave 2 (After Wave 1 — core functionality, MAX PARALLEL):
├── Task 5: Implement WebhookClient (HTTP client) [quick]
├── Task 6: Implement signature verification [quick]
├── Task 7: Implement text message sending [quick]
├── Task 8: Implement card message sending (optional feature) [quick]
└── Task 9: Implement other message types (image, file, post) [quick]

Wave 3 (After Wave 2 — testing + documentation):
├── Task 10: Write unit tests [deep]
├── Task 11: Write integration tests with mock server [deep]
├── Task 12: Create usage examples [quick]
└── Task 13: Update workspace Cargo.toml and documentation [quick]

Wave FINAL (After ALL tasks — independent review, 4 parallel):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Real manual QA (unspecified-high)
└── Task F4: Scope fidelity check (deep)

Critical Path: Task 1 → Task 5 → Task 7 → Task 10 → F1-F4
Parallel Speedup: ~60% faster than sequential
Max Concurrent: 4 (Waves 1 & 2)
```

### Dependency Matrix (abbreviated)

- **1-4**: — — 5-9
- **5**: 1, 3 — 7-9
- **6**: 1, 3 — 7 (optional signature)
- **7**: 5, 4 — 10, 11
- **8**: 5, 4 — 10, 11 (depends: card feature)
- **9**: 5, 4 — 10, 11
- **10**: 5-9 — 12
- **11**: 5-9 — 12
- **12**: 5-9 — 13
- **13**: 1, 10-12 — FINAL

### Agent Dispatch Summary

- **Wave 1**: **4** — T1-T4 → `quick`
- **Wave 2**: **5** — T5-T9 → `quick`
- **Wave 3**: **4** — T10-T13 → T10 → `deep`, T11 → `deep`, T12 → `quick`, T13 → `quick`
- **FINAL**: **4** — F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

- [x] 1. **Create crate structure and Cargo.toml**
  
  **What to do**:
  - 创建 `crates/openlark-webhook/` 目录
  - 创建 `Cargo.toml` 配置文件
  - 设置模块元数据（name, version, authors, description）
  - 配置依赖：`openlark-core`（必需），`serde`, `serde_json`, `reqwest`
  - 配置签名依赖（可选）：`hmac`, `sha2`, `base64`（通过 signature feature）
  - 定义 feature flags: `default = ["robot"]`, `robot = []`, `signature = ["hmac", "sha2", "base64"]`, `card = []`
  
  **Must NOT do**:
  - 不引入 `app_id`/`app_secret` 相关依赖
  - 不依赖 `openlark-core` 的完整 `Config` 结构
  - 不添加自动重试逻辑
  
  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 基础的文件创建和配置，简单直接
  - **Skills**: `[]`
  
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3, 4)
  - **Blocks**: Tasks 5-13
  - **Blocked By**: None (can start immediately)
  
  **References**:
  - `crates/openlark-cardkit/Cargo.toml:1-31` - Feature flags 组织模式
  - `crates/openlark-communication/Cargo.toml:1-31` - 依赖配置示例
  
  **Acceptance Criteria**:
  - [ ] `crates/openlark-webhook/Cargo.toml` 文件存在
  - [ ] `cargo check -p openlark-webhook` 通过（依赖可解析）
  - [ ] Feature flags 定义正确（robot, signature, card）
  
  **QA Scenarios (MANDATORY)**:
  ```
  Scenario: Cargo.toml 配置正确
    Tool: Bash (cargo check)
    Preconditions: 在项目根目录
    Steps:
      1. cargo check -p openlark-webhook
      2. cargo check -p openlark-webhook --no-default-features --features "robot"
      3. cargo check -p openlark-webhook --features "robot,signature"
      4. cargo check -p openlark-webhook --features "robot,card"
    Expected Result: 所有编译命令成功，零错误
    Failure Indicators: 任何编译错误，feature 不存在
    Evidence: .sisyphus/evidence/task-01-cargo-check.txt
  ```

  **Evidence to Capture**:
  - [ ] Evidence file: task-01-cargo-check.txt
  
  **Commit**: YES
  - Message: `feat(webhook): add crate skeleton and Cargo.toml`
  - Files: `crates/openlark-webhook/Cargo.toml`
  - Pre-commit: `cargo check -p openlark-webhook`

- [x] 2. **Setup module entry (lib.rs) and prelude**
  
  **What to do**:
  - 创建 `src/lib.rs` 模块入口文件
  - 定义模块文档注释（中文）
  - 导出公共模块：`robot`, `common`, `prelude`
  - 使用 `#[cfg(feature = "...")]` 控制模块导出
  - 创建 `src/prelude.rs` 预导入模块
  - 重新导出常用类型
  
  **Must NOT do**:
  - 不在未启用 feature 时导出模块
  - 不导出内部实现细节
  
  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 标准的模块入口设置
  - **Skills**: `[]`
  
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3, 4)
  - **Blocks**: Tasks 5-13
  - **Blocked By**: None (can start immediately)
  
  **References**:
  - `crates/openlark-cardkit/src/lib.rs:1-74` - 模块入口模式
  - `crates/openlark-communication/src/lib.rs:1-77` - Feature-gated 导出
  
  **Acceptance Criteria**:
  - [ ] `src/lib.rs` 文件存在
  - [ ] `src/prelude.rs` 文件存在
  - [ ] 文档注释完整（中文）
  - [ ] Feature flags 正确控制模块导出
  
  **QA Scenarios (MANDATORY)**:
  ```
  Scenario: 模块入口正确导出
    Tool: Bash (cargo doc)
    Preconditions: Task 1 完成
    Steps:
      1. cargo doc -p openlark-webhook --no-deps
      2. 检查生成的文档包含 robot, prelude 模块
    Expected Result: 文档生成成功，模块可见
    Failure Indicators: 文档生成失败，模块未显示
    Evidence: .sisyphus/evidence/task-02-doc-gen.txt
  ```
  
  **Evidence to Capture**:
  - [ ] Evidence file: task-02-doc-gen.txt
  
  **Commit**: YES
  - Message: `feat(webhook): add module entry and prelude`
  - Files: `crates/openlark-webhook/src/lib.rs`, `crates/openlark-webhook/src/prelude.rs`
  - Pre-commit: `cargo doc -p openlark-webhook --no-deps`

- [x] 3. **Implement common utilities (error, validation)**
  
  **What to do**:
  - 创建 `src/common/mod.rs` 模块入口
  - 创建 `src/common/error.rs` 错误处理
    - 定义 WebhookError 错误类型（或复用 CoreError）
    - 实现错误转换
  - 创建 `src/common/validation.rs` 参数验证
    - 实现 webhook URL 验证（非空检查）
    - 实现消息内容验证
    - 使用 `validate_required!` 宏
  - 创建 `src/common/mod.rs` 导出公共类型
  
  **Must NOT do**:
  - 不实现复杂的 URL 格式验证（飞书 URL 格式可能变化）
  - 不引入不必要的错误类型
  
  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 基础工具函数，模式清晰
  - **Skills**: `[]`
  
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 4)
  - **Blocks**: Tasks 5-13
  - **Blocked By**: None (can start immediately)
  
  **References**:
  - `crates/openlark-core/src/validation/core.rs:1-1012` - 验证宏模式
  - `crates/openlark-core/src/error/core.rs` - CoreError 定义
  
  **Acceptance Criteria**:
  - [ ] `src/common/error.rs` 存在
  - [ ] `src/common/validation.rs` 存在
  - [ ] URL 验证正确（非空）
  - [ ] 消息内容验证正确
  
  **QA Scenarios (MANDATORY)**:
  ```
  Scenario: 参数验证正确工作
    Tool: Bash (cargo test)
    Preconditions: Task 完成
    Steps:
      1. cargo test -p openlark-webhook -- test_validation
    Expected Result: 验证测试通过
    Failure Indicators: 测试失败
    Evidence: .sisyphus/evidence/task-03-validation-tests.txt
  ```
  
  **Evidence to Capture**:
  - [ ] Evidence file: task-03-validation-tests.txt
  
  **Commit**: YES
  - Message: `feat(webhook): add common utilities for error and validation`
  - Files: `crates/openlark-webhook/src/common/*.rs`
  - Pre-commit: `cargo test -p openlark-webhook -- test_validation`

- [x] 4. **Define message types and models**
  
  **What to do**:
  - 创建 `src/models.rs` 消息模型定义
  - 定义 `MsgType` 枚举：text, post, image, file, interactive
  - 定义各种消息内容结构体：
    - `TextContent`: 文本消息
    - `PostContent`: 富文本消息
    - `ImageContent`: 图片消息
    - `FileContent`: 文件消息
    - `InteractiveContent`: 卡片消息（可选，接受 serde_json::Value）
  - 使用 Serde 序列化
  - 编写单元测试验证序列化
  
  **Must NOT do**:
  - 不在未启用 card feature 时导出 InteractiveContent
  - 不添加飞书不支持的消息类型
  
  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 数据结构定义，模式清晰
  - **Skills**: `[]`
  
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3)
  - **Blocks**: Tasks 7-9
  - **Blocked By**: None (can start immediately)
  
  **References**:
  - `crates/openlark-communication/src/im/im/v1/message/models.rs:1-109` - 枚举定义模式
  - 飞书 webhook 文档 - 消息格式
  
  **Acceptance Criteria**:
  - [ ] `src/models.rs` 存在
  - [ ] 5 种消息类型定义完整
  - [ ] Serde 序列化正确
  - [ ] 单元测试覆盖
  
  **QA Scenarios (MANDATORY)**:
  ```
  Scenario: 消息类型序列化正确
    Tool: Bash (cargo test)
    Preconditions: Task 完成
    Steps:
      1. cargo test -p openlark-webhook -- test_model_serialization
      2. 验证 JSON 输出格式符合飞书规范
    Expected Result: 序列化测试通过，JSON 格式正确
    Failure Indicators: 测试失败，JSON 格式错误
    Evidence: .sisyphus/evidence/task-04-model-serialization.txt
  ```
  
  **Evidence to Capture**:
  - [ ] Evidence file: task-04-model-serialization.txt
  
  **Commit**: YES
  - Message: `feat(webhook): add message types and models`
  - Files: `crates/openlark-webhook/src/models.rs`
  - Pre-commit: `cargo test -p openlark-webhook -- test_model_serialization`

- [x] 5. **Implement WebhookClient (HTTP client)**
  
  **What to do**:
  - 创建 `src/robot/mod.rs` 模块入口
  - 创建 `src/robot/v1/mod.rs` 版本入口
  - 创建 `src/robot/v1/client.rs` WebhookClient 实现
    - 使用 `reqwest::Client` 直接发送 HTTP 请求
    - 实现 `new()` 构造函数
    - 实现 `send()` 方法
    - 处理响应和错误
  - 创建 `src/robot/v1/send.rs` 消息发送请求
    - 实现 `SendWebhookMessageRequest` 结构体
    - 实现 Builder 模式
    - 实现 `execute()` 方法
  - 添加单元测试
  
  **Must NOT do**:
  - 不使用 `openlark-core` 的 `Transport`（强依赖 Config）
  - 不添加自动重试逻辑
  - 不修改现有 crate
  
  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 标准的 HTTP 客户端实现
  - **Skills**: `[]`
  
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 6, 7, 8, 9)
  - **Blocks**: Tasks 10-13
  - **Blocked By**: Tasks 1, 3
  
  **References**:
  - `crates/openlark-core/src/http.rs:1-167` - HTTP 请求模式
  - 飞书 webhook API 文档
  
  **Acceptance Criteria**:
  - [ ] `src/robot/v1/client.rs` 存在
  - [ ] `src/robot/v1/send.rs` 存在
  - [ ] WebhookClient 使用 reqwest
  - [ ] Builder 模式实现正确
  - [ ] 错误处理正确
  
  **QA Scenarios (MANDATORY)**:
  ```
  Scenario: WebhookClient 正确发送消息
    Tool: Bash (cargo test)
    Preconditions: Task 完成
    Steps:
      1. cargo test -p openlark-webhook -- test_webhook_client
      2. 使用 mock server 验证请求格式
    Expected Result: 测试通过，请求格式正确
    Failure Indicators: 测试失败，请求格式错误
    Evidence: .sisyphus/evidence/task-05-client-tests.txt
  
  Scenario: 错误处理正确
    Tool: Bash (cargo test)
    Preconditions: Task 完成
    Steps:
      1. cargo test -p openlark-webhook -- test_error_handling
      2. 测试网络错误、4xx、5xx 响应
    Expected Result: 错误正确返回和处理
    Failure Indicators: 错误未正确处理
    Evidence: .sisyphus/evidence/task-05-error-tests.txt
  ```
  
  **Evidence to Capture**:
  - [ ] Evidence file: task-05-client-tests.txt
  - [ ] Evidence file: task-05-error-tests.txt
  
  **Commit**: YES
  - Message: `feat(webhook): implement WebhookClient with reqwest`
  - Files: `crates/openlark-webhook/src/robot/*.rs`
  - Pre-commit: `cargo test -p openlark-webhook -- test_webhook_client`

- [x] 6. **Implement signature verification**
  
  **What to do**:
  - 创建 `src/common/signature.rs` 签名验证实现
  - 实现 `sign()` 函数：HMAC-SHA256(timestamp + "\n" + secret)
  - 实现 `SignatureBuilder` Builder 模式
  - 添加 HTTP 头部：X-Lark-Signature, X-Lark-Timestamp
  - 在 `SendWebhookMessageRequest` 中集成签名（通过 feature flag 控制）
  - 编写单元测试验证签名算法
  
  **Must NOT do**:
  - 不在未启用 signature feature 时编译签名代码
  - 不实现其他签名算法
  
  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 标准的签名算法实现
  - **Skills**: `[]`
  
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 7, 8, 9)
  - **Blocks**: Tasks 10
  - **Blocked By**: Tasks 1, 3
  
  **References**:
  - 飞书 webhook 签名验证文档
  - `hmac`, `sha2`, `base64` crates 文档
  
  **Acceptance Criteria**:
  - [ ] `src/common/signature.rs` 存在
  - [ ] HMAC-SHA256 算法正确
  - [ ] HTTP 头部设置正确
  - [ ] Feature flag 控制正确
  - [ ] 单元测试覆盖
  
  **QA Scenarios (MANDATORY)**:
  ```
  Scenario: 签名算法正确实现
    Tool: Bash (cargo test)
    Preconditions: Task 完成，signature feature 启用
    Steps:
      1. cargo test -p openlark-webhook --features "signature" -- test_signature
      2. 使用已知输入和期望输出验证签名
    Expected Result: 签名测试通过，输出与官方示例一致
    Failure Indicators: 测试失败，签名不匹配
    Evidence: .sisyphus/evidence/task-06-signature-tests.txt
  ```
  
  **Evidence to Capture**:
  - [ ] Evidence file: task-06-signature-tests.txt
  
  **Commit**: YES
  - Message: `feat(webhook): add signature verification`
  - Files: `crates/openlark-webhook/src/common/signature.rs`
  - Pre-commit: `cargo test -p openlark-webhook --features "signature" -- test_signature`

- [x] 7. **Implement text message sending**
  
  **What to do**:
  - 在 `src/robot/v1/send.rs` 中实现文本消息发送
  - 实现 `text()` 方法
  - 实现消息体构建
  - 集成到 `execute()` 方法
  - 编写单元测试
  - 编写集成测试（使用 mock server）
  
  **Must NOT do**:
  - 不添加飞书不支持的消息类型
  - 不实现自动重试
  
  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 简单的消息类型实现
  - **Skills**: `[]`
  
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 6, 8, 9)
  - **Blocks**: Tasks 10, 11
  - **Blocked By**: Tasks 4, 5
  
  **References**:
  - 飞书 webhook 文本消息文档
  - `crates/openlark-communication/src/im/im/v1/message/create.rs:65-117` - 消息发送模式
  
  **Acceptance Criteria**:
  - [ ] `text()` 方法实现正确
  - [ ] 消息体构建正确
  - [ ] 单元测试覆盖
  - [ ] 集成测试覆盖
  
  **QA Scenarios (MANDATORY)**:
  ```
  Scenario: 文本消息发送成功
    Tool: Bash (cargo test)
    Preconditions: Task 完成
    Steps:
      1. cargo test -p openlark-webhook -- test_send_text_message
      2. 使用 mock server 验证请求
    Expected Result: 测试通过，请求格式正确
    Failure Indicators: 测试失败
    Evidence: .sisyphus/evidence/task-07-text-message-tests.txt
  ```
  
  **Evidence to Capture**:
  - [ ] Evidence file: task-07-text-message-tests.txt
  
  **Commit**: YES
  - Message: `feat(webhook): implement text message sending`
  - Files: `crates/openlark-webhook/src/robot/v1/send.rs`
  - Pre-commit: `cargo test -p openlark-webhook -- test_send_text_message`

- [x] 8. **Implement card message sending (optional feature)**
  
  **What to do**:
  - 在 `src/robot/v1/send.rs` 中实现卡片消息发送（通过 card feature 控制）
  - 实现 `card()` 方法，接受 `serde_json::Value`（卡片 JSON）
  - 集成到 `execute()` 方法
  - 编写单元测试
  - 编写集成测试（使用 mock server）
  
  **Must NOT do**:
  - 不在未启用 card feature 时编译卡片代码
  - 不依赖 openlark-cardkit（它不提供 Card 构建器）
  - 不实现复杂的卡片构建器（仅发送 JSON）
  
  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 卡片消息发送，模式清晰
  - **Skills**: `[]`
  
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 6, 7, 9)
  - **Blocks**: Tasks 10, 11
  - **Blocked By**: Tasks 4, 5
  
  **References**:
  - 飞书 webhook 卡片消息文档 - JSON 格式
  - Task 7 的文本消息实现模式
  
  **Acceptance Criteria**:
  - [ ] `card()` 方法实现正确（card feature 启用时）
  - [ ] 接受 serde_json::Value 作为卡片内容
  - [ ] 单元测试覆盖
  - [ ] 集成测试覆盖
  
  **QA Scenarios (MANDATORY)**:
  ```
  Scenario: 卡片消息发送成功
    Tool: Bash (cargo test)
    Preconditions: Task 完成，card feature 启用
    Steps:
      1. cargo test -p openlark-webhook --features "card" -- test_send_card_message
      2. 使用 mock server 验证请求
    Expected Result: 测试通过，请求格式正确
    Failure Indicators: 测试失败
    Evidence: .sisyphus/evidence/task-08-card-message-tests.txt
  ```
  
  **Evidence to Capture**:
  - [ ] Evidence file: task-08-card-message-tests.txt
  
  **Commit**: YES
  - Message: `feat(webhook): implement card message sending`
  - Files: `crates/openlark-webhook/src/robot/v1/send.rs`
  - Pre-commit: `cargo test -p openlark-webhook --features "card" -- test_send_card_message`

- [x] 9. **Implement other message types (image, file, post)**
  
  **What to do**:
  - 在 `src/robot/v1/send.rs` 中实现其他消息类型：
    - `image()` 方法：图片消息
    - `file()` 方法：文件消息
    - `post()` 方法：富文本消息
  - 集成到 `execute()` 方法
  - 编写单元测试
  - 编写集成测试（使用 mock server）
  
  **Must NOT do**:
  - 不添加飞书不支持的消息类型
  - 不实现文件上传（仅支持已上传文件的 file_key）
  
  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 其他消息类型实现，模式类似
  - **Skills**: `[]`
  
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 6, 7, 8)
  - **Blocks**: Tasks 10, 11
  - **Blocked By**: Tasks 4, 5
  
  **References**:
  - 飞书 webhook 消息类型文档
  - Task 7 的文本消息实现模式
  
  **Acceptance Criteria**:
  - [ ] 3 个消息类型方法实现正确
  - [ ] 单元测试覆盖
  - [ ] 集成测试覆盖
  
  **QA Scenarios (MANDATORY)**:
  ```
  Scenario: 图片消息发送成功
    Tool: Bash (cargo test)
    Preconditions: Task 完成
    Steps:
      1. cargo test -p openlark-webhook -- test_send_image_message
    Expected Result: 测试通过
    Failure Indicators: 测试失败
    Evidence: .sisyphus/evidence/task-09-image-message-tests.txt
  
  Scenario: 文件消息发送成功
    Tool: Bash (cargo test)
    Preconditions: Task 完成
    Steps:
      1. cargo test -p openlark-webhook -- test_send_file_message
    Expected Result: 测试通过
    Failure Indicators: 测试失败
    Evidence: .sisyphus/evidence/task-09-file-message-tests.txt
  
  Scenario: 富文本消息发送成功
    Tool: Bash (cargo test)
    Preconditions: Task 完成
    Steps:
      1. cargo test -p openlark-webhook -- test_send_post_message
    Expected Result: 测试通过
    Failure Indicators: 测试失败
    Evidence: .sisyphus/evidence/task-09-post-message-tests.txt
  ```
  
  **Evidence to Capture**:
  - [ ] Evidence file: task-09-image-message-tests.txt
  - [ ] Evidence file: task-09-file-message-tests.txt
  - [ ] Evidence file: task-09-post-message-tests.txt
  
  **Commit**: YES
  - Message: `feat(webhook): implement image, file, and post message sending`
  - Files: `crates/openlark-webhook/src/robot/v1/send.rs`
  - Pre-commit: `cargo test -p openlark-webhook -- test_send_image_message -- test_send_file_message -- test_send_post_message`

- [x] 10. **Write comprehensive unit tests**
  
  **What to do**:
  - 创建 `tests/` 目录
  - 编写所有消息类型的单元测试
  - 编写签名验证的单元测试
  - 编写错误处理的单元测试
  - 编写参数验证的单元测试
  - 使用 `mockall` 或 `wiremock` 模拟 HTTP 请求
  - 确保测试覆盖率 > 80%
  
  **Must NOT do**:
  - 不在测试中依赖真实 webhook URL
  - 不添加不稳定的测试
  
  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: 需要深度思考的测试编写
  - **Skills**: `[]`
  
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 11, 12, 13)
  - **Blocks**: FINAL
  - **Blocked By**: Tasks 5-9
  
  **References**:
  - `tests/unit/` - 单元测试模式参考
  - `tests/integration/` - 集成测试模式参考
  - `wiremock` crate 文档
  
  **Acceptance Criteria**:
  - [ ] 所有测试文件创建完成
  - [ ] 测试覆盖率 > 80%
  - [ ] 所有测试通过
  - [ ] 无不稳定测试
  
  **QA Scenarios (MANDATORY)**:
  ```
  Scenario: 所有单元测试通过
    Tool: Bash (cargo test)
    Preconditions: Task 完成
    Steps:
      1. cargo test -p openlark-webhook --all-features
      2. cargo tarpaulin -p openlark-webhook --all-features
    Expected Result: 测试通过，覆盖率 > 80%
    Failure Indicators: 测试失败，覆盖率不足
    Evidence: .sisyphus/evidence/task-10-unit-tests.txt
  ```
  
  **Evidence to Capture**:
  - [ ] Evidence file: task-10-unit-tests.txt
  
  **Commit**: YES
  - Message: `test(webhook): add comprehensive unit tests`
  - Files: `crates/openlark-webhook/tests/*.rs`
  - Pre-commit: `cargo test -p openlark-webhook --all-features`

- [x] 11. **Write integration tests with mock server**
  
  **What to do**:
  - 创建集成测试，使用 `wiremock` 模拟飞书服务器
  - 测试完整的消息发送流程
  - 测试签名验证流程
  - 测试错误响应处理（4xx, 5xx）
  - 测试速率限制（429）
  - 编写测试文档
  
  **Must NOT do**:
  - 不依赖真实网络请求
  - 不添加长时间运行的测试
  
  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: 复杂的集成测试编写
  - **Skills**: `[]`
  
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 10, 12, 13)
  - **Blocks**: FINAL
  - **Blocked By**: Tasks 5-9
  
  **References**:
  - `wiremock` crate 文档
  - `crates/openlark-core/src/testing/` - 测试工具
  
  **Acceptance Criteria**:
  - [ ] 集成测试文件创建完成
  - [ ] 所有集成测试通过
  - [ ] 错误场景覆盖
  
  **QA Scenarios (MANDATORY)**:
  ```
  Scenario: 集成测试通过
    Tool: Bash (cargo test)
    Preconditions: Task 完成
    Steps:
      1. cargo test -p openlark-webhook --all-features -- test_integration
    Expected Result: 集成测试通过
    Failure Indicators: 测试失败
    Evidence: .sisyphus/evidence/task-11-integration-tests.txt
  ```
  
  **Evidence to Capture**:
  - [ ] Evidence file: task-11-integration-tests.txt
  
  **Commit**: YES
  - Message: `test(webhook): add integration tests with mock server`
  - Files: `crates/openlark-webhook/tests/integration_*.rs`
  - Pre-commit: `cargo test -p openlark-webhook --all-features -- test_integration`

- [x] 12. **Create usage examples**
  
  **What to do**:
  - 创建 `examples/` 目录
  - 编写基础文本消息示例
  - 编写卡片消息示例
  - 编写带签名验证的示例
  - 编写错误处理示例
  - 创建 `examples/README.md` 文档
  
  **Must NOT do**:
  - 不在示例中使用真实 webhook URL
  - 不提交敏感信息（secret key）
  
  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 示例代码编写
  - **Skills**: `[]`
  
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 10, 11, 13)
  - **Blocks**: FINAL
  - **Blocked By**: Tasks 5-9
  
  **References**:
  - `examples/01_getting_started/` - 现有示例模式
  - 飞书 webhook 消息文档 - 消息格式
  
  **Acceptance Criteria**:
  - [ ] 示例文件创建完成
  - [ ] 示例代码编译通过
  - [ ] README 文档完整
  
  **QA Scenarios (MANDATORY)**:
  ```
  Scenario: 示例代码编译通过
    Tool: Bash (cargo check)
    Preconditions: Task 完成
    Steps:
      1. cargo check -p openlark-webhook --examples
    Expected Result: 所有示例编译通过
    Failure Indicators: 编译错误
    Evidence: .sisyphus/evidence/task-12-examples-check.txt
  ```
  
  **Evidence to Capture**:
  - [ ] Evidence file: task-12-examples-check.txt
  
  **Commit**: YES
  - Message: `docs(webhook): add usage examples`
  - Files: `crates/openlark-webhook/examples/*.rs`, `crates/openlark-webhook/examples/README.md`
  - Pre-commit: `cargo check -p openlark-webhook --examples`

- [x] 13. **Update workspace Cargo.toml and documentation**
  
  **What to do**:
  - 更新根 `Cargo.toml` 添加 `openlark-webhook` 到 workspace members
  - 添加 `webhook` feature 到根 Cargo.toml
  - 更新 `src/lib.rs` 重新导出 openlark-webhook
  - 更新 `README.md` 添加 webhook 功能说明
  - 更新 `CHANGELOG.md` 记录变更
  - 创建 `crates/openlark-webhook/README.md`
  
  **Must NOT do**:
  - 不破坏现有功能
  - 不修改其他 crate 的 API
  
  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 配置和文档更新
  - **Skills**: `[]`
  
  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 10, 11, 12)
  - **Blocks**: FINAL
  - **Blocked By**: Tasks 10-12
  
  **References**:
  - 根 `Cargo.toml:1-270` - Workspace 配置
  - 根 `README.md` - 文档结构
  
  **Acceptance Criteria**:
  - [ ] workspace Cargo.toml 更新
  - [ ] src/lib.rs 更新
  - [ ] README.md 更新
  - [ ] CHANGELOG.md 更新
  - [ ] crate README.md 创建
  
  **QA Scenarios (MANDATORY)**:
  ```
  Scenario: 工作空间集成正确
    Tool: Bash (cargo check)
    Preconditions: Task 完成
    Steps:
      1. cargo check --workspace
      2. cargo test --workspace
    Expected Result: 整体编译和测试通过
    Failure Indicators: 编译错误，测试失败
    Evidence: .sisyphus/evidence/task-13-workspace-integration.txt
  ```
  
  **Evidence to Capture**:
  - [ ] Evidence file: task-13-workspace-integration.txt
  
  **Commit**: YES
  - Message: `chore: integrate webhook into workspace`
  - Files: `Cargo.toml`, `src/lib.rs`, `README.md`, `CHANGELOG.md`, `crates/openlark-webhook/README.md`
  - Pre-commit: `cargo check --workspace`

---

## Final Verification Wave (MANDATORY — after ALL implementation tasks)

> 4 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [x] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, curl endpoint, run command). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in .sisyphus/evidence/. Compare deliverables against plan.
  Output: `Must Have [6/6] | Must NOT Have [8/8] | Tasks [13/13] | VERDICT: APPROVE`

- [x] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo clippy -p openlark-webhook --all-features` + `cargo test -p openlark-webhook --all-features`. Review all changed files for: `as any`/`@ts-ignore`, empty catches, console.log in prod, commented-out code, unused imports. Check AI slop: excessive comments, over-abstraction, generic names (data/result/item/temp).
  Output: `Build [PASS] | Lint [PASS] | Tests [47 pass/0 fail] | Files [12 clean/0 issues] | VERDICT: APPROVE`

- [x] F3. **Real Manual QA** — `unspecified-high`
  Start from clean state. Execute EVERY QA scenario from EVERY task — follow exact steps, capture evidence. Test cross-task integration (features working together, not isolation). Test edge cases: empty webhook_url, invalid secret, network errors. Save to `.sisyphus/evidence/final-qa/`.
  Output: `Scenarios [13/13 pass] | Integration [2/2] | Edge Cases [3 tested] | VERDICT: APPROVE`

- [x] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (git log/diff). Verify 1:1 — everything in spec was built (no missing), nothing beyond spec was built (no creep). Check "Must NOT do" compliance. Detect cross-task contamination: Task N touching Task M's files. Flag unaccounted changes.
  Output: `Tasks [13/13 compliant] | Contamination [CLEAN] | Unaccounted [CLEAN] | VERDICT: APPROVE`

---

## Commit Strategy

- **1**: `feat(webhook): add openlark-webhook crate skeleton` — Cargo.toml,- **2**: `feat(webhook): add core message types and models` — src/models.rs
- **3**: `feat(webhook): implement WebhookClient and message sending` — src/robot/*.rs
- **4**: `feat(webhook): add signature verification` — src/common/signature.rs
- **5**: `feat(webhook): add comprehensive tests` — tests/*.rs
- **6**: `feat(webhook): add usage examples` — examples/*.rs
- **7**: `feat(webhook): integrate into workspace` — root Cargo.toml, README.md

---

## Success Criteria

### Verification Commands
```bash
# 1. 编译检查（所有 features）
cargo check -p openlark-webhook --all-features
# Expected: 零警告，零错误

# 2. 基础测试
cargo test -p openlark-webhook --all-features
# Expected: 所有测试通过

# 3. Feature 组合测试
cargo check -p openlark-webhook --no-default-features
cargo check -p openlark-webhook --features "robot"
cargo check -p openlark-webhook --features "robot,signature"
cargo check -p openlark-webhook --features "robot,card"

# 4. 文档生成
cargo doc -p openlark-webhook --no-deps
# Expected: 文档生成成功

# 5. 集成到工作空间
cargo check --workspace
cargo test --workspace
# Expected: 整体编译和测试通过
```

### Final Checklist
- [x] 所有 "Must Have" 存在
- [x] 所有 "Must NOT Have" 不存在
- [x] 所有测试通过
- [x] Feature flags 正确工作
- [x] 文档完整
- [x] 示例代码可运行
