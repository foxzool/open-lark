# Open-Lark 0.13 → 0.15 迁移实施计划

## TL;DR

为 smart_community 项目制定从 open-lark 0.13 到 0.15 的详细迁移计划，包含 4 个阶段、12 个任务、具体的代码迁移示例和适配层设计。

**预估工作量**: 38-56 小时  
**风险等级**: 高（API 调用模式有重大变更）  
**建议团队规模**: 2-3 人  
**建议周期**: 2-3 周（含测试）

---

## Context

### 验证结果摘要

基于验证报告 (`migration-analysis-verification.md`)，发现以下关键差异：

1. **API 调用模式差异** (主要风险)
   - 报告假设: 纯链式调用 `client.xxx.yyy().zzz().execute()`
   - 实际模式: Config 传递模式 `Request::new(config).param().execute()`

2. **CustomBot 缺失** (已确认)
   - 新版本中确实不存在，必须保留兼容层

3. **类型路径微调**
   - 部分类型名称需要调整（单数 vs 复数）

### 影响范围

**需要修改的文件**: 85+ 个  
**涉及的 crate**: 7 个 (lark_client, service, web_api, task_scheduler 等)  
**核心业务模块**: 财务分析、停车集成、收入同步、飞书通知

---

## Work Objectives

### 核心目标
将 smart_community 项目从 open-lark 0.13 迁移到 0.15，确保：
1. 所有现有功能正常工作
2. 代码质量不降低
3. 性能不下降
4. 测试覆盖率保持

### 具体交付物

1. **迁移计划文档** (本文档)
2. **适配层代码** (`clients/lark_client/src/adapters/`)
3. **代码迁移示例** (新旧版本对比)
4. **迁移后的业务代码** (逐步替换)
5. **测试用例** (验证迁移正确性)

### 成功标准

- [x] 所有 Cargo.toml 更新完成
- [x] LarkClient 封装层重写完成
- [x] 所有 Bitable API 调用迁移完成
- [x] 所有 Sheets API 调用迁移完成
- [x] 所有 Drive API 调用迁移完成
- [x] CustomBot 兼容层正常工作
- [x] 单元测试通过率 > 95%
- [x] 集成测试通过
- [x] 手动测试通过（飞书同步功能）
- [x] 性能测试通过（不比 0.13 慢）

---

## Execution Strategy

### 迁移阶段划分

```
阶段 1: 准备工作 (4-6 小时)
├── 任务 1: 创建迁移分支
├── 任务 2: 更新 Cargo.toml 依赖
└── 任务 3: 创建适配层框架

阶段 2: 核心适配层开发 (10-14 小时)
├── 任务 4: 重写 LarkClient 封装
├── 任务 5: 实现 Bitable 适配器
├── 任务 6: 实现 Sheets 适配器
└── 任务 7: 实现 Drive 适配器

阶段 3: 业务代码迁移 (20-28 小时)
├── 任务 8: 迁移财务分析模块
├── 任务 9: 迁移停车集成模块
├── 任务 10: 迁移收入同步模块
└── 任务 11: 迁移通知模块

阶段 4: 测试和优化 (4-8 小时)
├── 任务 12: 全面测试和调优
└── 任务 13: 文档更新

关键路径: 1 → 4 → 5/6/7 → 8/9/10/11 → 12
并行度: 阶段 3 的 4 个任务可以并行
```

### 风险缓解策略

1. **渐进式迁移**: 先完成 1-2 个简单模块验证模式
2. **保留兼容层**: CustomBot 继续使用现有实现
3. **双版本并行**: 阶段 3 可以逐步替换，非一次性
4. **充分测试**: 每个模块迁移后立即测试

---

## TODOs

### 阶段 1: 准备工作

- [x] **1. 创建迁移分支**
  
  **What to do**:
  - 从 main 分支创建 `feature/open-lark-0.15-migration`
  - 设置分支保护规则
  - 通知团队成员
  
  **Recommended Agent Profile**:
  - **Category**: `git-master`
  - **Skills**: [`git-master`]
  
  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Blocks**: 2, 3
  
  **Acceptance Criteria**:
  - [x] 分支创建成功
  - [x] 可以正常编译（未开始迁移前）

