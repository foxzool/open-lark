---
name: openlark-api
description: 规范 OpenLark 项目 API 接口文件实现。用于添加新 API、实现飞书开放平台 API 端点、创建业务模块 API 或重构现有 API 接口。触发关键词：API 接口、API 文件、飞书 API、添加 API
allowed-tools:
  - Read
  - Write
  - Edit
  - Glob
  - LspDocumentSymbols
  - LspDiagnostics
---

# OpenLark API 接口实现规范

## 文件命名规范

API 接口文件必须遵循以下路径格式：

```
crates/{feature-crate}/src/{version}/{resource}/{name}.rs
```

### 路径组件说明

| 组件 | 说明 | 示例值 |
|------|------|--------|
| `feature-crate` | Feature crate 名称（对应 CSV 的 bizTag） | `openlark-communication`, `openlark-hr`, `openlark-meeting`, `openlark-docs`, `openlark-auth` |
| `version` | API 版本（对应 CSV 的 meta.Version） | `v1`, `v2`, `v3`, `old` |
| `resource` | 资源名称（对应 CSV 的 meta.Resource） | `message`, `user`, `document`, `sheet`, `candidate` |
| `name` | 具体操作或实体名称（对应 CSV 的 meta.Name） | `send`, `get`, `create`, `update`, `list`, `delete` |

### 完整路径示例

```
crates/openlark-communication/src/v1/message/send.rs
crates/openlark-docs/src/v1/document/create.rs
crates/openlark-hr/src/v1/candidate/get.rs
```

### Feature Crate 与 CSV bizTag 的映射关系

| CSV bizTag | Feature Crate | 说明 |
|-----------|--------------|------|
| `communication` | `openlark-communication` | 通讯协作模块 |
| `docs` | `openlark-docs` | 文档协作模块 |
| `hr` | `openlark-hr` | 人力管理模块 |
| `auth` | `openlark-auth` | 认证模块 |
| `meeting_room`, `vc`, `calendar` | `openlark-meeting` | 会议与日程模块 |
| `mail` | `openlark-mail` | 邮件服务模块 |
| `cardkit` | `openlark-cardkit` | 卡片工具模块 |
| `ai` | `openlark-ai` | AI 服务模块 |
| `helpdesk` | `openlark-helpdesk` | 帮助台模块 |
| `application` | `openlark-application` | 应用管理模块 |
| `security_and_compliance` | `openlark-security` | 安全合规模块 |

## API 接口文件模板

每个 API 文件必须包含以下组件：

### 1. Request 类型定义

```rust
use openlark_core::api::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {Name}Request {
    // 请求字段
    pub field1: String,
    pub field2: Option<i32>,
}
```

### 2. Response 类型定义

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {Name}Response {
    pub data: DataType,
    pub request_id: String,
}
```

### 3. 实现 ApiRequest trait

```rust
impl ApiRequest for {Name}Request {
    type Response = Response<{Name}Response>;

    fn method(&self) -> RequestMethod {
        RequestMethod::Post  // 或 Get, Put, Delete
    }

    fn path(&self) -> &'static str {
        "/open-apis/{bizTag}/{version}/{resource}/{action}"
    }

    fn access_token_type(&self) -> AccessTokenType {
        AccessTokenType::User  // 或 App, Tenant
    }
}
```

### 4. 端点定义（在 end_points 模块中）

```rust
pub mod end_points {
    use super::*;

    pub fn {name}() -> ApiEndpoint<{Name}Request, Response<{Name}Response>> {
        ApiEndpoint::new("/open-apis/{version}/{resource}/{action}")
            .method(RequestMethod::Post)
            .description("API 描述")
            .requires_auth(true)
    }
}
```

## 完整实现示例

参考以下完整示例：

```rust
// src/communication/open-apis/v1/message/send.rs

