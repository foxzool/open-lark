# OpenLark-HR Crate API 设计审查工作计划

## 会话信息
- **会话 ID**: `openlark-hr-design-review-2026-01-28`
- **审查目标**: `openlark-hr` crate
- **审查日期**: 2026-01-28
- **审查范围**: 8 个维度全覆盖

---

## Context

### 用户请求
对 `openlark-hr` crate 进行全面的 API 设计审查，涵盖 8 个核心维度：
1. Entry 设计和公共 API 表面
2. Feature gating (cfg feature)
3. Request/Service/Builder 一致性
4. Endpoint 体系 (endpoint architecture)
5. Config 和 error handling
6. 模块导出和文档
7. 测试和告警控制

### API 完成度统计
```
✅ 验证完成！
📊 统计结果：
   - API 总数: 546
   - 已实现: 546
   - 未实现: 0
   - 完成率: 100%
   - 额外文件: 16 个
```

### 采用的架构范式
- **范式 A**: Request 自持 Config（流式 Builder）
- Request 保存 `Config`
- `Request::execute()` / `execute_with_options(RequestOption)`
- Service 只负责分组/版本入口，不承载网络逻辑

---

## Task Dependency Graph

| Task ID | 任务名称 | Depends On | Reason |
|---------|----------|------------|--------|
| 1 | 修复 HrService 冗余 http_client | None | 独立清理任务，无依赖 |
| 2 | 添加 Feature gating 支持 | None | 新增功能，可并行 |
| 3 | 删除重复 AttendanceV1 定义 | None | 独立清理任务 |
| 4 | 清理废弃 endpoints 目录 | None | 独立清理任务 |
| 5 | 统一 Service 和 Request 职责 | 1 | 需要先修复 Service 问题 |
| 6 | 修复模块导出不完整 | 3 | 依赖 AttendanceV1 修复 |
| 7 | 统一代码风格 | 6 | 在导出修复后进行 |
| 8 | 添加单元测试 | 5 | 在架构稳定后添加 |
| 9 | 完善文档 | 7 | 在代码风格统一后 |
| 10 | 最终验证 | 8, 9 | 所有任务完成后 |

---

## Parallel Execution Graph

### Wave 1 (Start Immediately - 高优先级并行)
```
├── Task 1: 修复 HrService 冗余 http_client (P0)
│   └── 文件: crates/openlark-hr/src/common/service.rs
├── Task 2: 添加 Feature gating 支持 (P0)
│   └── 文件: crates/openlark-hr/Cargo.toml
├── Task 3: 删除重复 AttendanceV1 定义 (P1)
│   └── 文件: crates/openlark-hr/src/attendance/attendance/mod.rs
└── Task 4: 清理废弃 endpoints 目录 (P1)
    └── 目录: crates/openlark-hr/src/endpoints/
```

### Wave 2 (After Wave 1 - 架构调整)
```
├── Task 5: 统一 Service 和 Request 职责 (P1)
│   └── 依赖: Task 1 (Service 修复)
├── Task 6: 修复模块导出不完整 (P1)
│   └── 依赖: Task 3 (结构体定义清理)
└── Task 7: 统一代码风格 (P2)
    └── 依赖: Task 6 (导出修复)
```

### Wave 3 (After Wave 2 - 质量提升)
```
├── Task 8: 添加单元测试 (P2)
│   └── 依赖: Task 5 (架构稳定)
└── Task 9: 完善文档 (P2)
    └── 依赖: Task 7 (代码风格统一)
```

### Wave 4 (Final - 验证)
```
└── Task 10: 最终验证 (P0)
    └── 依赖: Task 8, Task 9
```

**Critical Path**: Task 1 → Task 5 → Task 8 → Task 10

**Estimated Parallel Speedup**: 40% faster than sequential

---

## Tasks

### Task 1: 修复 HrService 冗余 http_client

**Description**:
`HrService` 结构体持有 `Arc<reqwest::Client>` 字段但从未使用，实际 HTTP 请求通过 `Transport::request()` 发送。这是设计冗余。

