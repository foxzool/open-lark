# OpenLark SDK 示例代码库

本示例代码库展示了如何使用 OpenLark SDK 与飞书开放平台进行交互。示例按照混合架构组织，既提供循序渐进的学习路径，也方便快速查找特定功能。

## 📚 目录结构

### 🚀 [基础入门](./basic/)
适合新用户的完整学习路径，从客户端创建到基础API调用：

- **[basic_introduction.rs](./basic_introduction.rs)** - 完整基础教程（推荐入门，整合6章内容）
- **[00_client_setup.rs](./basic/00_client_setup.rs)** - 客户端建立（支持传统方式和SharedConfig）
- **[01_authentication.rs](./basic/01_authentication.rs)** - 认证机制和权限管理
- **[02_first_api_call.rs](./basic/02_first_api_call.rs)** - 第一个API调用示例
- **[03_error_handling.rs](./basic/03_error_handling.rs)** - 错误处理和异常管理

### 🛠️ [服务模块](./services/)
按飞书服务模块组织，方便快速查找特定功能：

#### 通讯协作 ([communication](./services/communication/))
- **IM消息** - 发送文本、图片、文件等各类消息
- **联系人管理** - 用户信息获取和管理
- **群组管理** - 群聊创建、成员管理等

#### 文档管理 ([documents](./services/documents/))
- **文件操作** - 文件上传、下载、分享
- **电子表格** - 表格数据读写和格式化
- **知识库** - Wiki文档创建和协作

#### 人力资源 ([hr](./services/hr/))
- **考勤跟踪** - 打卡记录、考勤统计
- **招聘管理** - 职位发布、候选人管理
- **组织架构** - 部门、员工信息管理

#### 智能服务 ([ai](./services/ai/))
- **AI助手** - 智能对话和分析
- **文档分析** - OCR、翻译、内容提取

### 🎯 [常用模式](./patterns/)
开发中的常用模式和最佳实践：

- **[builder_pattern.rs](./patterns/builder_pattern.rs)** - 构建器模式使用
- **[async_operations.rs](./patterns/async_operations.rs)** - 异步操作最佳实践
- **[batch_processing.rs](./patterns/batch_processing.rs)** - 批量数据处理
- **[websocket_events.rs](./patterns/websocket_events.rs)** - WebSocket事件处理

### 🏢 [企业级场景](./enterprise/)
企业级应用的部署和集成示例：

- **[multi_tenant_setup.rs](./enterprise/multi_tenant_setup.rs)** - 多租户配置
- **[service_integration.rs](./enterprise/service_integration.rs)** - 多服务集成
- **[production_deployment.rs](./enterprise/production_deployment.rs)** - 生产环境部署

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

# 客户端建立示例
cargo run --example 00_client_setup

# 第一个API调用
cargo run --example 02_first_api_call
```

运行服务特定示例（需要启用对应功能标志）：

```bash
# IM消息示例
cargo run --example im_messaging --features communication

# 文档操作示例
cargo run --example file_operations --features cloud-docs

# AI服务示例
cargo run --example intelligent_services --features ai
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