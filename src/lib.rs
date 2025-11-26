//! # 飞书开放平台 Rust SDK - 企业级高覆盖率 API 客户端
//!
//! **open-lark** 是一个为飞书开放平台构建的企业级 Rust SDK，提供对 1,134+ 个 API 的高覆盖率访问。
//! 专为企业应用设计，具备自动令牌管理、WebSocket 支持、事件处理和高级错误处理等功能。
//!
//! ## 快速开始
//!
//! 在您的 `Cargo.toml` 中添加：
//!
//! ```toml
//! [dependencies]
//! open-lark = "0.15"
//! ```
//!
//! ## 功能特性
//!
//! - 🚀 **极简依赖**：一条依赖命令即可开始使用
//! - 📦 **分层功能**：通过 feature flag 按需选择功能模块
//! - 🏢 **企业级**：高级错误处理、重试机制和监控支持
//! - 📚 **中文文档**：100% 中文文档，专为中国开发者优化
//! - ⚡ **高性能**：原生 async/await 支持，优化的 HTTP 客户端
//! - 🛡️ **类型安全**：完整的 Rust 类型系统和错误处理
//!
//! ## 功能分层
//!
//! ### 文档协作层（默认启用）
//! 满足 60% 用户的文档协作需求：
//! ```toml
//! [dependencies]
//! open-lark = "0.15"  # 默认启用 docs-collaboration（IM + 文档 + 认证）
//! ```
//!
//! ### IM通讯层
//! 纯通讯功能：
//! ```toml
//! [dependencies]
//! open-lark = { version = "0.15", features = ["communication-core"] }
//! ```
//!
//! ### 专业层
//! 企业协作套件：
//! ```toml
//! [dependencies]
//! open-lark = { version = "0.15", features = ["professional-suite"] }
//! ```
//!
//! ### 企业层
//! 完整企业功能：
//! ```toml
//! [dependencies]
//! open-lark = { version = "0.15", features = ["enterprise-suite"] }
//! ```
//!
//! ### 完整层
//! 所有可用功能：
//! ```toml
//! [dependencies]
//! open-lark = { version = "0.15", features = ["full-suite"] }
//! ```
//!
//! ## 场景化功能组合
//!
//! 根据您的具体需求选择：
//!
//! ```toml
//! # IM 通讯套件
//! open-lark = { version = "0.15", features = ["im-suite"] }
//!
//! # 文档协作套件
//! open-lark = { version = "0.15", features = ["docs-suite"] }
//!
//! # 人力资源套件
//! open-lark = { version = "0.15", features = ["hr-suite"] }
//!
//! # AI 智能套件
//! open-lark = { version = "0.15", features = ["ai-suite"] }
//!
//! # WebSocket 支持
//! open-lark = { version = "0.15", features = ["websocket"] }
//!
//! # OpenTelemetry 可观测性
//! open-lark = { version = "0.15", features = ["otel"] }
//! ```
//!
//! ## 基础使用示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 创建客户端
//!     let client = LarkClient::builder()
//!         .app_id("your_app_id")
//!         .app_secret("your_app_secret")
//!         .build()?;
//!
//!     // 发送文本消息
//!     let response = client
//!         .communication()
//!         .im()
//!         .v1()
//!         .message()
//!         .send_text()
//!         .receive_id_type("open_id")
//!         .receive_id("user_open_id")
//!         .content("Hello from open-lark!")
//!         .send()
//!         .await?;
//!
//!     println!("消息发送成功: {:?}", response);
//!     Ok(())
//! }
//! ```
//!
//! ## 功能模块对照表
//!
//! | 功能组合 | 包含模块 | 适用场景 |
//! |---------|---------|---------|
//! | `docs-collaboration`（默认） | IM + 文档 + 认证 | 文档协作应用 |
//! | `communication-core` | IM消息、联系人、群组 | 纯通讯应用 |
//! | `professional-suite` | 企业协作功能 | 专业协作平台 |
//! | `enterprise-suite` | 专业功能 + HR + AI | 大型企业应用 |
//! | `full-suite` | 所有可用功能 | 功能完备的应用 |
//!
//! ## 更多示例
//!
//! - [基础设置示例](https://github.com/foxzool/open-lark/tree/main/examples/basic)
//! - [API 使用示例](https://github.com/foxzool/open-lark/tree/main/examples/api)
//! - [业务场景演示](https://github.com/foxzool/open-lark/tree/main/examples/api/multi_service_integration_enhanced.rs)
//!
//! ## 架构说明
//!
//! 本 SDK 采用现代化 crate 架构，同时保持极简的用户体验：
//!
//! - **用户视角**：单一的 `open-lark` 依赖，简单易用
//! - **内部架构**：22 个专业 crate 模块，模块化设计
//! - **发布策略**：混合发布策略，兼顾简单性和灵活性
//!
//! ## 版本说明
//!
//! - 当前版本：`0.15.0-dev`
//! - API 覆盖：1,134+ 个飞书平台 API（86.3% 覆盖率）
//! - crate 模块：22 个专业服务模块
//! - 支持平台：飞书（Lark 中国版）
//!
//! ## 技术支持
//!
//! - 📖 [完整文档](https://docs.rs/open-lark)
//! - 🐛 [问题反馈](https://github.com/foxzool/open-lark/issues)
//! - 💬 [讨论区](https://github.com/foxzool/open-lark/discussions)
//!
//! ## 许可证
//!
//! 本项目采用 [Apache-2.0](LICENSE) 许可证。