**问题位置**:
- `crates/openlark-hr/src/common/service.rs:15`

**代码现状**:
```rust
pub struct HrService {
    config: Arc<Config>,
    http_client: Arc<reqwest::Client>, // 冗余，未使用
}
```

**What to do**:
1. 移除 `http_client` 字段
2. 更新 `new()` 和 `from_config()` 方法
3. 验证所有使用 HrService 的地方正常工作

**Must NOT do**:
- 不要改变 Service 的公共 API 签名（除了移除 http_client 相关）
- 不要影响 HrClient 的使用方式

**Delegation Recommendation**:
- **Category**: `quick` - 简单清理任务
- **Skills**: [`openlark-api`] - 需要了解 OpenLark API 模式

**Skills Evaluation**:
- ✅ INCLUDED `openlark-api`: 需要理解 Service/Transport 关系
- ❌ OMITTED `git-master`: 简单修改不需要复杂 git 操作

**Depends On**: None (can start immediately)

**Acceptance Criteria**:
- [x] `http_client` 字段已从 `HrService` 移除
- [x] `cargo check --all-features` 无错误
- [x] `cargo test` 通过

**Commit**:
- Message: `refactor(hr): 移除 HrService 冗余的 http_client 字段`
- Files: `crates/openlark-hr/src/common/service.rs`

---

### Task 2: 添加 Feature gating 支持

**Description**:
`Cargo.toml` 完全缺少 `[features]` 部分，无法实现按需编译。需要添加 feature 定义并更新代码中的条件编译。

**问题位置**:
- `crates/openlark-hr/Cargo.toml`

**代码现状**:
```toml
[dependencies]
# ... 依赖项
# 缺少 [features] 部分
```

**What to do**:
1. 在 `Cargo.toml` 添加 `[features]` 部分
2. 为每个 domain 添加 feature (attendance, corehr, compensation, payroll, performance, okr, hire, ehr)
3. 添加 `default` feature
4. 在 `lib.rs` 和各级 `mod.rs` 添加 `#[cfg(feature = "...")]` 属性

**建议的 Feature 结构**:
```toml
[features]
default = ["attendance", "corehr", "compensation", "payroll", "performance", "okr", "hire", "ehr"]

attendance = []
corehr = []
compensation = []
payroll = []
performance = []
okr = []
hire = []
ehr = []

# 组合 feature
hr-full = ["attendance", "corehr", "compensation", "payroll", "performance", "okr", "hire", "ehr"]
```

**Must NOT do**:
- 不要改变默认行为（default 应该包含所有模块）
- 不要移除现有的公开 API

**Delegation Recommendation**:
- **Category**: `unspecified-low` - 中等复杂度，需要细心
- **Skills**: [`openlark-api`] - 需要理解 feature gating 模式

**Skills Evaluation**:
- ✅ INCLUDED `openlark-api`: 需要理解条件编译模式
- ❌ OMITTED `git-master`: 不需要复杂 git 操作

**Depends On**: None (can start immediately)

**Acceptance Criteria**:
- [x] `Cargo.toml` 添加了 `[features]` 部分
- [x] `lib.rs` 添加了条件编译属性
- [x] `cargo build --features attendance` 只编译 attendance 模块
- [x] `cargo build --all-features` 编译所有模块
- [x] `cargo check --all-features` 无警告

**Commit**:
- Message: `feat(hr): 添加 feature gating 支持，支持按需编译`
- Files: `crates/openlark-hr/Cargo.toml`, `crates/openlark-hr/src/lib.rs`

---

### Task 3: 删除重复 AttendanceV1 定义

**Description**:
`AttendanceV1` 结构体在两层 mod 中重复定义，造成维护困难和潜在混淆。

**问题位置**:
- `crates/openlark-hr/src/attendance/attendance/mod.rs:10`
- `crates/openlark-hr/src/attendance/attendance/v1/mod.rs:22`

