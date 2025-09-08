# Builder模式实施规范和代码模板

## 概述

Builder模式为复杂的API请求结构提供流畅的构建接口，提升开发者体验和代码可读性。

## 适用标准

### 何时使用Builder模式

Builder模式适用于满足以下**任一条件**的请求结构：

1. **参数数量标准**: 包含4个或更多可选参数
2. **复杂度标准**: 包含嵌套对象或数组参数
3. **用户体验标准**: 需要提供便捷的组合设置方法
4. **业务标准**: 同一API有多种常见使用模式

### 不适用场景

以下情况**不建议**使用Builder模式：

1. 只有1-3个简单参数的请求结构
2. 所有参数都是必需参数
3. 参数类型都是基础类型且无复杂逻辑

## 代码模板

### 1. 基础Builder模板

```rust
// 为现有请求结构添加builder支持
impl RequestStruct {
    /// 创建Builder实例
    pub fn builder() -> RequestStructBuilder {
        RequestStructBuilder::default()
    }
}

/// Builder结构体
#[derive(Default)]
pub struct RequestStructBuilder {
    inner: RequestStruct,
}

impl RequestStructBuilder {
    /// 单一参数设置方法
    pub fn field_name(mut self, value: impl Into<FieldType>) -> Self {
        self.inner.field_name = Some(value.into());
        self
    }
    
    /// 可选参数设置方法
    pub fn optional_field(mut self, value: Option<FieldType>) -> Self {
        self.inner.optional_field = value;
        self
    }
    
    /// 完成构建
    pub fn build(self) -> RequestStruct {
        self.inner
    }
}
```

### 2. 增强Builder模板（推荐）

```rust
impl RequestStruct {
    pub fn builder() -> RequestStructBuilder {
        RequestStructBuilder::default()
    }
}

#[derive(Default)]
pub struct RequestStructBuilder {
    inner: RequestStruct,
}

impl RequestStructBuilder {
    // === 基础字段设置 ===
    
    /// 设置页面令牌
    pub fn page_token(mut self, token: impl Into<String>) -> Self {
        self.inner.page_token = Some(token.into());
        self
    }
    
    /// 设置页面大小
    pub fn page_size(mut self, size: i32) -> Self {
        self.inner.page_size = Some(size);
        self
    }
    
    // === 复合设置方法 ===
    
    /// 设置分页参数
    pub fn pagination(mut self, page_token: Option<String>, page_size: Option<i32>) -> Self {
        self.inner.page_token = page_token;
        self.inner.page_size = page_size;
        self
    }
    
    /// 设置时间范围
    pub fn time_range(mut self, start_time: i64, end_time: i64) -> Self {
        self.inner.start_time = Some(start_time);
        self.inner.end_time = Some(end_time);
        self
    }
    
    // === 便捷方法 ===
    
    /// 设置用户过滤
    pub fn user_filter(mut self, user_id: impl Into<String>) -> Self {
        self.inner.user_id = Some(user_id.into());
        self
    }
    
    /// 设置部门过滤
    pub fn department_filter(mut self, department_id: impl Into<String>) -> Self {
        self.inner.department_id = Some(department_id.into());
        self
    }
    
    // === 完成构建 ===
    
    /// 构建最终请求对象
    pub fn build(self) -> RequestStruct {
        self.inner
    }
}
```

### 3. 高级Builder模板（复杂场景）