#![deny(missing_docs)]
#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

// 重新导出核心类型，提供统一的入口
pub use openlark_core::*;

#[cfg(feature = "auth")]
pub use openlark_auth;

#[cfg(feature = "openlark-client")]
pub use openlark_client;

#[cfg(feature = "openlark-communication")]
pub use openlark_communication;

#[cfg(feature = "openlark-docs")]
pub use openlark_docs;

// #[cfg(feature = "openlark-hr")]
// pub use openlark_hr;

// #[cfg(feature = "openlark-ai")]
// pub use openlark_ai;

// 注意：以下模块暂未包含在发布包中，等待完全开发完成
// #[cfg(feature = "openlark-admin")]
// pub use openlark_admin;
//
// #[cfg(feature = "openlark-approval")]
// pub use openlark_approval;
//
// #[cfg(feature = "openlark-application")]
// pub use openlark_application;
//
// #[cfg(feature = "openlark-apass")]
// pub use openlark_apass;
//
// #[cfg(feature = "openlark-calendar")]
// pub use openlark_calendar;
//
// #[cfg(feature = "openlark-collab")]
// pub use openlark_collab;
//
// #[cfg(feature = "openlark-helpdesk")]
// pub use openlark_helpdesk;
//
// #[cfg(feature = "openlark-hire")]
// pub use openlark_hire;
//
// #[cfg(feature = "openlark-lingo")]
// pub use openlark_lingo;
//
// #[cfg(feature = "openlark-mail")]
// pub use openlark_mail;
//
// #[cfg(feature = "openlark-meeting")]
// pub use openlark_meeting;
//
// #[cfg(feature = "openlark-people")]
// pub use openlark_people;
//
// #[cfg(feature = "openlark-task")]
// pub use openlark_task;

#[cfg(feature = "openlark-protocol")]
pub use openlark_protocol;

/// 预导出模块，提供最常用的类型和功能
pub mod prelude {
    // 重新导出核心模块，包含基础类型如SDKResult
    pub use openlark_core::prelude::*;

    // 简化的模块重新导出，避免glob冲突
    #[cfg(feature = "auth")]
    pub use openlark_auth;

    #[cfg(feature = "openlark-communication")]
    pub use openlark_communication;

    #[cfg(feature = "openlark-docs")]
    pub use openlark_docs;

    // #[cfg(feature = "openlark-hr")]
    // pub use openlark_hr;

    // #[cfg(feature = "openlark-ai")]
    // pub use openlark_ai;

    #[cfg(feature = "openlark-protocol")]
    pub use openlark_protocol;
}

/// 便捷的客户端类型别名
#[cfg(feature = "openlark-client")]
pub type LarkClient = openlark_client::Client;
