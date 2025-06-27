# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

和我交互使用中文, git 注释使用中文, 报告生成到 reports 目录

## Project Overview

open-lark 是飞书开放平台的非官方 Rust SDK，支持自定义机器人、长连接机器人、云文档、飞书卡片、消息、群组等 API 调用。

## Development Commands

### Building and Testing

docs/apis 目录下是接口文档链接地址

编写接口前, 参考之前的api写法

生成代码后, 先格式化, 再代码检查

examples 需要用 dotenvy 读取 本地.env配置

在 examples/api目录下, 建立每个接口的单独example

发布版本前, 要更新Changelog 和 README

```bash
# Build the project
just build
# or: cargo build --all-features

# Build release version
just build-release

# Run tests
just test
# or: cargo test --all-features

# Run a specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Lint the code
just lint

# Format code
just fmt

# Check code formatting
just fmt-check

# Generate documentation
just docs

# Run all pre-release checks
just check-all

# Show all available commands
just help
```

### Release Process

```bash
# Release a new version (e.g., 0.4.0)
just release 0.4.0

# This will:
# 1. Check if on main branch and working directory is clean
# 2. Pull latest changes
# 3. Verify version matches Cargo.toml
# 4. Run all pre-release checks (format, lint, test, build, docs)
# 5. Check if CHANGELOG.md has entry for the version
# 6. Create and push git tag
# 7. Trigger GitHub Actions for automated release
```

### Running Examples

```bash
# Run specific example (requires .env file with API credentials)
cargo run --example create_message
cargo run --example ws_client --features websocket

# List all available examples
cargo build --examples
```

## Architecture Overview

### Core Components

- **LarkClient**: Main client class that aggregates all service modules
- **Transport Layer**: Unified HTTP request/response handling in `core/http.rs`
- **Token Management**: Automatic access token caching and refresh in `core/token_manager.rs`
- **Config**: Centralized configuration management in `core/config.rs`

### Service Organization

Services are organized by functional domain under `src/service/`:

- `authentication/`: User authentication (v1)
- `bitable/`: Multi-dimensional tables (v1)
- `drive/`: Cloud drive files (v1, v2)
- `im/`: Instant messaging (v1, v2)
- `search/`: Search functionality (v1)
- `sheets/`: Spreadsheets (v2, v3)

Each service follows version-based API organization (v1, v2, v3) with standardized request/response patterns.

### Event System

- **EventDispatcherHandler**: Central event dispatcher in `src/event/dispatcher/mod.rs`
- **Builder Pattern**: Used for registering event handlers with `.register_p2_im_message_receive_v1()` etc.
- **Version Support**: Handles both v1 (p1) and v2 (p2) event formats automatically
- **Type Safety**: Uses traits and generics for compile-time safety
- **WebSocket Support**: Real-time event handling via WebSocket connection (optional feature)

### Authentication Flow

- Supports multiple token types: App Access Token, Tenant Access Token, User Access Token
- Automatic token caching and refresh mechanism
- Configurable authentication options via `Config` struct

### Key Design Patterns

- **Transport Pattern**: All API requests go through `Transport<T>` for consistent handling
- **Builder Pattern**: Used for client configuration and event handler registration
- **Type Safety**: Extensive use of Rust's type system and traits
- **Async/Await**: Full async support throughout the codebase
- **Error Handling**: Unified `LarkAPIError` and `SDKResult<T>` types

## Configuration

Create a `.env` file based on `.env-example` with your API credentials:

```
APP_ID=your_app_id
APP_SECRET=your_app_secret
USER_ACCESS_TOKEN=your_user_access_token  # Optional: for user-specific operations
```

## Features

- Default: Basic API functionality
- `websocket`: Enables WebSocket support for real-time events (required for `ws_client` example)

## Working with Examples

Examples are extensively documented and located in `examples/` directory. They are organized by:

- `api/`: API usage examples by service and version
- `card/`: Lark card component examples
- Root level: High-level integration examples

When adding new examples, follow the existing naming convention and add corresponding `[[example]]` entries to
`Cargo.toml`.

## Client Usage Patterns

### Basic Client Setup

``` norun
use open_lark::prelude::*;

let client = LarkClient::builder(app_id, app_secret)
    .with_app_type(AppType::SelfBuilt)  // or AppType::Marketplace
    .with_enable_token_cache(true)
    .build();
```

### Error Handling

All API calls return `SDKResult<T>` which wraps `Result<T, LarkAPIError>`. Always handle errors appropriately:

``` norun
match client.im.create_message(&req).await {
    Ok(response) => println!("Success: {:?}", response),
    Err(e) => eprintln!("Error: {:?}", e),
}
```

### Event Handling

``` norun
let handler = EventDispatcherHandler::builder()
    .register_p2_im_message_receive_v1(|event| {
        println!("Received message: {:?}", event);
    })
    .build();
```

## Protobuf Integration

The project includes protobuf definitions in `crates/protobuf/` for WebSocket communication. Build scripts automatically
generate Rust code from `.proto` files.