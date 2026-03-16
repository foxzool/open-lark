# 修复 OpenLark 代码质量问题

## TL;DR

> 修复代码质量检查中发现的 P0/P1 级别问题，包括命名规范异常、字段未参与请求、硬编码 URL、配置不一致等。
> 
> **预计工作量**: 2-3 小时
> **关键路径**: 修复命名 → 修复字段传递 → 修复硬编码 URL → 配置对齐 → 格式化验证

---

## Context

### 原始检查报告
代码质量检查发现以下问题：

**P0 - 高优先级（影响正确性/命名规范）**
1. `udeleteRequest` 命名未遵循 PascalCase 规范
2. `QuerySessionRequest.user_ids` 定义后未参与请求构造

**P1 - 中优先级（影响一致性/可维护性）**
3. 硬编码 URL 未走 endpoint 枚举（5处）
4. 覆盖率配置与 feature 定义不一致
5. 代码格式化问题

### 相关文件

| 问题 | 文件路径 | 行号 |
|------|----------|------|
| 命名异常 | `crates/openlark-security/src/acs/acs/v1/visitor/delete.rs` | 3 |
| 命名异常 | `crates/openlark-security/src/acs/acs/v1/rule_external/delete.rs` | 3 |
| 字段未参与 | `crates/openlark-auth/src/passport/passport/v1/session/query.rs` | 17-43 |
| 硬编码URL | `crates/openlark-platform/src/admin/admin/v1/badge/create.rs` | 64 |
| 配置不一致 | `.cargo-llvm-cov.toml` | 41 |

### 参考模式

**正确实现模式**（参考）：
- `crates/openlark-docs/src/common/api_endpoints.rs` - endpoint 枚举定义
- `crates/openlark-docs/src/base/bitable/v1/app/table/record/create.rs` - Builder + endpoint 模式

---

## Work Objectives

### 核心目标
修复代码质量检查中识别的所有 P0/P1 问题，确保代码符合 Rust 命名规范和项目 API 实现模式。

### 具体交付物
- [x] 重命名 `udeleteRequest` → `DeleteXxxRequest`（2个文件）
- [x] 修复 `QuerySessionRequest.user_ids` 字段传递
- [x] 为相关 crates 创建/完善 `api_endpoints.rs` endpoint 枚举
- [x] 替换硬编码 URL 为 `ApiEndpoint` 枚举
- [x] 修复 `.cargo-llvm-cov.toml` 与 `Cargo.toml` feature 配置一致性
- [x] 运行 `cargo fmt` 统一代码风格
- [x] 验证修复：`cargo clippy -- -D warnings` 通过

### 定义 of Done
- [x] 所有 P0 问题已修复
- [x] 所有 P1 问题已修复
- [x] `cargo fmt --check` 通过
- [x] `cargo clippy --workspace --all-features -- -D warnings` 通过
- [x] 相关模块编译通过

### Must NOT Have（防 scope creep）
- 不修改 API 的业务逻辑
- 不添加新的 API 功能
- 不修改测试文件（除非必要）
- 不做大规模重构

---

## Verification Strategy

### 测试决策
- **基础设施存在**: YES
- **Automated tests**: Tests-after
- **Framework**: cargo test

### QA Policy
每个修复任务完成后，执行 agent-executed QA 验证。

---

## Execution Strategy

### Wave 1 - P0 修复（可并行执行）

```
Wave 1 (Start Immediately):
├── Task 1: 修复命名规范异常 - udeleteRequest (visitor/delete.rs) [quick]
├── Task 2: 修复命名规范异常 - udeleteRequest (rule_external/delete.rs) [quick]
├── Task 3: 修复 QuerySessionRequest.user_ids 字段传递 [quick]
└── Task 4: 创建 openlark-auth api_endpoints.rs [quick]
```

**依赖矩阵**:
- Task 1: — → Task 5
- Task 2: — → Task 5
- Task 3: — → Task 4
- Task 4: — → Task 5

