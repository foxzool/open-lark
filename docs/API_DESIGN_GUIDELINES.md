# API设计规范指南

生成时间: 2025-06-30
版本: v1.0
适用范围: open-lark SDK v0.11.0+

## 📚 目录

1. [设计原则](#设计原则)
2. [Builder模式规范](#builder模式规范)
3. [错误处理规范](#错误处理规范)
4. [异步编程规范](#异步编程规范)
5. [代码结构规范](#代码结构规范)
6. [测试规范](#测试规范)
7. [文档规范](#文档规范)
8. [示例代码规范](#示例代码规范)

## 🎯 设计原则

### 核心原则

1. **一致性优先** - 所有API应遵循统一的设计模式和命名约定
2. **类型安全** - 充分利用Rust类型系统防止运行时错误
3. **向后兼容** - 新功能不应破坏现有API的使用方式
4. **用户友好** - API设计应直观易用，错误信息应清晰有用
5. **性能优先** - 避免不必要的内存分配和数据拷贝

### 设计哲学

- **渐进式增强**: 从基础功能开始，逐步添加高级特性
- **约定优于配置**: 提供合理的默认值，减少样板代码
- **明确优于隐式**: 参数和行为应该明确清晰
- **简单优于复杂**: 简单的设计通常更可维护

## 🏗️ Builder模式规范

### 基本结构

所有Builder应遵循以下结构模式：

```rust
use async_trait::async_trait;
use crate::core::{
    trait_system::executable_builder::ExecutableBuilder,
    req_option::RequestOption,
    SDKResult,
};

/// 服务操作Builder
#[derive(Default)]
pub struct OperationBuilder {
    // 必需参数
    required_field: Option<String>,
    // 可选参数
    optional_field: Option<String>,
}

impl OperationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置必需参数
    pub fn required_field(mut self, value: impl ToString) -> Self {
        self.required_field = Some(value.to_string());
        self
    }

    /// 设置可选参数
    pub fn optional_field(mut self, value: impl ToString) -> Self {
        self.optional_field = Some(value.to_string());
        self
    }

    pub fn build(self) -> OperationRequest {
        OperationRequest {
            required_field: self.required_field.unwrap_or_default(),
            optional_field: self.optional_field,
        }
    }
}

#[async_trait]
impl ExecutableBuilder<ServiceType, OperationRequest, OperationResponse> for OperationBuilder {
    fn build(self) -> OperationRequest {
        self.build()
    }

    async fn execute(self, service: &ServiceType) -> SDKResult<OperationResponse> {
        let request = self.build();
        service.operation(request, None).await
    }

    async fn execute_with_options(
        self,
        service: &ServiceType,
        option: RequestOption,
    ) -> SDKResult<OperationResponse> {
        let request = self.build();
        service.operation(request, Some(option)).await
    }
}
```

### Builder方法命名规范

1. **构造方法**: 使用`new()`和`default()`
2. **设置方法**: 使用字段名作为方法名，如`user_id()`, `file_name()`
3. **构建方法**: 使用`build()`返回请求对象
4. **执行方法**: 使用`execute()`和`execute_with_options()`

### Builder方法返回类型

- **设置方法**: 返回`Self`支持链式调用
- **构建方法**: 返回具体的请求类型
- **执行方法**: 返回`SDKResult<T>`

### 参数处理规范

```rust
// ✅ 正确：使用impl ToString接受多种字符串类型
pub fn user_id(mut self, user_id: impl ToString) -> Self {
    self.user_id = Some(user_id.to_string());
    self
}

// ✅ 正确：可选参数使用Option
pub fn description(mut self, description: Option<String>) -> Self {
    self.description = description;
    self
}

// ✅ 正确：集合参数直接接受
pub fn tags(mut self, tags: Vec<String>) -> Self {
    self.tags = Some(tags);
    self
}

// ❌ 错误：不要使用&str作为参数类型
pub fn user_id(mut self, user_id: &str) -> Self { ... }
```

## 🛡️ 错误处理规范

### StandardResponse使用

所有服务方法都应使用StandardResponse特征：

```rust
use crate::core::standard_response::StandardResponse;

impl ServiceType {
    pub async fn operation(&self, request: OperationRequest, option: Option<RequestOption>) -> SDKResult<OperationResponse> {
        let api_req = ApiRequest {
            // ... 构建API请求
        };

        let api_resp: BaseResponse<OperationResponse> = Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()  // 使用StandardResponse
    }
}
```

### 错误信息设计

1. **具体性**: 错误信息应该具体描述问题
2. **可操作性**: 提供明确的解决建议
3. **一致性**: 同类错误使用相同的格式

```rust
// ✅ 正确的错误处理示例
match result {
    Ok(response) => { /* 处理成功 */ }
    Err(e) => {
        match &e {
            LarkAPIError::APIError { code, msg, .. } => {
                match *code {
                    403 => log::warn!("权限不足: {}. 建议检查应用权限配置", msg),
                    429 => log::warn!("请求频率过高: {}. 建议稍后重试", msg),
                    _ => log::error!("API错误: {} ({})", msg, code),
                }
            }
            LarkAPIError::DataError(msg) => {
                log::error!("数据错误: {}. 建议检查输入参数", msg);
            }
            _ => log::error!("未知错误: {}", e),
        }
    }
}
```

## ⚡ 异步编程规范

### async/await使用

1. **一致性**: 所有网络操作都应该是异步的
2. **错误传播**: 使用`?`操作符传播错误
3. **超时处理**: 提供合理的默认超时时间

```rust
// ✅ 正确的异步方法签名
pub async fn operation(&self, request: OperationRequest, option: Option<RequestOption>) -> SDKResult<OperationResponse> {
    // 实现
}

// ✅ 正确的Builder执行方法
async fn execute(self, service: &ServiceType) -> SDKResult<OperationResponse> {
    let request = self.build();
    service.operation(request, None).await
}
```

### 并发处理

```rust
// ✅ 正确：批量操作使用join_all
use futures::future::join_all;

pub async fn batch_operation(&self, requests: Vec<OperationRequest>) -> Vec<SDKResult<OperationResponse>> {
    let futures = requests.into_iter()
        .map(|req| self.operation(req, None));
    
    join_all(futures).await
}
```

## 📁 代码结构规范

### 文件组织

```
src/service/
├── module_name/
│   ├── v1/
│   │   ├── mod.rs          # 服务主要逻辑
│   │   ├── models.rs       # 数据模型定义
│   │   └── builders.rs     # Builder实现 (如果复杂)
│   └── v2/
│       └── ...
└── mod.rs
```

### 模块结构

```rust
// mod.rs 标准结构
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    trait_system::executable_builder::ExecutableBuilder,
    SDKResult,
};
use async_trait::async_trait;

// 1. 服务结构体定义
pub struct ServiceName {
    pub config: Config,
}

// 2. 响应数据结构定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResponse {
    // 字段定义
}

impl ApiResponseTrait for OperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// 3. 服务实现
impl ServiceName {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // 传统API方法
    pub async fn operation(&self, ...) -> SDKResult<OperationResponse> {
        // 实现
    }

    // Builder创建方法
    pub fn operation_builder(&self) -> OperationBuilder {
        OperationBuilder::new()
    }
}

// 4. Builder实现
#[derive(Default)]
pub struct OperationBuilder {
    // 字段定义
}

// Builder方法实现...
```

### 命名约定

1. **服务名**: 使用PascalCase，如`UserService`, `FileService`
2. **方法名**: 使用snake_case，如`create_user`, `upload_file`
3. **Builder名**: 服务名 + "Builder"，如`CreateUserBuilder`
4. **常量**: 使用SCREAMING_SNAKE_CASE

## 🧪 测试规范

### 单元测试结构

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::constants::AppType;

    // 1. Builder创建测试
    #[test]
    fn test_operation_builder_creation() {
        let client = LarkClient::builder("test_app_id", "test_app_secret")
            .with_app_type(AppType::SelfBuild)
            .build();

        let builder = client.service.operation_builder();
        let request = builder
            .required_field("test_value")
            .optional_field("optional_value")
            .build();

        assert_eq!(request.required_field, "test_value");
        assert_eq!(request.optional_field, Some("optional_value".to_string()));
    }

    // 2. 默认值测试
    #[test]
    fn test_builder_defaults() {
        let builder = OperationBuilder::new();
        let request = builder.build();

        assert_eq!(request.required_field, "");
        assert_eq!(request.optional_field, None);
    }

    // 3. 链式调用测试
    #[test]
    fn test_builder_chaining() {
        let request = OperationBuilder::new()
            .required_field("test")
            .optional_field("optional")
            .build();

        assert_eq!(request.required_field, "test");
    }
}
```

### 测试覆盖要求

1. **Builder创建**: 测试Builder的基本创建和参数设置
2. **默认值**: 测试所有字段的默认值行为
3. **链式调用**: 测试方法链的正确性
4. **边界条件**: 测试空值、极值等边界情况

## 📝 文档规范

### 代码文档

```rust
/// 创建用户Builder
/// 
/// 用于构建创建用户的请求。支持链式调用和可选参数。
/// 
/// # Examples
/// 
/// ```rust,ignore
/// let result = client
///     .contact
///     .v3
///     .user
///     .create_user_builder()
///     .user(user_data)
///     .user_id_type("open_id")
///     .execute(&client.contact.v3.user)
///     .await?;
/// ```
#[derive(Default)]
pub struct CreateUserBuilder {
    // ...
}

impl CreateUserBuilder {
    /// 设置用户信息
    /// 
    /// # Arguments
    /// 
    /// * `user` - 用户数据对象
    pub fn user(mut self, user: User) -> Self {
        self.user = Some(user);
        self
    }

    /// 设置用户ID类型
    /// 
    /// # Arguments
    /// 
    /// * `user_id_type` - 用户ID类型，可选值: "user_id", "union_id", "open_id"
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.user_id_type = Some(user_id_type.to_string());
        self
    }
}
```

### README文档要求

1. **功能概述**: 清晰描述功能和使用场景
2. **快速开始**: 提供完整的使用示例
3. **API参考**: 列出主要方法和参数
4. **最佳实践**: 提供使用建议和注意事项

## 💡 示例代码规范

### 示例文件结构

```rust
/// 服务名称功能演示
///
/// 这个示例展示了如何使用服务名称的各种功能，
/// 包括基础操作、高级用法和错误处理最佳实践。

use dotenvy::dotenv;
use open_lark::{
    client::LarkClient,
    core::{constants::AppType, trait_system::ExecutableBuilder},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    // 创建客户端
    let client = LarkClient::builder(
        &std::env::var("APP_ID").expect("APP_ID is required"),
        &std::env::var("APP_SECRET").expect("APP_SECRET is required"),
    )
    .with_app_type(AppType::SelfBuild)
    .build();

    println!("=== 服务名称功能演示 ===\n");

    // 1. 基础功能演示
    basic_operations(&client).await?;

    // 2. 高级功能演示  
    advanced_operations(&client).await?;

    // 3. 错误处理演示
    error_handling_demo(&client).await?;

    // 4. 最佳实践总结
    print_best_practices();

    Ok(())
}

async fn basic_operations(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 基础操作演示");
    
    // 传统方式
    println!("1. 传统API调用方式:");
    // ... 示例代码

    // Builder方式
    println!("2. Builder模式调用方式:");
    // ... 示例代码

    Ok(())
}

async fn advanced_operations(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ 高级功能演示");
    
    // 条件构建
    // 批量操作
    // 复杂参数设置
    
    Ok(())
}

async fn error_handling_demo(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🛡️ 错误处理演示");
    
    // 错误处理示例
    
    Ok(())
}

fn print_best_practices() {
    println!("📚 最佳实践总结:");
    println!("1. 新项目推荐使用Builder模式");
    println!("2. 充分利用类型安全特性");
    println!("3. 统一使用StandardResponse错误处理");
    // ... 更多建议
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // 测试用例
}
```

### 示例代码要求

1. **完整性**: 示例应该是可运行的完整程序
2. **教学性**: 代码应该清晰展示功能用法
3. **实用性**: 示例应该反映真实使用场景
4. **测试性**: 包含必要的测试用例

## 🔍 代码审查清单

### Builder模式检查

- [ ] Builder结构体使用`#[derive(Default)]`
- [ ] 所有设置方法返回`Self`
- [ ] 实现了`ExecutableBuilder`特征
- [ ] 提供了`new()`构造方法
- [ ] 使用`impl ToString`作为字符串参数类型
- [ ] 必需参数有合理的默认值处理

### 错误处理检查

- [ ] 使用`StandardResponse.into_result()`
- [ ] 错误信息具体且可操作
- [ ] 提供了错误处理示例
- [ ] 遵循统一的错误处理模式

### 异步编程检查

- [ ] 所有网络操作都是异步的
- [ ] 正确使用`async/await`
- [ ] 错误通过`?`操作符传播
- [ ] 提供了超时处理

### 测试检查

- [ ] 包含Builder创建测试
- [ ] 测试了默认值行为
- [ ] 测试了链式调用
- [ ] 覆盖了边界条件

### 文档检查

- [ ] 所有公开API都有文档注释
- [ ] 提供了使用示例
- [ ] 参数说明清晰完整
- [ ] 包含了最佳实践说明

## 📈 版本兼容性指南

### 破坏性变更政策

1. **主版本**: 允许破坏性变更
2. **次版本**: 新增功能，保持向后兼容
3. **补丁版本**: 仅修复bug，保持向后兼容

### 废弃功能处理

```rust
#[deprecated(since = "0.12.0", note = "使用new_method替代")]
pub fn old_method(&self) -> Result<(), Error> {
    // 向后兼容实现
    self.new_method()
}
```

### 迁移指南

每个破坏性变更都应该提供：

1. **变更说明**: 描述变更内容和原因
2. **迁移步骤**: 详细的迁移指导
3. **示例对比**: 变更前后的代码对比
4. **时间安排**: 废弃和移除的时间表

## 🚀 性能优化指南

### 内存优化

1. **避免不必要的clone**: 使用引用或所有权转移
2. **合理使用String和&str**: 根据生命周期选择
3. **池化重用**: 对于频繁创建的对象考虑池化

### 网络优化

1. **连接复用**: 使用连接池
2. **批量操作**: 减少网络往返次数
3. **异步处理**: 避免阻塞操作

### 编译优化

1. **合理使用泛型**: 避免代码膨胀
2. **内联函数**: 对热路径进行内联优化
3. **条件编译**: 使用feature gates控制功能

## 📋 发布检查清单

### 代码质量

- [ ] 所有测试通过
- [ ] 代码覆盖率达标
- [ ] 静态分析无警告
- [ ] 性能测试通过

### 文档完整性

- [ ] API文档更新
- [ ] README更新
- [ ] 变更日志更新
- [ ] 迁移指南完整

### 兼容性验证

- [ ] 向后兼容性测试
- [ ] 示例代码验证
- [ ] 依赖版本检查
- [ ] 平台兼容性测试

---

**注意**: 本文档会随着项目发展持续更新。如有任何建议或问题，请提交Issue讨论。

**维护者**: open-lark SDK 开发团队  
**最后更新**: 2025-06-30