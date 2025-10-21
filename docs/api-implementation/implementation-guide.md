# API Implementation Guide

> 🛠️ **Open-Lark SDK 开发实施指导**

## 📋 前言

本指导文档为开发者提供在 open-lark SDK 中实现新 API 的最佳实践、代码规范和开发流程。遵循这些指导原则将确保代码质量、一致性和可维护性。

## 🏗️ 项目架构概览

### 目录结构
```
src/service/
├── {module_name}/               # 服务模块目录
│   ├── mod.rs                   # 模块导出和服务结构体
│   ├── models.rs                # 数据模型定义
│   ├── v{version}/              # API版本目录
│   │   ├── mod.rs              # 版本模块导出
│   │   ├── {resource}.rs       # 资源相关API实现
│   │   └── builders/           # Builder模式实现
│   │       └── mod.rs
│   └── p2_*_event_v{version}.rs # 事件处理器
```

### 核心组件
- **Service**: 提供API方法的服务结构体
- **Models**: 请求/响应数据模型
- **Builders**: 构建器模式实现
- **Handlers**: 事件处理器

## 📝 开发规范

### 1. 代码风格规范

#### 命名约定
```rust
// 结构体使用 PascalCase
pub struct CreateUserRequest {
    pub user: User,
    pub user_id_type: Option<String>,
}

// 函数和方法使用 snake_case
pub async fn create_user(&self, request: CreateUserRequest) -> SDKResult<CreateUserResponse>;

// 常量使用 SCREAMING_SNAKE_CASE
pub const CONTACT_V3_USERS: &str = "/open-apis/contact/v3/users";
```

#### 文档注释规范
```rust
/// 创建用户
///
/// 该接口用于向通讯录创建一个用户，可以理解为员工入职。
/// 创建用户后只返回有数据权限的数据。
///
/// # API文档
///
/// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/contact-v1/user/create
///
/// # 示例
///
/// ```rust,no_run
/// use open_lark::prelude::*;
///
/// let client = LarkClient::builder()
///     .app_id("your_app_id")
///     .app_secret("your_app_secret")
///     .build()?;
///
/// let user = User {
///     name: Some("张三".to_string()),
///     mobile: Some("+8613800138000".to_string()),
///     ..Default::default()
/// };
///
/// let request = CreateUserRequest {
///     user,
///     user_id_type: Some("open_id".to_string()),
///     department_id_type: Some("department_id".to_string()),
/// };
///
/// let response = client.contact.v3.user.create(&request).await?;
/// println!("用户创建成功，用户ID: {}", response.user.user_id.unwrap_or_default());
/// # Ok(())
/// ```
pub async fn create(&self, req: &CreateUserRequest) -> SDKResult<CreateUserResponse> {
    // 实现...
}
```

### 2. API实现模式

#### 基础API实现模板
```rust
use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::ApiResponseTrait,
        config::Config,
        constants::AccessTokenType,
        endpoints::EndpointBuilder,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::{module_name::models::*, traits::*},
};
use async_trait::async_trait;

pub struct ResourceService {
    config: Config,
}

impl ResourceService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// API方法描述
    ///
    /// 详细描述API的功能和用途
    ///
    /// # API文档
    ///
    /// {DOCUMENTATION_URL}
    pub async fn method_name(
        &self,
        request: &RequestType,
    ) -> SDKResult<ResponseType> {
        let mut api_req = ApiRequest {
            http_method: reqwest::Method::{HTTP_METHOD},
            api_path: ENDPOINT_CONSTANT.to_string(),
            supported_access_token_types: vec![AccessTokenType::{TOKEN_TYPE}],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        // 添加查询参数（如果有）
        if let Some(params) = request.build_query_params() {
            api_req.query_params = params;
        }

        let response = Transport::<ResponseType>::request(api_req, &self.config, None).await?;
        response.into_result()
    }
}
```

#### 带路径参数的API实现
```rust
pub async fn get_resource(
    &self,
    resource_id: &str,
    request: &GetResourceRequest,
) -> SDKResult<GetResourceResponse> {
    let mut api_req = ApiRequest {
        http_method: reqwest::Method::GET,
        api_path: EndpointBuilder::replace_param(
            ENDPOINT_WITH_PARAM,
            "resource_id",
            resource_id,
        ),
        supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
        body: Vec::new(),
        ..Default::default()
    };

    // 添加查询参数
    let mut query_params = std::collections::HashMap::new();
    if let Some(user_id_type) = &request.user_id_type {
        query_params.insert("user_id_type", user_id_type.clone());
    }
    api_req.query_params = query_params;

    let response = Transport::<GetResourceResponse>::request(api_req, &self.config, None).await?;
    response.into_result()
}
```

### 3. 数据模型设计

#### 请求结构体
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    /// 用户信息
    pub user: User,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}
```

