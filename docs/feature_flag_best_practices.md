# 功能标志最佳实践指南

**目标用户**: open-lark SDK 开发者
**最后更新**: 2025-11-05

## 🎯 概述

本指南提供使用 open-lark SDK 功能标志的最佳实践，帮助您构建高效、可维护的应用程序。

## 📊 功能标志架构

### 设计原则

1. **URL路径映射**: 功能标志基于API路径中的服务名称
2. **向后兼容**: 通过别名支持旧的功能标志名称
3. **模块化编译**: 只编译需要的功能，减小二进制大小
4. **类型安全**: 编译时确保功能可用性

### 映射规则

```rust
// API路径: /open-apis/{service}/{version}/{endpoint}
// 功能标志: {service}

// 示例:
"/open-apis/authen/v1/user_info"     → "authen" → "auth"
"/open-apis/contact/v3/users"         → "contact"
"/open-apis/drive/v1/files"           → "drive" → "cloud-docs"
"/open-apis/sheets/v4/spreadsheets"   → "sheets"
```

## 🚀 推荐用法

### 1. 项目配置

#### 最小化配置（推荐）

```toml
[dependencies.open-lark]
version = "0.1.0"
default-features = false
features = [
    "auth",           # 认证服务
    "im",             # 即时消息
    "contact",        # 联系人管理
    "cloud-docs",     # 云文档（包含docx和drive）
]
```

#### 完整功能配置

```toml
[dependencies.open-lark]
version = "0.1.0"
features = ["full"]  # 启用所有功能
```

#### 按需选择

```toml
[dependencies.open-lark]
version = "0.1.0"
default-features = false
features = [
    # 核心服务
    "auth",
    "im",
    "contact",

    # 业务特定服务
    "approval",       # 审批流程
    "attendance",     # 考勤管理
    "calendar",       # 日历集成

    # AI和分析
    "ai",             # AI功能
    "sheets",         # 电子表格
]
```

### 2. 代码中的条件编译

#### 功能检查模式

```rust
use open_lark::prelude::*;

struct MyApplication {
    client: LarkClient,
}

impl MyApplication {
    // 检查功能是否可用
    fn has_auth_feature() -> bool {
        cfg!(feature = "auth") || cfg!(feature = "authen")
    }

    fn has_docs_feature() -> bool {
        cfg!(feature = "cloud-docs") ||
        cfg!(feature = "docx") ||
        cfg!(feature = "drive")
    }

    // 根据功能提供不同实现
    async fn get_user_info(&self, user_id: &str) -> SDKResult<UserInfo> {
        #[cfg(any(feature = "auth", feature = "authen"))]
        {
            let request = GetUserInfoRequest::builder()
                .user_id(user_id)
                .user_id_type("open_id")
                .build();

            self.client.auth.v1.user.get_info(&request).await
        }

        #[cfg(not(any(feature = "auth", feature = "authen")))]
        {
            Err(SdkError::FeatureNotEnabled("auth".to_string()).into())
        }
    }
}
```

#### 服务模块模式

```rust
// 创建专门的服务包装器
pub struct AuthService {
    client: LarkClient,
}

impl AuthService {
    #[cfg(any(feature = "auth", feature = "authen"))]
    pub async fn get_current_user(&self) -> SDKResult<UserInfo> {
        let request = GetUserInfoRequest::default();
        self.client.auth.v1.user.get_info(&request).await
    }

    #[cfg(not(any(feature = "auth", feature = "authen")))]
    pub async fn get_current_user(&self) -> SDKResult<UserInfo> {
        Err(SdkError::FeatureNotEnabled("auth".to_string()).into())
    }
}

pub struct DocumentsService {
    client: LarkClient,
}

impl DocumentsService {
    #[cfg(any(feature = "cloud-docs", feature = "docx", feature = "drive"))]
    pub async fn list_files(&self, folder_token: &str) -> SDKResult<Vec<FileInfo>> {
        let request = FileListRequest::builder()
            .folder_token(folder_token)
            .build();

        self.client.cloud_docs.v1.drive.file_list(&request).await
    }

    #[cfg(not(any(feature = "cloud-docs", feature = "docx", feature = "drive")))]
    pub async fn list_files(&self, _folder_token: &str) -> SDKResult<Vec<FileInfo>> {
        Err(SdkError::FeatureNotEnabled("cloud-docs".to_string()).into())
    }
}
```

### 3. 错误处理最佳实践

#### 优雅降级

```rust
use open_lark::prelude::*;

pub async fn send_message_with_fallback(
    client: &LarkClient,
    message: &str
) -> Result<String, String> {
    // 尝试发送富文本消息
    #[cfg(feature = "im")]
    {
        match send_rich_message(client, message).await {
            Ok(response) => return Ok("消息已发送".to_string()),
            Err(_) => {
                // 降级到简单文本
            }
        }
    }

    // 简单文本消息（所有IM功能都支持）
    #[cfg(feature = "im")]
    {
        match send_text_message(client, message).await {
            Ok(response) => Ok("文本消息已发送".to_string()),
            Err(e) => Err(format!("发送失败: {}", e)),
        }
    }

    #[cfg(not(feature = "im"))]
    {
        Err("IM功能未启用".to_string())
    }
}
```

