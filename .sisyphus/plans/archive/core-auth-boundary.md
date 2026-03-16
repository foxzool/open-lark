# Plan 1: Core-Auth 边界重构 (Issue A + C)

## TL;DR

> **Quick Summary**: 统一双重 Config 体系（Client Config 包装 Core Config），并将 core auth 模块中的具体实现迁移到 openlark-auth，使 core 只保留抽象。
> 
> **Deliverables**:
> - 重构后的 `openlark_client::Config`（内部包装 `openlark_core::config::Config`）
> - 瘦身后的 `openlark-core/src/auth/`（仅保留 trait + types）
> - 迁移到 `openlark-auth` 的具体实现（cache/refresh/validator/token/app_ticket）
> - 删除 `core_config.rs` 转换层
> - 关键路径补充测试
> 
> **Estimated Effort**: Medium
> **Parallel Execution**: YES - 3 waves
> **Critical Path**: Task 1 → Task 3 → Task 5 → Task 7 → Task 8

---

## Context

### Original Request
检查 openlark-core / openlark-auth / openlark-client 的代码组织关系，发现双重 Config 体系和 core auth 模块职责越界两个高优先级问题。

### Interview Summary
**Key Discussions**:
- Config 合并策略：Client Config 包装 Core Config（组合模式），不合并为单一类型
- Breaking changes：允许，当前 0.15.0-dev
- 测试策略：补充关键路径测试

### 关键发现
- `openlark_client::Config`（plain struct）与 `openlark_core::config::Config`（Arc 包装）字段名不一致（`headers` vs `header`，`timeout` vs `req_timeout`）
- `core_config.rs` 是两者之间的转换层，增加维护负担
- `openlark-core/src/auth/` 包含 7 个文件，其中 5 个是具体实现（违反 core 不引入业务逻辑原则）
- `openlark-auth` 已有自己的 token 缓存（`AuthTokenProvider` 内部 `RwLock<HashMap>`），未复用 core 的 `MemoryTokenCache`
- 18 个业务 crate 依赖 `core::Config`，变更影响面大

---

## Work Objectives

### Core Objective
消除双重 Config 体系和 core auth 职责越界，建立清晰的 core（抽象）→ auth（实现）→ client（组装）分层。

### Concrete Deliverables
- `crates/openlark-client/src/config.rs` — 重构为包装 `openlark_core::config::Config`
- `crates/openlark-client/src/core_config.rs` — 删除
- `crates/openlark-core/src/auth/` — 仅保留 `mod.rs` + `token_provider.rs`
- `crates/openlark-auth/src/` — 接收迁移的具体实现
- 补充测试覆盖关键路径

### Definition of Done
- [x] `cargo check --workspace --all-features` 零错误
- [x] `cargo test --workspace` 全部通过
- [x] `cargo clippy --workspace --all-features` 零警告
- [x] `openlark-core/src/auth/` 仅包含 `mod.rs` + `token_provider.rs` + `app_ticket.rs`
- [x] `openlark-client/src/core_config.rs` 已删除
- [x] `openlark_client::Config` 内部持有 `openlark_core::config::Config`

### Must Have
- Core Config 的 Arc 零拷贝语义保持不变
- 所有 18 个业务 crate 正常编译
- TokenProvider trait 接口不变
- client.docs / client.auth / client.communication 等 meta 链式调用不受影响

### Must NOT Have (Guardrails)
- ❌ 不修改任何业务 crate 的内部实现（docs/communication/hr 等）
- ❌ 不改变 core::Config 的公开 API 签名（`app_id()`, `base_url()` 等 getter）
- ❌ 不引入新的第三方依赖
- ❌ 不修改 TokenProvider trait 定义
- ❌ 不在 core 中保留任何具体 token 管理实现

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed.

### Test Decision
- **Infrastructure exists**: YES（cargo test, rstest, wiremock, mockall）
- **Automated tests**: Tests-after（重构后补充关键路径测试）
- **Framework**: cargo test + rstest

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Library/Module**: Use Bash (cargo test / cargo check) — 编译检查、测试运行、clippy 检查

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — 分析 + 准备):
├── Task 1: 审计 core auth 模块的外部引用 [quick]
├── Task 2: 审计 client Config 的所有使用点 [quick]

