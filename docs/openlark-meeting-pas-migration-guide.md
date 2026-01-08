# openlark-meeting Crate Pas 规范迁移指南

> **目标**: 将 `openlark-meeting` crate 从 Request-Object Builder 模式重构为符合 Pas 规范的纯结构体序列化模式

---

## 📋 目录

- [1. Pas 规范要求](#1-pas-规范要求)
- [2. 当前架构分析](#2-当前架构分析)
- [3. 迁移规范](#3-迁移规范)
- [4. 详细重构步骤](#4-详细重构步骤)
- [5. 代码示例对比](#5-代码示例对比)
- [6. 检查清单](#6-检查清单)
- [7. 迁移执行计划](#7-迁移执行计划)

---

## 1. Pas 规范要求

### 1.1 请求与响应模型 (Request & Response Models)

#### 请求体 (Request)

```rust
/// 定义 struct 并派生必要的 traits
#[derive(Debug, Clone, Serialize)]
pub struct CreateRoomRequest {
    pub room_name: String,
    pub capacity: u32,
    pub building_id: Option<String>,
    pub floor: Option<String>,
    pub description: Option<String>,
    pub devices: Option<Vec<Device>>,
    pub status: Option<String>,
    pub room_level_id: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Device {
    pub device_id: String,
    pub device_name: String,
    pub device_type: String,
}
```

**要求清单:**
- ✅ 派生 `Debug`
- ✅ 派生 `Clone`
- ✅ 派生 `Serialize` (用于请求序列化)
- ✅ 包含 API 所需的所有字段
- ✅ 使用 `Option<T>` 表示可选字段
- ✅ 字段名称使用 snake_case

#### 响应体 (Response)

```rust
/// 定义 struct 并派生必要的 traits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoomResponse {
    pub room_id: String,
}

/// 实现 ApiResponseTrait 指定 ResponseFormat
impl ApiResponseTrait for CreateRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data  // 指示数据在 "data" 字段中
    }
}
```

**要求清单:**
- ✅ 派生 `Debug`
- ✅ 派生 `Clone`
- ✅ 派生 `Serialize`
- ✅ 派生 `Deserialize` (用于响应反序列化)
- ✅ 实现 `ApiResponseTrait`
- ✅ `data_format()` 返回 `ResponseFormat::Data`
- ✅ 字段使用 `pub` 公开访问

### 1.2 操作实现模式 (Function Implementation)

#### 标准函数签名

```rust
pub async fn create_room(
    config: &Config,
    params: CreateRoomRequest,
) -> SDKResult<CreateRoomResponse>
```

**要求清单:**
- ✅ 使用 `pub async fn` 声明
- ✅ 第一个参数为 `&Config`
- ✅ 路径参数作为独立参数传递（如 `meeting_id: &str`）
- ✅ 请求体参数命名为 `params: RequestType`
- ✅ 返回类型为 `SDKResult<ResponseType>`

#### 标准实现流程

```rust
use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};
use crate::{endpoints::VC_V1_ROOMS, common::api_utils::extract_response_data, serialize_params};

pub async fn create_room(
    config: &Config,
    params: CreateRoomRequest,
) -> SDKResult<CreateRoomResponse> {
    // 1. 构建端点: 实例化对应的 Endpoint 枚举
    let endpoint = VC_V1_ROOMS;  // 或使用 Endpoint 枚举

    // 2. 构建请求: 使用 ApiRequest::post/get 方法
    let req: ApiRequest<serde_json::Value> =
        ApiRequest::post(endpoint.to_url())
            .body(serialize_params(&params, "创建会议室")?);

    // 3. 序列化: 使用 serialize_params 辅助函数
    //    (已在第2步中使用)

    // 4. 发送请求: 调用 Transport::request 发送 HTTP 请求
    let resp = Transport::request(req, config, None).await?;

    // 5. 提取响应: 调用 extract_response_data 处理错误并解包数据
    extract_response_data(resp, "创建会议室")
}
```

**要求清单:**
- ✅ 实例化 Endpoint 枚举（或使用端点常量）
- ✅ 使用 `ApiRequest::post` 或 `ApiRequest::get` 构建请求
- ✅ 使用 `serialize_params` 序列化请求参数
- ✅ 调用 `Transport::request` 发送请求
- ✅ 使用 `extract_response_data` 提取响应数据
- ✅ 传递描述性上下文字符串（如 "创建会议室"）

---

## 2. 当前架构分析

### 2.1 Request-Object Builder 模式

#### 当前实现示例

```rust
// ❌ 当前实现 - vc/v1/room/create.rs
pub struct CreateRoomRequest {
    config: Config,              // 包含 config 实例
    room_name: Option<String>,
    capacity: Option<u32>,
    building_id: Option<String>,
    // ...
}

impl CreateRoomRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            room_name: None,
            capacity: None,
            // ...
        }
    }

    pub fn room_name(mut self, value: impl Into<String>) -> Self {
        self.room_name = Some(value.into());
        self
    }

    pub fn capacity(mut self, value: u32) -> Self {
        self.capacity = Some(value);
        self
    }

    // 手动构建请求体
    pub async fn execute(self) -> SDKResult<CreateRoomResponse> {
        let room_name = self.room_name.ok_or_else(|| {
            openlark_core::error::validation_error("room_name", "room_name 不能为空")
        })?;

        let mut body = serde_json::Map::new();
        body.insert("room_name".to_string(), serde_json::json!(room_name));
        // 手动插入每个字段...

        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(VC_V1_ROOMS).body(serde_json::Value::Object(body));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建会议室")
    }
}
```

#### 使用方式对比

```rust
// ❌ 当前 Builder 模式调用方式
let room = CreateRoomRequest::new(config)
    .room_name("会议室A")
    .capacity(20)
    .building_id("building_123")
    .execute()
    .await?;

// ✅ Pas 规范调用方式
let params = CreateRoomRequest {
    room_name: "会议室A".to_string(),
    capacity: 20,
    building_id: Some("building_123".to_string()),
    floor: None,
    description: None,
    devices: None,
    status: None,
    room_level_id: None,
    tags: None,
};

let room = create_room(&config, params).await?;
```

### 2.2 架构差异总结

| 特性 | Pas 规范 | 当前实现 | 差异影响 |
|-----|---------|---------|---------|
| **Request 派生** | `Serialize` | 无派生 | 无法自动序列化 |
| **Request 字段** | `pub` 访问 | `config` 字段私有 | 需要重构字段定义 |
| **函数签名** | `fn(&Config, params: Request)` | `fn(self)` (builder) | 完全不同的调用方式 |
| **序列化方式** | `serialize_params` | 手动构建 `serde_json::Map` | 代码重复且易错 |
| **Endpoint** | Endpoint 枚举 | 常量字符串 | 缺少类型安全 |
| **用户调用** | 直接构造结构体 | 链式 Builder | 用户体验变化 |

---

## 3. 迁移规范

### 3.1 迁移原则

1. **向后兼容性**: 优先考虑现有用户，提供过渡期
2. **渐进式迁移**: 逐模块迁移，避免一次性大规模重构
3. **充分测试**: 每个模块迁移后立即测试
4. **文档更新**: 同步更新示例和文档

### 3.2 迁移策略

#### 阶段 1: 准备阶段
- 创建 Endpoint 枚举定义
- 为所有 Response 补全 `ApiResponseTrait` 实现
- 准备迁移检查清单

#### 阶段 2: 试点迁移
- 选择一个简单的 API 进行试点（如 `vc/v1/room/create`）
- 验证迁移流程
- 收集问题和改进点

#### 阶段 3: 批量迁移
- 按模块顺序迁移（vc → calendar → meeting_room）
- 每个模块迁移后进行测试
- 修复发现的问题

#### 阶段 4: 清理阶段
- 删除旧的 Builder 实现
- 更新所有文档和示例
- 发布新版本

### 3.3 兼容性处理

为了保持向后兼容，建议在过渡期提供双模式支持：

```rust
// ✅ 保留 Builder 模式（标记为 deprecated）
#[deprecated(since = "0.16.0", note = "请使用 create_room 函数")]
pub struct CreateRoomBuilder {
    config: Config,
    room_name: Option<String>,
    // ...
}

impl CreateRoomBuilder {
    pub fn new(config: Config) -> Self { /* ... */ }

    pub async fn execute(self) -> SDKResult<CreateRoomResponse> {
        // 内部调用新的 Pas 规范函数
        let params = CreateRoomRequest {
            room_name: self.room_name.ok_or(/* ... */)?,
            // ...
        };
        create_room(&self.config, params).await
    }
}

// ✅ 新增 Pas 规范函数
pub async fn create_room(
    config: &Config,
    params: CreateRoomRequest,
) -> SDKResult<CreateRoomResponse> {
    // 标准实现...
}
```

---

## 4. 详细重构步骤

### 4.1 步骤 1: 创建 Endpoint 枚举

**文件**: `crates/openlark-meeting/src/endpoints/mod.rs`

```rust
/// Endpoint 枚举定义
#[derive(Debug, Clone)]
pub enum VcV1Endpoint {
    CreateRoom,
    GetRoom { room_id: String },
    ListRoom,
    DeleteRoom { room_id: String },
    CreateReserve,
    GetMeeting { meeting_id: String },
    ListByNo { meeting_no: String },
    // ... 添加所有 vc/v1 端点
}

impl VcV1Endpoint {
    pub fn to_url(&self) -> String {
        match self {
            Self::CreateRoom => "/open-apis/vc/v1/rooms".to_string(),
            Self::GetRoom { room_id } => format!("/open-apis/vc/v1/rooms/{}", room_id),
            Self::ListRoom => "/open-apis/vc/v1/rooms".to_string(),
            Self::DeleteRoom { room_id } => format!("/open-apis/vc/v1/rooms/{}", room_id),
            Self::CreateReserve => "/open-apis/vc/v1/reserves".to_string(),
            Self::GetMeeting { meeting_id } => format!("/open-apis/vc/v1/meetings/{}", meeting_id),
            Self::ListByNo { meeting_no } => format!("/open-apis/vc/v1/meetings/by_no/{}", meeting_no),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CalendarV4Endpoint {
    CreateCalendar,
    GetCalendar { calendar_id: String },
    ListCalendar,
    DeleteCalendar { calendar_id: String },
    // ... 添加所有 calendar/v4 端点
}

impl CalendarV4Endpoint {
    pub fn to_url(&self) -> String {
        match self {
            Self::CreateCalendar => "/open-apis/calendar/v4/calendars".to_string(),
            Self::GetCalendar { calendar_id } => format!("/open-apis/calendar/v4/calendars/{}", calendar_id),
            Self::ListCalendar => "/open-apis/calendar/v4/calendars".to_string(),
            Self::DeleteCalendar { calendar_id } => format!("/open-apis/calendar/v4/calendars/{}", calendar_id),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MeetingRoomEndpoint {
    CreateRoom,
    GetRoom { room_id: String },
    ListRoom,
    // ... 添加所有 meeting_room 端点
}

impl MeetingRoomEndpoint {
    pub fn to_url(&self) -> String {
        match self {
            Self::CreateRoom => "/open-apis/meeting_room/room/create".to_string(),
            Self::GetRoom { room_id } => format!("/open-apis/meeting_room/room/get/{}", room_id),
            Self::ListRoom => "/open-apis/meeting_room/room/list".to_string(),
        }
    }
}
```

### 4.2 步骤 2: 重构 Request 结构体

**文件**: `crates/openlark-meeting/src/vc/v1/room/create.rs`

```rust
//! 创建会议室
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/create

use serde::Serialize;

/// 创建会议室请求
#[derive(Debug, Clone, Serialize)]
pub struct CreateRoomRequest {
    /// 会议室名称（必填）
    pub room_name: String,

    /// 容量（必填）
    pub capacity: u32,

    /// 建筑ID（可选）
    pub building_id: Option<String>,

    /// 楼层（可选）
    pub floor: Option<String>,

    /// 描述（可选）
    pub description: Option<String>,

    /// 设备列表（可选）
    pub devices: Option<Vec<Device>>,

    /// 状态（可选）
    pub status: Option<String>,

    /// 会议室层级ID（可选）
    pub room_level_id: Option<String>,

    /// 标签（可选）
    pub tags: Option<Vec<String>>,
}

/// 设备信息
#[derive(Debug, Clone, Serialize)]
pub struct Device {
    pub device_id: String,
    pub device_name: String,
    pub device_type: String,
}
```

### 4.3 步骤 3: 补全 Response 的 ApiResponseTrait

**文件**: `crates/openlark-meeting/src/vc/v1/responses.rs`

```rust
//! 会议相关响应结构
//!
//! 定义视频会议 API 的响应数据类型。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 创建会议室响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoomResponse {
    pub room_id: String,
}

impl ApiResponseTrait for CreateRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取会议室响应（补全实现）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoomResponse {
    pub room_id: String,
    pub room_name: String,
    pub capacity: u32,
    pub devices: Option<Vec<DeviceInfo>>,
    pub floor: Option<String>,
    pub description: Option<String>,
}

// ✅ 新增 ApiResponseTrait 实现
impl ApiResponseTrait for GetRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取会议响应（补全实现）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMeetingResponse {
    pub meeting_id: String,
    pub topic: String,
    pub start_time: String,
    pub end_time: String,
    pub status: String,
    pub creator: UserInfo,
}

// ✅ 新增 ApiResponseTrait 实现
impl ApiResponseTrait for GetMeetingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// ... 其他 Response 结构体保持不变
```

### 4.4 步骤 4: 重构为标准函数实现

**文件**: `crates/openlark-meeting/src/vc/v1/room/create.rs` (继续)

```rust
use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};
use crate::{
    endpoints::VcV1Endpoint,
    common::api_utils::{extract_response_data, serialize_params},
};
use super::responses::CreateRoomRequest;  // Response 在 responses.rs 中定义

/// 创建会议室
///
/// # 参数
///
/// - `config`: 配置对象
/// - `params`: 创建会议室请求参数
///
/// # 返回
///
/// 返回创建成功的会议室 ID
///
/// # 示例
///
/// ```rust,no_run
/// use openlark_meeting::vc::v1::room::{create_room, CreateRoomRequest};
///
/// let params = CreateRoomRequest {
///     room_name: "会议室A".to_string(),
///     capacity: 20,
///     building_id: Some("building_123".to_string()),
///     floor: None,
///     description: None,
///     devices: None,
///     status: None,
///     room_level_id: None,
///     tags: None,
/// };
///
/// let response = create_room(&config, params).await?;
/// println!("创建成功: {}", response.room_id);
/// ```
///
/// # 文档
///
/// docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/create
pub async fn create_room(
    config: &Config,
    params: CreateRoomRequest,
) -> SDKResult<CreateRoomResponse> {
    // 1. 实例化 Endpoint
    let endpoint = VcV1Endpoint::CreateRoom;

    // 2. 构建请求并序列化参数
    let req: ApiRequest<serde_json::Value> =
        ApiRequest::post(endpoint.to_url())
            .body(serialize_params(&params, "创建会议室")?);

    // 3. 发送请求
    let resp = Transport::request(req, config, None).await?;

    // 4. 提取响应
    extract_response_data(resp, "创建会议室")
}
```

### 4.5 步骤 5: 处理带路径参数的 API

**文件**: `crates/openlark-meeting/src/vc/v1/room/get.rs`

```rust
//! 查询会议室详情
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/get

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};
use crate::{
    endpoints::VcV1Endpoint,
    common::api_utils::extract_response_data,
};
use super::responses::{GetRoomRequest, GetRoomResponse};

/// 查询会议室详情
///
/// # 参数
///
/// - `config`: 配置对象
/// - `room_id`: 会议室 ID（路径参数）
/// - `params`: 查询请求参数（查询参数）
///
/// # 返回
///
/// 返回会议室详细信息
///
/// # 示例
///
/// ```rust,no_run
/// use openlark_meeting::vc::v1::room::{get_room, GetRoomRequest};
///
/// let params = GetRoomRequest {
///     user_id_type: Some("open_id".to_string()),
///     user_id: Some("user_123".to_string()),
/// };
///
/// let response = get_room(&config, "room_123", params).await?;
/// println!("会议室名称: {}", response.room_name);
/// ```
///
/// # 文档
///
/// docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/get
pub async fn get_room(
    config: &Config,
    room_id: &str,
    params: GetRoomRequest,
) -> SDKResult<GetRoomResponse> {
    // 1. 实例化 Endpoint（带路径参数）
    let endpoint = VcV1Endpoint::GetRoom {
        room_id: room_id.to_string(),
    };

    // 2. 构建请求
    let mut req: ApiRequest<serde_json::Value> =
        ApiRequest::get(endpoint.to_url());

    // 3. 添加查询参数
    if let Some(user_id_type) = &params.user_id_type {
        req = req.query("user_id_type", user_id_type);
    }
    if let Some(user_id) = &params.user_id {
        req = req.query("user_id", user_id);
    }

    // 4. 发送请求
    let resp = Transport::request(req, config, None).await?;

    // 5. 提取响应
    extract_response_data(resp, "查询会议室详情")
}
```

---

## 5. 代码示例对比

### 5.1 完整重构示例: 创建会议室

#### ❌ 重构前 (Builder 模式)

```rust
// vc/v1/room/create.rs
pub struct CreateRoomRequest {
    config: Config,
    room_name: Option<String>,
    capacity: Option<u32>,
    building_id: Option<String>,
    // ...
}

impl CreateRoomRequest {
    pub fn new(config: Config) -> Self { /* ... */ }

    pub fn room_name(mut self, value: impl Into<String>) -> Self {
        self.room_name = Some(value.into());
        self
    }

    pub fn capacity(mut self, value: u32) -> Self {
        self.capacity = Some(value);
        self
    }

    pub async fn execute(self) -> SDKResult<CreateRoomResponse> {
        let room_name = self.room_name.ok_or(/* ... */)?;
        let capacity = self.capacity.ok_or(/* ... */)?;

        let mut body = serde_json::Map::new();
        body.insert("room_name".to_string(), serde_json::json!(room_name));
        body.insert("capacity".to_string(), serde_json::json!(capacity));
        // ... 手动插入所有字段

        let req = ApiRequest::post(VC_V1_ROOMS).body(serde_json::Value::Object(body));
        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建会议室")
    }
}

// 使用方式
let room = CreateRoomRequest::new(config)
    .room_name("会议室A")
    .capacity(20)
    .execute()
    .await?;
```

#### ✅ 重构后 (Pas 规范)

```rust
// vc/v1/room/create.rs
#[derive(Debug, Clone, Serialize)]
pub struct CreateRoomRequest {
    pub room_name: String,
    pub capacity: u32,
    pub building_id: Option<String>,
    pub floor: Option<String>,
    pub description: Option<String>,
    pub devices: Option<Vec<Device>>,
    pub status: Option<String>,
    pub room_level_id: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub async fn create_room(
    config: &Config,
    params: CreateRoomRequest,
) -> SDKResult<CreateRoomResponse> {
    let endpoint = VcV1Endpoint::CreateRoom;
    let req = ApiRequest::post(endpoint.to_url())
        .body(serialize_params(&params, "创建会议室")?);
    let resp = Transport::request(req, config, None).await?;
    extract_response_data(resp, "创建会议室")
}

// 使用方式
let params = CreateRoomRequest {
    room_name: "会议室A".to_string(),
    capacity: 20,
    building_id: Some("building_123".to_string()),
    floor: None,
    description: None,
    devices: None,
    status: None,
    room_level_id: None,
    tags: None,
};

let room = create_room(&config, params).await?;
```

### 5.2 代码减少量统计

| 指标 | 重构前 | 重构后 | 减少量 |
|-----|-------|-------|--------|
| **Request 结构体行数** | ~70 行 (含 builder) | ~25 行 | **-64%** |
| **函数实现行数** | ~50 行 (手动序列化) | ~15 行 | **-70%** |
| **总代码量** | ~120 行 | ~40 行 | **-67%** |
| **可维护性** | 低（重复代码） | 高（统一模式） | **大幅提升** |

---

## 6. 检查清单

### 6.1 Request 结构体检查清单

- [ ] 派生 `#[derive(Debug, Clone, Serialize)]`
- [ ] 移除 `config` 字段
- [ ] 所有字段使用 `pub` 公开访问
- [ ] 必填字段不使用 `Option<T>`
- [ ] 可选字段使用 `Option<T>`
- [ ] 字段命名使用 snake_case
- [ ] 移除所有 Builder 方法 (`fn xxx(mut self) -> Self`)
- [ ] 移除 `new()` 构造函数（用户直接构造结构体）
- [ ] 移除 `execute()` 方法

### 6.2 Response 结构体检查清单

- [ ] 派生 `#[derive(Debug, Clone, Serialize, Deserialize)]`
- [ ] 实现 `ApiResponseTrait` trait
- [ ] `data_format()` 返回 `ResponseFormat::Data`
- [ ] 所有字段使用 `pub` 公开访问
- [ ] 字段命名使用 snake_case

### 6.3 函数实现检查清单

- [ ] 函数签名符合规范：`pub async fn function_name(config: &Config, params: RequestType) -> SDKResult<ResponseType>`
- [ ] 路径参数作为独立函数参数
- [ ] 使用 Endpoint 枚举或常量
- [ ] 使用 `ApiRequest::post` 或 `ApiRequest::get` 构建请求
- [ ] 使用 `serialize_params` 序列化请求参数
- [ ] 调用 `Transport::request` 发送请求
- [ ] 使用 `extract_response_data` 提取响应
- [ ] 包含完整的文档注释
- [ ] 包含示例代码
- [ ] 包含文档链接 (docPath)

### 6.4 端点定义检查清单

- [ ] Endpoint 枚举派生 `Debug, Clone`
- [ ] `to_url()` 方法正确实现
- [ ] 路径参数在 `to_url()` 中正确拼接
- [ ] 所有端点都已定义

### 6.5 测试检查清单

- [ ] 每个新函数都有对应的单元测试
- [ ] 测试覆盖正常情况
- [ ] 测试覆盖错误情况
- [ ] 集成测试验证完整流程
- [ ] 文档测试 (doctest) 通过

---

## 7. 迁移执行计划

### 7.1 迁移优先级

#### 高优先级 (P0)
- ✅ 创建 Endpoint 枚举定义
- ✅ 补全所有 Response 的 `ApiResponseTrait` 实现
- ✅ 创建迁移检查清单

#### 中优先级 (P1)
- 🔄 `vc/v1/room/` 模块（相对简单，适合试点）
- 🔄 `vc/v1/reserve/` 模块（核心功能）
- 🔄 `vc/v1/meeting/` 模块（核心功能）

#### 低优先级 (P2)
- 📅 `calendar/v4/` 模块
- 📅 `meeting_room/` 模块（历史版本）

### 7.2 迁移时间表

| 阶段 | 任务 | 预计工时 | 里程碑 |
|-----|------|---------|--------|
| **准备阶段** | | 4h | |
| - | 创建 Endpoint 枚举 | 2h | Endpoint 枚举完成 |
| - | 补全 Response ApiTrait | 1h | 所有 Response 实现完整 |
| - | 创建检查清单 | 0.5h | 检查清单完成 |
| - | 准备测试环境 | 0.5h | 测试环境就绪 |
| **试点阶段** | | 6h | |
| - | 迁移 vc/v1/room/create | 2h | 试点完成 |
| - | 编写测试 | 2h | 测试通过 |
| - | 验证迁移效果 | 1h | 验证通过 |
| - | 优化迁移流程 | 1h | 流程优化 |
| **批量迁移** | | 40h | |
| - | vc/v1/ 模块 (10 个文件) | 15h | vc/v1 迁移完成 |
| - | calendar/v4/ 模块 (15 个文件) | 15h | calendar/v4 迁移完成 |
| - | meeting_room/ 模块 (8 个文件) | 10h | meeting_room 迁移完成 |
| **清理阶段** | | 8h | |
| - | 删除 Builder 代码 | 2h | 旧代码清理 |
| - | 更新文档 | 3h | 文档更新 |
| - | 更新示例 | 2h | 示例更新 |
| - | 发布版本 | 1h | 新版本发布 |
| **总计** | | **58h** | |

### 7.3 验收标准

#### 代码质量
- ✅ 所有代码通过 `cargo fmt` 检查
- ✅ 所有代码通过 `cargo clippy` 检查（零警告）
- ✅ 所有测试通过 (`cargo test`)
- ✅ 文档测试通过 (`cargo test --doc`)
- ✅ 构建成功 (`cargo build`)

#### 功能完整性
- ✅ 所有 API 功能保持不变
- ✅ 所有示例代码正常运行
- ✅ 所有文档链接有效
- ✅ 返回类型正确

#### 向后兼容性（可选）
- ✅ 保留 Builder 模式（标记为 deprecated）
- ✅ 旧的调用方式仍然可用
- ✅ 迁移指南完整

---

## 8. 附录

### 8.1 相关文档

- [Pas 请求与响应模型规范](ARCHITECTURE.md)
- [openlark-meeting README](../crates/openlark-meeting/README.md)
- [Rust API 设计最佳实践](https://rust-lang.github.io/api-guidelines/)
- [Serde 使用指南](https://serde.rs/)

### 8.2 问题反馈

如遇到迁移问题，请通过以下方式反馈：
- 提交 GitHub Issue
- 联系维护团队

### 8.3 版本历史

| 版本 | 日期 | 变更说明 |
|-----|------|---------|
| 1.0.0 | 2025-01-07 | 初始版本 |

---

**文档状态**: 🟢 已完成
**维护者**: OpenLark Team
**最后更新**: 2025-01-07