use openlark_core::{
    api::{ApiRequest, ApiEndpoint, RequestMethod},
    error::SDKResult,
    constants::AccessTokenType,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSendRequest {
    #[serde(rename = "receive_id")]
    pub receive_id: String,
    #[serde(rename = "receive_id_type")]
    pub receive_id_type: String,
    #[serde(rename = "msg_type")]
    pub msg_type: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSendResponse {
    #[serde(rename = "message_id")]
    pub message_id: String,
    #[serde(rename = "create_time")]
    pub create_time: String,
}

impl ApiRequest for MessageSendRequest {
    type Response = Response<MessageSendResponse>;

    fn method(&self) -> RequestMethod {
        RequestMethod::Post
    }

    fn path(&self) -> &'static str {
        "/open-apis/im/v1/messages"
    }

    fn access_token_type(&self) -> AccessTokenType {
        AccessTokenType::User
    }
}

pub mod end_points {
    use super::*;

    pub fn send_message() -> ApiEndpoint<MessageSendRequest, Response<MessageSendResponse>> {
        ApiEndpoint::new("/open-apis/im/v1/messages")
            .method(RequestMethod::Post)
            .description("发送文本、图片、富文本等消息")
            .requires_auth(true)
    }
}

pub fn text_message(
    receive_id: String,
    receive_id_type: String,
    text: String,
) -> MessageSendRequest {
    MessageSendRequest {
        receive_id,
        receive_id_type,
        msg_type: "text".to_string(),
        content: format!(r#"{{"text":"{}"}}"#, text),
        uuid: None,
    }
}
```

## 核心组件使用

### 错误处理

```rust
use openlark_core::error::{SDKResult, LarkAPIError};

pub async fn send(&self) -> SDKResult<MessageSendResponse> {
    Err(LarkAPIError::ValidationError {
        field: "receive_id".to_string(),
        message: "接收者 ID 不能为空".to_string(),
    })
}
```

**错误码对齐规则**：
- 优先级：飞书通用 code > HTTP status > 内部业务码
- 关键错误码：99991661/99991671/99991677（Token 相关），99991672/99991676（权限相关）

### 客户端访问

```rust
use openlark_client::Client;

let client = Client::from_env()?;
let response = client
    .communication()?
    .send_message("user_open_id", "open_id", "Hello!")
    .await?;
```

### 请求构建

```rust
use openlark_core::request_builder::UnifiedRequestBuilder;
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;

let mut request = ApiRequest::post("https://open.feishu.cn/open-apis/im/v1/messages")
    .body(serde_json::json!({
        "receive_id": "user_open_id",
        "content": "{\"text\":\"Hello World\"}",
        "msg_type": "text"
    }));

let reqwest_request = UnifiedRequestBuilder::build(
    &mut request,
    AccessTokenType::User,
    &config,
    &request_option
).await?;
```

### 响应处理

```rust
let response: Response<MessageSendResponse> = transport.send(request).await?;

match response.into_result() {
    Ok(data) => println!("消息发送成功: {}", data.message_id),
    Err(error) => eprintln!("发送失败: {}", error.user_friendly_message()),
}
```

## 模块组织

```
src/
├── communication/
│   └── open-apis/
│       └── v1/
│           ├── im/
│           │   └── message/
│           │       ├── send.rs
│           │       └── get.rs
│           └── contact/
│               └── user/
│                   └── get.rs
```

每个 `mod.rs` 文件必须导出类型：

```rust
// src/communication/open-apis/v1/mod.rs

pub mod im;
pub mod contact;

pub use im::message::{MessageSendRequest, MessageSendResponse, end_points};
```

## 实现检查清单

创建或修改 API 接口文件时，必须确保：

- [ ] 文件路径遵循 `src/bizTag/open-apis/version/resource/name.rs` 格式
- [ ] Request 类型实现了 `ApiRequest` trait
- [ ] Response 类型正确定义并支持序列化/反序列化
- [ ] 正确使用 `AccessTokenType`（User/App/Tenant）
- [ ] 错误处理使用 `SDKResult<T>` 和 `LarkAPIError`
- [ ] 端点定义在 `end_points` 子模块中
- [ ] 添加了文档注释（///）
- [ ] 字段使用 `#[serde(rename)]` 属性
- [ ] 可选字段标记为 `Option<T>` 并使用 `#[serde(skip_serializing_if = "Option::is_none")]`
- [ ] 在模块的 `mod.rs` 中正确导出类型

## Enum+Builder 端点系统

项目提供了现代化的 Enum+Builder 端点系统，替代传统的字符串拼接方式，提供类型安全和 IDE 友好性。

### 系统架构

#### 1. Enum 端点定义

所有 API 端点通过 Enum 定义，提供类型安全的 URL 生成：

```rust
// 基础模式：单参数
#[derive(Debug, Clone, PartialEq)]
pub enum BaseApiV2 {
    RoleCreate(String),              // 创建角色
    RoleUpdate(String, String),     // 更新角色
    RoleList(String),               // 角色列表
}

impl BaseApiV2 {
    pub fn to_url(&self) -> String {
        match self {
            BaseApiV2::RoleCreate(app_token) => {
                format!("/open-apis/base/v2/apps/{}/roles", app_token)
            }
            BaseApiV2::RoleUpdate(app_token, role_id) => {
                format!("/open-apis/base/v2/apps/{}/roles/{}", app_token, role_id)
            }
            BaseApiV2::RoleList(app_token) => {
                format!("/open-apis/base/v2/apps/{}/roles", app_token)
            }
        }
    }
}
```

#### 2. 复杂嵌套模式

支持深度资源路径（最多 4 层嵌套）：

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum BitableApiV1 {
    // App 管理
    AppCreate,
    AppGet(String),
    AppUpdate(String),
    
    // 表格管理
    TableCreate(String, String),         // app_token, table_id
    TableUpdate(String, String),         // app_token, table_id
    TableDelete(String, String),
    
    // 字段管理
    FieldCreate(String, String, String),    // app_token, table_id, field_id
    FieldUpdate(String, String, String),  // app_token, table_id, field_id
    
    // 记录管理
    RecordCreate(String, String),         // app_token, table_id
    RecordGet(String, String, String),     // app_token, table_id, record_id
    RecordBatchCreate(String, String),  // app_token, table_id
    
    // 权限管理
    FormGet(String, String, String),         // app_token, table_id, form_id
    FormFieldPatch(String, String, String, String), // app_token, table_id, form_id, field_id
}

impl BitableApiV1 {
    pub fn to_url(&self) -> String {
        match self {
            // 零参数
            BitableApiV1::AppCreate => "/open-apis/bitable/v1/apps".to_string(),
            
            // 单参数
            BitableApiV1::AppGet(app_token) => {
                format!("/open-apis/bitable/v1/apps/{}", app_token)
            }
            
            // 双参数
            BitableApiV1::TableCreate(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}", app_token, table_id)
            }
            
            // 三参数（最深嵌套）
            BitableApiV1::RecordGet(app_token, table_id, record_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/records/{}",
                    app_token, table_id, record_id
                )
            }
            
            // 四参数（特殊场景）
            BitableApiV1::FormFieldPatch(app_token, table_id, form_id, field_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/fields/{}",
                    app_token, table_id, form_id, field_id
                )
            }
        }
    }
}
```

#### 3. 特殊操作模式

支持智能操作、搜索、批量操作等特殊模式：

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum SheetsApiV3 {
    // 筛选视图管理
    CreateFilterView(String, String),           // (spreadsheet_token, sheet_id)
    QueryFilterViews(String, String),           // (spreadsheet_token, sheet_id)
    GetFilterView(String, String, String),      // (spreadsheet_token, sheet_id, filter_view_id)
    PatchFilterView(String, String, String),      // (spreadsheet_token, sheet_id, filter_view_id)
    DeleteFilterView(String, String, String),   // (spreadsheet_token, sheet_id, filter_view_id)
    
    // 筛选条件管理
    CreateFilterCondition(String, String, String),    // (spreadsheet_token, sheet_id, filter_view_id)
    QueryFilterConditions(String, String, String),    // (spreadsheet_token, sheet_id, filter_view_id)
    GetFilterCondition(String, String, String, String), // (spreadsheet_token, sheet_id, filter_view_id, condition_id)
    UpdateFilterCondition(String, String, String, String), // (spreadsheet_token, sheet_id, filter_view_id, condition_id)
    DeleteFilterCondition(String, String, String, String), // (spreadsheet_token, sheet_id, filter_view_id, condition_id)
}

impl SheetsApiV3 {
    pub fn to_url(&self) -> String {
        match self {
            SheetsApiV3::CreateSpreadsheet => "/open-apis/sheets/v3/spreadsheets".to_string(),
            SheetsApiV3::GetSpreadsheet(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}", spreadsheet_token)
            }
            SheetsApiV3::CreateFilterView(spreadsheet_token, sheet_id) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filterViews",
                    spreadsheet_token, sheet_id
                )
            }
            SheetsApiV3::GetFilterCondition(spreadsheet_token, sheet_id, filter_view_id, condition_id)
            | SheetsApiV3::UpdateFilterCondition(spreadsheet_token, sheet_id, filter_view_id, condition_id)
            | SheetsApiV3::DeleteFilterCondition(spreadsheet_token, sheet_id, filter_view_id, condition_id) => {
                format!(
                    "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filterViews/{}/conditions/{}",
                    spreadsheet_token, sheet_id, filter_view_id, condition_id
                )
            }
        }
    }
}
```