#### 响应结构体
```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateUserResponse {
    /// 用户信息
    pub user: User,
}

impl ApiResponseTrait for CreateUserResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}
```

#### 枚举设计
```rust
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    /// 激活
    Active,
    /// 冻结
    Frozen,
    /// 离职
    Resigned,
}

impl Default for UserStatus {
    fn default() -> Self {
        UserStatus::Active
    }
}
```

### 4. Builder模式实现

#### 基础Builder模板
```rust
#[derive(Default)]
pub struct CreateUserBuilder {
    user: Option<User>,
    user_id_type: Option<String>,
    department_id_type: Option<String>,
}

impl CreateUserBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置用户信息
    pub fn user(mut self, user: User) -> Self {
        self.user = Some(user);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: impl ToString) -> Self {
        self.department_id_type = Some(department_id_type.to_string());
        self
    }

    pub fn build(self) -> CreateUserRequest {
        CreateUserRequest {
            user: self.user.unwrap_or_default(),
            user_id_type: self.user_id_type,
            department_id_type: self.department_id_type,
        }
    }
}

#[async_trait]
impl ExecutableBuilder<UserService, CreateUserRequest, CreateUserResponse> for CreateUserBuilder {
    fn build(self) -> CreateUserRequest {
        self.build()
    }

    async fn execute(self, service: &UserService) -> SDKResult<CreateUserResponse> {
        let req = self.build();
        service.create(&req).await
    }

    async fn execute_with_options(
        self,
        service: &UserService,
        option: RequestOption,
    ) -> SDKResult<CreateUserResponse> {
        let req = self.build();
        service.create_with_options(&req, Some(option)).await
    }
}
```

### 5. 错误处理规范

#### 统一错误处理
```rust
pub async fn method_with_error_handling(
    &self,
    request: &RequestType,
) -> SDKResult<ResponseType> {
    // 验证输入参数
    if request.user_id.is_empty() {
        return Err(LarkAPIError::illegal_param("用户ID不能为空".to_string()));
    }

    // 构建API请求
    let api_req = self.build_request(request)?;

    // 发送请求并处理响应
    match Transport::<ResponseType>::request(api_req, &self.config, None).await {
        Ok(response) => response.into_result(),
        Err(error) => {
            // 记录错误日志
            tracing::error!("API调用失败: {:?}", error);
            Err(error)
        }
    }
}
```

#### 自定义错误类型
```rust
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("参数错误: {0}")]
    InvalidParameter(String),

    #[error("网络错误: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("序列化错误: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("API调用失败: {code} - {message}")]
    APIError { code: i32, message: String },
}
```

## 🧪 测试规范

### 1. 单元测试

#### 测试文件结构
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{config::Config, constants::AccessTokenType};

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    fn create_test_request() -> RequestType {
        RequestType {
            // 测试数据
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_create_user_success() {
        let config = create_test_config();
        let service = UserService::new(config);
        let request = create_test_request();

        // Mock Transport::request to return successful response
        // 这里需要使用mock框架或测试工具

        // let result = service.create(&request).await;
        // assert!(result.is_ok());
    }

    #[test]
    fn test_request_serialization() {
        let request = create_test_request();
        let json = serde_json::to_string(&request).unwrap();

        // 验证序列化结果
        assert!(json.contains("expected_field"));
    }

    #[test]
    fn test_response_deserialization() {
        let json = r#"{
            "user": {
                "user_id": "test_user_123",
                "name": "Test User"
            }
        }"#;

        let response: CreateUserResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.user.user_id, Some("test_user_123".to_string()));
    }
}
```

### 2. 集成测试

#### 集成测试示例
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use testcontainers::*;

    #[tokio::test]
    #[ignore] // 需要真实API环境，默认忽略
    async fn test_real_api_call() {
        let config = Config::from_env().expect("缺少环境变量配置");
        let client = LarkClient::new(config);

        let result = client.contact.v3.user.get_user("test_user_id").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_with_mock_server() {
        // 使用mock server进行测试
        let mock_server = setup_mock_server().await;

        let config = Config::builder()
            .base_url(&mock_server.url())
            .app_id("test_app")
            .app_secret("test_secret")
            .build();

        let service = UserService::new(config);
        let result = service.get_user("test_user").await;

        assert!(result.is_ok());
    }
}
```