Wave 2 (After Wave 1 — 核心重构):
├── Task 3: 重构 client::Config 为 Core Config 包装器 [deep]
├── Task 4: 迁移 core auth 具体实现到 openlark-auth [deep]

Wave 3 (After Wave 2 — 清理 + 验证):
├── Task 5: 删除 core_config.rs + 更新 client 构建逻辑 [unspecified-high]
├── Task 6: 补充关键路径测试 [unspecified-high]
├── Task 7: 全工作区编译验证 [quick]

Wave FINAL (After ALL — 独立审查):
├── Task F1: Plan compliance audit [oracle]
├── Task F2: Code quality review [unspecified-high]
├── Task F3: Scope fidelity check [deep]

Critical Path: Task 1 → Task 3 → Task 5 → Task 7 → F1-F3
Max Concurrent: 2 (Waves 1 & 2)
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|-----------|--------|
| 1 | — | 4 |
| 2 | — | 3, 5 |
| 3 | 2 | 5, 6 |
| 4 | 1 | 5, 6 |
| 5 | 3, 4 | 7 |
| 6 | 3, 4 | 7 |
| 7 | 5, 6 | F1-F3 |

### Agent Dispatch Summary

- **Wave 1**: 2 tasks — T1 → `quick`, T2 → `quick`
- **Wave 2**: 2 tasks — T3 → `deep`, T4 → `deep`
- **Wave 3**: 3 tasks — T5 → `unspecified-high`, T6 → `unspecified-high`, T7 → `quick`
- **FINAL**: 3 tasks — F1 → `oracle`, F2 → `unspecified-high`, F3 → `deep`

---

## TODOs

- [x] 1. 审计 core auth 模块的外部引用

  **What to do**:
  - 使用 `lsp_find_references` 和 `grep` 查找 `openlark-core/src/auth/` 中所有公开类型的外部引用
  - 记录哪些 crate 使用了 `MemoryTokenCache`, `TokenRefresher`, `TokenValidator`, `TokenManager`, `CacheConfig`, `CacheStats`, `TokenInfo`, `TokenType`, `TokenValidationResult`, `RefreshTokenResponse`, `TokenRefreshConfig`
  - 记录 `app_ticket.rs` 的外部引用
  - 输出引用清单到 `.sisyphus/evidence/task-1-core-auth-refs.md`

  **Must NOT do**:
  - 不修改任何代码

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Task 2)
  - **Blocks**: Task 4
  - **Blocked By**: None

  **References**:
  - `crates/openlark-core/src/auth/mod.rs:12-16` — 当前 re-export 的所有公开类型
  - `crates/openlark-core/src/auth/cache.rs` — MemoryTokenCache, CacheConfig, CacheStats
  - `crates/openlark-core/src/auth/token.rs` — TokenManager, TokenInfo, TokenType
  - `crates/openlark-core/src/auth/refresh.rs` — TokenRefresher, RefreshTokenResponse
  - `crates/openlark-core/src/auth/validator.rs` — TokenValidator
  - `crates/openlark-core/src/auth/app_ticket.rs` — App Ticket 管理

  **Acceptance Criteria**:
  - [ ] 引用清单文件存在: `.sisyphus/evidence/task-1-core-auth-refs.md`
  - [ ] 清单包含每个公开类型的引用位置（crate + file + line）

  **QA Scenarios:**
  ```
  Scenario: 引用清单完整性
    Tool: Bash (grep)
    Steps:
      1. grep -r "MemoryTokenCache\|TokenRefresher\|TokenValidator\|TokenManager\|CacheConfig" crates/ --include="*.rs" -l
      2. 对比清单文件中列出的文件
    Expected Result: 清单覆盖所有引用文件
    Evidence: .sisyphus/evidence/task-1-core-auth-refs.md
  ```

  **Commit**: NO