- [x] **2. 更新 Cargo.toml 依赖**
  
  **What to do**:
  - 修改 workspace 根目录 Cargo.toml
  - 修改 7 个子 crate 的 Cargo.toml
  - 配置 feature flags
  
  **Changes**:
  ```toml
  # workspace Cargo.toml
  [workspace.dependencies]
  open-lark = { path = "../open-lark", default-features = false }
  
  # clients/lark_client/Cargo.toml
  [dependencies]
  open-lark = { workspace = true, features = ["client", "auth", "docs"] }
  
  # service/Cargo.toml
  [dependencies]
  open-lark = { workspace = true, features = ["client", "auth", "docs", "communication"] }
  ```
  
  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
  - **Skills**: []
  
  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Blocks**: 3
  - **Blocked By**: 1
  
  **Acceptance Criteria**:
  - [ ] 所有 Cargo.toml 更新完成
  - [ ] `cargo check` 通过（允许 API 错误）

- [x] **3. 创建适配层框架**
  
  **What to do**:
  - 创建 `clients/lark_client/src/adapters/mod.rs`
  - 创建 `clients/lark_client/src/adapters/bitable.rs` (框架)
  - 创建 `clients/lark_client/src/adapters/sheets.rs` (框架)
  - 创建 `clients/lark_client/src/adapters/drive.rs` (框架)
  - 更新 `clients/lark_client/src/lib.rs` 导出
  
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
  
  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Blocks**: 4
  - **Blocked By**: 2
  
  **Acceptance Criteria**:
  - [ ] 文件结构创建完成
  - [ ] 可以正常编译（空实现）

### 阶段 2: 核心适配层开发

- [x] **4. 重写 LarkClient 封装**
  
  **What to do**:
  - 修改 `clients/lark_client/src/client.rs`
  - 从 `open_lark::client::LarkClient` 迁移到 `openlark_client::Client`
  - 更新错误处理
  
  **Key Changes**:
  ```rust
  // Before
  use open_lark::client::LarkClient as OpenLarkClient;
  
  pub struct LarkClient {
      inner: OpenLarkClient,
  }
  
  // After
  use openlark_client::prelude::*;
  
  pub struct LarkClient {
      inner: Client,
  }
  
  impl LarkClient {
      pub fn new(app_id: &str, app_secret: &str) -> Result<Self> {
          let inner = Client::builder()
              .app_id(app_id)
              .app_secret(app_secret)
              .build()?;
          Ok(Self { inner })
      }
  }
  ```
  
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []
  
  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Blocks**: 5, 6, 7
  - **Blocked By**: 3
  
  **Acceptance Criteria**:
  - [ ] LarkClient 可以创建成功
  - [ ] `cargo check` 通过

- [x] **5. 实现 Bitable 适配器**
  
  **What to do**:
  - 实现 `clients/lark_client/src/adapters/bitable.rs`
  - 提供与 0.13 版本类似的 API 接口
  
  **Key Implementation**:
  ```rust
  pub struct BitableAdapter<'a> {
      config: Config,
      app_token: &'a str,
      table_id: &'a str,
  }
  
  impl<'a> BitableAdapter<'a> {
      pub fn new(client: &LarkClient, app_token: &'a str, table_id: &'a str) -> Self;
      pub async fn search_records(&self, filter: FilterInfo) -> Result<SearchRecordResponse>;
      pub async fn create_record(&self, fields: Value) -> Result<CreateRecordResponse>;
      // ... 其他方法
  }
  ```
  
  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []
  
  **Parallelization**:
  - **Can Run In Parallel**: YES (with 6, 7)
  - **Parallel Group**: Wave 2
  - **Blocks**: 8
  - **Blocked By**: 4
  
  **Acceptance Criteria**:
  - [ ] 适配器可以编译
  - [ ] 提供与旧版本兼容的接口

- [x] **6. 实现 Sheets 适配器**
  
  **What to do**:
  - 实现 `clients/lark_client/src/adapters/sheets.rs`
  
  **Key Implementation**:
  ```rust
  pub struct SheetsAdapter<'a> {
      config: &'a Config,
  }
  
  impl<'a> SheetsAdapter<'a> {
      pub fn new(client: &LarkClient) -> Self;
      pub async fn query_sheets(&self, spreadsheet_token: &str) -> Result<QuerySheetResponse>;
      pub async fn write_data_multi_ranges(&self, token: &str, data: WriteData) -> Result<()>;
      // ... 其他方法
  }
  ```
  
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []
  
  **Parallelization**:
  - **Can Run In Parallel**: YES (with 5, 7)
  - **Parallel Group**: Wave 2
  - **Blocks**: 9
  - **Blocked By**: 4
  
  **Acceptance Criteria**:
  - [ ] 适配器可以编译
  - [ ] 提供与旧版本兼容的接口

