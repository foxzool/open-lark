# Open-Lark 设计规范指南

## 概述

本文档定义了Open-Lark项目的开发规范和最佳实践，确保代码质量、一致性和可维护性。

## 命名规范

### Crate命名

**格式**: `openlark-{business-domain}`

**规则**:
- 全小写字母
- 使用连字符分隔单词
- 简洁明了，体现业务领域
- 英文命名，符合Rust生态习惯

**示例**:
```rust
openlark-core           // 核心基础设施
openlark-hr            // 人力资源管理
openlark-communication // 通讯协作
openlark-docs          // 文档协作
openlark-ai            // AI智能服务
```

### 目录结构规范

**项目根目录结构**:
```
open-lark/
├── crates/                    // 所有crate模块
│   ├── openlark-core/        // 核心基础设施
│   ├── openlark-client/      // 客户端封装
│   ├── openlark-protocol/    // 协议定义
│   └── openlark-{domain}/    // 业务模块
├── examples/                 // 示例代码
├── docs/                     // 项目文档
├── tools/                    // 开发工具
├── tests/                    // 集成测试
└── Cargo.toml               // 工作空间配置
```

**业务模块内部结构**:
```
crates/openlark-{domain}/
├── Cargo.toml               // 模块配置
├── README.md                // 模块说明
├── src/
│   ├── lib.rs               // 模块入口
│   ├── models/              // 共享数据模型
│   │   ├── mod.rs
│   │   ├── common.rs        // 通用模型
│   │   └── requests.rs      // 请求模型
│   └── {project}/           // 按项目分组
│       ├── {version}/       // 按版本分组
│       │   ├── mod.rs
│       │   ├── {resource}.rs // 具体资源API
│       │   └── models.rs    // 版本特定模型
│       └── models.rs        // 项目级模型
├── tests/                   // 模块测试
│   ├── unit/               // 单元测试
│   ├── integration/        // 集成测试
│   └── e2e/                // 端到端测试
└── examples/               // 模块示例
    ├── basic_usage.rs
    └── advanced_usage.rs
```

**具体示例 - HR模块**:
```
crates/openlark-hr/
├── src/
│   ├── lib.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── employee.rs      // 员工模型
│   │   ├── department.rs    // 部门模型
│   │   └── common.rs        // 通用模型
│   ├── hire/                // 招聘管理
│   │   ├── v1/
│   │   │   ├── mod.rs
│   │   │   ├── offer.rs     // Offer管理
│   │   │   ├── candidate.rs // 候选人管理
│   │   │   └── interview.rs // 面试管理
│   │   └── models/
│   │       ├── offer.rs
│   │       └── candidate.rs
│   ├── corehr/              // 核心HR
│   │   ├── v1/
│   │   └── v2/
│   ├── attendance/          // 考勤管理
│   │   └── v1/
│   └── payroll/             // 薪酬管理
│       └── v1/
```

### 文件命名规范

**Rust文件**:
- 全小写字母，使用下划线分隔
- 模块文件与模块名一致
- 实现文件以功能命名

**示例**:
```rust
src/
├── lib.rs                  // 库入口
├── models.rs               // 模型定义
├── client.rs               // 客户端实现
├── user_management.rs      // 用户管理
├── department_service.rs   // 部门服务
└── utils/
    ├── mod.rs
    ├── http_client.rs      // HTTP客户端工具
    └── validation.rs       // 验证工具
```

### API接口命名规范

**格式**: `{resource}_{action}_{version}`

**规则**:
- 资源名称使用单数形式
- 动词使用明确的行为词
- 版本号使用v1, v2, v3格式

**示例**:
```rust
// 用户管理API
pub async fn user_create_v3(&self, request: UserCreateRequestV3) -> SDKResult<UserCreateResponseV3>
pub async fn user_get_v3(&self, request: UserGetRequestV3) -> SDKResult<UserGetResponseV3>
pub async fn user_update_v3(&self, request: UserUpdateRequestV3) -> SDKResult<UserUpdateResponseV3>
pub async fn user_delete_v3(&self, request: UserDeleteRequestV3) -> SDKResult<()>

// 消息管理API
pub async fn message_send_v1(&self, request: MessageSendRequestV1) -> SDKResult<MessageSendResponseV1>
pub async fn message_list_v1(&self, request: MessageListRequestV1) -> SDKResult<MessageListResponseV1>

// 文档管理API
pub async fn document_create_v1(&self, request: DocumentCreateRequestV1) -> SDKResult<DocumentCreateResponseV1>
pub async fn document_read_v1(&self, request: DocumentReadRequestV1) -> SDKResult<DocumentReadResponseV1>
```

