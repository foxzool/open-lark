# 修复 openlark-docs P0 级架构问题计划

## Context

### Original Request
审计 @crates/openlark-docs 代码并修复 P0 问题。

### Interview Summary
**Key Discussions**:
- **Breaking Change**: 用户明确授权可以进行破坏性变更，以追求极致的架构规范。
- **重构范围**: 包含路径规范化、命名冗余清理、接口契约统一以及 Wiki 领域的架构对齐。
- **技术栈**: Rust, Cargo Features, Builder Pattern.

**Research Findings**:
- **Baike 路径违规**: `src/baike/v1/baike` 作为入口，但实现却在 `src/baike/baike/v1`，违反 `{bizTag}/{project}/{version}` 规范。
- **CCM 冗余前缀**: `src/ccm/ccm_sheet` 等目录名称重复包含了业务标签前缀。
- **下载契约不一致**: 百科模块的下载接口未返回 `Response<Vec<u8>>`，无法获取 HTTP 头部信息。
- **Wiki 入口侵入**: `WikiService` 直接包含了 `v2` 逻辑，未通过 `.v1()` 或 `.v2()` 区分。

### Metis Review
**Identified Gaps** (addressed):
- **Wiki 规范化**: 指出 Wiki 领域的入口模式也需一并对齐，已加入 TODO。
- **Feature 连锁反应**: 目录更名需同步调整 `Cargo.toml` 和 `lib.rs` 导出。
- **测试语法错误**: 定位到 `bitable_tests.rs:933` 的 Unicode 转义序列错误。

---

## Work Objectives

### Core Objective
通过深度重构，使 `openlark-docs` 的目录结构、Feature 命名、API 契约以及链式调用入口完全对齐 OpenLark 企业级架构标准。

### Concrete Deliverables
- [x] 规范化的目录结构（无 `ccm_` 前缀，Baike 路径修正）
- [x] 更新后的 `Cargo.toml`（Feature 重命名）
- [x] 统一的下载接口签名（全员 `Response<Vec<u8>>`）
- [x] 规范化的 Wiki 链式入口（`wiki().v1()` / `wiki().v2()`）
- [x] 修复后的测试文件

### Definition of Done
- [x] `cargo check --all-features` 通过
- [x] `python3 tools/validate_apis.py --crate openlark-docs` 100% 匹配
- [x] `cargo test --all-features` 中涉及重构的部分无回归错误（或已同步更新）

---

## Verification Strategy

### Test Decision
- **Infrastructure exists**: YES
- **User wants tests**: TDD (Red-Green-Refactor)
- **Framework**: cargo test

### If TDD Enabled
每一项重构（特别是返回类型变更）都将先导致编译错误（Red），随后通过代码修正达到编译通过（Green）。

---

## Task Flow

```
Task 1 (目录重构) → Task 2 (Feature/导出更新) → Task 3 (API 契约统一) → Task 4 (Wiki 规范化) → Task 5 (语法修复)
```

---

## TODOs

### 1. 目录结构规范化 (Breaking Change)

**What to do**:
- 移除 `src/ccm/` 下的冗余前缀。
- 修正 Baike 路径逻辑。

**Actions**:
- [x] `mv src/ccm/ccm_doc src/ccm/doc` (注：若已有 doc 且 ccm_doc 为 old，则合并清理)
- [x] `mv src/ccm/ccm_docs src/ccm/docs`
- [x] `mv src/ccm/ccm_drive_explorer src/ccm/explorer`
- [x] `mv src/ccm/ccm_drive_permission src/ccm/permission`
- [x] `mv src/ccm/ccm_sheet src/ccm/sheets_v2` (注：sheets 目录已被 v3 占用)
- [x] `mv src/baike/v1/baike src/baike/baike/v1/service` (暂存服务定义)
- [x] 重构 `src/baike/baike/v1/mod.rs` 以容纳服务入口。

**Parallelizable**: NO

**Acceptance Criteria**:
- [x] 文件物理位置移动完成。
- [x] 没有任何 `ccm_` 前缀的目录残留在 `src/ccm/`。

---

### 2. Cargo Feature 与模块导出对齐

**What to do**:
- 更新 `Cargo.toml` 中的 feature 命名，保持与目录名一致。
- 修正 `lib.rs` 和各级 `mod.rs` 的导出声明。

**Actions**:
- [x] 在 `Cargo.toml` 中重命名 `ccm-drive-permission` -> `drive-permission` (或按 project 名)。（已在任务 1-7 完成）
- [x] 更新 `src/lib.rs` 中的 `#[cfg(feature = "...")]`。（已在任务 1-7 完成）
- [x] 更新 `src/ccm/mod.rs` 以匹配更名后的目录。（已在任务 1-7 完成）

**Acceptance Criteria**:
- [x] `cargo check` 不报 `mod not found` 错误。

---

### 3. 百科下载 API 契约统一

**What to do**:
- 将 Baike 和 Lingo 的下载接口统一为 `Response` 包装。

**Actions**:
- [x] 修改 `src/baike/baike/v1/file/download.rs` 的 `execute` 返回类型。
- [x] 修改 `src/baike/lingo/v1/file/download.rs` 的 `execute` 返回类型。
- [x] 同步更新 `BaikeV1Service` 中的调用代码。

**Acceptance Criteria**:
- [x] 下载接口均返回 `SDKResult<Response<Vec<u8>>>`。

---

### 4. Wiki 服务入口规范化

**What to do**:
- 重构 Wiki 模块，支持版本化入口，对齐 `Drive` 模式。

**Actions**:
- [x] 修改 `src/ccm/wiki/mod.rs`（或新建），定义 `WikiService`。
- [x] 实现 `WikiService::v1()` 和 `WikiService::v2()`。
- [x] 将现有 `WikiService` (位于 v2) 重命名为 `WikiV2Service`。

**Acceptance Criteria**:
- [x] 调用链支持 `client.docs.ccm.wiki().v2()...`。

---

### 5. 测试语法修复

**What to do**:
- 修复 `tests/unit/cloud_docs/bitable_tests.rs` 语法。

**Actions**:
- [x] 定位第 933 行附近的无效 Unicode 转义。（文件不存在，无需修复）
- [x] 修正为合法的 Rust 字符串转义。（文件不存在，无需修复）

**Acceptance Criteria**:
- [x] 该文件编译错误消失。

---

## Success Criteria

### Verification Commands
```bash
cargo check --all-features
python3 tools/validate_apis.py --crate openlark-docs
```

### Final Checklist
- [x] 所有目录命名无冗余
- [x] 路径深度符合规范
- [x] 下载接口契约一致
- [x] Wiki 链式调用已对齐
- [x] 测试文件语法无误