- [x] 2. 审计 client Config 的所有使用点

  **What to do**:
  - 查找 `openlark_client::Config` 的所有字段访问点（`config.app_id`, `config.timeout`, `config.headers` 等）
  - 查找 `openlark_client::config::ConfigBuilder` 的所有方法调用
  - 查找 `core_config.rs` 中 `build_base_core_config` 和 `build_core_config_with_default_token_provider` 的调用点
  - 记录字段映射关系：client 字段 → core 字段
  - 输出到 `.sisyphus/evidence/task-2-config-usage.md`

  **Must NOT do**:
  - 不修改任何代码

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Task 1)
  - **Blocks**: Task 3, Task 5
  - **Blocked By**: None

  **References**:
  - `crates/openlark-client/src/config.rs` — client Config 定义，字段: app_id, app_secret, app_type, enable_token_cache, base_url, timeout, retry_count, enable_log, headers
  - `crates/openlark-core/src/config.rs` — core Config 定义，字段: app_id, app_secret, base_url, enable_token_cache, app_type, http_client, req_timeout, header, token_provider
  - `crates/openlark-client/src/core_config.rs` — 转换层，`build_base_core_config()` 和 `build_core_config_with_default_token_provider()`
  - `crates/openlark-client/src/client.rs:155-156` — 调用转换函数的位置

  **Acceptance Criteria**:
  - [ ] 使用点清单文件存在: `.sisyphus/evidence/task-2-config-usage.md`
  - [ ] 清单包含字段映射表（client field → core field）

  **QA Scenarios:**
  ```
  Scenario: 字段映射完整性
    Tool: Bash (grep)
    Steps:
      1. grep -rn "config\.timeout\|config\.retry_count\|config\.enable_log\|config\.headers\|config\.app_id\|config\.app_secret\|config\.base_url\|config\.app_type\|config\.enable_token_cache" crates/openlark-client/src/ --include="*.rs"
      2. 对比清单中的使用点
    Expected Result: 所有字段访问点都被记录
    Evidence: .sisyphus/evidence/task-2-config-usage.md
  ```

  **Commit**: NO

- [x] 3. 重构 client::Config 为 Core Config 包装器
  **What to do**:
  - 修改 `crates/openlark-client/src/config.rs`：
    - `Config` 内部持有 `openlark_core::config::Config` 字段（命名为 `core_config`）
    - 保留 client 独有字段：`retry_count: u32`, `enable_log: bool`
    - 删除与 core 重复的字段（app_id, app_secret, base_url, timeout, app_type, enable_token_cache, headers）
    - 通过 `Deref<Target=openlark_core::config::Config>` 或委托方法暴露 core 字段
  - 修改 `ConfigBuilder`：
    - 内部构建 `openlark_core::config::ConfigBuilder` + client 独有字段
    - `build()` 先构建 core Config，再包装为 client Config
  - 修改 `Config::validate()` 保持现有验证逻辑
  - 修改 `Config::from_env()` / `load_from_env()` 适配新结构
  - 更新 `Config::summary()` 适配新结构
  **Must NOT do**:
  - 不修改 `openlark_core::config::Config` 的任何公开 API
  - 不修改业务 crate 代码
  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: [`openlark-code-standards`]
    - `openlark-code-standards`: 确保重构后代码符合项目规范
  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 4)
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 5, Task 6
  - **Blocked By**: Task 2
  **References**:
  - `crates/openlark-client/src/config.rs` — 当前 client Config 完整定义（需重构）
  - `crates/openlark-core/src/config.rs:38-45` — core Config 的 Arc<ConfigInner> 结构（包装目标）
  - `crates/openlark-core/src/config.rs:90-96` — core Config 的 Deref 实现（参考模式）
  - `crates/openlark-core/src/config.rs:181-284` — core ConfigBuilder（需在 client builder 中复用）
  - `.sisyphus/evidence/task-2-config-usage.md` — Task 2 产出的字段映射清单
  **Acceptance Criteria**:
  - [ ] `openlark_client::Config` 内部持有 `openlark_core::config::Config`
  - [ ] `config.app_id()` / `config.base_url()` 等通过委托访问 core Config
  - [ ] `config.retry_count` / `config.enable_log` 仍为 client 独有字段
  - [ ] `cargo check -p openlark-client --all-features` 通过
  - [ ] 现有 client config 测试全部通过
  **QA Scenarios:**
  ```
  Scenario: Config 包装正确性
    Tool: Bash (cargo test)
    Steps:
      1. cargo test -p openlark-client config -- --nocapture
      2. 验证 app_id/app_secret/base_url 通过委托正确访问
    Expected Result: 所有 config 相关测试通过
    Evidence: .sisyphus/evidence/task-3-config-wrap.txt
  Scenario: 编译兼容性
    Tool: Bash (cargo check)
    Steps:
      1. cargo check --workspace --all-features
    Expected Result: 0 errors
    Evidence: .sisyphus/evidence/task-3-workspace-check.txt
  ```
  **Commit**: YES
  - Message: `refactor(config): client Config 包装 core Config，消除双重配置体系`
  - Files: `crates/openlark-client/src/config.rs`, `crates/openlark-client/src/client.rs`
  - Pre-commit: `cargo check --workspace --all-features`

