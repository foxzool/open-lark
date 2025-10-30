# 服务开发指南

本指南详细说明如何在 open-lark 项目中实现新的飞书API服务。我们提供了标准化的架构模板和自动化工具来确保所有服务的一致性和质量。

## 📋 目录

1. [快速开始](#快速开始)
2. [架构概览](#架构概览)
3. [开发步骤](#开发步骤)
4. [最佳实践](#最佳实践)
5. [示例演示](#示例演示)
6. [质量检查](#质量检查)

## 🚀 快速开始

### 使用自动化模板生成器

我们提供了一个命令行工具来快速生成标准化的服务模板：

```bash
# 生成新的服务模板
cargo run --bin service_template_generator -- <service_name> <version> <feature_flag>

# 示例：生成联系人服务
cargo run --bin service_template_generator -- contact v1 contact

# 示例：生成会议室服务
cargo run --bin service_template_generator -- meeting_room v1 meeting-room
```

### 参数说明

- `service_name`: 服务名称（使用下划线命名，如 `user_management`）
- `version`: API版本号（如 `v1`, `v4`）
- `feature_flag`: Cargo功能标志（推荐使用连字符命名，如 `user-management`）

### 自动生成的内容

工具会自动创建以下文件和内容：

1. **目录结构**
   ```
   src/service/{service_name}/
   ├── mod.rs                    # 主服务模块
   └── {version}/                # 版本目录
       ├── mod.rs               # 版本服务实现
       └── models.rs            # 数据模型定义
   ```

2. **核心文件**
   - 主服务结构体和构造函数
   - 版本服务实现（包含基础CRUD操作）
   - 完整的数据模型定义
   - 功能示例代码

3. **集成配置**
   - 客户端集成代码
   - Cargo.toml 配置建议
   - 功能标志配置

## 🏗️ 架构概览

### 标准化架构模式

```
src/service/{service_name}/
├── mod.rs                      # 主服务模块
│   ├── 服务结构体定义
│   ├── 版本服务组合
│   └── 构造函数实现
└── {version}/                  # 版本目录（v1, v4等）
    ├── mod.rs                 # 版本服务实现
    │   ├── API方法实现
    │   ├── 业务逻辑处理
    │   └── 错误处理
    ├── models.rs              # 数据模型
    │   ├── 实体结构
    │   ├── 请求响应模型
    │   └── 枚举定义
    └── {feature}/             # 功能模块（可选）
        ├── mod.rs
        └── api_*.rs
```

### 命名规范

1. **文件命名**
   - 主模块：`mod.rs`
   - 版本目录：`v1/`, `v4/` 等
   - 数据模型：`models.rs`
   - 功能模块：使用功能名称，如 `user/`, `meeting/`

2. **结构体命名**
   - 服务：`{ServiceName}Service`
   - 版本服务：`{ServiceName}Service{Version}`
   - 实体：使用业务含义，如 `User`, `MeetingRoom`
   - 请求：`Create{Entity}Request`, `Update{Entity}Request`
   - 响应：`{Entity}Response`, `BaseResponse<T>`

3. **方法命名**
   - CRUD操作：`create`, `get`, `update`, `delete`
   - 查询操作：`list`, `query`, `search`
   - 业务操作：使用业务语言，如 `approve`, `reject`, `schedule`

## 📝 开发步骤

### 步骤1：生成基础模板

```bash
# 示例：创建用户管理服务
cargo run --bin service_template_generator -- user_management v1 user-management
```

### 步骤2：配置功能标志

在 `Cargo.toml` 中添加功能标志：

```toml
[features]
user-management = []
```

### 步骤3：客户端集成

在 `src/client/mod.rs` 中添加：

```rust
// 1. 添加条件导入
#[cfg(feature = "user-management")]
use crate::service::user_management::UserManagementService;

// 2. 在 LarkClient 结构体中添加字段
#[cfg(feature = "user-management")]
pub user_management: UserManagementService,

// 3. 在构造函数中初始化
#[cfg(feature = "user-management")]
user_management: UserManagementService::new(config.clone()),
```

### 步骤4：自定义数据模型

编辑 `src/service/{service_name}/{version}/models.rs`，根据实际API定义数据结构：

```rust
/// 用户实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 邮箱
    pub email: Option<String>,
    /// 用户状态
    pub status: UserStatus,
    // ... 其他字段
}
```

### 步骤5：实现API方法

编辑 `src/service/{service_name}/{version}/mod.rs`，实现具体的API方法：

```rust
impl UserManagementServiceV1 {
    /// 创建用户
    pub async fn create_user(&self, request: &CreateUserRequest) -> SDKResult<BaseResponse<User>> {
        // 1. 参数验证
        // 2. 构建API请求
        // 3. 发送HTTP请求
        // 4. 处理响应
        // 5. 返回结果
    }
}
```

### 步骤6：更新示例代码

编辑生成的示例文件，展示实际的使用场景。

### 步骤7：测试和验证

```bash
# 编译检查
cargo check --features user-management

# 运行示例
cargo run --example user_management_demo --features user-management

# 运行测试
cargo test --features user-management
```

## 💡 最佳实践

### 1. 代码组织

- **单一职责**：每个方法只做一件事
- **清晰的错误处理**：使用 `SDKResult<T>` 统一错误类型
- **完整的文档**：为所有公共API提供详细的中文文档
- **类型安全**：充分利用Rust的类型系统

### 2. API设计

- **一致性**：遵循统一的命名和参数约定
- **向后兼容**：版本间保持API稳定性
- **渐进式**：从基础功能开始，逐步添加高级特性
- **模拟实现**：在开发阶段提供合理的模拟数据

### 3. 错误处理

```rust
// 标准错误处理模式
pub async fn get_user(&self, user_id: &str) -> SDKResult<BaseResponse<User>> {
    // 参数验证
    if user_id.is_empty() {
        return Err(SDKError::InvalidArgument("用户ID不能为空".to_string()));
    }

    // 模拟实现（开发阶段）
    let user = User {
        user_id: user_id.to_string(),
        name: "示例用户".to_string(),
        // ... 其他字段
    };

    Ok(BaseResponse {
        code: 0,
        msg: "success".to_string(),
        data: Some(user),
    })
}
```

### 4. 数据验证

```rust
impl CreateUserRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("用户名不能为空".to_string());
        }

        if let Some(email) = &self.email {
            if !email.contains('@') {
                return Err("邮箱格式不正确".to_string());
            }
        }

        Ok(())
    }
}
```

### 5. 测试策略

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_user() {
        let config = Config::builder("test_app_id", "test_app_secret").build();
        let service = UserManagementServiceV1::new(config);

        let request = CreateUserRequest {
            name: "测试用户".to_string(),
            email: Some("test@example.com".to_string()),
        };

        let result = service.create_user(&request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.code, 0);
        assert!(response.data.is_some());
    }
}
```

## 🎯 示例演示

### 基础服务实现

以下是一个完整的联系人服务实现示例：

```rust
// src/service/contact/mod.rs
//! 联系人服务模块
//!
//! 提供飞书联系人管理相关的API功能。

use crate::core::config::Config;

/// 联系人服务
#[derive(Debug, Clone)]
pub struct ContactService {
    pub config: Config,
    pub v3: v3::ContactServiceV3,
}

impl ContactService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v3: v3::ContactServiceV3::new(config),
        }
    }
}

pub mod v3;
```

### 版本服务实现

```rust
// src/service/contact/v3/mod.rs
//! 联系人 API v3版本
//!
//! 实现联系人管理的核心功能。

use crate::core::config::Config;
use open_lark_core::prelude::*;

/// 联系人服务 v3版本
#[derive(Debug, Clone)]
pub struct ContactServiceV3 {
    pub config: Config,
    pub user: UserService,
    pub department: DepartmentService,
}

impl ContactServiceV3 {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            user: UserService::new(config.clone()),
            department: DepartmentService::new(config),
        }
    }
}