### Builder 模式实现

#### 1. RequestBuilder 结构体

所有 Builder 遵循统一的结构体定义：

```rust
#[derive(Debug, Clone, Default)]
pub struct CreateRecordRequestBuilder {
    request: CreateRecordRequest,
}

impl CreateRecordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: CreateRecordRequest::new(config),
        }
    }

    // 设置单个值
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request.app_token = app_token;
        self
    }

    pub fn table_id(mut self, table_id: String) -> Self {
        self.request.table_id = table_id;
        self
    }

    // 添加到集合
    pub fn add_tag(mut self, tag: String) -> Self {
        self.request.tags.push(tag);
        self
    }

    pub fn add_tags(mut self, tags: Vec<String>) -> Self {
        self.request.tags.extend(tags);
        self
    }

    // 添加到 Map
    pub fn add_custom_field(mut self, key: String, value: serde_json::Value) -> Self {
        self.request.custom_fields
            .get_or_insert_with(HashMap::new())
            .insert(key, value);
        self
    }

    // 构建请求
    pub fn build(self) -> CreateRecordRequest {
        self.request
    }
}
```

#### 2. 使用示例

```rust
use openlark_docs::bitable::v1::app::table::record::*;
use openlark_docs::bitable::v1::app::table::record::end_points::*;

// 链式调用
let request = CreateRecordRequestBuilder::new(config)
    .app_token("app_token")
    .table_id("table_id")
    .user_id_type("open_id")
    .client_token("request_token")
    .ignore_consistency_check(true)
    .fields(json!({
        "姓名": "张三",
        "邮箱": "zhangsan@example.com",
        "电话": "13800138000",
        "入职日期": "2024-01-15"
    }))
    .build();

let response = request.execute().await?;
println!("记录ID: {}", response.record.record_id);
```

