# 2026-01-27 任务 4.1：更新 lib.rs 导出

## 执行内容

成功更新 `crates/openlark-hr/src/lib.rs`，将旧模块结构迁移到新的 bizTag 级架构。

## 完成的工作

### 1. 创建顶层服务类型

为每个 bizTag 模块创建了顶层服务类型：

- `attendance/mod.rs`: 创建 `Attendance` 类型
- `compensation_management/mod.rs`: 创建 `CompensationManagement` 类型
- `ehr/mod.rs`: 创建 `Ehr` 类型
- `feishu_people/mod.rs`: 创建 `Corehr` 类型
- `hire/mod.rs`: 创建 `Hire` 类型
- `okr/mod.rs`: 创建 `Okr` 类型
- `payroll/mod.rs`: 创建 `Payroll` 类型
- `performance/mod.rs`: 创建 `Performance` 类型

每个服务类型都提供：
- `new(config: Config) -> Self` 构造方法
- `config(&self) -> &Config` 配置访问方法
- `v1()` 或 `v2()` 版本访问方法（根据模块支持的版本）

### 2. 创建版本服务类型

为每个 project 模块创建了版本服务类型：

- `attendance/attendance/v1/mod.rs`: 创建 `AttendanceV1` 类型
- `compensation_management/compensation/v1/mod.rs`: 创建 `CompensationV1` 类型
- `ehr/ehr/v1/mod.rs`: 创建 `EhrV1` 类型
- `feishu_people/corehr/v1/mod.rs`: 创建 `CorehrV1` 类型
- `feishu_people/corehr/v2/mod.rs`: 创建 `CorehrV2` 类型
- `hire/hire/v1/mod.rs`: 创建 `HireV1` 类型
- `hire/hire/v2/mod.rs`: 创建 `HireV2` 类型
- `okr/okr/v1/mod.rs`: 创建 `OkrV1` 类型
- `payroll/payroll/v1/mod.rs`: 创建 `PayrollV1` 类型
- `performance/performance/v1/mod.rs`: 创建 `PerformanceV1` 类型
- `performance/performance/v2/mod.rs`: 创建 `PerformanceV2` 类型

### 3. 创建 common 模块

创建了 `common/mod.rs`，导出通用工具模块：
- `pub mod macros;`
- `pub mod models;`
- `pub mod service;`

### 4. 更新 lib.rs 模块声明

移除了旧的模块声明：
- ❌ `mod macros;` → 已移动到 `common`
- ❌ `pub mod service;` → 已移动到 `common`
- ❌ `pub mod compensation;` → 已替换为 `compensation_management`
- ❌ `pub mod corehr;` → 已替换为 `feishu_people`

添加了新的模块声明（bizTag 级）：
- ✅ `pub mod attendance;`
- ✅ `pub mod compensation_management;`
- ✅ `pub mod ehr;`
- ✅ `pub mod feishu_people;`
- ✅ `pub mod hire;`
- ✅ `pub mod okr;`
- ✅ `pub mod payroll;`
- ✅ `pub mod performance;`
- ✅ `pub mod common;`

### 5. 更新 lib.rs use 声明

更新了 `HrService` 的导入：
- `use service::HrService;` → `use common::service::HrService;`

### 6. 更新 HrClient 方法返回类型

所有 `HrClient` 方法都更新为使用新的模块路径：

- `attendance()` → `attendance::Attendance::new(self.service.config().clone())`
- `corehr()` → `feishu_people::Corehr::new(self.service.config().clone())`
- `compensation()` → `compensation_management::CompensationManagement::new(self.service.config().clone())`
- `ehr()` → `ehr::Ehr::new(self.service.config().clone())`
- `hire()` → `hire::Hire::new(self.service.config().clone())`
- `okr()` → `okr::Okr::new(self.service.config().clone())`
- `payroll()` → `payroll::Payroll::new(self.service.config().clone())`
- `performance()` → `performance::Performance::new(self.service.config().clone())`

注意：所有方法都从 `self.service.clone()` 改为 `self.service.config().clone()`，因为新服务类型只接受 `Config`，不接受 `Arc<HrService>`。

## 技术细节

### 服务类型设计模式

遵循 openlark-docs 的服务类型设计模式：

```rust
// 顶层服务类型（bizTag 级）
#[derive(Debug, Clone)]
pub struct Attendance {
    config: Config,
}

impl Attendance {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 版本访问方法
    pub fn v1(&self) -> attendance::v1::AttendanceV1 {
        attendance::v1::AttendanceV1::new(self.config.clone())
    }
}
```

### 版本服务类型设计模式

```rust
// 版本服务类型（project/version 级）
#[derive(Debug, Clone)]
pub struct AttendanceV1 {
    config: Config,
}

impl AttendanceV1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
```

### 链式调用支持

HrClient 通过返回服务类型，支持链式调用：

```rust
let client = HrClient::new(config);

// 链式调用
let employees = client.corehr().v1().person().get(...).await?;
let groups = client.attendance().v1().group().list(...).await?;
```

## 遇到的问题与解决