#### 功能可用性检查

```rust
pub struct FeatureChecker;

impl FeatureChecker {
    pub fn validate_required_features() -> Result<(), Vec<String>> {
        let mut missing_features = Vec::new();

        if !Self::has_auth() {
            missing_features.push("auth".to_string());
        }

        if !Self::has_im() {
            missing_features.push("im".to_string());
        }

        if missing_features.is_empty() {
            Ok(())
        } else {
            Err(missing_features)
        }
    }

    pub fn has_auth() -> bool {
        cfg!(any(feature = "auth", feature = "authen"))
    }

    pub fn has_im() -> bool {
        cfg!(feature = "im")
    }

    pub fn has_cloud_docs() -> bool {
        cfg!(any(feature = "cloud-docs", feature = "docx", feature = "drive"))
    }
}

// 应用启动时检查
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 检查必需的功能
    match FeatureChecker::validate_required_features() {
        Ok(_) => println!("✅ 所有必需功能已启用"),
        Err(missing) => {
            eprintln!("❌ 缺少必需功能: {:?}", missing);
            eprintln!("请在 Cargo.toml 中添加: features = [\"{}\"]",
                     missing.join("\", \""));
            return Err("缺少必需功能".into());
        }
    }

    // 继续应用初始化...
    Ok(())
}
```

## 🏗️ 架构模式

### 1. 分层架构

```rust
// 应用层
pub struct Application {
    client: LarkClient,
    services: ServiceContainer,
}

pub struct ServiceContainer {
    #[cfg(any(feature = "auth", feature = "authen"))]
    auth: AuthService,

    #[cfg(feature = "im")]
    im: MessageService,

    #[cfg(any(feature = "cloud-docs", feature = "docx", feature = "drive"))]
    docs: DocumentsService,
}

// 业务逻辑层
impl Application {
    pub async fn handle_user_request(&self, request: UserRequest) -> SDKResult<Response> {
        match request.request_type {
            RequestType::GetUserInfo => {
                #[cfg(any(feature = "auth", feature = "authen"))]
                {
                    self.services.auth.get_user_info(&request.user_id).await
                }

                #[cfg(not(any(feature = "auth", feature = "authen")))]
                {
                    Err(SdkError::FeatureNotEnabled("auth".to_string()).into())
                }
            },

            RequestType::SendMessage => {
                #[cfg(feature = "im")]
                {
                    self.services.im.send_message(&request.message).await
                }

                #[cfg(not(feature = "im"))]
                {
                    Err(SdkError::FeatureNotEnabled("im".to_string()).into())
                }
            },
        }
    }
}
```

### 2. 插件化架构

```rust
pub trait ServicePlugin {
    type Config;
    type Error;

    fn name(&self) -> &'static str;
    fn is_enabled(&self) -> bool;
    fn initialize(&mut self, client: &LarkClient, config: Self::Config) -> Result<(), Self::Error>;
}

pub struct AuthPlugin {
    enabled: bool,
}

impl ServicePlugin for AuthPlugin {
    type Config = AuthConfig;
    type Error = SdkError;

    fn name(&self) -> &'static str {
        "auth"
    }

    fn is_enabled(&self) -> bool {
        cfg!(any(feature = "auth", feature = "authen")) && self.enabled
    }

    fn initialize(&mut self, client: &LarkClient, config: AuthConfig) -> Result<(), Self::Error> {
        if self.is_enabled() {
            // 初始化认证服务
            self.enabled = true;
            println!("✅ 认证插件已启用");
        }
        Ok(())
    }
}

// 插件管理器
pub struct PluginManager {
    plugins: HashMap<String, Box<dyn ServicePlugin>>,
}

impl PluginManager {
    pub fn register<P: ServicePlugin + 'static>(&mut self, plugin: P) {
        self.plugins.insert(plugin.name().to_string(), Box::new(plugin));
    }

    pub fn initialize_all(&mut self, client: &LarkClient) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        for (name, plugin) in &mut self.plugins {
            if plugin.is_enabled() {
                if let Err(e) = plugin.initialize(client, Default::default()) {
                    errors.push(format!("插件 {} 初始化失败: {}", name, e));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

## 📈 性能优化

### 1. 编译时优化

```toml
# 生产环境配置
[dependencies.open-lark]
version = "0.1.0"
default-features = false
features = [
    # 只启用需要的功能
    "auth",
    "im",
    "contact",
]
# 禁用不需要的默认功能
default-features = false

