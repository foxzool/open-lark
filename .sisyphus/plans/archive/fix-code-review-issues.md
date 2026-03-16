# 修复代码审查发现的技术债务

## TL;DR

> **Quick Summary**: 修复 token_provider.rs 和 explorer/v2/mod.rs 中的架构违规和代码质量问题。
> 
> **Deliverables**:
> - 移除硬编码 URL，统一使用 ApiEndpoint 枚举
> - 修复代码风格问题（缩进、类型转换）
> - 确保 Clippy 零警告
> 
> **Estimated Effort**: Short（约 30 分钟）
> **Parallel Execution**: NO - 有依赖关系

---

## Context

### 代码审查发现

来自 Code Review 工具对未提交变更的分析：

| 文件 | 问题 | 严重程度 |
|------|------|---------|
| `openlark-auth/src/token_provider.rs` | 绕过 Service 层，硬编码 URL | 中等 |
| `openlark-docs/src/ccm/explorer/v2/mod.rs` | 硬编码 API URL（违反 AGENTS.md）| **高** |

### 具体问题

**1. explorer/v2/mod.rs（高优先级）**
- 第 210 行：`"/open-apis/drive/v1/files"` 硬编码
- 应改为：`DriveApi::ListFiles.to_url()`

**2. token_provider.rs（中优先级）**
- 使用直接 HTTP 调用而非 `AuthServiceV3`
- 第 197, 235 行：不必要的 `as i64` 转换
- 代码缩进不一致

---

## Work Objectives

### Core Objective
消除硬编码 URL 和技术债务，确保代码符合项目规范。

### Concrete Deliverables
- `token_provider.rs` - 修复 Clippy 警告和风格问题
- `explorer/v2/mod.rs` - 替换硬编码 URL 为 ApiEndpoint 枚举

### Definition of Done
- [x] `cargo clippy --all-features` 零警告
- [x] 无硬编码 `/open-apis/` URL（api_endpoints.rs 除外）
- [x] 代码缩进一致

---

## TODOs

- [x] 1. **修复 explorer/v2/mod.rs 硬编码 URL** [quick]

  **What to do**:
  - 将 `"/open-apis/drive/v1/files"` 替换为 `DriveApi::ListFiles`
  - 添加必要的 import：`use crate::common::api_endpoints::DriveApi`
  - 修复缩进问题

  **File**: `crates/openlark-docs/src/ccm/explorer/v2/mod.rs`

  **QA**: `cargo clippy -p openlark-docs -- -D warnings`

---

- [x] 2. **修复 token_provider (无需修改).rs Clippy 警告** [quick]

  **What to do**:
  - 移除第 197 和 235 行的 `as i64`
  - 修复代码缩进
  - （可选）添加注释解释为何绕过 Service 层

  **File**: `crates/openlark-auth/src/token_provider.rs`

  **QA**: `cargo clippy -p openlark-auth -- -D warnings`

---

- [x] 3. **全局验证** [quick]

  **What to do**:
  - 运行 `cargo clippy --all-features` 确保零警告
  - 运行 `cargo test --all-features` 确保测试通过

  **QA**: 
  ```bash
  cargo clippy --all-features -- -D warnings
  cargo test --all-features
  ```

---

## Final Verification

- [x] `cargo clippy --all-features` 零警告
- [x] `cargo test --all-features` 通过
- [x] 无新增硬编码 URL

---

## Commit Strategy

1. `refactor(docs): 使用 DriveApi 枚举替换硬编码 URL` - explorer/v2/mod.rs
2. `style(auth): 修复 Clippy 警告和代码缩进` - token_provider.rs

---

## Success Criteria

```bash
# 验证命令
cargo clippy --all-features -- -D warnings  # 应无警告
cargo test --all-features                    # 应全部通过

# 检查无硬编码 URL
grep -r "open-apis" crates/ --include="*.rs" | grep -v "api_endpoints.rs" | grep -v "to_url()" | wc -l  # 应为 0
```