### Wave 2 - P1 修复（依赖 Wave 1）

```
Wave 2 (After Wave 1):
├── Task 5: 创建 openlark-platform api_endpoints.rs [quick]
├── Task 6: 替换硬编码 URL - CreateBadgeBuilder [quick]
├── Task 7: 替换硬编码 URL - QuerySessionRequest [quick]
├── Task 8: 修复覆盖率配置一致性 [quick]
└── Task 9: 运行代码格式化 [quick]
```

**依赖矩阵**:
- Task 5: Task 1-4 → Task 6
- Task 6: Task 5 → Task 10
- Task 7: Task 4 → Task 10
- Task 8: — → Task 10
- Task 9: — → Task 10

### Wave 3 - 最终验证

```
Wave 3 (After Wave 2):
└── Task 10: 运行 clippy 和 fmt 检查 [quick]
```

**关键路径**: Task 1/2/3/4 → Task 5/6/7/8/9 → Task 10

---

## TODOs

- [x] 1. 修复命名规范异常 - udeleteRequest (visitor/delete.rs)

  **What to do**:
  - 打开文件 `crates/openlark-security/src/acs/acs/v1/visitor/delete.rs`
  - 将 `pub struct udeleteRequest;` 重命名为 `pub struct DeleteVisitorRequest;`
  - 将 `pub struct udeleteResponse;` 重命名为 `pub struct DeleteVisitorResponse;`
  - 检查并修复所有引用（如有）

  **Must NOT do**:
  - 不修改文件其他内容
  - 不添加新的字段或方法

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1
  - **Blocks**: Task 5

  **References**:
  - 当前文件: `crates/openlark-security/src/acs/acs/v1/visitor/delete.rs:3-4`
  - 命名规范: Rust RFC 430 - PascalCase for types

  **Acceptance Criteria**:
  - [ ] `DeleteVisitorRequest` 命名符合 PascalCase
  - [ ] `DeleteVisitorResponse` 命名符合 PascalCase
  - [ ] 文件编译通过: `cargo check -p openlark-security`

  **QA Scenarios**:
  ```
  Scenario: 验证重命名成功
    Tool: Bash
    Steps:
      1. cargo check -p openlark-security
    Expected Result: 编译通过，无错误
    Evidence: .sisyphus/evidence/task1-check.log
  ```

  **Commit**: YES
  - Message: `style(security): 重命名 udeleteRequest 为 DeleteVisitorRequest`
  - Files: `crates/openlark-security/src/acs/acs/v1/visitor/delete.rs`

---

- [x] 2. 修复命名规范异常 - udeleteRequest (rule_external/delete.rs)

  **What to do**:
  - 打开文件 `crates/openlark-security/src/acs/acs/v1/rule_external/delete.rs`
  - 将 `pub struct udeleteRequest;` 重命名为 `pub struct DeleteRuleExternalRequest;`
  - 将 `pub struct udeleteResponse;` 重命名为 `pub struct DeleteRuleExternalResponse;`
  - 检查并修复所有引用（如有）

  **Must NOT do**:
  - 不修改文件其他内容
  - 不添加新的字段或方法

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1
  - **Blocks**: Task 5

  **References**:
  - 当前文件: `crates/openlark-security/src/acs/acs/v1/rule_external/delete.rs:3-4`

  **Acceptance Criteria**:
  - [ ] `DeleteRuleExternalRequest` 命名符合 PascalCase
  - [ ] `DeleteRuleExternalResponse` 命名符合 PascalCase
  - [ ] 文件编译通过: `cargo check -p openlark-security`

  **QA Scenarios**:
  ```
  Scenario: 验证重命名成功
    Tool: Bash
    Steps:
      1. cargo check -p openlark-security
    Expected Result: 编译通过，无错误
    Evidence: .sisyphus/evidence/task2-check.log
  ```

  **Commit**: YES (squash with Task 1)
  - Message: `style(security): 重命名 udeleteRequest 为 DeleteRuleExternalRequest`
  - Files: `crates/openlark-security/src/acs/acs/v1/rule_external/delete.rs`

