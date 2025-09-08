# API设计标准和最佳实践

## StandardResponse使用模式分析和标准

基于对项目中现有StandardResponse使用情况的分析，建立统一的API设计标准。

### 当前使用情况分析

**已使用StandardResponse的模块**：
- `contact/v3/user.rs` - 11个方法使用
- `im/v1/message.rs` - 2个方法使用  
- `im/v1/image/mod.rs` - 2个方法使用
- `im/v1/file/mod.rs` - 2个方法使用
- `cloud_docs/drive/v1/files.rs` - 2个方法使用

**覆盖率统计**：
- 总计19个方法使用StandardResponse
- 项目总方法数3,753个
- 当前覆盖率：0.5%

### 最佳实践模式

#### 1. 标准方法签名模式

```rust
// ✅ 推荐的标准模式
pub async fn method_name(
    &self,
    request: RequestType,
    option: Option<RequestOption>,
) -> SDKResult<ResponseDataType> {
    // 构建API请求
    let api_req = ApiRequest {
        http_method: Method::POST, // 或 GET/PUT/DELETE
        api_path: "/open-apis/module/v1/endpoint".to_string(),
        supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
        body: serde_json::to_vec(request)?,
        ..Default::default()
    };
    
    // 执行请求并使用StandardResponse
    let api_resp: BaseResponse<ResponseDataType> = 
        Transport::request(api_req, &self.config, option).await?;
    
    // 标准化响应处理 - 关键步骤
    api_resp.into_result()
}
```

#### 2. 当前非标准模式对比

```rust
// ❌ 当前workplace模块使用的非标准模式
pub async fn search(
    &self,
    request: AccessDataSearchRequest,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<AccessDataSearchResponse>> {
    // ... 构建请求 ...
    
    // 直接返回BaseResponse，没有使用StandardResponse
    Transport::request(api_req, &self.config, option).await
}
```

### 标准化改进方案

#### 1. 返回类型标准化

**改进前**：
```rust
-> SDKResult<BaseResponse<ResponseType>>
```

**改进后**：
```rust
-> SDKResult<ResponseType>
```

#### 2. 响应处理标准化

**改进前**：
```rust
Transport::request(api_req, &self.config, option).await
```

**改进后**：
```rust
let api_resp: BaseResponse<ResponseType> = 
    Transport::request(api_req, &self.config, option).await?;
api_resp.into_result()
```

#### 3. 错误处理优势

使用StandardResponse.into_result()的优势：
- **统一错误处理**：将API错误、数据解析错误等统一转换为LarkAPIError
- **类型安全**：直接返回业务数据类型，减少用户端的类型转换
- **更好的用户体验**：用户不需要手动检查response.success()和提取data字段

### Builder模式实施标准

#### 适用场景判断标准

Builder模式适用于以下场景的API方法：
1. **复杂参数结构**：请求参数超过5个可选字段
2. **分层参数**：包含嵌套的配置参数
3. **多种调用模式**：同一API支持多种不同的参数组合

#### Builder模式实施示例

```rust
// 请求结构体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccessDataSearchRequest {
    pub page_token: Option<String>,
    pub page_size: Option<i32>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub user_id: Option<String>,
    pub department_id: Option<String>,
    pub access_type: Option<String>,
}

// Builder实现
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
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.inner.page_token = Some(page_token.into());
        self
    }
    
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.inner.page_size = Some(page_size);
        self
    }
    
    pub fn time_range(mut self, start_time: i64, end_time: i64) -> Self {
        self.inner.start_time = Some(start_time);
        self.inner.end_time = Some(end_time);
        self
    }
    
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.inner.user_id = Some(user_id.into());
        self
    }
    
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.inner.department_id = Some(department_id.into());
        self
    }
    
    pub fn access_type(mut self, access_type: impl Into<String>) -> Self {
        self.inner.access_type = Some(access_type.into());
        self
    }
    
    pub fn build(self) -> AccessDataSearchRequest {
        self.inner
    }
}
```

### API命名约定

1. **方法命名**：
   - 查询类：`search`, `list`, `get`
   - 创建类：`create`, `add`  
   - 更新类：`update`, `patch`
   - 删除类：`delete`, `remove`

2. **Builder方法命名**：
   - 构造器：`builder()`
   - 完成构建：`build()`
   - 设置方法：使用字段名，如`page_size()`, `user_id()`
   - 复合设置：如`time_range()`, `pagination()`

### 文档注释标准

```rust
/// 获取工作台访问数据
///
/// 获取工作台的访问数据统计信息，支持按时间范围、用户、部门等维度查询。
///
/// # Arguments
///
/// * `request` - 访问数据查询请求
/// * `option` - 请求选项，可选
///
/// # Returns
///
/// 返回工作台访问数据列表
///
/// # Example
///
/// ```rust,no_run
/// use open_lark::prelude::*;
/// 
/// let client = LarkClient::builder("app_id", "app_secret").build();
/// let request = AccessDataSearchRequest::builder()
///     .page_size(20)
///     .time_range(start_time, end_time)
///     .build();
/// 
/// let result = client.workplace.workplace_access_data.search(request, None).await?;
/// ```
pub async fn search(
    &self,
    request: AccessDataSearchRequest,
    option: Option<RequestOption>,
) -> SDKResult<AccessDataSearchResponse> {
    // 实现...
}
```

### 兼容性保障策略

1. **向后兼容**：保持现有API签名不变
2. **渐进迁移**：提供过渡期，同时支持新旧两种调用方式
3. **文档更新**：及时更新示例和文档，引导用户使用新模式
4. **测试覆盖**：确保改进不破坏现有功能

### 质量验证标准

1. **编译检查**：所有改进必须编译通过
2. **单元测试**：保持现有测试通过率100%
3. **API检查**：使用检查工具验证StandardResponse覆盖率
4. **性能基准**：确保改进不显著影响性能
5. **文档一致性**：API变更同步更新相关文档

---

**生成时间**: 2025-09-08
**适用范围**: open-lark项目API设计标准化改进