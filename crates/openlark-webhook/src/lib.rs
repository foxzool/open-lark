//! OpenLark Webhook Module
//!
//! 飞书 Webhook 自定义机器人模块，提供通过 webhook URL 发送消息的功能。
//!
//! ## 主要功能
//!
//! - **消息发送**: 支持文本、卡片、图片、文件、富文本等多种消息类型
//! - **签名验证**: 可选的 HMAC-SHA256 签名验证（通过 `signature` feature）
//! - **Builder 模式**: 流畅的链式调用 API
//! - **类型安全**: 编译时验证所有参数
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use openlark_webhook::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = WebhookClient::new();
//!
//!     // 发送文本消息
//!     client
//!         .send_message("https://open.feishu.cn/open-apis/bot/v2/hook/...")
//!         .text("Hello, Webhook!")
//!         .execute()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## 功能模块
//!
//! - `robot` - 机器人功能（消息发送）
//! - `common` - 共享工具（验证、错误处理）
//! - `prelude` - 常用导入

#![allow(missing_docs)]

pub mod common;

#[cfg(feature = "robot")]
pub mod robot;

pub mod prelude;

#[cfg(feature = "robot")]
pub use robot::v1::client::WebhookClient;