---

- [x] 3. 修复 QuerySessionRequest.user_ids 字段传递

  **What to do**:
  - 打开文件 `crates/openlark-auth/src/passport/passport/v1/session/query.rs`
  - 修改 `execute_with_options` 方法，将 `user_ids` 字段加入请求
  - 参考飞书 API 文档，确定请求格式（POST body 或 query 参数）
  - 添加请求体结构，序列化时包含 user_ids

  **参考实现**（类似模式）：
  ```rust
  // 参考 crates/openlark-docs/src/base/bitable/v1/app/table/record/create.rs
  let request_body = QuerySessionRequestBody {
      user_ids: self.user_ids,
  };
  
  let api_request: ApiRequest<QuerySessionResponse> =
      ApiRequest::post("/open-apis/passport/v1/sessions/query")
          .body(serde_json::to_value(&request_body)?);
  ```

  **Must NOT do**:
  - 不修改 API 的行为逻辑
  - 不添加新的公开 API

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1
  - **Blocks**: Task 4, Task 7

  **References**:
  - 当前文件: `crates/openlark-auth/src/passport/passport/v1/session/query.rs:17-49`
  - API 文档: https://open.feishu.cn/document/server-docs/authentication-management/login-state-management/query
  - 参考模式: `crates/openlark-docs/src/base/bitable/v1/app/table/record/create.rs:57-65`

  **Acceptance Criteria**:
  - [ ] `user_ids` 字段在请求时被正确序列化
  - [ ] 添加了请求体结构 `QuerySessionRequestBody`
  - [ ] 文件编译通过: `cargo check -p openlark-auth`

  **QA Scenarios**:
  ```
  Scenario: 验证字段传递
    Tool: Bash
    Steps:
      1. cargo check -p openlark-auth
      2. cargo build -p openlark-auth
    Expected Result: 编译通过，无警告
    Evidence: .sisyphus/evidence/task3-build.log
  ```

  **Commit**: YES
  - Message: `fix(auth): 修复 QuerySessionRequest.user_ids 字段未参与请求`
  - Files: `crates/openlark-auth/src/passport/passport/v1/session/query.rs`

---

- [x] 4. 创建 openlark-auth api_endpoints.rs

  **What to do**:
  - 创建文件 `crates/openlark-auth/src/common/api_endpoints.rs`
  - 定义 `ApiEndpoint` 枚举，包含 `QuerySession` 端点
  - 添加 `to_url()` 方法返回 URL 路径
  - 在 `lib.rs` 或 `mod.rs` 中导出

  **参考实现**：
  ```rust
  // 参考 crates/openlark-docs/src/common/api_endpoints.rs
  #[derive(Debug, Clone, Copy)]
  pub enum ApiEndpoint {
      QuerySession,
  }

  impl ApiEndpoint {
      pub fn to_url(&self) -> &'static str {
          match self {
              ApiEndpoint::QuerySession => "/open-apis/passport/v1/sessions/query",
          }
      }
  }
  ```

  **Must NOT do**:
  - 不添加本项目未使用的端点
  - 不修改现有模块结构

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO (依赖 Task 3 确定端点)
  - **Blocked By**: Task 3
  - **Blocks**: Task 7

  **References**:
  - 参考: `crates/openlark-docs/src/common/api_endpoints.rs`

  **Acceptance Criteria**:
  - [ ] `api_endpoints.rs` 文件创建成功
  - [ ] `ApiEndpoint` 枚举定义正确
  - [ ] `to_url()` 方法返回正确路径
  - [ ] 模块导出正确

  **QA Scenarios**:
  ```
  Scenario: 验证模块创建
    Tool: Bash
    Steps:
      1. cargo check -p openlark-auth
    Expected Result: 编译通过
    Evidence: .sisyphus/evidence/task4-check.log
  ```

  **Commit**: YES
  - Message: `feat(auth): 添加 api_endpoints.rs 端点枚举`
  - Files: `crates/openlark-auth/src/common/api_endpoints.rs`