**代码现状**:
```rust
// attendance/attendance/mod.rs
pub struct AttendanceV1 { ... }

// attendance/attendance/v1/mod.rs
pub struct AttendanceV1 { ... } // 重复定义
```

**What to do**:
1. 删除 `attendance/attendance/v1/mod.rs` 中的 `AttendanceV1` 定义
2. 确保 `attendance/attendance/mod.rs` 的定义保留
3. 更新 `attendance/mod.rs` 中的 `v1()` 方法返回类型

**Must NOT do**:
- 不要改变公共 API 的调用方式
- 不要删除任何实际功能

**Delegation Recommendation**:
- **Category**: `quick` - 简单清理任务
- **Skills**: [`openlark-api`]

**Skills Evaluation**:
- ✅ INCLUDED `openlark-api`: 需要理解模块结构

**Depends On**: None (can start immediately)

**Acceptance Criteria**:
- [x] 只有一个 `AttendanceV1` 定义存在
- [x] `cargo check` 无错误
- [x] 检查其他模块是否有类似问题

**Commit**:
- Message: `refactor(hr): 删除重复的 AttendanceV1 结构体定义`
- Files: `crates/openlark-hr/src/attendance/attendance/v1/mod.rs`

---

### Task 4: 清理废弃 endpoints 目录

**Description**:
`endpoints/` 目录包含常量定义但未被实际使用，实际使用 `common/api_endpoints.rs` 的枚举系统。

**问题位置**:
- `crates/openlark-hr/src/endpoints/mod.rs`

**代码现状**:
```rust
// lib.rs
pub mod endpoints; // 导出但未在代码中使用

// 实际使用
use crate::common::api_endpoints::AttendanceApiV1;
```

**What to do**:
1. 检查 `endpoints/` 目录是否在任何地方被使用
2. 如果未被使用，删除整个目录
3. 从 `lib.rs` 移除导出
4. 更新相关文档

**Must NOT do**:
- 确保没有任何代码使用这些常量后再删除

**Delegation Recommendation**:
- **Category**: `quick`
- **Skills**: [`openlark-api`]

**Depends On**: None (can start immediately)

**Acceptance Criteria**:
- [x] 确认 `endpoints/` 目录未被使用
- [x] 删除 `endpoints/` 目录
- [x] 从 `lib.rs` 移除导出
- [x] `cargo check` 无错误

**Commit**:
- Message: `chore(hr): 清理废弃的 endpoints 目录`
- Files: `crates/openlark-hr/src/endpoints/`, `crates/openlark-hr/src/lib.rs`

---

### Task 5: 统一 Service 和 Request 职责

**Description**:
当前架构中 `HrService` 存在但 Request 直接使用 `Transport`，Service 层没有发挥应有作用。需要明确职责或移除冗余。

**问题分析**:
- `HrService` 创建但未在 Request 中使用
- Request 直接调用 `Transport::request()`
- 这种模式在其他 crate 中可能不同

**可能的解决方案**:

**方案 A** - 移除 HrService（推荐）:
```rust
// HrClient 直接使用 Config
pub struct HrClient {
    config: Arc<Config>,
}
```

**方案 B** - Request 使用 Service:
```rust
// Request 通过 Service 发送请求
impl Request {
    pub async fn execute(self) -> SDKResult<Response> {
        self.service.request(...).await
    }
}
```

**What to do**:
1. 分析全仓库其他 crate 的模式
2. 选择一种方案统一实现
3. 如果选择方案 A，移除 HrService
4. 如果选择方案 B，修改 Request 使用 Service

**Must NOT do**:
- 不要创造与其他 crate 不同的新模式

**Delegation Recommendation**:
- **Category**: `unspecified-high` - 需要架构决策
- **Skills**: [`openlark-api`]

**Skills Evaluation**:
- ✅ INCLUDED `openlark-api`: 需要理解全仓库架构模式

**Depends On**: Task 1 (Service 修复)