- [x] 4. 迁移 core auth 具体实现到 openlark-auth
  **What to do**:
  - 从 `crates/openlark-core/src/auth/` 迁移以下文件到 `crates/openlark-auth/src/`：
    - `cache.rs` → `crates/openlark-auth/src/common/cache.rs`（MemoryTokenCache, CacheConfig, CacheStats, TokenStorage）
    - `refresh.rs` → `crates/openlark-auth/src/common/refresh.rs`（TokenRefresher, RefreshTokenResponse）
    - `validator.rs` → `crates/openlark-auth/src/common/validator.rs`（TokenValidator）
    - `token.rs` → `crates/openlark-auth/src/common/token.rs`（TokenManager, TokenInfo, TokenType 等）
    - `app_ticket.rs` → `crates/openlark-auth/src/common/app_ticket.rs`
  - 更新 `crates/openlark-core/src/auth/mod.rs`：仅保留 `pub mod token_provider;` 和对应 re-export
  - 删除 core auth 中已迁移的文件
  - 更新 `crates/openlark-auth/src/common/mod.rs` 导出迁移的模块
  - 更新 `crates/openlark-auth/src/lib.rs` 导出新模块
  - 根据 Task 1 的引用清单，更新所有外部引用路径（如有）
  **Must NOT do**:
  - 不修改 `TokenProvider` trait 定义
  - 不修改 `NoOpTokenProvider`
  - 不修改 `TokenRequest`
  - 不修改业务 crate 代码（如果业务 crate 引用了被迁移的类型，通过 openlark-auth re-export 解决）
  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: [`openlark-code-standards`]
    - `openlark-code-standards`: 确保迁移后模块组织符合项目规范
  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 3)
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 5, Task 6
  - **Blocked By**: Task 1
  **References**:
  - `crates/openlark-core/src/auth/mod.rs` — 当前 re-export 清单（第 12-16 行）
  - `crates/openlark-core/src/auth/cache.rs` — MemoryTokenCache 实现
  - `crates/openlark-core/src/auth/token.rs` — TokenManager/TokenInfo 实现
  - `crates/openlark-core/src/auth/refresh.rs` — TokenRefresher 实现
  - `crates/openlark-core/src/auth/validator.rs` — TokenValidator 实现
  - `crates/openlark-core/src/auth/app_ticket.rs` — App Ticket 管理
  - `crates/openlark-auth/src/common/` — 迁移目标目录
  - `.sisyphus/evidence/task-1-core-auth-refs.md` — Task 1 产出的引用清单
  **Acceptance Criteria**:
  - [ ] `crates/openlark-core/src/auth/` 仅包含 `mod.rs` + `token_provider.rs`
  - [ ] 迁移的类型在 `openlark-auth` 中可正常导入
  - [ ] `cargo check -p openlark-core` 通过
  - [ ] `cargo check -p openlark-auth --all-features` 通过
  **QA Scenarios:**
  ```
  Scenario: core auth 瘦身验证
    Tool: Bash (ls + cargo check)
    Steps:
      1. ls crates/openlark-core/src/auth/
      2. 验证仅包含 mod.rs 和 token_provider.rs
      3. cargo check -p openlark-core
    Expected Result: 仅 2 个文件，编译通过
    Evidence: .sisyphus/evidence/task-4-core-auth-slim.txt
  Scenario: auth crate 编译验证
    Tool: Bash (cargo check)
    Steps:
      1. cargo check -p openlark-auth --all-features
    Expected Result: 0 errors
    Evidence: .sisyphus/evidence/task-4-auth-check.txt
  ```
  **Commit**: YES
  - Message: `refactor(core): 迁移 auth 具体实现到 openlark-auth，core 仅保留抽象`
  - Files: `crates/openlark-core/src/auth/*`, `crates/openlark-auth/src/common/*`
  - Pre-commit: `cargo check --workspace --all-features`