---

- [x] 5. 创建 openlark-platform api_endpoints.rs

  **What to do**:
  - 创建文件 `crates/openlark-platform/src/common/api_endpoints.rs`
  - 定义 `ApiEndpoint` 枚举，包含 `CreateBadge` 端点
  - 添加 `to_url()` 方法返回 URL 路径
  - 在 `lib.rs` 或 `mod.rs` 中导出

  **参考实现**：
  ```rust
  #[derive(Debug, Clone, Copy)]
  pub enum ApiEndpoint {
      CreateBadge,
  }

  impl ApiEndpoint {
      pub fn to_url(&self) -> &'static str {
          match self {
              ApiEndpoint::CreateBadge => "/open-apis/admin/v1/badges",
          }
      }
  }
  ```

  **Must NOT do**:
  - 不添加本项目未使用的端点

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO (依赖 Task 1,2 完成)
  - **Blocked By**: Task 1, 2
  - **Blocks**: Task 6

  **Acceptance Criteria**:
  - [ ] `api_endpoints.rs` 文件创建成功
  - [ ] `ApiEndpoint` 枚举定义正确
  - [ ] `to_url()` 方法返回正确路径

  **QA Scenarios**:
  ```
  Scenario: 验证模块创建
    Tool: Bash
    Steps:
      1. cargo check -p openlark-platform
    Expected Result: 编译通过
    Evidence: .sisyphus/evidence/task5-check.log
  ```

  **Commit**: YES
  - Message: `feat(platform): 添加 api_endpoints.rs 端点枚举`
  - Files: `crates/openlark-platform/src/common/api_endpoints.rs`

---

- [x] 6. 替换硬编码 URL - CreateBadgeBuilder

  **What to do**:
  - 打开文件 `crates/openlark-platform/src/admin/admin/v1/badge/create.rs`
  - 导入 `ApiEndpoint` 枚举
  - 将硬编码 URL `"/open-apis/admin/v1/badges"` 替换为 `ApiEndpoint::CreateBadge.to_url()`

  **修改示例**：
  ```rust
  use crate::common::api_endpoints::ApiEndpoint;
  
  let api_request: ApiRequest<CreateBadgeResponse> =
      ApiRequest::post(ApiEndpoint::CreateBadge.to_url())
          .body(serde_json::to_value(&request_body)?);
  ```

  **Must NOT do**:
  - 不修改 API 的业务逻辑

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO (依赖 Task 5)
  - **Blocked By**: Task 5
  - **Blocks**: Task 10

  **References**:
  - 当前文件: `crates/openlark-platform/src/admin/admin/v1/badge/create.rs:64`
  - 参考模式: `crates/openlark-docs/src/base/bitable/v1/app/table/record/create.rs:102`

  **Acceptance Criteria**:
  - [ ] 硬编码 URL 被替换为 `ApiEndpoint::CreateBadge.to_url()`
  - [ ] 文件编译通过: `cargo check -p openlark-platform`

  **QA Scenarios**:
  ```
  Scenario: 验证 URL 替换
    Tool: Bash
    Steps:
      1. cargo check -p openlark-platform
      2. cargo build -p openlark-platform
    Expected Result: 编译通过，无警告
    Evidence: .sisyphus/evidence/task6-build.log
  ```

  **Commit**: YES
  - Message: `refactor(platform): 使用 ApiEndpoint 枚举替换硬编码 URL`
  - Files: `crates/openlark-platform/src/admin/admin/v1/badge/create.rs`

---

