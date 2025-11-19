# open-lark 快速开始指南

**目标用户**: 新手开发者
**最后更新**: 2025-11-05

## 🚀 欢迎使用 open-lark

open-lark 是飞书开放平台的官方 Rust SDK，提供对 1551+ 个 API 的类型安全访问。本指南将帮助您在 5 分钟内构建第一个飞书应用。

## 📋 前置要求

- Rust 1.70+
- 飞书开发者账号
- 创建的应用凭证（App ID 和 App Secret）

## 🛠️ 安装配置

### 1. 创建新项目

```bash
cargo new my_lark_app
cd my_lark_app
```

### 2. 添加依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
open-lark = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

### 3. 配置功能标志

根据需要选择功能：

```toml
[dependencies.open-lark]
version = "0.1.0"
default-features = false
features = [
    # "auth",         # 认证服务（现在默认启用，禁用时才需要显式指定）
    "im",             # 即时消息
    "contact",        # 联系人管理
    "cloud-docs",     # 云文档
]
```

### 4. 设置环境变量

创建 `.env` 文件：

```bash
APP_ID="your_app_id"
APP_SECRET="your_app_secret"
```

## 🎯 第一个应用：消息机器人

### 1. 基础客户端设置

```rust
// src/main.rs
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化客户端
    let client = LarkClient::builder(
        std::env::var("APP_ID")?,
        std::env::var("APP_SECRET")?
    )
    .with_enable_token_cache(true)  // 启用令牌缓存
    .build();

    println!("✅ 客户端初始化成功");
    Ok(())
}
```

### 2. 发送第一条消息

```rust
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LarkClient::builder(
        std::env::var("APP_ID")?,
        std::env::var("APP_SECRET")?
    )
    .build();

    // 发送文本消息
    #[cfg(feature = "im")]
    {
        let request = SendMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(SendMessageRequestBody::builder()
                .receive_id("user_open_id_here")
                .msg_type("text")
                .content(r#"{"text":"Hello from open-lark!"}"#)
                .build())
            .build();

        match client.im.v1.message.send(&request).await {
            Ok(response) => println!("✅ 消息发送成功: {:?}", response),
            Err(e) => println!("❌ 消息发送失败: {}", e),
        }
    }

    #[cfg(not(feature = "im"))]
    {
        println!("❌ 请在 Cargo.toml 中启用 'im' 功能");
    }

    Ok(())
}
```

### 3. 获取用户信息

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LarkClient::builder(
        std::env::var("APP_ID")?,
        std::env::var("APP_SECRET")?
    )
    .build();

    // 获取用户信息
    #[cfg(any(feature = "auth", feature = "authen"))]
    {
        let request = GetUserInfoRequest::builder()
            .user_id("user_open_id_here")
            .user_id_type("open_id")
            .build();

        match client.auth.v1.user.get_info(&request).await {
            Ok(user_info) => {
                println!("✅ 用户信息: {}", user_info.name);
                println!("邮箱: {}", user_info.email.unwrap_or_default());
            },
            Err(e) => println!("❌ 获取用户信息失败: {}", e),
        }
    }

    Ok(())
}
```

## 🏗️ 常用功能示例

### 1. 联系人管理

```rust
#[cfg(feature = "contact")]
async fn list_users(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    let request = GetUserListRequest::builder()
        .page_size(20)
        .user_id_type("open_id")
        .build();

    match client.contact.v3.user.get_list(&request).await {
        Ok(response) => {
            println!("✅ 找到 {} 个用户", response.data.items.len());
            for user in response.data.items {
                println!("- {} ({})", user.name, user.user_id);
            }
        },
        Err(e) => println!("❌ 获取用户列表失败: {}", e),
    }

    Ok(())
}
```

### 2. 云文档操作

```rust
#[cfg(any(feature = "cloud-docs", feature = "docx", feature = "drive"))]
async fn list_files(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    let request = FileListRequest::builder()
        .folder_token("root_folder_token")
        .page_size(50)
        .build();

    match client.cloud_docs.v1.drive.file_list(&request).await {
        Ok(response) => {
            println!("✅ 找到 {} 个文件", response.data.items.len());
            for file in response.data.items {
                println!("- {} (类型: {})", file.name, file.type_);
            }
        },
        Err(e) => println!("❌ 获取文件列表失败: {}", e),
    }

    Ok(())
}
```

### 3. 完整的应用结构

```rust
use open_lark::prelude::*;

struct LarkBot {
    client: LarkClient,
}

impl LarkBot {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = LarkClient::builder(
            std::env::var("APP_ID")?,
            std::env::var("APP_SECRET")?
        )
        .with_enable_token_cache(true)
        .build();

        Ok(Self { client })
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🤖 飞书机器人启动中...");

        // 检查功能
        self.check_features().await?;