- [x] 5. 删除 core_config.rs + 更新 client 构建逻辑
  **What to do**:
  - 删除 `crates/openlark-client/src/core_config.rs`
  - 修改 `crates/openlark-client/src/client.rs` 中 `Client::with_config()`：
    - 不再调用 `build_base_core_config()` / `build_core_config_with_default_token_provider()`
    - 直接从新的 client Config 中获取内部 core Config
    - AuthTokenProvider 注入逻辑移到 client Config 构建过程中
  - 修改 `crates/openlark-client/src/lib.rs`：移除 `mod core_config;`
  - 确保 AuthClient 使用 base core config（不含 TokenProvider），其他业务 client 使用带 TokenProvider 的 config
  **Must NOT do**:
  - 不修改业务 crate 代码
  - 不改变 meta 链式调用行为
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: [`openlark-code-standards`]
  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (sequential after Wave 2)
  - **Blocks**: Task 7
  - **Blocked By**: Task 3, Task 4
  **References**:
  - `crates/openlark-client/src/core_config.rs` — 待删除的转换层
  - `crates/openlark-client/src/client.rs:155-172` — Client::with_config() 中调用转换函数的位置
  - `crates/openlark-client/src/lib.rs:217` — `mod core_config;` 声明
  - `crates/openlark-auth/src/token_provider.rs:71` — AuthTokenProvider::new() 接受 core Config
  **Acceptance Criteria**:
  - [ ] `crates/openlark-client/src/core_config.rs` 已删除
  - [ ] `Client::with_config()` 不再引用 core_config 模块
  - [ ] `cargo check --workspace --all-features` 通过
  - [ ] `cargo test -p openlark-client` 全部通过
  **QA Scenarios:**
  ```
  Scenario: core_config.rs 已删除
    Tool: Bash
    Steps:
      1. test ! -f crates/openlark-client/src/core_config.rs && echo "DELETED"
    Expected Result: 输出 "DELETED"
    Evidence: .sisyphus/evidence/task-5-core-config-deleted.txt
  Scenario: 客户端构建正常
    Tool: Bash (cargo test)
    Steps:
      1. cargo test -p openlark-client -- test_client_builder test_client_config --nocapture
    Expected Result: 测试通过
    Evidence: .sisyphus/evidence/task-5-client-build.txt
  ```
  **Commit**: YES (groups with Task 6)
  - Message: `refactor(client): 删除 core_config.rs 转换层，简化配置流转`
  - Files: `crates/openlark-client/src/core_config.rs` (deleted), `crates/openlark-client/src/client.rs`, `crates/openlark-client/src/lib.rs`
  - Pre-commit: `cargo check --workspace --all-features`
- [x] 6. 补充关键路径测试
  **What to do**:
  - 在 `crates/openlark-client/src/config.rs` 的 tests 模块中添加：
    - 测试 client Config 包装 core Config 的正确性（字段透传）
    - 测试 ConfigBuilder 构建后 core_config 字段一致性
    - 测试 from_env() 加载后 core Config 字段正确
  - 在 `crates/openlark-client/src/client.rs` 的 tests 模块中添加：
    - 测试 Client 构建后 core_config() 返回正确的 Config
    - 测试 AuthClient 使用 base config（无 TokenProvider）
    - 测试业务 client（docs/communication）使用带 TokenProvider 的 config
  - 在 `crates/openlark-auth/` 中验证迁移的模块可正常导入和使用
  **Must NOT do**:
  - 不修改生产代码
  - 不添加需要网络请求的集成测试
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 5)
  - **Parallel Group**: Wave 3
  - **Blocks**: Task 7
  - **Blocked By**: Task 3, Task 4
  **References**:
  - `crates/openlark-client/src/config.rs:449-615` — 现有 config 测试（需扩展）
  - `crates/openlark-client/src/client.rs:354-557` — 现有 client 测试（需扩展）
  - `crates/openlark-core/src/config.rs:286-519` — core config 测试（参考模式）
  **Acceptance Criteria**:
  - [ ] 新增 ≥5 个测试用例
  - [ ] `cargo test -p openlark-client` 全部通过
  - [ ] `cargo test -p openlark-auth` 全部通过
  **QA Scenarios:**
  ```
  Scenario: 新测试全部通过
    Tool: Bash (cargo test)
    Steps:
      1. cargo test -p openlark-client -- --nocapture 2>&1 | tail -5
      2. cargo test -p openlark-auth -- --nocapture 2>&1 | tail -5
    Expected Result: 所有测试通过，0 failures
    Evidence: .sisyphus/evidence/task-6-tests.txt
  ```
  **Commit**: YES (groups with Task 5)
  - Message: `test(client): 补充 Config 包装和 TokenProvider 注入的关键路径测试`
  - Files: `crates/openlark-client/src/config.rs`, `crates/openlark-client/src/client.rs`
  - Pre-commit: `cargo test -p openlark-client -p openlark-auth`
