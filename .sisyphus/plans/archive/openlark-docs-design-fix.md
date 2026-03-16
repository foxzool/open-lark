# openlark-docs 代码设计问题修复计划

## TL;DR

**快速 summary**: 修复 openlark-docs crate 中发现的 7 个设计问题（P0-P3）。

**修复范围**:
- P0: 移除 CreateDocumentParams 废弃类型导出
- P0: 文档化入口设计决策（不重构）
- P1: 移除 chain.rs 中不必要的 Config.clone() 调用
- P1: 解耦 baike/wiki Feature 循环依赖
- P2: 清理未使用的 re-export
- P2: 移除过时注释
- P3: 修复 README.md 示例代码

**预计工作量**: 2-3 小时

---

## Context

### 原始请求
用户要求审查 openlark-docs crate 的代码规范，审查完成后要求创建修复计划。

### 审查摘要

**关键发现**:
- API 覆盖率 100%（202/202，排除旧版本）
- 存在 2 个 deprecated 警告
- 入口设计混乱：DocsClient + *Client + *Service 三层
- Feature 存在循环依赖：baike = ["ccm-wiki"], wiki = ["baike"]
- chain.rs 中存在不必要的 Config.clone() 调用
- README 示例使用不存在的 LarkClient

**用户确认**:
- 审查范围：全仓库一致性
- 改造约束：允许 breaking change

---

## Work Objectives

### Core Objective
修复 openlark-docs crate 中的设计问题，提升代码质量和用户使用体验。

### Concrete Deliverables

| 文件 | 修改内容 |
|------|---------|
| crates/openlark-docs/src/lib.rs | 移除过时注释、添加入口设计说明 |
| crates/openlark-docs/src/ccm/docx/v1/document/mod.rs | 移除 CreateDocumentParams 导出 |
| crates/openlark-docs/src/common/chain.rs | 移除不必要的 Config.clone() 调用 |
| crates/openlark-docs/Cargo.toml | 解耦 baike/wiki Feature 依赖 |
| crates/openlark-docs/README.md | 修复示例代码（使用 DocsClient） |

### Definition of Done
- cargo check --all-features 无 deprecated 警告
- chain.rs 中无不必要的 Config.clone() 调用
- Cargo.toml 中 baike 和 wiki 无循环依赖
- README 示例使用正确的 DocsClient
- 所有测试通过

### Must Have
- P0 问题全部解决（废弃警告、入口文档化）
- P1 问题优先解决（Config clone、Feature 依赖）

### Must NOT Have
- 不新增功能或抽象
- 不修改正常工作的 API 实现
- 不扩大范围到其他 crate

---

## Verification Strategy

### Test Decision
- **基础设施存在**: YES
- **用户需要测试**: YES (tests-after)
- **QA 方式**: 编译检查 + 单元测试

### 验证命令
```bash
# 编译检查
cargo check --package openlark-docs --all-features

# 检查 deprecated 警告数量（预期: 0）
cargo check --package openlark-docs --all-features 2>&1 | grep -c "deprecated"

# Config clone 检查
grep -n "config.clone()" crates/openlark-docs/src/common/chain.rs

# Feature 依赖检查
grep -E "^(baike|wiki) " crates/openlark-docs/Cargo.toml

# README 示例检查
grep -c "LarkClient" crates/openlark-docs/README.md

# 运行测试
cargo test --package openlark-docs --all-features
```

---

## Execution Strategy

### Parallel Execution Waves

Wave 1 (并行):
- T1: 移除 CreateDocumentParams 废弃导出
- T3: 修复 Cargo.toml Feature 依赖
- T7: 修复 README.md 示例

Wave 2 (T1,T3,T7 完成后):
- T2: 文档化入口设计决策
- T4: 移除 chain.rs Config clone
- T5: 清理未使用的 re-export

Wave 3:
- T6: 移除过时注释
- [x] 最终验证

### Dependency Matrix

| 任务 | 依赖 | 可并行 |
|------|------|--------|
| T1 | 无 | T3, T7 |
| T2 | T1 | - |
| T3 | 无 | T1, T2 |
| T4 | T1, T3 | - |
| T5 | T1 | - |
| T6 | T1 | - |
| T7 | 无 | T1, T2 |

---

## TODOs

- [x] 1. 移除 CreateDocumentParams 废弃类型导出

  **What to do**:
  - 搜索 CreateDocumentParams 在 openlark-docs 内部的所有引用
  - 移除 src/ccm/docx/v1/document/mod.rs 中的 pub use create::CreateDocumentParams;
  - 确认无内部引用后，删除 create.rs 中的 CreateDocumentParams 定义

  **Must NOT do**:
  - 不修改 CreateDocumentRequest 结构

  **References**:
  - crates/openlark-docs/src/ccm/docx/v1/document/mod.rs:23 - 导出位置
  - crates/openlark-docs/src/ccm/docx/v1/document/create.rs:108-121 - 定义位置

  **Acceptance Criteria**:
  - cargo check 无 deprecated 警告
  - rg "CreateDocumentParams" crates/openlark-docs/src/ 返回 0 结果

  **Commit**: YES
  - Message: fix(docs): 移除废弃的 CreateDocumentParams 类型导出