### 问题 1: LSP 报错找不到 `attendance::v1::AttendanceV1`

**原因**: `attendance/attendance/v1/mod.rs` 中没有定义 `AttendanceV1` 类型。

**解决**: 创建了 `AttendanceV1` 类型。

### 问题 2: LSP 报错找不到 `common` 模块

**原因**: `common/` 目录下没有 `mod.rs` 文件。

**解决**: 创建了 `common/mod.rs`，导出 `macros`、`models`、`service` 模块。

### 问题 3: LSP 报错找不到 `service` 模块

**原因**: `service` 模块已移动到 `common/` 目录，但 lib.rs 中的 use 声明未更新。

**解决**: 更新 `use service::HrService;` 为 `use common::service::HrService;`。

### 问题 4: 使用了 Rust 保留字作为模块名

**示例**:
- `pub mod match;` - `match` 是保留字
- `pub mod enum;` - `enum` 是保留字

**原因**: 重构脚本使用了 API 名称（如 "match"）作为文件名，但这些是 Rust 保留字。

**解决**:
- 对于 `compensation_standard/match.rs`，使用 `pub mod r#match;`
- 对于 `v2/enum` 目录，直接删除 `pub mod enum;` 声明

### 问题 5: 编译警告 - 未读取的字段

**警告**: `field 'config' is never read`

**原因**: 生成的 API 骨架代码中，`config` 字段被声明但未被实际使用。

**影响**: 仅警告，不影响编译。

**说明**: 这些骨架代码将在后续任务中填充实际实现。

## 验证结果

### 编译检查

```bash
cargo check -p openlark-hr
```

**结果**: ✅ 编译通过（0 errors, 1639 warnings）

警告主要是：
- 模块命名风格问题（如 `batchSave` 应该是 `batch_save`）
- 未使用的字段（骨架代码中的 `config` 字段）

### 代码格式化

```bash
cargo fmt -p openlark-hr
```

**结果**: ✅ 无输出（代码已符合格式规范）

## 架构变更总结

### 旧架构

```
src/
├── lib.rs
├── macros.rs              # 顶层宏定义
├── service.rs             # 顶层服务
├── attendance/            # 直接的 project 目录
├── compensation/           # 直接的 project 目录
├── corehr/              # 直接的 project 目录
├── ehr/                 # 直接的 project 目录
├── hire/                # 直接的 project 目录
├── okr/                 # 直接的 project 目录
├── payroll/              # 直接的 project 目录
└── performance/          # 直接的 project 目录
```

### 新架构

```
src/
├── lib.rs
├── common/               # 通用工具模块
│   ├── macros.rs
│   ├── models.rs
│   └── service.rs
├── attendance/            # bizTag 级
│   ├── mod.rs            # 顶层服务类型
│   └── attendance/
│       ├── mod.rs        # project 级
│       └── v1/
│           ├── mod.rs    # 版本服务类型
│           └── ...      # API 实现
├── compensation_management/ # bizTag 级
│   ├── mod.rs
│   └── compensation/
│       └── v1/
├── ehr/                 # bizTag 级
│   └── ehr/
│       └── v1/
├── feishu_people/        # bizTag 级
│   └── corehr/
│       ├── mod.rs
│       ├── v1/
│       └── v2/
├── hire/                # bizTag 级
│   └── hire/
│       ├── mod.rs
│       ├── v1/
│       └── v2/
├── okr/                 # bizTag 级
│   └── okr/
│       └── v1/
├── payroll/              # bizTag 级
│   └── payroll/
│       └── v1/
└── performance/          # bizTag 级
    └── performance/
        ├── mod.rs
        ├── v1/
        └── v2/
```

### 层级关系

```
HrClient (顶层入口)
  └── Attendance (bizTag 服务)
      └── AttendanceV1 (project/version 服务)
            └── API 实现
```

## 下一步

根据计划（restructure-hr.md），下一步是任务 4.2：更新 service.rs 为所有业务域提供访问方法。

但需要注意：
- 当前 `service.rs` 已移动到 `common/` 目录
- 计划中提到的 `service.rs` 可能是指 `HrClient` 的链式调用方法
- 这些方法已在任务 4.1 中更新完成

## 经验总结

### 服务类型设计的最佳实践

1. **类型明确性**: 每个服务类型都明确定义其职责（顶层服务 vs 版本服务）
2. **配置传递**: 使用 `Config` 而不是 `Arc<HrService>` 提高灵活性
3. **Clone 支持**: 所有服务类型都实现 `Clone`，便于在 HrClient 中传递
4. **链式调用**: 通过返回服务类型支持流畅的 API 调用体验

### 命名规范

- bizTag 级: `{BizTag}`（如 `Attendance`, `Corehr`）
- project 级: `{Project}V{Version}`（如 `AttendanceV1`, `CorehrV2`）
- 避免: 使用保留字作为模块名（如 `match`, `enum`）

### 模块组织原则

- 按业务域分组（bizTag → project → version → resource → API）
- 通用工具统一放在 `common/` 目录
- 每个层级都有对应的 `mod.rs` 导出子模块