```rust
impl ComplexRequestStruct {
    pub fn builder() -> ComplexRequestStructBuilder {
        ComplexRequestStructBuilder::default()
    }
}

#[derive(Default)]
pub struct ComplexRequestStructBuilder {
    inner: ComplexRequestStruct,
}

impl ComplexRequestStructBuilder {
    // === 基础设置 ===
    
    pub fn field_name(mut self, value: impl Into<String>) -> Self {
        self.inner.field_name = Some(value.into());
        self
    }
    
    // === 数组/列表设置 ===
    
    /// 添加单个项目
    pub fn add_item(mut self, item: ItemType) -> Self {
        if self.inner.items.is_none() {
            self.inner.items = Some(Vec::new());
        }
        self.inner.items.as_mut().unwrap().push(item);
        self
    }
    
    /// 批量添加项目
    pub fn add_items(mut self, items: impl IntoIterator<Item = ItemType>) -> Self {
        if self.inner.items.is_none() {
            self.inner.items = Some(Vec::new());
        }
        self.inner.items.as_mut().unwrap().extend(items);
        self
    }
    
    /// 设置完整列表
    pub fn items(mut self, items: Vec<ItemType>) -> Self {
        self.inner.items = Some(items);
        self
    }
    
    // === 条件设置 ===
    
    /// 条件性设置字段
    pub fn field_if(mut self, condition: bool, value: impl Into<String>) -> Self {
        if condition {
            self.inner.field_name = Some(value.into());
        }
        self
    }
    
    // === 验证和构建 ===
    
    /// 构建并验证
    pub fn build(self) -> Result<ComplexRequestStruct, BuilderError> {
        // 可以添加验证逻辑
        if self.inner.required_field.is_none() {
            return Err(BuilderError::MissingRequiredField("required_field"));
        }
        
        Ok(self.inner)
    }
    
    /// 构建但不验证（向后兼容）
    pub fn build_unchecked(self) -> ComplexRequestStruct {
        self.inner
    }
}

/// Builder错误类型
#[derive(Debug)]
pub enum BuilderError {
    MissingRequiredField(&'static str),
    InvalidValue(&'static str),
}

impl std::fmt::Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuilderError::MissingRequiredField(field) => {
                write!(f, "Missing required field: {}", field)
            },
            BuilderError::InvalidValue(field) => {
                write!(f, "Invalid value for field: {}", field)
            }
        }
    }
}

impl std::error::Error for BuilderError {}
```

## 命名规范

### Builder相关命名

| 组件 | 命名规则 | 示例 |
|------|----------|------|
| Builder结构体 | `{RequestName}Builder` | `AccessDataSearchRequestBuilder` |
| 构造方法 | `builder()` | `AccessDataSearchRequest::builder()` |
| 字段设置方法 | 字段名去掉类型后缀 | `page_size()`, `user_id()` |
| 复合设置方法 | 描述性名称 | `time_range()`, `pagination()` |
| 便捷方法 | `{purpose}_filter()` 或 `{purpose}_config()` | `user_filter()`, `advanced_config()` |
| 完成方法 | `build()` | `builder.build()` |

### 方法命名模式

```rust
// ✅ 推荐的命名
pub fn page_size(mut self, size: i32) -> Self             // 直接字段名
pub fn user_filter(mut self, user_id: String) -> Self     // 语义化命名
pub fn time_range(mut self, start: i64, end: i64) -> Self // 复合设置
pub fn pagination(mut self, token: Option<String>, size: Option<i32>) -> Self  // 批量设置

// ❌ 避免的命名
pub fn set_page_size(mut self, size: i32) -> Self         // 避免set_前缀
pub fn with_user_id(mut self, id: String) -> Self         // 避免with_前缀（除非是通用模式）
pub fn page_size_value(mut self, size: i32) -> Self       // 避免冗余后缀
```

## 实现指南

### 1. workplace模块实际应用

基于workplace模块的具体需求，以下是实际的Builder实现：

#### AccessDataSearchRequest Builder

```rust
impl AccessDataSearchRequest {
    pub fn builder() -> AccessDataSearchRequestBuilder {
        AccessDataSearchRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct AccessDataSearchRequestBuilder {
    inner: AccessDataSearchRequest,
}

impl AccessDataSearchRequestBuilder {
    /// 设置页面令牌
    pub fn page_token(mut self, token: impl Into<String>) -> Self {
        self.inner.page_token = Some(token.into());
        self
    }
    
    /// 设置每页数量
    pub fn page_size(mut self, size: i32) -> Self {
        self.inner.page_size = Some(size);
        self
    }
    
    /// 设置开始时间戳
    pub fn start_time(mut self, timestamp: i64) -> Self {
        self.inner.start_time = Some(timestamp);
        self
    }
    
    /// 设置结束时间戳  
    pub fn end_time(mut self, timestamp: i64) -> Self {
        self.inner.end_time = Some(timestamp);
        self
    }
    
    /// 设置时间范围（复合方法）
    pub fn time_range(mut self, start_time: i64, end_time: i64) -> Self {
        self.inner.start_time = Some(start_time);
        self.inner.end_time = Some(end_time);
        self
    }
    
    /// 设置分页参数（复合方法）
    pub fn pagination(mut self, page_token: Option<String>, page_size: Option<i32>) -> Self {
        self.inner.page_token = page_token;
        self.inner.page_size = page_size;
        self
    }
    
    /// 设置用户ID筛选
    pub fn user_filter(mut self, user_id: impl Into<String>) -> Self {
        self.inner.user_id = Some(user_id.into());
        self
    }
    
    /// 设置部门ID筛选
    pub fn department_filter(mut self, department_id: impl Into<String>) -> Self {
        self.inner.department_id = Some(department_id.into());
        self
    }
    
    /// 设置访问类型筛选
    pub fn access_type_filter(mut self, access_type: impl Into<String>) -> Self {
        self.inner.access_type = Some(access_type.into());
        self
    }
    
    /// 构建请求对象
    pub fn build(self) -> AccessDataSearchRequest {
        self.inner
    }
}
```

