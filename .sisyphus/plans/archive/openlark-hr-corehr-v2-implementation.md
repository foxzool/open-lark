# openlark-hr CoreHR V2 API 实现计划

## 📋 任务概述

实现 openlark-hr crate 中的 14 个 CoreHR V2 API 接口，包括：
- 公司管理 (1个)
- 合同管理 (1个)
- 部门管理 (7个)
- 员工管理 (3个)
- 地点管理 (1个)

## 📊 文件状态检查

### 需要删除的文件（实际上不存在，无需操作）
| 文件路径 | 状态 | 操作 |
|---------|------|------|
| `feishu_people/corehr/v2/employee/delete.rs` | ❌ 不存在 | 无需操作 |
| `feishu_people/corehr/v2/employee/list.rs` | ❌ 不存在 | 无需操作 |

### 14 个目标 API 文件状态

#### ✅ 已存在（10个 stub 文件，需要完善实现）

| # | 文件路径 | API 名称 | 文档链接 | 状态 |
|---|---------|---------|---------|------|
| 1 | `v2/company/active.rs` | 启用/停用公司 | [doc](https://open.feishu.cn/document/server-docs/corehr-v2/company/active) | stub |
| 2 | `v2/contract/search.rs` | 搜索合同 | [doc](https://open.feishu.cn/document/server-docs/corehr-v2/contract/search) | stub |
| 3 | `v2/department/batch_get.rs` | 批量查询部门 | [doc](https://open.feishu.cn/document/server-docs/corehr-v2/department/batch_get) | stub |
| 4 | `v2/department/parents.rs` | 获取父部门信息 | [doc](https://open.feishu.cn/document/server-docs/corehr-v2/department/parents) | stub |
| 5 | `v2/department/search.rs` | 搜索部门信息 | [doc](https://open.feishu.cn/document/server-docs/corehr-v2/department/search) | stub |
| 6 | `v2/department/tree.rs` | 查询部门架构树 | [doc](https://open.feishu.cn/document/server-docs/corehr-v2/department/tree) | stub |
| 7 | `v2/employee/batch_get.rs` | 批量查询员工信息 | [doc](https://open.feishu.cn/document/server-docs/corehr-v2/employee/batch_get) | stub |
| 8 | `v2/employee/create.rs` | 添加人员 | [doc](https://open.feishu.cn/document/server-docs/corehr-v2/employee/create) | stub |
| 9 | `v2/employee/search.rs` | 搜索员工信息 | [doc](https://open.feishu.cn/document/server-docs/corehr-v2/employee/search) | stub |
| 10 | `v2/location/patch.rs` | 更新地点 | [doc](https://open.feishu.cn/document/server-docs/corehr-v2/location/patch) | stub |

#### ❌ 不存在（4个文件，需要创建）

| # | 目标文件 | 实际文件 | API 名称 | 文档链接 |
|---|---------|---------|---------|---------|
| 11 | `department/multi_timeline.rs` | `query_multi_timeline.rs` | 批量查询部门版本信息 | [doc](https://open.feishu.cn/document/server-docs/corehr-v2/department/query_multi_timeline) |
| 12 | `department/operation_logs.rs` | `query_operation_logs.rs` | 批量查询部门操作日志 | [doc](https://open.feishu.cn/document/server-docs/corehr-v2/department/query_operation_logs) |
| 13 | `department/timeline.rs` | `query_timeline.rs` | 查询指定生效日期的部门基本信息 | [doc](https://open.feishu.cn/document/server-docs/corehr-v2/department/query_timeline) |

**注意**：目标文件名与现有文件名不一致，需要统一命名规范。

## 🗂️ 实现参考

### 参考实现
参考 `v1/department/create.rs` 作为成熟实现模式：

```rust
// 关键实现要素：
// 1. Request 结构体包含 config 和参数字段
// 2. new() 构造函数
// 3. 参数 setter 方法（链式调用）
// 4. execute() 和 execute_with_options() 方法
// 5. 使用 validate_required! 宏验证必填字段
// 6. 使用 ApiRequest 和 Transport 发送请求
// 7. Response 结构体实现 ApiResponseTrait
```

### v1 部门模型定义（`v1/department/models.rs`）

已包含以下模型可直接复用或参考：

```rust
// 基础数据结构
pub struct Department { ... }
pub struct DepartmentTimeline { ... }
pub struct DepartmentOperationLog { ... }
pub struct DepartmentTreeNode { ... }
pub struct DepartmentChange { ... }

// 请求/响应结构体
pub struct BatchGetRequestBody { ... }
pub struct BatchGetResponse { ... }
pub struct ParentsResponse { ... }
pub struct TreeRequestBody { ... }
pub struct TreeResponse { ... }
pub struct TimelineRequestBody { ... }
pub struct TimelineResponse { ... }
pub struct MultiTimelineRequestBody { ... }
pub struct MultiTimelineResponse { ... }
pub struct OperationLogsRequestBody { ... }
pub struct OperationLogsResponse { ... }
```

### 端点枚举（`common/api_endpoints.rs`）

**FeishuPeopleApiV1** 已定义相关端点：

```rust
pub enum FeishuPeopleApiV1 {
    // Department
    DepartmentBatchGet,
    DepartmentParents(String),
    DepartmentTree,
    DepartmentQueryTimeline,
    DepartmentQueryMultiTimeline,
    DepartmentQueryOperationLogs,
    DepartmentSearch,
    
    // Employee
    EmployeeBatchGet,
    EmployeeCreate,
    EmployeeSearch,
    
    // Company
    CompanyActive(String),
    
    // Contract
    ContractSearch,
    
    // Location
    LocationPatch(String),
}
```

**⚠️ 注意**：`FeishuPeopleApiV2` 枚举不存在，需要创建。

## 📁 文件命名规范统一

### 问题
mod.rs 中的导出名称与目标文件名不一致：

| mod.rs 导出 | 目标文件名 | 一致性 |
|------------|-----------|-------|
| `query_multi_timeline` | `multi_timeline` | ❌ |
| `query_operation_logs` | `operation_logs` | ❌ |
| `query_timeline` | `timeline` | ❌ |

### 解决方案
保持与 mod.rs 中现有导出名称一致，不需要重命名文件。
mod.rs 已有：
```rust
pub mod query_multi_timeline;  // 对应批量查询部门版本信息
pub mod query_operation_logs;  // 对应批量查询部门操作日志
pub mod query_timeline;        // 对应查询指定生效日期的部门基本信息
```

**结论**：用户的列表中的文件名需要调整为与实际文件一致，或者统一命名为 `query_` 前缀。

建议方案：**保持现有文件名，更新用户期望的列表**。

## 🔨 任务依赖图

```
Wave 1: 基础设施（1个任务，阻塞后续所有任务）
├── Task 1: 创建 FeishuPeopleApiV2 枚举和端点定义
│   └── 在 common/api_endpoints.rs 中添加 V2 端点

Wave 2: 部门相关 API（3个任务，依赖 Wave 1）
├── Task 2: 完善 department/batch_get.rs
├── Task 3: 完善 department/search.rs
└── Task 4: 完善 department/parents.rs

Wave 3: 部门高级功能（3个任务，依赖 Wave 2）
├── Task 5: 完善 department/tree.rs
├── Task 6: 完善 department/query_timeline.rs
└── Task 7: 完善 department/query_multi_timeline.rs

Wave 4: 部门操作日志（1个任务，依赖 Wave 1）
└── Task 8: 完善 department/query_operation_logs.rs

Wave 5: 员工相关 API（3个任务，依赖 Wave 1）
├── Task 9: 完善 employee/create.rs
├── Task 10: 完善 employee/batch_get.rs
└── Task 11: 完善 employee/search.rs

Wave 6: 其他 API（3个任务，依赖 Wave 1）
├── Task 12: 完善 company/active.rs
├── Task 13: 完善 contract/search.rs
└── Task 14: 完善 location/patch.rs

Wave 7: 集成测试（1个任务，依赖所有前述任务）
└── Task 15: 运行 just fmt && just lint && just test
```

## 📝 详细任务说明

### Task 1: 创建 FeishuPeopleApiV2 枚举

**文件**: `crates/openlark-hr/src/common/api_endpoints.rs`

**添加内容**: 
- `FeishuPeopleApiV2` 枚举定义
- 14个 API 端点的枚举变体
- `to_url()` 方法的 V2 URL 实现

**V2 端点列表**:
```rust
pub enum FeishuPeopleApiV2 {
    // Company
    CompanyActive(String),
    
    // Contract
    ContractSearch,
    
    // Department
    DepartmentBatchGet,
    DepartmentParents(String),
    DepartmentTree,
    DepartmentQueryTimeline,
    DepartmentQueryMultiTimeline,
    DepartmentQueryOperationLogs,
    DepartmentSearch,
    
    // Employee
    EmployeeCreate,
    EmployeeBatchGet,
    EmployeeSearch,
    
    // Location
    LocationPatch(String),
}
```

### Task 2-14: 完善各 API 实现

每个文件的实现步骤：

1. **查阅官方文档** 获取准确的请求/响应字段
2. **创建 RequestBody 和 Response 结构体**（如 v1/models.rs 不存在）
3. **完善 Request 结构体**:
   - 添加参数字段
   - 实现 setter 方法
   - 实现 `execute()` 和 `execute_with_options()`
4. **添加字段验证** 使用 `validate_required!` 宏
5. **实现 API 调用逻辑**:
   ```rust
   let api_endpoint = FeishuPeopleApiV2::Xxx;
   let request = ApiRequest::<Response>::post(&api_endpoint.to_url());
   let response = Transport::request(request, &self.config, Some(option)).await?;
   ```
6. **更新 mod.rs** 导出（如需要）

### 每个 API 的具体字段

#### company/active.rs - 启用/停用公司
- **Path**: POST `/open-apis/corehr/v2/companies/{company_id}/active`
- **Request**: `{ active_status: bool, effective_date: Option<String> }`
- **Response**: `{ department_id: String }`

#### contract/search.rs - 搜索合同
- **Path**: POST `/open-apis/corehr/v2/contracts/search`
- **Request**: `{ query: String, page_size: Option<i32>, page_token: Option<String> }`
- **Response**: `{ items: Vec<Contract>, has_more: bool, page_token: Option<String> }`

#### department/batch_get.rs - 批量查询部门
- **Path**: POST `/open-apis/corehr/v2/departments/batch_get`
- **Request**: `{ department_ids: Vec<String> }`
- **Response**: `{ items: Vec<Department> }`

#### department/parents.rs - 获取父部门信息
- **Path**: GET `/open-apis/corehr/v2/departments/{department_id}/parents`
- **Request**: 路径参数 `department_id`
- **Response**: `{ items: Vec<Department> }`

#### department/search.rs - 搜索部门信息
- **Path**: POST `/open-apis/corehr/v2/departments/search`
- **Request**: `{ query: String, page_size: Option<i32>, page_token: Option<String> }`
- **Response**: `{ items: Vec<Department>, has_more: bool, page_token: Option<String> }`

#### department/tree.rs - 查询部门架构树
- **Path**: POST `/open-apis/corehr/v2/departments/tree`
- **Request**: `{ department_id: Option<String>, include_inactive: Option<bool> }`
- **Response**: `{ items: Vec<DepartmentTreeNode> }`

#### department/query_timeline.rs - 查询指定生效日期的部门基本信息
- **Path**: POST `/open-apis/corehr/v2/departments/query_timeline`
- **Request**: `{ department_id: String, start_time: String, end_time: String }`
- **Response**: `{ items: Vec<DepartmentTimeline> }`

#### department/query_multi_timeline.rs - 批量查询部门版本信息
- **Path**: POST `/open-apis/corehr/v2/departments/query_multi_timeline`
- **Request**: `{ department_ids: Vec<String>, start_time: String, end_time: String }`
- **Response**: `{ items: HashMap<String, Vec<DepartmentTimeline>> }`

#### department/query_operation_logs.rs - 批量查询部门操作日志
- **Path**: POST `/open-apis/corehr/v2/departments/query_operation_logs`
- **Request**: `{ department_id: String, start_time: String, end_time: String, page_size: Option<i32>, page_token: Option<String> }`
- **Response**: `{ items: Vec<DepartmentOperationLog>, has_more: bool, page_token: Option<String> }`

#### employee/create.rs - 添加人员
- **Path**: POST `/open-apis/corehr/v2/employees`
- **Request**: 包含基本信息、工作信息、合同信息等
- **Response**: `{ employee_id: String }`

#### employee/batch_get.rs - 批量查询员工信息
- **Path**: POST `/open-apis/corehr/v2/employees/batch_get`
- **Request**: `{ employee_ids: Vec<String> }`
- **Response**: `{ items: Vec<Employee> }`

#### employee/search.rs - 搜索员工信息
- **Path**: POST `/open-apis/corehr/v2/employees/search`
- **Request**: `{ query: String, page_size: Option<i32>, page_token: Option<String> }`
- **Response**: `{ items: Vec<Employee>, has_more: bool, page_token: Option<String> }`

#### location/patch.rs - 更新地点
- **Path**: PATCH `/open-apis/corehr/v2/locations/{location_id}`
- **Request**: `{ name: Option<String>, code: Option<String>, ... }`
- **Response**: `{ location_id: String }`

## 🔗 需要创建/更新的模型

在 `v2/department/models.rs` 中创建（参考 v1）：
- `Department` - 部门基础信息
- `DepartmentTimeline` - 部门时间轴
- `DepartmentOperationLog` - 部门操作日志
- `DepartmentTreeNode` - 部门树节点
- `DepartmentChange` - 部门变更项
- 各 RequestBody 和 Response 结构体

在 `v2/employee/models.rs` 中创建：
- `Employee` - 员工基础信息
- `CreateRequestBody` / `CreateResponse`
- `BatchGetRequestBody` / `BatchGetResponse`
- `SearchRequestBody` / `SearchResponse`

## ✅ 验收标准

每个 API 实现后需要验证：

1. **代码质量**:
   - [ ] 使用 `validate_required!` 验证必填字段
   - [ ] 所有参数字段有 setter 方法
   - [ ] 使用 `FeishuPeopleApiV2` 枚举获取 URL
   - [ ] 响应结构体实现 `ApiResponseTrait`

2. **编译检查**:
   - [ ] `just fmt` 通过
   - [ ] `just lint` 通过（零警告）
   - [ ] `just test` 通过

3. **文档**:
   - [ ] 文件顶部有中文注释说明 API 功能
   - [ ] docPath 链接正确
   - [ ] 所有 pub 项有文档注释

## 🚀 执行命令

```bash
# 格式化
just fmt

# 代码检查
just lint

# 运行测试
just test

# 完整检查
just check-all
```

## 📁 文件列表汇总

### 需要完善的文件（10个）
1. `crates/openlark-hr/src/feishu_people/corehr/v2/company/active.rs`
2. `crates/openlark-hr/src/feishu_people/corehr/v2/contract/search.rs`
3. `crates/openlark-hr/src/feishu_people/corehr/v2/department/batch_get.rs`
4. `crates/openlark-hr/src/feishu_people/corehr/v2/department/parents.rs`
5. `crates/openlark-hr/src/feishu_people/corehr/v2/department/search.rs`
6. `crates/openlark-hr/src/feishu_people/corehr/v2/department/tree.rs`
7. `crates/openlark-hr/src/feishu_people/corehr/v2/department/query_timeline.rs`
8. `crates/openlark-hr/src/feishu_people/corehr/v2/department/query_multi_timeline.rs`
9. `crates/openlark-hr/src/feishu_people/corehr/v2/department/query_operation_logs.rs`
10. `crates/openlark-hr/src/feishu_people/corehr/v2/employee/batch_get.rs`
11. `crates/openlark-hr/src/feishu_people/corehr/v2/employee/create.rs`
12. `crates/openlark-hr/src/feishu_people/corehr/v2/employee/search.rs`
13. `crates/openlark-hr/src/feishu_people/corehr/v2/location/patch.rs`

### 需要修改的文件（2个）
1. `crates/openlark-hr/src/common/api_endpoints.rs` - 添加 FeishuPeopleApiV2 枚举
2. `crates/openlark-hr/src/feishu_people/corehr/v2/department/mod.rs` - 确认导出（可能无需修改）

### 需要创建的模型文件（2个，可选）
1. `crates/openlark-hr/src/feishu_people/corehr/v2/department/models.rs`
2. `crates/openlark-hr/src/feishu_people/corehr/v2/employee/models.rs`

---

**计划生成时间**: 2026-01-28  
**预期工作量**: 中等（14个API + 端点枚举）  
**建议并行度**: 按 Wave 分组并行执行