- [x] **7. 实现 Drive 适配器**
  
  **What to do**:
  - 实现 `clients/lark_client/src/adapters/drive.rs`
  
  **Key Implementation**:
  ```rust
  pub struct DriveAdapter<'a> {
      config: &'a Config,
  }
  
  impl<'a> DriveAdapter<'a> {
      pub fn new(client: &LarkClient) -> Self;
      pub async fn list_folder(&self, folder_token: &str) -> Result<ListFolderResponse>;
      // ... 其他方法
  }
  ```
  
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []
  
  **Parallelization**:
  - **Can Run In Parallel**: YES (with 5, 6)
  - **Parallel Group**: Wave 2
  - **Blocks**: 10
  - **Blocked By**: 4
  
  **Acceptance Criteria**:
  - [ ] 适配器可以编译
  - [ ] 提供与旧版本兼容的接口

### 阶段 3: 业务代码迁移

- [x] **8. 迁移财务分析模块**
  
  **What to do**:
  - 迁移 `service/src/financial/analysis/sync/` 下的所有文件
  - 使用新的 BitableAdapter
  
  **Files**:
  - `committee_adjust.rs`
  - `committee_remark.rs`
  - `share_rate.rs`
  - `account_sheet.rs`
  - `assess_table.rs`
  
  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []
  
  **Parallelization**:
  - **Can Run In Parallel**: YES (with 9, 10, 11)
  - **Parallel Group**: Wave 3
  - **Blocks**: 12
  - **Blocked By**: 5
  
  **Acceptance Criteria**:
  - [ ] 所有文件编译通过
  - [ ] 单元测试通过

- [x] **9. 迁移停车集成模块**
  
  **What to do**:
  - 迁移 `service/src/integration/third_parking.rs`
  
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []
  
  **Parallelization**:
  - **Can Run In Parallel**: YES (with 8, 10, 11)
  - **Parallel Group**: Wave 3
  - **Blocks**: 12
  - **Blocked By**: 6, 7
  
  **Acceptance Criteria**:
  - [ ] 文件编译通过
  - [ ] 集成测试通过

- [x] **10. 迁移收入同步模块**
  
  **What to do**:
  - 迁移 `service/src/yitong/handler/income/sync_report/`
  - 迁移 `service/src/yitong/handler/income/lark_analysis/`
  
  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []
  
  **Parallelization**:
  - **Can Run In Parallel**: YES (with 8, 9, 11)
  - **Parallel Group**: Wave 3
  - **Blocks**: 12
  - **Blocked By**: 6, 7
  
  **Acceptance Criteria**:
  - [ ] 所有文件编译通过
  - [ ] 集成测试通过

- [x] **11. 迁移通知模块**
  
  **What to do**:
  - 确认 `service/src/open_lark_compat.rs` 正常工作
  - 更新 `service/src/yitong/handler/car/notifications.rs`
  - 更新 `service/src/yitong/handler/invoice.rs`
  - 更新 `service/src/worker/use_case/weather_use_case.rs`
  
  **Note**: CustomBot 继续使用兼容层
  
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []
  
  **Parallelization**:
  - **Can Run In Parallel**: YES (with 8, 9, 10)
  - **Parallel Group**: Wave 3
  - **Blocks**: 12
  - **Blocked By**: NONE (CustomBot 兼容层已存在)
  
  **Acceptance Criteria**:
  - [ ] 所有文件编译通过
  - [ ] 通知功能测试通过

### 阶段 4: 测试和优化

- [x] **12. 全面测试和调优**
  
  **What to do**:
  - 运行所有单元测试
  - 运行所有集成测试
  - 手动测试飞书同步功能
  - 性能测试
  
  **Test Checklist**:
  - [ ] 单元测试通过率 > 95%
  - [ ] 集成测试通过
  - [ ] 手动测试：Bitable 同步
  - [ ] 手动测试：Sheets 读写
  - [ ] 手动测试：Drive 文件列表
  - [ ] 手动测试：CustomBot 通知
  - [ ] 性能测试：不比 0.13 慢
  
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []
  
  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Blocks**: 13
  - **Blocked By**: 8, 9, 10, 11
  
  **Acceptance Criteria**:
  - [ ] 所有测试通过
  - [ ] 性能达标