- [x] 2. 文档化入口设计决策

  **What to do**:
  - 在 chain.rs 顶部添加注释说明入口设计
  - 在 lib.rs 中添加注释说明 *Client 的定位

  **References**:
  - crates/openlark-docs/src/common/chain.rs:1-10 - 现有注释位置

  **Acceptance Criteria**:
  - 编译检查无错误

  **Commit**: YES
  - Message: docs(docs): 添加入口设计文档说明

- [x] 3. 修复 Cargo.toml Feature 依赖循环

  **What to do**:
  - 分析 baike 和 wiki 的实际依赖关系
  - 解耦循环依赖：baike 不再依赖 ccm-wiki，wiki 独立
  - 验证单独开启任一 feature 能正常编译

  **References**:
  - crates/openlark-docs/Cargo.toml:46-47 - 循环依赖位置

  **Acceptance Criteria**:
  - cargo check --features baike 无错误
  - cargo check --features wiki 无错误
  - cargo check --features "baike wiki" 无错误

  **Commit**: YES
  - Message: fix(deps): 解耦 baike/wiki Feature 循环依赖

- [x] 4. 移除 chain.rs 中不必要的 Config.clone()

  **What to do**:
  - 分析 chain.rs 中所有 config.clone() 调用
  - 移除不必要的 clone（Config 内部已使用 Arc）

  **References**:
  - crates/openlark-docs/src/common/chain.rs:29-42 - DocsClient::new 中的 clone
  - crates/openlark-docs/src/common/chain.rs:67-78 - CcmClient::new 中的 clone

  **Acceptance Criteria**:
  - grep -n "config.clone()" crates/openlark-docs/src/common/chain.rs 返回 0 或 1
  - 编译检查无错误
  - 测试通过

  **Commit**: YES
  - Message: perf(docs): 移除 chain.rs 中不必要的 Config.clone() 调用

- [x] 5. 清理未使用的 re-export

  **What to do**:
  - 检查 lib.rs:86-93 中的 re-export 是否被使用
  - 移除未使用的 re-export

  **References**:
  - crates/openlark-docs/src/lib.rs:86-93 - re-export 位置

  **Acceptance Criteria**:
  - 编译检查无错误

  **Commit**: YES
  - Message: refactor(docs): 清理未使用的 re-export

- [x] 6. 移除过时注释

  **What to do**:
  - 移除 lib.rs:6 的过时注释 #![deny(missing_docs)]

  **References**:
  - crates/openlark-docs/src/lib.rs:6 - 过时注释位置

  **Acceptance Criteria**:
  - 编译检查无错误

  **Commit**: YES
  - Message: cleanup(docs): 移除 lib.rs 中过时注释

- [x] 7. 修复 README.md 示例代码

  **What to do**:
  - 将示例中的 LarkClient 替换为 DocsClient
  - 使用 DocsClient::new(config) 替代不存在的 LarkClient::builder()

  **References**:
  - crates/openlark-docs/README.md:73-88 - 示例代码位置
  - crates/openlark-docs/src/common/chain.rs - DocsClient 实际使用方式

  **Acceptance Criteria**:
  - grep -c "LarkClient" crates/openlark-docs/README.md 返回 0

  **Commit**: YES
  - Message: docs(docs): 修复 README.md 示例代码

---

## Success Criteria

### 验证命令
```bash
# 1. 编译检查（主要验证）
cargo check --package openlark-docs --all-features 2>&1 | grep -E "warning|error"

# 2. 预期结果：
# - warning 数量: 0 (或仅预期内的 warning)
# - deprecated 警告: 0

# 3. Config clone 检查
grep -n "config.clone()" crates/openlark-docs/src/common/chain.rs
# 预期结果: 0 或 1 (最后一个不 clone)

# 4. Feature 依赖检查
grep -E "^(baike|wiki) " crates/openlark-docs/Cargo.toml
# 预期结果: 无循环依赖

# 5. README 示例检查
grep -c "LarkClient" crates/openlark-docs/README.md
# 预期结果: 0

# 6. 运行测试
cargo test --package openlark-docs --all-features
```

### 最终 Checklist
- [x] 所有 Must Have 条件满足
- [x] cargo check --all-features 无 error
- [x] 所有测试通过
- [x] 无 breaking change 影响现有功能
