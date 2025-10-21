#![allow(rustdoc::broken_intra_doc_links)]

//! # open-lark
//!
//! 飞书开放平台的非官方 Rust SDK，支持自定义机器人、长连接机器人、云文档、飞书卡片、消息、群组、招聘管理等 API 调用。
//!
//! ## 功能特性
//!
//! - 🚀 **完整的 API 覆盖**: 支持飞书开放平台的主要 API，包括消息、群组、云文档、招聘等
//! - 🔐 **自动令牌管理**: 自动缓存和刷新访问令牌，支持多种令牌类型
//! - 🔌 **事件系统**: 完整的事件处理和分发机制，支持 v1 和 v2 事件格式
//! - 📡 **WebSocket 支持**: 实时事件处理，支持长连接机器人
//! - 🎨 **飞书卡片**: 丰富的卡片组件支持，轻松构建交互式消息
//! - 🛡️ **类型安全**: 充分利用 Rust 的类型系统，编译时保证类型安全
//! - ⚡ **异步支持**: 完全异步的 API，高性能并发处理
//!
//! ## 快速开始
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 创建客户端
//!     let client = LarkClient::builder("your_app_id", "your_app_secret")
//!         .with_app_type(AppType::SelfBuild)
//!         .with_enable_token_cache(true)
//!         .build();
//!
//!     println!("飞书开放平台 Rust SDK 初始化成功");
//!     Ok(())
//! }
//! ```
//!
//! ## 主要模块
//!
//! - [`client`][]: 客户端实现和WebSocket支持
//! - [`core`][]: 核心功能：HTTP传输、配置、错误处理等
//! - [`service`][]: 所有飞书开放平台API服务的实现
//! - [`event`][]: 事件处理和分发器
//! - [`card`][]: 飞书卡片组件和工具
//! - [`custom_bot`][]: 自定义机器人相关功能
//! - [`prelude`][]: 常用类型和trait的重导出，方便用户使用

/// 飞书卡片组件和工具
pub mod card;
/// 客户端实现和WebSocket支持
pub mod client;
/// 核心功能：HTTP传输、配置、错误处理等
pub mod core;
// pub mod message;
/// 自定义机器人相关功能
pub mod custom_bot;
/// 事件处理和分发器
pub mod event;
/// 常用类型和trait的重导出，方便用户使用
pub mod prelude;
/// 所有飞书开放平台API服务的实现
pub mod service;