**Acceptance Criteria**:
- [x] 选定统一方案
- [x] 实施架构调整
- [x] `cargo test` 通过
- [x] 与其他 crate 模式一致

**Commit**:
- Message: `refactor(hr): 统一 Service 和 Request 职责`
- Files: 根据方案而定

---

### Task 6: 修复模块导出不完整

**Description**:
`group` 模块的 `mod.rs` 缺少 `list_user` 导出。

**问题位置**:
- `crates/openlark-hr/src/attendance/attendance/v1/group/mod.rs`

**代码现状**:
```rust
pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod list_user;  // 声明了模块
pub mod models;
pub mod search;

pub use create::*;
pub use delete::*;
pub use get::*;
pub use list::*;
// 缺少: pub use list_user::*;
pub use models::*;
pub use search::*;
```

**What to do**:
1. 添加 `pub use list_user::*;`
2. 检查其他模块是否有类似问题
3. 统一导出风格

**Must NOT do**:
- 不要遗漏任何模块

**Delegation Recommendation**:
- **Category**: `quick`
- **Skills**: [`openlark-api`]

**Depends On**: Task 3 (结构体定义清理)

**Acceptance Criteria**:
- [x] `group/mod.rs` 添加 `list_user` 导出
- [x] 检查所有 resource 模块的完整性
- [x] `cargo check` 无错误

**Commit**:
- Message: `fix(hr): 修复 group 模块导出不完整`
- Files: `crates/openlark-hr/src/attendance/attendance/v1/group/mod.rs`

---

### Task 7: 统一代码风格

**Description**:
不同模块的导出风格不一致，有些使用通配符，有些使用具体导出。

**What to do**:
1. 制定统一的导出规范
2. 统一所有 `mod.rs` 的导出风格

**推荐规范**:
```rust
// 推荐：明确导出
pub use create::{CreateRequest, CreateRequestBuilder, CreateResponse};
pub use list::{ListRequest, ListResponse};

// 不推荐：通配导出
pub use create::*;
```

**Must NOT do**:
- 不要改变公共 API 的可见性

**Delegation Recommendation**:
- **Category**: `unspecified-low`
- **Skills**: [`openlark-api`]

**Depends On**: Task 6 (导出修复)

**Acceptance Criteria**:
- [x] 所有 `mod.rs` 导出风格一致
- [x] 公共 API 保持不变
- [x] `cargo check` 无警告

**Commit**:
- Message: `style(hr): 统一模块导出风格`
- Files: 多个 mod.rs 文件

---

### Task 8: 添加单元测试

**Description**:
API 实现缺少单元测试，只有 `endpoints` 有测试。

**What to do**:
1. 为每个 domain 的核心 API 添加测试
2. 测试 Request 的构建和序列化
3. 测试 Response 的反序列化
4. 测试 validation 逻辑

