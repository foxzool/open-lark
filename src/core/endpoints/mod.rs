//! API端点常量定义模块 - 统一架构
//!
//! 本模块提供飞书开放平台API端点的统一、分层架构，旨在：
//! 1. 消除代码重复 - 统一管理所有API端点常量
//! 2. 提高可维护性 - 按业务功能分层组织
//! 3. 保持向后兼容 - 渐进式迁移策略
//! 4. 改善开发体验 - 清晰的命名和分类
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::core::endpoints::Endpoints;
//!
//! // 新的分类访问方式（推荐）
//! let messaging = Endpoints::messaging();
//! let endpoint = messaging.send_message;
//!
//! // 传统直接访问方式（向后兼容）
//! let endpoint = Endpoints::IM_V1_SEND_MESSAGE;
//! ```
//!
//! # 架构说明
//!
//! - **新架构**: 按业务功能分组的分类模块
//! - **向后兼容**: 保留所有原有的端点常量
//! - **渐进迁移**: 可以逐步迁移到新的分类架构

// ==================== 新架构 - 分类模块 ====================

// 包含新的统一架构
mod endpoints_lib;

// 重新导出新的分类结构
pub use endpoints_lib::{
    Endpoints, Messaging, Content, HrManagement, Collaboration,
    AiServices, Admin, Integration
};

// 重新导出分类模块，提供更细粒度的访问
pub use endpoints_lib::categories;

// ==================== 向后兼容层 ====================

// 重新导出原有的统一端点，确保现有代码继续工作
pub use super::endpoints_unified::Endpoints as EndpointsUnified;

// 重新导出原有的端点定义，确保现有代码继续工作
pub use super::endpoints_original::Endpoints as EndpointsOriginal;

// 重新导出 EndpointBuilder
pub use super::endpoints_unified::EndpointBuilder;

// 保留原有的域模块导出（用于向后兼容）
pub mod acs;
pub mod admin;
pub mod ai;
pub mod ai_embedding;
pub mod ai_workflow;
pub mod aily;
pub mod analytics;
pub mod apass;
pub mod application;
pub mod approval;
pub mod attendance;
pub mod auth;
pub mod calendar;
pub mod cardkit;
pub mod cloud_docs;
pub mod contact;
pub mod corehr;
pub mod directory;
pub mod drive;
pub mod elearning;
pub mod helpdesk;
pub mod hire;
pub mod im;
pub mod lingo;
pub mod mail;
pub mod mdm;
pub mod minutes;
pub mod okr;
pub mod payroll;
pub mod performance;
pub mod personal_settings;
pub mod platform_integration;
pub mod report;
pub mod search;
pub mod security_and_compliance;
pub mod task;
pub mod tenant;
pub mod tenant_tag;
pub mod trust_party;
pub mod vc;
pub mod verification;
pub mod workplace;
pub mod zero_trust;

// 重新导出各个域模块的常量（保持向后兼容）
pub use lingo::*;

// ==================== 完整兼容层 ====================
// 确保所有现有的 import 路径都能继续工作

// 为最常用的使用模式提供直接别名
pub type EndpointsCompat = endpoints_lib::Endpoints;

// 注意：无法直接导出结构体的常量字段，需要通过结构体实例访问
// 所有的端点常量现在都可以通过 Endpoints::常量名 的方式访问

// ==================== 兼容性别名 ====================

// 为新架构提供传统常量名的别名
impl Endpoints {
    // 从新的分类模块重新导出常用常量，保持向后兼容
    pub const IM_V1_MESSAGES: &'static str = Self::IM_V1_SEND_MESSAGE;
    pub const IM_V1_MESSAGE_SEND: &'static str = Self::IM_V1_SEND_MESSAGE;
    pub const IM_V1_MESSAGE_REPLY: &'static str = Self::IM_V1_REPLY_MESSAGE;
    pub const IM_V1_CHATS: &'static str = Self::IM_CHAT_CREATE;
    pub const IM_V1_CHATS_GET: &'static str = Self::IM_CHAT_GET;
    pub const IM_V1_CHATS_UPDATE: &'static str = Self::IM_CHAT_UPDATE;
    pub const IM_V1_CHAT_MEMBERS: &'static str = Self::IM_CHAT_MEMBERS;
}
