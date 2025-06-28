# CLAUDE.md

本文件为 Claude Code (claude.ai/code) 在此代码库中工作提供指导。

与我交互使用中文，git 提交信息使用英文，报告生成到 reports 目录

## 项目概述

open-lark 是飞书开放平台的非官方 Rust SDK，支持自定义机器人、长连接机器人、云文档、飞书卡片、消息、群组、招聘管理等 API 调用。

## 开发命令

### 构建和测试

docs/apis 目录下是接口文档链接地址

编写接口前，参考之前的 API 写法

生成代码后，先格式化，再代码检查

examples 需要用 dotenvy 读取本地 .env 配置

在 examples/api 目录下，建立每个接口的单独 example

发布版本前，要更新 Changelog 和 README

```bash
# 构建项目
just build
# 或者: cargo build --all-features

# 构建发布版本
just build-release

# 运行测试
just test
# 或者: cargo test --all-features

# 运行特定测试
cargo test test_name

# 运行测试并显示输出
cargo test -- --nocapture

# 代码检查
just lint

# 格式化代码
just fmt

# 检查代码格式
just fmt-check

# 生成文档
just docs

# 运行所有发布前检查
just check-all

# 显示所有可用命令
just help
```

### 发布流程

```bash
# 发布新版本 (例如: 0.11.0)
just release 0.11.0

# 这将会:
# 1. 检查是否在 main 分支且工作目录干净
# 2. 拉取最新更改
# 3. 验证版本是否匹配 Cargo.toml
# 4. 运行所有发布前检查 (格式化, 检查, 测试, 构建, 文档)
# 5. 检查 CHANGELOG.md 是否有该版本条目
# 6. 创建并推送 git 标签
# 7. 触发 GitHub Actions 自动发布
```

### 运行示例

```bash
# 运行特定示例 (需要 .env 文件配置 API 凭据)
cargo run --example create_message
cargo run --example ws_client --features websocket

# 列出所有可用示例
cargo build --examples
```

## 架构概述

### 核心组件

- **LarkClient**: 聚合所有服务模块的主客户端类
- **传输层**: 在 `core/http.rs` 中统一处理 HTTP 请求/响应
- **令牌管理**: 在 `core/token_manager.rs` 中自动缓存和刷新访问令牌
- **配置**: 在 `core/config.rs` 中集中配置管理

### 服务组织

服务按功能域组织在 `src/service/` 下:

- `authentication/`: 用户认证 (v1)
- `bitable/`: 多维表格 (v1)
- `drive/`: 云盘文件 (v1, v2)
- `hire/`: 招聘管理 (v1)
- `im/`: 即时消息 (v1, v2)
- `search/`: 搜索功能 (v1)
- `sheets/`: 电子表格 (v2, v3)

每个服务遵循基于版本的 API 组织 (v1, v2, v3)，具有标准化的请求/响应模式。

### 事件系统

- **EventDispatcherHandler**: 在 `src/event/dispatcher/mod.rs` 中的中央事件分发器
- **构建器模式**: 用于注册事件处理器，如 `.register_p2_im_message_receive_v1()` 等
- **版本支持**: 自动处理 v1 (p1) 和 v2 (p2) 事件格式
- **类型安全**: 使用特征和泛型实现编译时安全
- **WebSocket 支持**: 通过 WebSocket 连接进行实时事件处理 (可选功能)

### 认证流程

- 支持多种令牌类型: App Access Token, Tenant Access Token, User Access Token
- 自动令牌缓存和刷新机制
- 通过 `Config` 结构体进行可配置的认证选项

### 关键设计模式

- **传输模式**: 所有 API 请求通过 `Transport<T>` 进行一致处理
- **构建器模式**: 用于客户端配置和事件处理器注册
- **类型安全**: 广泛使用 Rust 的类型系统和特征
- **异步/等待**: 整个代码库的完全异步支持
- **错误处理**: 统一的 `LarkAPIError` 和 `SDKResult<T>` 类型

## 配置

基于 `.env-example` 创建 `.env` 文件，配置你的 API 凭据:

```
APP_ID=your_app_id
APP_SECRET=your_app_secret
USER_ACCESS_TOKEN=your_user_access_token  # 可选: 用于用户特定操作
```

## 功能特性

- 默认: 基本 API 功能
- `websocket`: 启用 WebSocket 支持以进行实时事件处理 (`ws_client` 示例需要)

## 使用示例

示例具有详尽的文档，位于 `examples/` 目录中。它们按以下方式组织:

- `api/`: 按服务和版本的 API 使用示例
- `card/`: 飞书卡片组件示例
- 根目录: 高级集成示例

添加新示例时，遵循现有的命名约定，并在 `Cargo.toml` 中添加相应的 `[[example]]` 条目。

## 客户端使用模式

### 基本客户端设置

``` norun
use open_lark::prelude::*;

let client = LarkClient::builder(app_id, app_secret)
    .with_app_type(AppType::SelfBuilt)  // 或者 AppType::Marketplace
    .with_enable_token_cache(true)
    .build();
```

### 错误处理

所有 API 调用返回 `SDKResult<T>`，它包装了 `Result<T, LarkAPIError>`。始终适当地处理错误:

``` norun
match client.im.create_message(&req).await {
    Ok(response) => println!("成功: {:?}", response),
    Err(e) => eprintln!("错误: {:?}", e),
}
```

### 事件处理

``` norun
let handler = EventDispatcherHandler::builder()
    .register_p2_im_message_receive_v1(|event| {
        println!("收到消息: {:?}", event);
    })
    .build();
```

## Protobuf 集成

项目在 `crates/protobuf/` 中包含用于 WebSocket 通信的 protobuf 定义。构建脚本自动从 `.proto` 文件生成 Rust 代码。