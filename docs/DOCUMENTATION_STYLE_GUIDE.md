# Open-Lark 文档风格指南

本指南规定了 Open-Lark 项目中 Rust 代码文档的标准格式和要求。

## 基本原则

### 语言使用
- **首选中文**：所有用户面向的API文档使用中文
- **英文补充**：关键术语可以在中文后用英文标注
- **代码示例**：注释使用中文，代码变量名使用英文

### 文档完整性
- 所有公共API（`pub`）必须有文档
- 结构体、枚举、trait、函数都需要文档
- 复杂的私有函数也应该有文档

## 文档结构

### 模块文档 (`//!`)
```rust
//! 模块简要描述
//!
//! 详细说明模块的用途、主要功能和使用场景。
//!
//! # 示例
//!
//! ```rust
//! // 基本使用示例
//! ```
//!
//! # 重要说明
//!
//! - 注意事项1
//! - 注意事项2
```

### 结构体文档
```rust
/// 结构体简要描述
///
/// 详细描述结构体的用途和设计意图。
///
/// # 主要功能
///
/// - 功能1
/// - 功能2
///
/// # 示例
///
/// ```rust
/// // 使用示例
/// let instance = MyStruct::new();
/// ```
///
/// # 注意事项
///
/// 重要的使用注意事项。
pub struct MyStruct {
    /// 字段描述，说明用途和取值范围
    pub field: Type,
}
```

### 函数/方法文档
```rust
/// 函数简要描述
///
/// 详细描述函数的功能、行为和副作用。
///
/// # 参数
///
/// - `param1`: 参数1的描述
/// - `param2`: 参数2的描述和取值说明
///
/// # 返回值
///
/// 返回值的描述和可能的错误情况。
///
/// # 错误
///
/// 可能抛出的错误类型和原因：
/// - `ErrorType1`: 错误原因1
/// - `ErrorType2`: 错误原因2
///
/// # 示例
///
/// ```rust
/// // 使用示例
/// let result = my_function(param1, param2)?;
/// ```
///
/// # 安全性
///
/// 如果涉及unsafe代码，说明安全要求。
pub fn my_function(param1: Type1, param2: Type2) -> Result<ReturnType, ErrorType> {
    // 实现
}
```

### Trait文档
```rust
/// Trait简要描述
///
/// 详细描述trait的设计目的和使用场景。
///
/// # 实现要求
///
/// 描述实现此trait的要求和约束。
///
/// # 实现者
///
/// - `Type1`: 实现描述
/// - `Type2`: 实现描述
///
/// # 示例
///
/// ```rust
/// // 实现示例
/// impl MyTrait for MyType {
///     // 实现代码
/// }
/// ```
pub trait MyTrait {
    /// 关联方法描述
    fn method(&self) -> ReturnType;
}
```

## 代码示例规范

### 示例代码要求
- 必须是可编译的完整代码
- 使用真实的API调用
- 包含必要的错误处理
- 展示常见使用场景

### 示例格式
```rust
/// # 示例
///
/// ```rust
/// use open_lark::prelude::*;
///
/// // 创建客户端
/// let client = LarkClient::builder("app_id", "app_secret")
///     .with_app_type(AppType::SelfBuilt)
///     .build();
///
/// // 使用客户端
/// let result = client.some_method().await?;
/// ```
```

### 不可运行的示例
如果示例无法编译（如需要外部依赖），使用：
```rust
/// ```rust,no_run
/// // 需要网络连接的示例
/// ```
```

### 忽略某些行
```rust
/// ```rust
/// # use open_lark::prelude::*;
/// # let client = get_test_client();
/// // 用户看到的代码
/// let result = client.method().await?;
/// ```
```

## 链接和引用

### 内部链接
- 使用 `[`Type`]` 链接到类型
- 使用 `[`function`]` 链接到函数
- 使用 `[`module::Type`]` 链接到其他模块的类型

### 外部链接
- API文档：`[飞书开放平台文档](https://open.feishu.cn/document/...)`
- 相关概念：`[OAuth 2.0](https://oauth.net/2/)`

## 警告和提示

### 使用标准标记
```rust
/// # 注意事项
///
/// ⚠️ **重要**: 关键提醒
///
/// # 安全性
///
/// 🔒 **安全**: 安全相关说明
///
/// # 性能
///
/// ⚡ **性能**: 性能相关提示
```

## 版本和兼容性

### 标记API状态
```rust
/// # 稳定性
///
/// 此API在v1.0后保持稳定。
///
/// # 弃用
///
/// ⚠️ 此方法将在v2.0中移除，请使用 [`new_method`] 替代。
```

## 自动化检查

### CI集成
```yaml
- name: Check documentation
  run: |
    RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps --lib
    cargo doc --no-deps --document-private-items --lib
```

### 本地检查
```bash
# 检查缺失文档
RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps --lib

# 生成完整文档
cargo doc --no-deps --document-private-items --lib
```

## 常见模式

### 构建器模式
```rust
/// 配置构建器
///
/// 使用链式调用配置各种选项。
///
/// # 示例
///
/// ```rust
/// let config = ConfigBuilder::new()
///     .option1(value1)
///     .option2(value2)
///     .build();
/// ```
pub struct ConfigBuilder {
    // 字段
}

impl ConfigBuilder {
    /// 设置选项1
    ///
    /// # 参数
    /// - `value`: 选项值和说明
    pub fn option1(mut self, value: Type) -> Self {
        // 实现
    }
}
```

### 错误类型
```rust
/// 错误类型枚举
///
/// 描述所有可能的错误情况。
#[derive(Debug, thiserror::Error)]
pub enum MyError {
    /// 网络连接错误
    #[error("网络连接失败: {0}")]
    Network(String),
    
    /// 认证失败
    #[error("认证失败")]
    Auth,
}
```

## 最佳实践

1. **简洁明了**：文档应该清晰、简洁，避免冗余
2. **用户视角**：从用户角度描述功能和用法
3. **示例先行**：提供实用的代码示例
4. **保持更新**：代码变更时同步更新文档
5. **统一风格**：遵循项目统一的文档风格

## 工具推荐

- **rustdoc**: 官方文档生成工具
- **cargo-doc**: 本地文档生成
- **mdbook**: 复杂文档编写
- **rustfmt**: 代码格式化
- **clippy**: 代码质量检查