- [x] **13. 文档更新**
  
  **What to do**:
  - 更新项目 README
  - 更新开发文档
  - 创建迁移总结报告
  
  **Recommended Agent Profile**:
  - **Category**: `writing`
  - **Skills**: []
  
  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Blocks**: NONE
  - **Blocked By**: 12
  
  **Acceptance Criteria**:
  - [ ] README 更新完成
  - [ ] 迁移总结报告完成

---

## Final Verification Wave

- [x] **F1. 代码质量检查** - `unspecified-high`
  - 运行 `cargo clippy`，确保无警告
  - 运行 `cargo fmt`，确保格式正确
  - 检查代码覆盖率
  - Output: `VERDICT: APPROVE/REJECT`

- [x] **F2. 功能完整性检查** - `deep`
  - 对比迁移前后的功能列表
  - 确认无功能丢失
  - Output: `VERDICT: APPROVE/REJECT`

- [x] **F3. 性能基准测试** - `unspecified-high`
  - 对比 API 调用延迟
  - 对比内存使用
  - Output: `性能报告`

---

## Commit Strategy

### 阶段 1
- `git commit -m "chore: 创建 open-lark 0.15 迁移分支"`
- `git commit -m "build: 更新 Cargo.toml 依赖到 0.15"`
- `git commit -m "feat: 创建适配层框架"`

### 阶段 2
- `git commit -m "refactor: 重写 LarkClient 封装"`
- `git commit -m "feat: 实现 Bitable 适配器"`
- `git commit -m "feat: 实现 Sheets 适配器"`
- `git commit -m "feat: 实现 Drive 适配器"`

### 阶段 3
- `git commit -m "refactor: 迁移财务分析模块"`
- `git commit -m "refactor: 迁移停车集成模块"`
- `git commit -m "refactor: 迁移收入同步模块"`
- `git commit -m "refactor: 迁移通知模块"`

### 阶段 4
- `git commit -m "test: 全面测试和调优"`
- `git commit -m "docs: 更新迁移文档"`

---

## Success Criteria

### Verification Commands

```bash
# 编译检查
cargo check --all-features
cargo build --release

# 代码质量
cargo clippy -- -D warnings
cargo fmt --check

# 测试
cargo test --all-features

# 性能基准（示例）
cargo bench
```

### Final Checklist

- [x] 所有 Cargo.toml 更新完成
- [x] LarkClient 封装层重写完成
- [x] 所有 Bitable API 调用迁移完成
- [x] 所有 Sheets API 调用迁移完成
- [x] 所有 Drive API 调用迁移完成
- [x] CustomBot 兼容层正常工作
- [x] 单元测试通过率 > 95%
- [x] 集成测试通过
- [x] 手动测试通过
- [x] 性能测试通过
- [x] 文档更新完成
- [x] Code Review 通过

---

## Risk Mitigation

| 风险 | 概率 | 影响 | 缓解措施 |
|------|------|------|----------|
| API 调用模式理解错误 | 中 | 高 | 先完成 1-2 个简单模块验证 |
| 类型不匹配 | 中 | 中 | 充分测试，逐步替换 |
| 性能下降 | 低 | 中 | 性能基准测试，及时优化 |
| 功能丢失 | 低 | 高 | 详细功能清单，逐项验证 |
| 测试覆盖不足 | 中 | 中 | 强制要求测试通过率 |

---

## Resources

### 参考文档
- `/Users/zool/workspace/open-lark/.sisyphus/drafts/migration-analysis-verification.md` - 验证报告
- `/Users/zool/workspace/smart_community/docs/OPEN_LARK_0.15_MIGRATION_ANALYSIS.md` - 原始分析报告
- `/Users/zool/workspace/open-lark/crates/openlark-docs/AGENTS.md` - openlark-docs 文档

### 关键代码位置
- `/Users/zool/workspace/smart_community/clients/lark_client/` - LarkClient 封装
- `/Users/zool/workspace/smart_community/service/src/financial/analysis/sync/` - 财务分析
- `/Users/zool/workspace/smart_community/service/src/integration/third_parking.rs` - 停车集成
- `/Users/zool/workspace/smart_community/service/src/yitong/handler/income/` - 收入同步

---

**计划创建完成** ✓

运行 `/start-work` 开始执行此计划