### bizTag 目录层级说明

#### 1. bizTag 作为目录层级

当 `bizTag` 与 `feature-crate` 名称不同时（如 `base` → `openlark-docs`），保留 bizTag 作为目录层级：

```
实际路径 = crates/{feature-crate}/src/{bizTag}/{meta.Project}/{meta.Version}/{meta.Resource}/{name}.rs
```

#### 2. 完整路径映射表

| CSV 字段组合 | 文件路径示例 | 说明 |
|--------------|--------------|------|
| bizTag=base, meta.Project=bitable | `crates/openlark-docs/src/base/bitable/v1/app/table/record/create.rs` | base 目录下的 bitable 模块 |
| bizTag=contact, meta.Project=contact | `crates/openlark-communication/src/contact/contact/v3/user/create.rs` | contact 目录下的 contact 模块 |
| bizTag=vc, meta.Project=vc | `crates/openlark-meeting/src/vc/v1/room_level/list.rs` | vc 自成目录 |
| bizTag=ccm, meta.Project=drive | `crates/openlark-docs/src/ccm/drive/v1/file/list.rs` | ccm 目录下的 drive 模块 |

#### 3. bizTag 目录层级示例

```
crates/openlark-docs/src/
├── base/
│   └── bitable/v1/
│       ├── app/
│       │   └── table/
│       │       ├── field/
│       │       ├── record/
│       │       └── view/
│       └── wiki/v2/
└── ccm/
    ├── drive/v1/
    │   ├── file/
    │   ├── import_task/
    │   └── export_task/
    ├── docs/v1/
    └── sheets/v3/
```

### Enum 命名规范

#### 1. 基础模式

```
{Project}Api{Version}
```