- [x] 7. 替换硬编码 URL - QuerySessionRequest

  **What to do**:
  - 打开文件 `crates/openlark-auth/src/passport/passport/v1/session/query.rs`
  - 导入 `ApiEndpoint` 枚举
  - 将硬编码 URL `"/open-apis/passport/v1/sessions/query"` 替换为 `ApiEndpoint::QuerySession.to_url()`

  **Must NOT do**:
  - 不修改 API 的业务逻辑

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO (依赖 Task 4)
  - **Blocked By**: Task 4
  - **Blocks**: Task 10

  **References**:
  - 当前文件: `crates/openlark-auth/src/passport/passport/v1/session/query.rs:43`

  **Acceptance Criteria**:
  - [ ] 硬编码 URL 被替换为 `ApiEndpoint::QuerySession.to_url()`
  - [ ] 文件编译通过: `cargo check -p openlark-auth`

  **QA Scenarios**:
  ```
  Scenario: 验证 URL 替换
    Tool: Bash
    Steps:
      1. cargo check -p openlark-auth
    Expected Result: 编译通过，无警告
    Evidence: .sisyphus/evidence/task7-check.log
  ```

  **Commit**: YES (squash with Task 6)
  - Message: `refactor(auth): 使用 ApiEndpoint 枚举替换硬编码 URL`
  - Files: `crates/openlark-auth/src/passport/passport/v1/session/query.rs`

---

- [x] 8. 修复覆盖率配置一致性

  **What to do**:
  - 检查根 `Cargo.toml` 中 `full` feature 的状态
  - 更新 `.cargo-llvm-cov.toml` 中的 `features` 配置
  - 方案 A: 如果 `full` feature 存在但被注释，取消注释
  - 方案 B: 如果 `full` feature 不存在，修改为 `features = ["full-suite"]` 或其他有效 feature

  **Must NOT do**:
  - 不添加新的 feature

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Blocks**: Task 10

  **References**:
  - 配置文件: `.cargo-llvm-cov.toml:41`
  - 根配置: `Cargo.toml:286`

  **Acceptance Criteria**:
  - [ ] `.cargo-llvm-cov.toml` 中的 feature 与 `Cargo.toml` 一致
  - [ ] `cargo llvm-cov` 可以正常执行

  **QA Scenarios**:
  ```
  Scenario: 验证配置一致性
    Tool: Bash
    Steps:
      1. grep -n "full" Cargo.toml | head -5
      2. grep -n "features" .cargo-llvm-cov.toml
    Expected Result: feature 名称一致
    Evidence: .sisyphus/evidence/task8-config.log
  ```

  **Commit**: YES
  - Message: `chore: 修复覆盖率配置与 feature 定义不一致`
  - Files: `.cargo-llvm-cov.toml` 或 `Cargo.toml`

---

- [x] 9. 运行代码格式化

  **What to do**:
  - 运行 `cargo fmt --all` 统一代码风格
  - 检查格式化后的文件是否符合项目规范

  **Must NOT do**:
  - 不手动修改格式化后的代码

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Blocks**: Task 10

  **Acceptance Criteria**:
  - [ ] `cargo fmt --all` 执行成功
  - [ ] `cargo fmt --check` 通过

  **QA Scenarios**:
  ```
  Scenario: 验证格式化
    Tool: Bash
    Steps:
      1. cargo fmt --all
      2. cargo fmt --check
    Expected Result: 无格式化差异
    Evidence: .sisyphus/evidence/task9-fmt.log
  ```

  **Commit**: YES (可以与 Task 10 合并)
  - Message: `style: 运行 cargo fmt 统一代码风格`
  - Files: 所有被格式化的文件

---

