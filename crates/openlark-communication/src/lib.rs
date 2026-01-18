//! Open-Lark Communication Module
//!
//! 飞书通讯协作服务模块，提供完整的沟通和协作功能。
//!
//! ## 主要功能
//!
//! - **即时通讯**: 消息发送、聊天管理、批量消息处理
//! - **邮件服务**: 邮件组管理、邮箱事件订阅、文件夹操作
//! - **视频会议**: 会议室管理、会议创建、参会控制
//! - **动态分享**: 朋友圈动态、内容分享、社交互动
//! - **事件系统**: 事件订阅、处理、分发机制
//! - **AILY (AI学习平台)**: 数据知识管理、AI 会话和技能调用
//! - **通讯录**: 用户、部门、用户组、角色等管理
//!
//! ## 使用示例
//!
//! ```rust
//! use openlark_communication::endpoints::*;
//!
//! let messages_endpoint = IM_V1_MESSAGES;
//! let meetings_endpoint = VC_V1_MEETING_CREATE;
//! let mailgroups_endpoint = MAIL_V1_MAILGROUPS;
//! ```
//!
//! ## 子模块
//!
//! 端点模块按服务分类组织，便于维护和查找：
//! - `endpoints/aily.rs` - AILY 相关端点
//! - `endpoints/im.rs` - IM 相关端点
//! - `endpoints/mail.rs` - Mail 相关端点
//! - `endpoints/vc.rs` - VC 相关端点
//! - `endpoints/event.rs` - Event 相关端点
//! - `endpoints/moments.rs` - Moments 相关端点
//! - `endpoints/contact.rs` - Contact 相关端点
//!
//! ## 功能模块
//!
//! 这些模块包含完整的 API 端点常量定义和测试用例。

#![allow(missing_docs)]

// Include macros first
#[macro_use]
mod macros;

// 通用工具与类型
pub mod common;

// AILY 模块
#[cfg(feature = "aily")]
pub mod aily;

// IM 模块
#[cfg(feature = "im")]
pub mod im;

// Contact 模块
#[cfg(feature = "contact")]
pub mod contact;

// Moments 模块
#[cfg(feature = "moments")]
pub mod moments;

// V1 版本的历史遗留模块（标记为 deprecated）
// 注意：这些模块保留用于向后兼容，建议使用新的 v3 版本
#[cfg(all(feature = "contact", feature = "contact-v1-v2"))]
pub mod contact_v1_v2;

#[cfg(all(feature = "im", feature = "im-v1-v2"))]
pub mod im_v1_v2;

#[cfg(all(feature = "contact", feature = "contact-v1-v2"))]
pub mod contact_user_v1_v2;

#[cfg(all(feature = "contact", feature = "contact-v1-v2"))]
pub mod contact_search_v1_v2;

#[cfg(all(feature = "im", feature = "im-v1-v2"))]
pub mod im_message_v1_v2;

#[cfg(all(feature = "im", feature = "im-v1-v2"))]
pub mod im_card_v1_v2;

#[cfg(all(feature = "im", feature = "im-v1-v2"))]
pub mod im_ephemeral_v1_v2;

#[cfg(all(feature = "contact", feature = "contact-v1-v2"))]
pub mod card_v1_v2;

// Re-exports from openlark-core for convenience.
pub mod prelude {
    pub use openlark_core::{config::Config, SDKResult};
}

pub use common::chain::CommunicationClient;