## 📚 开发流程

### 1. 环境准备

#### 开发环境设置
```bash
# 克隆项目
git clone https://github.com/your-org/open-lark.git
cd open-lark

# 安装Rust工具链
rustup update stable
rustup component add rustfmt clippy

# 安装开发工具
cargo install cargo-watch
cargo install cargo-expand

# 设置环境变量
cp .env-example .env
# 编辑.env文件，填入测试用的app_id和app_secret
```

#### IDE配置推荐
```json
// .vscode/settings.json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.loadOutDirsFromCheck": true,
    "rust-analyzer.procMacro.enable": true,
    "rust-analyzer.imports.granularity.group": "module",
    "editor.formatOnSave": true,
    "editor.codeActionsOnSave": {
        "source.fixAll": true
    }
}
```

### 2. 实现新API的步骤

#### Step 1: 需求分析
```bash
# 查看API文档
curl https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/api-reference

# 确认API参数和响应格式
# 分析端点、HTTP方法、请求/响应结构
```

#### Step 2: 创建数据模型
```rust
// 在相应的models.rs文件中定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewResourceRequest {
    // 请求字段
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NewResourceResponse {
    // 响应字段
}

impl ApiResponseTrait for NewResourceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
```

#### Step 3: 实现API方法
```rust
// 在相应的服务文件中实现
impl ResourceService {
    /// 新增资源
    ///
    /// API功能描述
    ///
    /// # API文档
    ///
    /// {DOCUMENTATION_URL}
    pub async fn create_resource(
        &self,
        request: &NewResourceRequest,
    ) -> SDKResult<NewResourceResponse> {
        // 实现逻辑
    }
}
```

#### Step 4: 添加Builder模式（可选）
```rust
impl ResourceService {
    pub fn create_resource_builder(&self) -> CreateResourceBuilder {
        CreateResourceBuilder::new()
    }
}
```

#### Step 5: 编写测试
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_create_resource() {
        // 测试实现
    }
}
```

#### Step 6: 更新模块导出
```rust
// 在mod.rs中添加新功能的导出
pub mod new_resource;
pub use new_resource::{NewResourceRequest, NewResourceResponse, ResourceService};
```

### 3. 代码审查清单

#### 功能性检查
- [ ] API实现符合官方文档规范
- [ ] 请求参数验证完整
- [ ] 响应数据解析正确
- [ ] 错误处理覆盖所有情况

#### 代码质量检查
- [ ] 代码风格符合项目规范
- [ ] 所有公共方法都有文档注释
- [ ] 变量和函数命名清晰
- [ ] 没有unused代码和警告

#### 测试检查
- [ ] 单元测试覆盖主要功能
- [ ] 边界条件测试完整
- [ ] 错误场景测试覆盖
- [ ] 集成测试通过

#### 性能检查
- [ ] 没有不必要的内存分配
- [ ] 异步操作正确使用
- [ ] 大数据量处理合理
- [ ] 没有性能瓶颈

## 🚀 最佳实践

### 1. 性能优化

#### 异步处理
```rust
// ✅ 好的做法：使用异步函数
pub async fn batch_get_users(&self, user_ids: Vec<String>) -> SDKResult<Vec<User>> {
    let futures: Vec<_> = user_ids
        .into_iter()
        .map(|id| self.get_user(&id))
        .collect();

    let results = futures::future::join_all(futures).await;
    let users = results.into_iter().collect::<Result<Vec<_>, _>>()?;
    Ok(users)
}

