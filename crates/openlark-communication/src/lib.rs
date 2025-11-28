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
//!
//! ## 使用示例
//!
//! ```rust
//! use openlark_communication::endpoints::*;
//!
//! // 使用端点常量
//! let messages_endpoint = IM_V1_MESSAGES;
//! let meetings_endpoint = VC_V1_MEETING_CREATE;
//! let mailgroups_endpoint = MAIL_V1_MAILGROUPS;
//! println!("IM消息端点: {}", messages_endpoint);
//! println!("会议创建端点: {}", meetings_endpoint);
//! println!("邮件组端点: {}", mailgroups_endpoint);
//! ```
//!
//! ## 端点组织
//!
//! - `im`: 即时通讯端点
//! - `mail`: 邮件服务端点
//! - `vc`: 视频会议端点
//! - `moments`: 动态分享端点
//! - `event`: 事件系统端点

#![allow(missing_docs)]

// Include macros first
#[macro_use]
mod macros;

// Core service
pub mod service;

// 项目模块（按 meta.Project）
pub mod contact;
pub mod im;
pub mod card;
pub mod contact_search;
pub mod contact_user;
pub mod im_message;
pub mod im_ephemeral;
pub mod aily;

// 导入端点模块（保留）
pub mod endpoints;
pub use endpoints::*;

/// Re-exports from openlark-core for convenience.
pub mod prelude {
    pub use openlark_core::{config::Config, SDKResult};
}

use std::sync::Arc;
use service::CommunicationService;

/// CommunicationClient：统一入口，提供 project-version-resource 链式访问
#[derive(Clone)]
pub struct CommunicationClient {
    service: Arc<CommunicationService>,
}

impl CommunicationClient {
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self {
            service: Arc::new(CommunicationService::new(config)),
        }
    }

    pub fn contact(&self) -> contact::Contact {
        contact::Contact::new(self.service.clone())
    }

    pub fn im(&self) -> im::Im {
        im::Im::new(self.service.clone())
    }

    pub fn card(&self) -> card::Card {
        card::Card::new(self.service.clone())
    }

    pub fn contact_search(&self) -> contact_search::ContactSearch {
        contact_search::ContactSearch::new(self.service.clone())
    }

    pub fn contact_user(&self) -> contact_user::ContactUser {
        contact_user::ContactUser::new(self.service.clone())
    }

    pub fn im_message(&self) -> im_message::ImMessage {
        im_message::ImMessage::new(self.service.clone())
    }

    pub fn im_ephemeral(&self) -> im_ephemeral::ImEphemeral {
        im_ephemeral::ImEphemeral::new(self.service.clone())
    }

    pub fn aily(&self) -> aily::Aily {
        aily::Aily::new(self.service.clone())
    }
}
