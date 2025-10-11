# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概览

**open-lark** 是一个为飞书开放平台构建的全面 Rust SDK，提供对 43 个服务模块中 291+ 个 API 的类型安全访问。专为企业应用设计，具备自动令牌管理、WebSocket 支持、事件处理和高级错误处理等功能。

### 核心特性
- **291+ API**: 完整覆盖飞书开放平台服务
- **功能标志**: 40+ 可选服务模块的模块化编译
- **企业级**: 高级错误处理、重试机制和监控
- **中文文档**: 100% 中文文档，专为中国开发者优化
- **构建器模式**: 现代化流畅 API 设计，类型安全
- **WebSocket 支持**: 实时事件处理能力

## 开发命令

### 核心开发
使用 `just`（推荐）或直接 cargo 命令：

```bash
# 代码格式化和检查
just fmt              # 格式化代码
just fmt-check        # 检查代码格式
just lint             # 运行 clippy 检查

# 构建和测试
just build            # 构建所有功能
just build-release    # 构建优化版本
just test             # 运行所有测试
just test-features-quick  # 测试常用功能组合

# 文档和覆盖率
just docs             # 生成文档
just coverage         # 运行覆盖率分析（需要 cargo-llvm-cov）

# 质量保证
just check-all        # 运行所有发布前检查
just audit            # 使用 cargo-deny 进行安全审计
```

### 功能标志测试
项目使用广泛的功能标志。测试不同配置：

```bash
# 测试单个功能
cargo test --no-default-features --features im
cargo test --no-default-features --features "cloud-docs,contact"

# 测试默认配置
cargo test

# 测试所有功能（全面但较慢）
cargo test --all-features
```

### 运行示例
示例在 `Cargo.toml` 中配置，展示特定功能：

```bash
# 基础设置
cargo run --example client_setup

# WebSocket 功能（需要 websocket 功能）
cargo run --example websocket_client --features websocket

# 服务特定示例（需要相应功能）
cargo run --example hire_v1_example --features hire
cargo run --example im_v1_demo --features im
cargo run --example ai_comprehensive --features ai
```

## 架构设计

### 高层结构

```
src/
├── client/          # 主 LarkClient，条件编译服务
├── core/            # HTTP 客户端、配置、错误处理、特征
├── service/         # 43+ 服务模块（im、hire、attendance 等）
├── event/           # 事件处理和 WebSocket 消息分发
├── card/            # 交互式卡片组件和构建器
├── custom_bot       # 自定义机器人功能
└── prelude.rs       # 常用导出，便于使用
```

### 服务架构模式

每个服务遵循一致的模式：
```rust
service/
├── mod.rs           # 带版本 API 的服务结构体
├── models/          # 数据模型和序列化
│   └── mod.rs
└── v1/              # API 版本实现
    ├── mod.rs
    └── specific_api.rs
```

### 客户端配置

主 `LarkClient` 使用条件编译根据启用的功能包含服务：

```rust
pub struct LarkClient {
    // 核心服务（默认功能）
    #[cfg(feature = "im")]
    pub im: ImService,
    #[cfg(feature = "contact")]
    pub contact: ContactService,

    // 高级服务（可选功能）
    #[cfg(feature = "hire")]
    pub hire: HireService,
    #[cfg(feature = "ai")]
    pub ai: AiService,
    // ... 40+ 更多条件服务
}
```

### 构建器模式实现

服务支持传统和现代构建器模式：

```rust
// 传统方法
let request = CreateUserRequest { user, user_id_type: Some("open_id".to_string()) };
let response = client.contact.v3.user.create(&request).await?;

// 现代构建器模式（推荐）
let response = client
    .contact
    .v3
    .user
    .create_user_builder()
    .user(user)
    .user_id_type("open_id")
    .execute(&client.contact.v3.user)
    .await?;
```

## 开发指南

### 添加新服务

1. **功能标志**: 在 `Cargo.toml` 功能部分添加
2. **服务结构**: 在 `src/service/` 中遵循既定模式
3. **条件编译**: 使用 `#[cfg(feature = "...")]` 添加到 `LarkClient`
4. **文档**: 包含全面的中文文档
5. **示例**: 在 `examples/api/` 创建演示示例
6. **测试**: 添加单元测试和集成测试

### 错误处理标准

使用标准化错误系统：
```rust
// 所有 API 方法返回 SDKResult<T>
pub async fn your_method(&self, request: YourRequest) -> SDKResult<YourResponse> {
    // 实现自动错误转换
}

// 带用户友好消息的错误处理
match result {
    Ok(response) => println!("✅ 成功: {:?}", response),
    Err(error) => {
        println!("❌ {}", error.user_friendly_message());
        // 错误分析和恢复建议
    }
}
```

### 文档要求

- **中文优先**: 所有文档应使用中文服务目标用户
- **完整示例**: 每个公共 API 必须有可运行示例
- **模块文档**: 解释业务价值的全面模块级文档
- **功能标志**: 清楚记录每个功能所需的功能依赖

### 代码质量标准

- **零警告**: 代码必须无任何警告编译
- **Clippy 合规**: 通过所有 clippy 检查，使用 `-Dwarnings`
- **格式一致性**: 使用 `cargo fmt` 保持格式一致
- **测试覆盖**: 新功能必须包含全面测试

## 测试策略

### 功能矩阵测试
项目包含广泛功能组合测试，确保模块化编译正确工作。

### API 一致性
`tools/` 中的工具验证 API 一致性和兼容性：
```bash
cargo run --bin api_consistency_checker
cargo run --bin enhanced_api_checker
```

### WebSocket 测试
WebSocket 功能需要特殊测试实时事件处理。

## 重要说明

### 功能标志至关重要
项目严重依赖功能标志进行模块化编译。始终：
- 测试不同功能组合
- 适当使用条件编译 `#[cfg(feature = "...")]`
- 清楚记录功能依赖

### 中国市场焦点
此 SDK 专为使用飞书（Lark 的中国版本）的中国开发者设计。所有文档、示例和错误消息应尽可能使用中文。

### 企业要求
代码库服务企业应用，需要：
- 全面的错误处理和恢复
- 广泛的日志记录和监控支持
- 类型安全保证
- 性能优化要求

### WebSocket 事件
使用 WebSocket 功能时，理解事件分发系统和处理程序注册模式。

## 配置

### 环境设置
复制 `.env-example` 到 `.env` 并配置：
```bash
APP_ID="your_app_id"
APP_SECRET="your_app_secret"
USER_ACCESS_TOKEN="test_user_access_token"
```

### 构建功能
常用功能组合：
- `default`: im、cloud-docs、contact、group、authentication、search
- `full`: 所有 40+ 服务（大二进制文件大小）
- 自定义: 选择特定服务以获得最小二进制文件大小

这是一个企业级 SDK，专注于可靠性、全面的 API 覆盖和在飞书平台上构建应用的中国开发者的优秀开发体验。