示例：
- `BaseApiV2` - 基础服务 v2
- `BitableApiV1` - 多维表格 v1
- `ContactApiV3` - 通讯录 v3
- `ImApiV1` - 即时消息 v1

#### 2. 复杂模式

```
{Project}{Resource}Api{Version}
```

示例：
- `SheetsApiV3` - 电子表格 v3（包含多个资源）
- `BaikeApiV1` - 百科词条 v1
- `LingoApiV1` - 词典系统 v1

#### 3. 特殊模式

```
{Project}{SpecialModule}Api{Version}
```

示例：
- `CcmDriveExplorerApi` - CCM 云盘探索器
- `AISearchApiV1` - AI 搜索服务

### 端点使用对比

#### 传统模式（不推荐）

```rust
// ❌ 字符串拼接，易出错
let url = format!("/open-apis/bitable/v1/apps/{}/tables/{}", 
                  app_token, table_id);

// ❌ 无类型安全，拼写错误编译期无法发现
let url = "/open-apis/bitable/v1/appps/{}/tables/{}";  // appps 拼写错误
```

#### Enum 模式（推荐）

```rust
// ✅ 类型安全，编译时检查
use crate::common::api_endpoints::BitableApiV1;

let api_endpoint = BitableApiV1::TableCreate(app_token, table_id);
let url = api_endpoint.to_url();

// ✅ IDE 自动补全和跳转
BitableApiV1::  // IDE 自动提示所有可用变体
```

### API 配置数据源

所有 API 的详细配置信息存储在 `api_list_export.csv` 文件中（包含 1687+ 个 API）。

### CSV 文件结构

| 列名 | 说明 | 示例 |
|------|------|------|
| `id` | API 唯一标识 | `7083805115594227714` |
| `name` | API 中文名称 | `获取用户信息` |
| `bizTag` | 业务领域标签 | `auth`, `contact`, `im`, `docs`, `hr` |
| `meta.Project` | 项目标识 | `auth`, `contact`, `im` |
| `meta.Version` | API 版本 | `v1`, `v2`, `v3` |
| `meta.Resource` | 资源名称 | `user`, `message`, `department` |
| `meta.Name` | 操作名称 | `get`, `create`, `list`, `delete` |
| `detail` | API 详细描述 | 功能说明文档 |
| `url` | API 端点 URL | `GET:/open-apis/contact/v3/scopes` |
| `chargingMethod` | 计费方式 | `none`, `basic` |
| `isCharge` | 是否收费 | `true`, `false` |
| `supportAppTypes` | 支持的应用类型 | `["isv", "custom"]` |
| `docPath` | 官方文档链接 | `https://open.feishu.cn/...` |

### 使用 CSV 配置实现 API

**步骤 1**: 在 CSV 中查找目标 API
```bash
# 查找特定 API
grep "获取用户信息" api_list_export.csv

# 查找某个业务模块的所有 API
grep ",contact," api_list_export.csv | cut -d',' -f2

# 查找特定资源的所有 API
grep ",message," api_list_export.csv | grep "im,"
```

**步骤 2**: 提取 API 配置信息
```csv
id,name,bizTag,meta.Project,meta.Version,meta.Resource,meta.Name,url
7180265937329537028,获取用户信息,auth,authen,v1,user_info,get,GET:/open-apis/authen/v1/user_info
```

**步骤 3**: 根据 CSV 配置确定文件路径
- bizTag → `src/{bizTag}/`
- meta.Version → `open-apis/{meta.Version}/`
- meta.Resource → `{meta.Resource}/`
- meta.Name → `{meta.Name}.rs`

示例：`src/authen/open-apis/v1/user_info/get.rs`

**步骤 4**: 根据 `url` 字段实现 ApiRequest trait
```rust
fn path(&self) -> &'static str {
    "/open-apis/authen/v1/user_info"  // 从 url: GET:/open-apis/authen/v1/user_info 提取
}

fn method(&self) -> RequestMethod {
    RequestMethod::Get  // 从 url: GET:... 提取
}
```

**步骤 5**: 参考 `docPath` 链接查看官方文档，了解请求/响应字段

### CSV 配置到代码的映射关系