### 2. 使用示例

#### 基础使用
```rust
let request = AccessDataSearchRequest::builder()
    .page_size(20)
    .user_filter("user123")
    .build();
```

#### 复合方法使用
```rust
let request = AccessDataSearchRequest::builder()
    .time_range(start_timestamp, end_timestamp)
    .pagination(Some("page_token".to_string()), Some(50))
    .department_filter("dept456")
    .access_type_filter("view")
    .build();
```

#### 链式调用
```rust
let request = AccessDataSearchRequest::builder()
    .page_size(10)
    .start_time(1609459200)  // 2021-01-01 00:00:00 UTC
    .end_time(1640995200)    // 2022-01-01 00:00:00 UTC  
    .user_filter("user789")
    .access_type_filter("edit")
    .build();
```

### 3. 向后兼容性

Builder模式的添加不应破坏现有代码：

```rust
// 现有用法仍然有效
let request = AccessDataSearchRequest {
    page_token: Some("token".to_string()),
    page_size: Some(20),
    start_time: Some(timestamp),
    // ...其他字段
};

// 新的Builder用法
let request = AccessDataSearchRequest::builder()
    .page_token("token")
    .page_size(20)
    .start_time(timestamp)
    .build();

// 两种方式创建的对象完全等价
```

## 最佳实践

### 1. 方法设计原则

- **流畅性**: 所有设置方法都返回`Self`，支持链式调用
- **类型安全**: 使用`impl Into<T>`接受多种输入类型
- **语义清晰**: 方法名直观反映其功能
- **一致性**: 项目内所有Builder遵循相同模式

### 2. 复合方法设计

复合方法应该提供常用参数组合的快捷方式：

```rust
// 时间范围设置
pub fn time_range(mut self, start: i64, end: i64) -> Self

// 分页设置
pub fn pagination(mut self, token: Option<String>, size: Option<i32>) -> Self

// 过滤器设置
pub fn user_filter(mut self, user_id: impl Into<String>) -> Self
```

### 3. 错误处理

对于简单的Builder，通常不需要复杂的验证：

```rust
// 简单场景：直接返回结构体
pub fn build(self) -> RequestStruct {
    self.inner
}

// 复杂场景：可以添加验证
pub fn build(self) -> Result<RequestStruct, BuilderError> {
    // 验证逻辑
    Ok(self.inner)
}
```

### 4. 文档注释

为Builder方法添加清晰的文档：

```rust
impl AccessDataSearchRequestBuilder {
    /// 设置查询的时间范围
    ///
    /// # Arguments
    /// 
    /// * `start_time` - 开始时间戳（Unix时间戳，秒）
    /// * `end_time` - 结束时间戳（Unix时间戳，秒）
    ///
    /// # Example
    ///
    /// ```rust
    /// let request = AccessDataSearchRequest::builder()
    ///     .time_range(1609459200, 1640995200)  // 2021年全年
    ///     .build();
    /// ```
    pub fn time_range(mut self, start_time: i64, end_time: i64) -> Self {
        self.inner.start_time = Some(start_time);
        self.inner.end_time = Some(end_time);
        self
    }
}
```

## 质量检查清单

在实施Builder模式时，使用以下清单确保质量：

- [ ] Builder结构体命名遵循`{RequestName}Builder`模式
- [ ] 提供`builder()`静态方法创建Builder实例
- [ ] 所有设置方法返回`Self`支持链式调用
- [ ] 使用`impl Into<T>`提高类型灵活性
- [ ] 提供`build()`方法完成构建
- [ ] 复杂参数提供复合设置方法
- [ ] 添加清晰的文档注释和使用示例
- [ ] 保持与现有API的向后兼容性
- [ ] 通过编译和基础测试

---

**文档版本**: v1.0  
**适用范围**: open-lark项目Builder模式实施  
**维护**: 随着项目发展持续更新标准