        // 主循环
        self.run_main_loop().await
    }

    async fn check_features(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 检查功能状态:");

        #[cfg(any(feature = "auth", feature = "authen"))]
        println!("  ✅ 认证功能已启用");

        #[cfg(feature = "im")]
        println!("  ✅ 即时消息功能已启用");

        #[cfg(feature = "contact")]
        println!("  ✅ 联系人功能已启用");

        #[cfg(any(feature = "cloud-docs", feature = "docx", feature = "drive"))]
        println!("  ✅ 云文档功能已启用");

        Ok(())
    }

    async fn run_main_loop(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 这里可以实现您的业务逻辑
        println!("✅ 机器人运行中...");

        // 示例：发送欢迎消息
        #[cfg(feature = "im")]
        {
            if let Err(e) = self.send_welcome_message().await {
                println!("⚠️ 发送欢迎消息失败: {}", e);
            }
        }

        Ok(())
    }

    #[cfg(feature = "im")]
    async fn send_welcome_message(&self) -> Result<(), Box<dyn std::error::Error>> {
        let request = SendMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(SendMessageRequestBody::builder()
                .receive_id("admin_user_id")  // 替换为管理员用户ID
                .msg_type("text")
                .content(r#"{"text":"🤖 飞书机器人已上线！"}"#)
                .build())
            .build();

        self.client.im.v1.message.send(&request).await?;
        println!("✅ 欢迎消息已发送");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bot = LarkBot::new()?;
    bot.start().await
}
```

## 🔧 高级配置

### 1. 自定义HTTP客户端

```rust
use open_lark::prelude::*;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 自定义HTTP客户端
    let http_client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let client = LarkClient::builder(
        std::env::var("APP_ID")?,
        std::env::var("APP_SECRET")?
    )
    .with_http_client(http_client)
    .with_enable_token_cache(true)
    .build();

    println!("✅ 自定义客户端初始化成功");
    Ok(())
}
```

### 2. 错误处理最佳实践

```rust
use open_lark::prelude::*;

async fn safe_api_call<T>(
    operation: impl std::future::Future<Output = SDKResult<T>>,
    operation_name: &str
) -> Option<T> {
    match operation.await {
        Ok(result) => {
            println!("✅ {} 成功", operation_name);
            Some(result)
        },
        Err(error) => {
            eprintln!("❌ {} 失败: {}", operation_name, error.user_friendly_message());

            // 根据错误类型进行不同处理
            match error {
                SdkError::NetworkError(_) => {
                    println!("💡 建议检查网络连接");
                },
                SdkError::AuthenticationError(_) => {
                    println!("💡 建议检查应用凭证");
                },
                SdkError::FeatureNotEnabled(feature) => {
                    println!("💡 请在 Cargo.toml 中启用 '{}' 功能", feature);
                },
                _ => {
                    println!("💡 请查看错误详情并重试");
                }
            }

            None
        }
    }
}

// 使用示例
async fn get_user_safely(client: &LarkClient, user_id: &str) -> Option<UserInfo> {
    #[cfg(any(feature = "auth", feature = "authen"))]
    {
        let request = GetUserInfoRequest::builder()
            .user_id(user_id)
            .user_id_type("open_id")
            .build();

        safe_api_call(client.auth.v1.user.get_info(&request), "获取用户信息").await
    }

    #[cfg(not(any(feature = "auth", feature = "authen")))]
    {
        eprintln!("❌ 认证功能未启用");
        None
    }
}
```

### 3. 日志配置

```rust
use open_lark::prelude::*;

fn setup_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging();

    let client = LarkClient::builder(
        std::env::var("APP_ID")?,
        std::env::var("APP_SECRET")?
    )
    .build();

    log::info!("✅ 应用初始化完成");
    Ok(())
}
```

## 🧪 测试您的应用

### 1. 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        // 注意：这里需要有效的环境变量
        if let (Ok(app_id), Ok(app_secret)) = (
            std::env::var("APP_ID"),
            std::env::var("APP_SECRET")
        ) {
            let client = LarkClient::builder(app_id, app_secret).build();
            assert!(true, "客户端创建成功");
        }
    }

    #[tokio::test]
    #[cfg(any(feature = "auth", feature = "authen"))]
    async fn test_user_info() {
        // 这个测试需要有效的用户ID和网络连接
        // 在实际测试中应该使用模拟数据
    }
}
```

### 2. 集成测试

```rust
// tests/integration_test.rs
use open_lark::prelude::*;

#[tokio::test]
async fn test_full_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let client = LarkClient::builder(
        std::env::var("APP_ID")?,
        std::env::var("APP_SECRET")?
    )
    .build();

    // 测试认证功能
    #[cfg(any(feature = "auth", feature = "authen"))]
    {
        println!("测试认证功能...");
        // 添加认证测试代码
    }

    // 测试消息功能
    #[cfg(feature = "im")]
    {
        println!("测试消息功能...");
        // 添加消息测试代码
    }

    Ok(())
}
```

## 📚 下一步

恭喜！您已经成功创建了第一个飞书应用。接下来可以：

### 学习更多功能
- 📖 [功能标志最佳实践](feature_flag_best_practices.md)
- 🔄 [功能标志迁移指南](feature_flag_migration_guide.md)
- 📋 [完整API文档](https://docs.open-lark.com)

### 探索更多服务
- 📅 日历集成 (`calendar`)
- 📊 电子表格 (`sheets`)
- 🤖 AI 功能 (`ai`)
- 👥 人事管理 (`hire`)

### 社区资源
- 💬 [GitHub Discussions](https://github.com/open-lark/open-lark/discussions)
- 🐛 [报告问题](https://github.com/open-lark/open-lark/issues)
- 📧 [邮件支持](mailto:support@open-lark.com)

## 🆘 获取帮助

如果遇到问题：

1. **查看错误信息**: open-lark 提供详细的错误说明和修复建议
2. **检查功能标志**: 确保在 `Cargo.toml` 中正确配置了所需功能
3. **验证凭证**: 确认 App ID 和 App Secret 正确
4. **查看文档**: 阅读相关功能的详细文档
5. **社区求助**: 在 GitHub 上提问或搜索类似问题

---

**开始构建您的飞书应用吧！** 🚀

*最后更新: 2025-11-05*