# 开发环境配置
[dev-dependencies.open-lark]
version = "0.1.0"
features = ["full"]  # 开发时启用所有功能进行测试
```

### 2. 运行时优化

```rust
// 懒加载服务
pub struct LazyService<T> {
    service: Option<T>,
    factory: Box<dyn Fn() -> T>,
}

impl<T> LazyService<T> {
    pub fn new<F>(factory: F) -> Self
    where
        F: Fn() -> T + 'static
    {
        Self {
            service: None,
            factory: Box::new(factory),
        }
    }

    pub fn get(&mut self) -> &T {
        if self.service.is_none() {
            self.service = Some((self.factory)());
        }
        self.service.as_ref().unwrap()
    }
}

// 使用示例
pub struct LazyServices {
    #[cfg(any(feature = "auth", feature = "authen"))]
    auth: LazyService<AuthService>,

    #[cfg(feature = "im")]
    im: LazyService<MessageService>,
}

impl LazyServices {
    pub fn new(client: LarkClient) -> Self {
        Self {
            #[cfg(any(feature = "auth", feature = "authen"))]
            auth: LazyService::new(move || AuthService::new(client.clone())),

            #[cfg(feature = "im")]
            im: LazyService::new(move || MessageService::new(client.clone())),
        }
    }
}
```

## 🧪 测试策略

### 1. 功能矩阵测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // 测试必需功能
    #[test]
    fn test_required_features() {
        assert!(cfg!(any(feature = "auth", feature = "authen")),
                "认证功能是必需的");
    }

    // 测试功能组合
    #[test]
    fn test_feature_combinations() {
        let has_im = cfg!(feature = "im");
        let has_auth = cfg!(any(feature = "auth", feature = "authen"));

        if has_im && !has_auth {
            panic!("IM功能需要认证功能支持");
        }
    }

    // 集成测试
    #[tokio::test]
    #[cfg(any(feature = "auth", feature = "authen"))]
    async fn test_auth_service_integration() {
        let client = create_test_client().await;
        let result = client.auth.v1.user.get_info(&test_request()).await;

        match result {
            Ok(_) => println!("✅ 认证服务集成测试通过"),
            Err(e) => println!("⚠️ 认证服务测试失败（可能是网络问题）: {}", e),
        }
    }
}
```

### 2. 模拟测试

```rust
#[cfg(test)]
mod mock_tests {
    use super::*;

    // 模拟服务用于测试
    pub struct MockAuthService {
        should_fail: bool,
    }

    impl MockAuthService {
        pub async fn get_user_info(&self, user_id: &str) -> SDKResult<UserInfo> {
            if self.should_fail {
                Err(SdkError::NetworkError("模拟网络错误".to_string()).into())
            } else {
                Ok(UserInfo {
                    user_id: user_id.to_string(),
                    name: "测试用户".to_string(),
                })
            }
        }
    }

    #[tokio::test]
    async fn test_error_handling() {
        let service = MockAuthService { should_fail: true };
        let result = service.get_user_info("test_user").await;

        assert!(result.is_err());
        println!("✅ 错误处理测试通过");
    }
}
```

## 📋 部署检查清单

### 开发阶段

- [ ] 明确定义所需的功能标志
- [ ] 在 `Cargo.toml` 中正确配置功能
- [ ] 实现条件编译和错误处理
- [ ] 编写功能矩阵测试

### 测试阶段

- [ ] 验证所有功能组合正常工作
- [ ] 测试功能未启用时的错误处理
- [ ] 运行功能标志验证工具
- [ ] 检查编译后二进制大小

### 生产部署

- [ ] 确认生产环境功能标志配置
- [ ] 验证所有必需功能已启用
- [ ] 测试生产环境的功能可用性
- [ ] 监控功能使用情况

## 🆘 故障排除

### 常见问题

1. **功能未找到错误**
   ```rust
   // 检查功能是否正确启用
   if !cfg!(feature = "auth") {
       eprintln!("请在 Cargo.toml 中添加 auth 功能");
   }
   ```

2. **编译错误**
   ```bash
   # 检查功能标志拼写
   cargo check --features list  # 查看可用功能
   ```

3. **运行时错误**
   ```rust
   // 添加详细的错误信息
   match result {
     Ok(data) => data,
     Err(e) => {
         eprintln!("功能调用失败: {}", e);
         eprintln!("请检查相关功能标志是否启用");
         return Err(e);
     }
   }
   ```

### 调试工具

```bash
# 验证功能映射
cargo run --bin feature_flag_validator

# 检查特定功能
cargo test --no-default-features --features "auth,im"

# 查看编译后的功能
cargo tree --format "{p}" | grep open-lark
```

---

**遵循这些最佳实践**，您将能够：
- 🎯 构建高效、可维护的应用程序
- 🔧 优化编译时间和二进制大小
- 📚 提供良好的开发体验
- 🚀 确保生产环境稳定性

*最后更新: 2025-11-05*