### 数据模型命名规范

**请求模型**: `{Resource}{Action}Request{Version}`
```rust
pub struct UserCreateRequestV3 {
    pub name: String,
    pub department_id: String,
    pub email: Option<String>,
    // ...
}

pub struct MessageSendRequestV1 {
    pub receive_id_type: ReceiveIdType,
    pub receive_id: String,
    pub content: String,
    pub msg_type: MessageType,
}
```

**响应模型**: `{Resource}{Action}Response{Version}`
```rust
pub struct UserCreateResponseV3 {
    pub user: User,
    pub request_id: String,
}

pub struct MessageSendResponseV1 {
    pub message_id: String,
    pub request_id: String,
}
```

**数据模型**: 使用业务领域的自然名称
```rust
pub struct User {
    pub user_id: String,
    pub name: String,
    pub email: Option<String>,
    pub department_ids: Vec<String>,
}

pub struct Message {
    pub message_id: String,
    pub content: String,
    pub create_time: i64,
    pub sender: User,
}
```

## 代码组织规范

### 模块结构模式

**标准模块结构**:
```rust
// src/lib.rs - 模块入口
pub mod models;
pub mod hire;
pub mod corehr;
pub mod attendance;

use openlark_core::{Config, SDKResult};

pub struct HRService {
    pub config: Config,
}

impl HRService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // 提供子服务访问
    pub fn hire(&self) -> hire::HireService {
        hire::HireService::new(self.config.clone())
    }

    pub fn corehr(&self) -> corehr::CoreHRService {
        corehr::CoreHRService::new(self.config.clone())
    }

    pub fn attendance(&self) -> attendance::AttendanceService {
        attendance::AttendanceService::new(self.config.clone())
    }
}
```

**版本化服务模式**:
```rust
// src/hire/v1/mod.rs
pub mod offer;
pub mod candidate;
pub mod interview;
pub mod models;

use openlark_core::{Config, SDKResult};

pub struct HireV1Service {
    pub config: Config,
}

impl HireV1Service {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // 构建器模式API
    pub fn offer_create(&self) -> offer::OfferCreateBuilder {
        offer::OfferCreateBuilder::new(self.config.clone())
    }

    pub fn candidate_list(&self) -> candidate::CandidateListBuilder {
        candidate::CandidateListBuilder::new(self.config.clone())
    }
}
```

**构建器模式实现**:
```rust
// src/hire/v1/offer.rs
use openlark_core::{Config, SDKResult, HttpClient};
use crate::models::{Offer, OfferCreateRequestV1, OfferCreateResponseV1};

pub struct OfferCreateBuilder {
    config: Config,
    request: OfferCreateRequestV1,
}

impl OfferCreateBuilder {
    pub(crate) fn new(config: Config) -> Self {
        Self {
            config,
            request: OfferCreateRequestV1::default(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = name.into();
        self
    }

    pub fn candidate_id(mut self, id: impl Into<String>) -> Self {
        self.request.candidate_id = id.into();
        self
    }

    pub fn department_id(mut self, id: impl Into<String>) -> Self {
        self.request.department_id = id.into();
        self
    }

    pub fn start_date(mut self, date: impl Into<String>) -> Self {
        self.request.start_date = date.into();
        self
    }

    pub async fn send(self) -> SDKResult<OfferCreateResponseV1> {
        let client = HttpClient::new(&self.config)?;
        let url = format!("{}/hire/v1/offers", self.config.api_base_url);

        let response = client
            .post(&url)
            .json(&self.request)
            .send()
            .await?;

        let result: OfferCreateResponseV1 = response.json().await?;
        Ok(result)
    }
}
```

### 错误处理规范