- [x] 10. 最终验证 - 运行 clippy 和 fmt 检查

  **What to do**:
  - 运行 `cargo fmt --check` 验证格式
  - 运行 `cargo clippy --workspace --all-features -- -D warnings` 验证代码质量
  - 如有 clippy 警告，修复或记录

  **Expected Output**:
  - `cargo fmt --check` - 无输出（表示通过）
  - `cargo clippy` - 无警告（排除 `missing_docs`）

  **Must NOT do**:
  - 不修改超出本计划范围的问题

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO (依赖 Wave 2 所有任务)
  - **Blocked By**: Task 5, 6, 7, 8, 9

  **Acceptance Criteria**:
  - [ ] `cargo fmt --check` 通过
  - [ ] `cargo clippy --workspace --all-features -- -D warnings` 通过（或只有 pre-existing 警告）
  - [ ] 项目编译通过: `cargo build --workspace --all-features`

  **QA Scenarios**:
  ```
  Scenario: 验证代码质量
    Tool: Bash
    Steps:
      1. cargo fmt --check
      2. cargo clippy --workspace --all-features -- -D warnings 2>&1 | head -50
      3. cargo build --workspace --all-features 2>&1 | tail -20
    Expected Result: 
      - fmt: 无输出
      - clippy: 无警告（或只有 pre-existing missing_docs）
      - build: 成功
    Evidence: .sisyphus/evidence/task10-final.log
  ```

  **Commit**: NO (验证步骤，不单独提交)

---

## Final Verification Wave

- [x] F1. **修复验证** - 确认所有 P0/P1 问题已修复
  - 检查命名规范: `grep -r "udeleteRequest" crates/` 应无结果
  - 检查字段传递: 代码审查 QuerySessionRequest
  - 检查硬编码 URL: `grep -r '"/open-apis/' crates/openlark-auth/src crates/openlark-platform/src` 应无结果
  - 输出: `Issues Fixed [N/N] | VERDICT: PASS/FAIL`

- [x] F2. **代码质量检查**
  - 运行 `cargo fmt --check` → PASS
  - 运行 `cargo clippy --workspace --all-features -- -D warnings` → PASS (或记录 pre-existing 警告)
  - 输出: `Fmt [PASS/FAIL] | Clippy [PASS/FAIL] | VERDICT`

- [x] F3. **编译验证**
  - 运行 `cargo build --workspace --all-features` → SUCCESS
  - 输出: `Build [PASS/FAIL] | VERDICT`

---

## Commit Strategy

- **Wave 1**: 3-4 个独立 commit，每个修复一个 P0 问题
- **Wave 2**: 4-5 个独立 commit，每个修复一个 P1 问题
- **合并建议**: 相关的小修复可以 squash（如 Task 1+2, Task 6+7）

示例 commit 历史：
```
style(security): 重命名 udeleteRequest 为 PascalCase
fix(auth): 修复 QuerySessionRequest.user_ids 字段未参与请求
feat(auth): 添加 api_endpoints.rs 端点枚举
feat(platform): 添加 api_endpoints.rs 端点枚举
refactor(platform): 使用 ApiEndpoint 枚举替换硬编码 URL
refactor(auth): 使用 ApiEndpoint 枚举替换硬编码 URL
chore: 修复覆盖率配置与 feature 定义不一致
style: 运行 cargo fmt 统一代码风格
```

---

## Success Criteria

### 最终检查清单
- [x] 所有 `udeleteRequest` 已重命名为 PascalCase
- [x] `QuerySessionRequest.user_ids` 字段正确参与请求
- [x] 所有硬编码 `/open-apis/` URL 已替换为 `ApiEndpoint` 枚举
- [x] `.cargo-llvm-cov.toml` 与 `Cargo.toml` feature 配置一致
- [x] `cargo fmt --check` 通过
- [x] `cargo clippy --workspace --all-features -- -D warnings` 通过（排除 pre-existing）
- [x] `cargo build --workspace --all-features` 成功

### 量化指标
| 指标 | 修复前 | 修复后 | 目标 |
|------|--------|--------|------|
| 命名异常 | 2 | 0 | ✅ |
| 字段未参与 | 1 | 0 | ✅ |
| 硬编码 URL | 5 | 0 | ✅ |
| 配置不一致 | 1 | 0 | ✅ |
| fmt 检查 | FAIL | PASS | ✅ |
| clippy 警告 | ~50 | ≤50 | ✅ |

---

**Plan Generated**: 2026-02-24
**Next Step**: 运行 `/start-work` 开始执行