**测试示例**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_group_request_builder() {
        let request = CreateGroupRequest::new(config)
            .group_name("Test".to_string())
            .group_type(0);
        
        assert_eq!(request.group_name, "Test");
    }
}
```

**Delegation Recommendation**:
- **Category**: `unspecified-low`
- **Skills**: [`openlark-api`]

**Depends On**: Task 5 (架构稳定)

**Acceptance Criteria**:
- [x] 每个 domain 至少有一个测试文件
- [x] 测试覆盖率 > 60%
- [x] `cargo test` 全部通过

**Commit**:
- Message: `test(hr): 添加 API 单元测试`
- Files: 新增 tests/

---

### Task 9: 完善文档

**Description**:
部分模块缺少文档注释，或者文档可以更加详细。

**What to do**:
1. 为所有公开 API 添加文档注释
2. 添加使用示例
3. 完善模块级别文档

**Delegation Recommendation**:
- **Category**: `writing`
- **Skills**: [`openlark-api`]

**Depends On**: Task 7 (代码风格统一)

**Acceptance Criteria**:
- [x] 所有公开类型有文档注释
- [x] `cargo doc` 无警告
- [x] 文档测试通过

**Commit**:
- Message: `docs(hr): 完善 API 文档`
- Files: 多个文件

---

### Task 10: 最终验证

**Description**:
所有任务完成后的综合验证。

**What to do**:
1. 运行完整测试套件
2. 检查 clippy 警告
3. 检查代码格式
4. 验证 feature gating 工作正常
5. 生成最终报告

**Verification Commands**:
```bash
cargo build --all-features
cargo test --all-features
cargo clippy --all-features -- -D warnings
cargo fmt --check
cargo doc --all-features
```

**Delegation Recommendation**:
- **Category**: `unspecified-low`
- **Skills**: [`openlark-api`]

**Depends On**: Task 8, Task 9

**Acceptance Criteria**:
- [x] 所有检查通过
- [x] 无编译警告
- [x] 所有测试通过
- [x] 文档生成成功

**Commit**:
- Message: `chore(hr): 最终验证和清理`
- Files: 根据发现的问题

---

## Commit Strategy

| After Task | Message | Files | Verification |
|------------|---------|-------|--------------|
| 1 | `refactor(hr): 移除 HrService 冗余的 http_client 字段` | service.rs | cargo check |
| 2 | `feat(hr): 添加 feature gating 支持` | Cargo.toml, lib.rs | cargo build --features ... |
| 3 | `refactor(hr): 删除重复的 AttendanceV1 结构体定义` | v1/mod.rs | cargo check |
| 4 | `chore(hr): 清理废弃的 endpoints 目录` | endpoints/, lib.rs | cargo check |
| 5 | `refactor(hr): 统一 Service 和 Request 职责` | 多个文件 | cargo test |
| 6 | `fix(hr): 修复 group 模块导出不完整` | group/mod.rs | cargo check |
| 7 | `style(hr): 统一模块导出风格` | 多个 mod.rs | cargo check |
| 8 | `test(hr): 添加 API 单元测试` | tests/ | cargo test |
| 9 | `docs(hr): 完善 API 文档` | 多个文件 | cargo doc |
| 10 | `chore(hr): 最终验证和清理` | 根据需要 | 完整检查 |

---

## Success Criteria

### 最终检查清单
- [x] ✅ 所有 P0 问题已修复
- [x] ✅ Feature gating 正常工作
- [x] ✅ `cargo build --all-features` 成功
- [x] ✅ `cargo test --all-features` 通过
- [x] ✅ `cargo clippy --all-features` 无警告
- [x] ✅ `cargo fmt --check` 通过
- [x] ✅ `cargo doc --all-features` 成功
- [x] ✅ API 公共接口保持不变（向后兼容）
- [x] ✅ 代码风格统一
- [x] ✅ 文档完整

### 预期改进
| 指标 | 当前 | 目标 |
|------|------|------|
| Feature gating | ❌ 缺失 | ✅ 完整支持 |
| 代码警告 | ⚠️ 有 | ✅ 零警告 |
| 模块导出 | ⚠️ 不完整 | ✅ 完整 |
| 架构一致性 | ⚠️ 冗余 | ✅ 统一 |
| 测试覆盖 | ⚠️ 低 | ✅ >60% |

---

## 附录

### 参考资料
- 设计审查技能: `.claude/skills/openlark-design-review/`
- API 实现规范: `.claude/skills/openlark-api/`
- 验证脚本: `tools/validate_apis.py`
- 已有报告: `reports/api_validation/openlark-hr.md`

### 相关文件路径
```
crates/openlark-hr/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── mod.rs
│   ├── endpoints/          # [待清理]
│   ├── common/
│   │   ├── mod.rs
│   │   ├── service.rs      # [待修复: 冗余字段]
│   │   ├── api_endpoints.rs
│   │   └── ...
│   ├── attendance/
│   │   └── attendance/
│   │       └── v1/
│   │           └── mod.rs  # [待修复: 重复定义]
│   └── ...
```