**统一错误类型**:
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SDKError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("API error (code: {code}): {message}")]
    API { code: i32, message: String },

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Rate limited: retry after {retry_after} seconds")]
    RateLimited { retry_after: u64 },

    #[error("Invalid request: {field} - {message}")]
    InvalidRequest { field: String, message: String },

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type SDKResult<T> = Result<T, SDKError>;
```

**错误处理最佳实践**:
```rust
impl HRService {
    pub async fn create_user(&self, request: UserCreateRequestV3) -> SDKResult<UserCreateResponseV3> {
        // 参数验证
        if request.name.is_empty() {
            return Err(SDKError::InvalidRequest {
                field: "name".to_string(),
                message: "Name cannot be empty".to_string(),
            });
        }

        // API调用
        let result = self.make_api_call("/user/v3/create", &request).await?;

        // 结果处理
        match result.status {
            0 => Ok(result.data),
            11000 => Err(SDKError::Authentication("Token expired".to_string())),
            11001 => Err(SDKError::RateLimited { retry_after: 60 }),
            code => Err(SDKError::API {
                code,
                message: result.message,
            }),
        }
    }
}
```

## 配置管理规范

### Cargo.toml配置

**业务模块配置模板**:
```toml
[package]
name = "openlark-hr"
version = "0.1.0"
edition = "2021"
authors = ["Open-Lark Team"]
description = "HR management module for Open-Lark SDK"
license = "MIT"
repository = "https://github.com/open-lark/open-lark"
keywords = ["lark", "hr", "sdk", "api"]
categories = ["api-bindings", "web-programming"]

[dependencies]
# 核心依赖
openlark-core = { workspace = true }
openlark-protocol = { workspace = true, optional = true }

# 外部依赖
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.11", features = ["json"] }
thiserror = "1.0"
tokio = { version = "1.0", features = ["full"] }
uuid = { version = "1.0", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }

# 可选依赖
async-trait = { version = "0.1", optional = true }

[dev-dependencies]
tokio-test = "0.4"
mockito = "1.0"

[features]
default = []
websocket = ["openlark-protocol"]
async = ["async-trait"]
```

### Feature配置规范

**功能特性定义**:
```toml
# 工作空间级别 Cargo.toml
[workspace.features]
# 核心功能组
default = ["communication", "docs", "security"]
minimal = ["core", "security"]
full = [
    "communication", "docs", "hr", "workflow",
    "meeting", "ai", "security", "analytics",
    "mail", "helpdesk", "platform", "user"
]

# 业务模块功能
communication = ["openlark-communication"]
docs = ["openlark-docs"]
hr = ["openlark-hr"]
workflow = ["openlark-workflow"]
meeting = ["openlark-meeting"]
mail = ["openlark-mail"]
helpdesk = ["openlark-helpdesk"]
platform = ["openlark-platform"]
ai = ["openlark-ai"]
security = ["openlark-security"]
analytics = ["openlark-analytics"]
user = ["openlark-user"]

# 技术功能
websocket = ["openlark-protocol"]
async = []
```

## 测试规范

### 测试组织结构

```
crates/openlark-{domain}/
├── tests/
│   ├── unit/                   // 单元测试
│   │   ├── mod.rs
│   │   ├── models_tests.rs
│   │   ├── client_tests.rs
│   │   └── api_tests.rs
│   ├── integration/            // 集成测试
│   │   ├── mod.rs
│   │   ├── service_tests.rs
│   │   └── workflow_tests.rs
│   └── e2e/                   // 端到端测试
│       ├── mod.rs
│       ├── full_workflow.rs
│       └── error_handling.rs
└── examples/                   // 示例代码
    ├── basic_usage.rs
    ├── advanced_usage.rs
    └── error_handling.rs
```

### 测试编写规范

**单元测试示例**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_user_create_request_serialization() {
        let request = UserCreateRequestV3 {
            name: "张三".to_string(),
            department_id: "dept_123".to_string(),
            email: Some("zhangsan@example.com".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("张三"));
        assert!(json.contains("dept_123"));
    }

    #[test]
    fn test_user_validation() {
        let config = Config::default();
        let service = HRService::new(config);

        // 测试空名称验证
        let request = UserCreateRequestV3 {
            name: "".to_string(),
            department_id: "dept_123".to_string(),
            email: None,
        };

        let result = service.validate_user_request(&request);
        assert!(result.is_err());
    }
}
```

**集成测试示例**:
```rust
// tests/integration/hr_service_tests.rs
use openlark_client::LarkClient;
use openlark_hr::{HRService, UserCreateRequestV3};

#[tokio::test]
async fn test_full_user_workflow() {
    // 设置测试客户端
    let client = LarkClient::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

    let hr_service = client.hr();

    // 创建用户
    let create_request = UserCreateRequestV3 {
        name: "测试用户".to_string(),
        department_id: "test_dept".to_string(),
        email: Some("test@example.com".to_string()),
    };

    let create_response = hr_service.v3().user_create(create_request).await.unwrap();
    assert!(!create_response.user.user_id.is_empty());

    // 获取用户信息
    let get_response = hr_service.v3().user_get(
        UserGetRequestV3 {
            user_id: create_response.user.user_id.clone(),
        }
    ).await.unwrap();

    assert_eq!(get_response.user.name, "测试用户");

    // 清理测试数据
    hr_service.v3().user_delete(
        UserDeleteRequestV3 {
            user_id: create_response.user.user_id,
        }
    ).await.unwrap();
}
```