// ❌ 避免的做法：在异步函数中使用阻塞操作
pub async fn bad_example(&self) -> SDKResult<()> {
    std::thread::sleep(Duration::from_secs(1)); // 阻塞操作
    Ok(())
}
```

#### 内存管理
```rust
// ✅ 好的做法：使用引用避免不必要的数据复制
pub async fn update_user(
    &self,
    user_id: &str,
    update_data: &UpdateUserRequest,
) -> SDKResult<User> {
    // 实现...
}

// ✅ 使用Cow类型避免不必要的字符串分配
use std::borrow::Cow;

pub fn process_text(text: &str) -> Cow<str> {
    if text.contains("pattern") {
        Cow::Owned(text.replace("pattern", "replacement"))
    } else {
        Cow::Borrowed(text)
    }
}
```

### 2. 错误处理

#### 详细的错误信息
```rust
// ✅ 提供详细错误信息
pub async fn create_user(&self, request: &CreateUserRequest) -> SDKResult<CreateUserResponse> {
    if request.user.name.is_none() {
        return Err(LarkAPIError::illegal_param(
            "用户姓名不能为空".to_string()
        ));
    }

    if !is_valid_phone(&request.user.mobile) {
        return Err(LarkAPIError::illegal_param(
            format!("手机号格式无效: {}", request.user.mobile.unwrap_or_default())
        ));
    }

    // 继续处理...
}
```

#### 错误恢复机制
```rust
// ✅ 实现重试机制
pub async fn resilient_api_call<T>(
    &self,
    operation: impl Fn() -> Pin<Box<dyn Future<Output = SDKResult<T>>>>,
    max_retries: usize,
) -> SDKResult<T> {
    let mut attempts = 0;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) if attempts < max_retries && is_retryable_error(&error) => {
                attempts += 1;
                tokio::time::sleep(Duration::from_millis(1000 * attempts as u64)).await;
                continue;
            }
            Err(error) => return Err(error),
        }
    }
}
```

### 3. 安全考虑

#### 输入验证
```rust
// ✅ 验证所有输入参数
impl CreateUserRequest {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref name) = self.user.name {
            if name.len() > 100 {
                return Err(ValidationError::InvalidName("姓名长度不能超过100字符".to_string()));
            }
        } else {
            return Err(ValidationError::MissingName("姓名不能为空".to_string()));
        }

        if let Some(ref email) = self.user.email {
            if !email.contains('@') {
                return Err(ValidationError::InvalidEmail("邮箱格式无效".to_string()));
            }
        }

        Ok(())
    }
}
```

#### 敏感信息处理
```rust
// ✅ 避免在日志中记录敏感信息
impl std::fmt::Debug for CreateUserRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateUserRequest")
            .field("user", &"[REDACTED]")
            .field("user_id_type", &self.user_id_type)
            .finish()
    }
}

// ✅ 安全地处理密码等敏感数据
pub struct Password(String);

impl Password {
    pub fn new(password: &str) -> Self {
        Self(password.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Drop for Password {
    fn drop(&mut self) {
        // 清零内存中的密码数据
        self.0.clear();
        unsafe {
            std::ptr::write_bytes(self.0.as_mut_ptr(), 0, self.0.len());
        }
    }
}
```

## 📖 参考资料

### 官方文档
- [飞书开放平台API文档](https://open.feishu.cn/document)
- [Rust异步编程指南](https://rust-lang.github.io/async-book/)
- [Serde使用指南](https://serde.rs/)

### 相关工具
- [cargo-watch](https://github.com/passcod/cargo-watch) - 自动重新编译
- [cargo-expand](https://github.com/dtolnay/cargo-expand) - 宏展开工具
- [tokio](https://tokio.rs/) - 异步运行时

### 代码示例
- 项目中的现有实现是最好的参考
- 查看 `src/service/im/v1/` 了解完整的实现模式
- 参考 `src/service/contact/v3/user.rs` 了解复杂API的实现

---

*文档版本: 1.0*
*最后更新: 2025-10-21*
*维护者: Open-Lark 开发团队*