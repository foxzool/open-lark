# OpenLark SDK 示例代码库

本示例代码库展示了如何使用 OpenLark SDK 与飞书开放平台进行交互。示例按照功能和使用场景组织，既提供循序渐进的学习路径，也方便快速查找特定功能。

## 📚 目录结构

### 🚀 基础入门
适合新用户的完整学习路径，从客户端创建到基础API调用：

- **[basic_introduction.rs](./basic_introduction.rs)** - 完整基础教程（推荐入门，展示SDK核心功能）
- **[quick_start.rs](./quick_start.rs)** - 快速开始示例，简洁的客户端设置和API调用
- **[feature_demo.rs](./feature_demo.rs)** - 功能标志演示，展示模块化编译特性
- **[ws_client_example.rs](./ws_client_example.rs)** - WebSocket客户端连接和事件处理示例

### 🛠️ [服务模块](./services/)
按飞书服务模块组织，方便快速查找特定功能：

#### 通讯协作 ([communication](./services/communication/))
- **[im_messaging.rs](./services/communication/im_messaging.rs)** - IM消息发送（文本、图片、文件等）
- ~~cardkit_example.rs~~ - CardKit消息卡片创建和发送（暂时禁用，修复中）

### 🎯 高级功能
展示SDK的高级特性和最佳实践：

- **[service_registry_demo.rs](./service_registry_demo.rs)** - 服务注册和使用演示
- **[feature_combination_examples.rs](./feature_combination_examples.rs)** - 多功能组合使用示例

## 🚀 快速开始

### 1. 环境配置

复制环境变量配置文件：

```bash
cp .env-example .env
# 编辑 .env 文件，填入你的应用配置
```

必需的环境变量：

```bash
APP_ID=your_app_id
APP_SECRET=your_app_secret
USER_ACCESS_TOKEN=your_user_access_token  # 可选，用于用户相关API
```

### 2. 运行示例

运行基础示例：

```bash
# 完整基础教程（推荐新用户）
cargo run --example basic_introduction

# 快速开始示例
cargo run --example quick_start

# 功能标志演示
cargo run --example feature_demo

# WebSocket客户端
cargo run --example ws_client_example --features websocket
```

运行服务特定示例（需要启用对应功能标志）：

```bash
# IM消息示例
cargo run --example im_messaging --features communication

# CardKit消息卡片示例（暂时禁用，修复中）
# cargo run --example cardkit_example --features communication

# 服务注册演示
cargo run --example service_registry_demo --features client

# 多功能组合示例
cargo run --example feature_combination_examples --features "communication,client,auth"
```

### 3. 功能标志

本SDK支持按需编译，使用功能标志减少二进制文件大小：

```toml
[dependencies]
open-lark = { version = "0.13.2", features = [
    "communication",  # 即时消息、联系人
    "cloud-docs",     # 文档、表格、Wiki
    "hr",             # 人力资源
    "ai",             # 智能服务
    "client"          # 统一客户端
] }
```

## 🏗️ 架构说明

### 新的 Crates 架构

项目已迁移到模块化 crates 架构，主要模块包括：

- **openlark-core** - 核心基础设施（HTTP客户端、配置、错误处理）
- **openlark-client** - 高级客户端接口（LarkClient、服务注册）
- **openlark-communication** - 通讯服务模块
- **openlark-docs** - 文档服务模块
- **openlark-hr** - 人力资源服务模块
- **openlark-ai** - 智能服务模块

### 客户端创建方式

#### 传统方式（向后兼容）

```rust
use open_lark::prelude::*;

let client = LarkClient::builder("app_id", "app_secret")
    .with_app_type(AppType::SelfBuild)
    .with_enable_token_cache(true)
    .build()?;
```

#### SharedConfig 方式（推荐）

```rust
use open_lark::prelude::*;
use open_lark::service_registry::{SharedConfig, SharedConfigFactory};

let shared_config = SharedConfigFactory::create_shared(
    ConfigBuilder::default()
        .app_id("app_id")
        .app_secret("app_secret")
        .app_type(AppType::SelfBuild)
        .enable_token_cache(true)
        .build(),
);

let client = LarkClient::new(shared_config.config().clone());
```

## 💡 最佳实践

1. **使用 SharedConfig** - 多服务场景下显著优化内存使用
2. **启用功能标志** - 仅编译需要的服务，减少二进制大小
3. **错误处理** - 参考错误处理示例，实现健壮的应用
4. **异步优先** - 所有API调用都是异步的，正确使用 async/await
5. **环境配置** - 使用环境变量管理敏感信息

## 🤝 贡献

欢迎提交新的示例代码！请遵循以下规范：

- 代码风格与项目保持一致
- 包含完整的中文注释和说明
- 展示错误处理和最佳实践
- 通过 `cargo check` 和 `cargo clippy` 检查

## 📖 更多资源

- [项目文档](../README.md)
- [API参考文档](https://docs.rs/open-lark)
- [问题反馈](https://github.com/foxzool/open-lark/issues)
- [讨论区](https://github.com/foxzool/open-lark/discussions)

---

**注意**：本项目正在进行架构重构，示例代码会持续更新以反映最新的API和最佳实践。