- [x] 7. 全工作区编译验证
  **What to do**:
  - 运行全工作区编译检查
  - 运行全工作区测试
  - 运行 clippy 检查
  - 更新 `crates/openlark-core/AGENTS.md` 中 auth 模块的结构描述
  - 更新 `crates/openlark-auth/AGENTS.md`（如有）反映新增的 common/ 模块
  **Must NOT do**:
  - 不修改生产代码（仅文档）
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (after Task 5, 6)
  - **Blocks**: F1-F3
  - **Blocked By**: Task 5, Task 6
  **References**:
  - `crates/openlark-core/AGENTS.md` — 需更新 STRUCTURE 部分
  **Acceptance Criteria**:
  - [ ] `cargo check --workspace --all-features` 通过
  - [ ] `cargo test --workspace` 全部通过
  - [ ] `cargo clippy --workspace --all-features` 零警告
  - [ ] AGENTS.md 已更新
  **QA Scenarios:**
  ```
  Scenario: 全工作区验证
    Tool: Bash
    Steps:
      1. cargo check --workspace --all-features 2>&1 | tail -3
      2. cargo test --workspace 2>&1 | tail -5
      3. cargo clippy --workspace --all-features -- -D warnings 2>&1 | tail -3
    Expected Result: 全部通过，0 errors，0 warnings
    Evidence: .sisyphus/evidence/task-7-workspace-verify.txt
  ```
  **Commit**: YES
  - Message: `docs(core,auth): 更新 AGENTS.md 反映 auth 模块重构后的结构`
  - Files: `crates/openlark-core/AGENTS.md`
  - Pre-commit: `cargo test --workspace`

## Final Verification Wave (MANDATORY)

- [x] F1. **Plan compliance audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists. For each "Must NOT Have": search codebase for forbidden patterns. Check evidence files exist.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **Code quality review** — `unspecified-high`
  Run `cargo check --workspace --all-features` + `cargo clippy --workspace --all-features` + `cargo test --workspace`. Review changed files for: `unwrap()` in lib code, dead imports, commented-out code.
  Output: `Build [PASS/FAIL] | Clippy [PASS/FAIL] | Tests [N pass/N fail] | VERDICT`

- [x] F3. **Scope fidelity check** — `deep`
  For each task: read "What to do", read actual diff. Verify 1:1 — everything in spec was built, nothing beyond spec was built. Check "Must NOT do" compliance. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

- **Wave 2 commit**: `refactor(config): client Config 包装 core Config，消除双重配置体系`
  - Files: `crates/openlark-client/src/config.rs`, `crates/openlark-client/src/client.rs`
  - Pre-commit: `cargo check --workspace --all-features`

- **Wave 2 commit**: `refactor(core): 迁移 auth 具体实现到 openlark-auth，core 仅保留抽象`
  - Files: `crates/openlark-core/src/auth/*`, `crates/openlark-auth/src/*`
  - Pre-commit: `cargo check --workspace --all-features`

- **Wave 3 commit**: `refactor(client): 删除 core_config.rs 转换层，补充测试`
  - Files: `crates/openlark-client/src/core_config.rs` (deleted), tests
  - Pre-commit: `cargo test --workspace`

---

## Success Criteria

### Verification Commands
```bash
cargo check --workspace --all-features  # Expected: 0 errors
cargo test --workspace                   # Expected: all pass
cargo clippy --workspace --all-features  # Expected: 0 warnings
# Verify core auth is slim:
ls crates/openlark-core/src/auth/        # Expected: mod.rs, token_provider.rs only
# Verify core_config.rs deleted:
test ! -f crates/openlark-client/src/core_config.rs  # Expected: exit 0
```

### Final Checklist
- [x] All "Must Have" present
- [x] All "Must NOT Have" absent
- [x] All tests pass
- [x] AGENTS.md for core updated to reflect new structure