| CSV 字段 | 代码实现 |
|----------|----------|
| `bizTag` | Feature crate 名称（如 `openlark-communication`） |
| `meta.Project` | 对应 bizTag 或子模块标识 |
| `meta.Version` | API 版本目录（`v1`, `v2`, `v3`, `old`） |
| `meta.Resource` | 资源目录名 |
| `meta.Name` | 文件名 |
| `url` | `path()` 方法返回值（提取路径部分） |
| `url` 中的方法 | `method()` 方法返回值（GET/POST/PUT/DELETE/PATCH） |
| `detail` | 文档注释和端点描述 |
| `docPath` | 详细字段说明链接 |

### bizTag 目录层级说明

#### 1. bizTag 作为目录层级

当 `bizTag` 与 `feature-crate` 名称不同时（如 `base` → `openlark-docs`），保留 bizTag 作为目录层级：

```
实际文件路径 = crates/{feature-crate}/src/{bizTag}/{meta.Project}/{meta.Version}/{meta.Resource}/{name}.rs
```

#### 2. 完整映射示例

**示例 1: bizTag = base（与 feature-crate 不同）**

```csv
bizTag: base
meta.Project: bitable
meta.Version: v1
meta.Resource: app.table.record
meta.Name: create
```

```
实际文件路径: crates/openlark-docs/src/base/bitable/v1/app/table/record/create.rs
```

**示例 2: bizTag = vc（与 feature-crate 相同）**

```csv
bizTag: vc
meta.Project: vc
meta.Version: v1
meta.Resource: room_level
meta.Name: list
```

```
实际文件路径: crates/openlark-meeting/src/vc/v1/room_level/list.rs
```

#### 3. 主要 bizTag 目录层级

| bizTag | Feature Crate | 目录结构说明 |
|--------|--------------|----------------|
| **base** | openlark-docs | base/ → bitable（多维表格） |
| **ccm** | openlark-docs | ccm/ → drive（云盘）、docs（文档）、sheets（表格）、wiki（知识库） |
| **contact** | openlark-communication | contact/ → 用户、部门、用户组管理 |
| **im** | openlark-communication | im/ → 消息、群聊、文件、图片 |
| **vc** | openlark-meeting | vc/ → 视频会议、会议室、预约管理 |
| **hire** | openlark-hr | hire/ → 招聘、候选人、面试、Offer |
| **ai** | openlark-ai | ai/ → OCR、翻译、语音识别 |
| **auth** | openlark-auth | auth/ → 认证、令牌管理 |

#### 4. 目录结构示例

```
crates/openlark-docs/src/
├── base/
│   └── bitable/v1/
│       ├── app/
│       │   ├── table/
│       │   └── record/
│       └── wiki/v2/
├── ccm/
│   ├── drive/v1/
│   ├── docs/v1/
│   └── sheets/v3/

crates/openlark-communication/src/
├── contact/v3/
├── im/v1/
└── contact_search/
```

### CSV 到完整文件路径的转换示例

```bash
# CSV 信息
bizTag: communication
meta.Version: v1
meta.Resource: message
meta.Name: send

# 转换为文件路径
crates/openlark-communication/src/v1/message/send.rs
```

```bash
# CSV 信息
bizTag: hr
meta.Version: v1
meta.Resource: candidate
meta.Name: get

# 转换为文件路径
crates/openlark-hr/src/v1/candidate/get.rs
```

## 参考资源

- **主架构文档**: `ARCHITECTURE.md`
- **API 配置清单**: `api_list_export.csv`（所有 1687+ 个 API 的详细配置）
- **核心模块**: `openlark-core/src/api/`
- **客户端模块**: `openlark-client/src/`
- **错误处理**: `openlark-core/src/error/`
- **请求构建**: `openlark-core/src/request_builder/`

## 常见场景

### 文件上传

```rust
impl ApiRequest for FileUploadRequest {
    fn method(&self) -> RequestMethod {
        RequestMethod::Post
    }

    fn path(&self) -> &'static str {
        "/open-apis/drive/v1/files/upload_all"
    }

    fn files(&self) -> &[FileInfo] {
        &self.files
    }
}
```

### 分页请求

```rust
pub struct ListRequest {
    pub page_size: Option<u32>,
    pub page_token: Option<String>,
}
```

### 批量操作

```rust
pub struct BatchDeleteRequest {
    #[serde(rename = "request_ids")]
    pub request_ids: Vec<String>,
}
```