pub mod user;
pub mod department;
pub mod models;

pub use user::*;
pub use department::*;
pub use models::*;
```

### 客户端使用示例

```rust
// 使用示例
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LarkClient::builder("app_id", "app_secret")
        .with_app_type(AppType::SelfBuild)
        .build();

    // 创建用户
    let create_request = CreateUserRequest {
        name: "张三".to_string(),
        mobile: "13800138000".to_string(),
        department_ids: vec!["dept_001".to_string()],
    };

    match client.contact.v3.user.create(&create_request).await {
        Ok(response) => {
            println!("✅ 用户创建成功: {:?}", response.data);
        }
        Err(e) => {
            println!("❌ 用户创建失败: {}", e);
        }
    }

    Ok(())
}
```

## 🔍 质量检查

### 代码质量检查

```bash
# 1. 格式检查
cargo fmt --check

# 2. Clippy检查
cargo clippy --features {feature_flag} -- -Dwarnings

# 3. 编译检查
cargo check --features {feature_flag}

# 4. 文档检查
cargo doc --features {feature_flag} --no-deps
```

### API一致性检查

```bash
# 运行API一致性检查工具
cargo run --bin api_consistency_checker

# 运行增强API检查
cargo run --bin enhanced_api_checker
```

### 测试覆盖率

```bash
# 运行单元测试
cargo test --features {feature_flag}

# 生成测试覆盖率报告
cargo llvm-cov --features {feature_flag} --html
```

## 📚 参考资源

- [飞书开放平台文档](https://open.feishu.cn/)
- [Rust异步编程指南](https://rust-lang.github.io/async-book/)
- [Serde序列化文档](https://serde.rs/)
- [Tokio异步运行时](https://tokio.rs/)

## 🤝 贡献指南

1. 遵循现有的代码风格和架构模式
2. 为新功能添加相应的测试用例
3. 更新相关文档和示例
4. 确保所有质量检查通过
5. 提交前运行完整的测试套件

---

通过遵循本指南，您可以快速、高效地实现高质量的飞书API服务，为用户提供一致、可靠的SDK体验。