## 文档规范

### 代码文档

**公共API必须包含文档**:
```rust
/// 创建用户
///
/// 在飞书通讯录中创建新用户，相当于员工入职操作。
///
/// # 参数
///
/// * `request` - 用户创建请求，包含用户基本信息
///
/// # 返回值
///
/// 返回创建成功的用户信息，包括系统生成的user_id
///
/// # 错误
///
/// * `SDKError::Authentication` - 认证失败，token无效或过期
/// * `SDKError::InvalidRequest` - 请求参数错误，如名称为空
/// * `SDKError::RateLimited` - 请求频率超限
///
/// # 示例
///
/// ```rust
/// use openlark_hr::{HRService, UserCreateRequestV3};
///
/// let hr_service = HRService::new(config);
/// let request = UserCreateRequestV3 {
///     name: "张三".to_string(),
///     department_id: "dept_123".to_string(),
///     email: Some("zhangsan@example.com".to_string()),
/// };
///
/// let response = hr_service.v3().user_create(request).await?;
/// println!("创建用户成功，ID: {}", response.user.user_id);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub async fn user_create(&self, request: UserCreateRequestV3) -> SDKResult<UserCreateResponseV3> {
    // 实现
}
```

### 模块文档

**每个crate必须有README.md**:
```markdown
# Open-Lark HR Module

飞书开放平台人力资源管理模块，提供完整的招聘、员工管理、考勤、薪酬等功能。

## 功能特性

- **招聘管理**: Offer管理、候选人管理、面试安排
- **员工管理**: 员工信息、部门管理、组织架构
- **考勤管理**: 考勤记录、请假审批、加班管理
- **薪酬管理**: 薪资结构、薪资发放、个税计算

## 快速开始

```rust
use openlark_client::LarkClient;

let client = LarkClient::builder()
    .app_id("your_app_id")
    .app_secret("your_app_secret")
    .feature("hr")
    .build()?;

// 创建用户
let response = client.hr().v3().user_create(request).await?;
```

## API参考

详见[API文档](https://docs.open-lark.com/hr)

## 许可证

MIT License
```

## 性能优化规范

### 异步编程

**使用async/await模式**:
```rust
// 推荐：使用异步方法
pub async fn get_user(&self, user_id: &str) -> SDKResult<User> {
    let url = format!("{}/user/v3/get", self.config.api_base_url);
    let response = self.http_client.get(&url).send().await?;
    // ...
}

// 批量操作支持并发
pub async fn batch_get_users(&self, user_ids: Vec<String>) -> SDKResult<Vec<User>> {
    let futures = user_ids.into_iter()
        .map(|id| self.get_user(&id))
        .collect::<Vec<_>>();

    let results = futures::future::try_join_all(futures).await?;
    Ok(results)
}
```

### 内存管理

**避免不必要的克隆**:
```rust
// 推荐：使用引用
pub fn validate_user_name(name: &str) -> SDKResult<()> {
    if name.is_empty() {
        return Err(SDKError::InvalidRequest {
            field: "name".to_string(),
            message: "Name cannot be empty".to_string(),
        });
    }
    Ok(())
}

// 推荐：使用Arc<T>共享数据
use std::sync::Arc;

pub struct HRService {
    pub config: Arc<Config>,
}
```

## 版本管理规范

### 语义化版本

**版本号格式**: `MAJOR.MINOR.PATCH`

- **MAJOR**: 破坏性变更，API不兼容
- **MINOR**: 新功能添加，向后兼容
- **PATCH**: Bug修复，向后兼容

### API版本化

**支持多版本并存**:
```rust
impl HRService {
    pub fn v1(&self) -> HRV1Service { /* ... */ }
    pub fn v2(&self) -> HRV2Service { /* ... */ }
    pub fn v3(&self) -> HRV3Service { /* ... */ }
}

// 在Cargo.toml中管理版本特性
[features]
v1 = []
v2 = ["v1"]
v3 = ["v2"]
default = ["v3"]
```

---

遵循这些规范可以确保Open-Lark项目的代码质量、一致性和可维护性，为开发者提供优秀的使用体验。