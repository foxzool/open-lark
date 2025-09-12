//! 即时消息（IM）服务
//!
//! 提供飞书即时消息相关的所有API功能，包括消息发送、接收、管理等。
//! 支持多种消息类型：文本、富文本、图片、文件、卡片等。
//!
//! # API版本
//!
//! - **v1**: 稳定版本，包含核心消息功能
//! - **v2**: 新版本，包含增强功能
//!
//! # 主要功能
//!
//! - 📨 消息发送和接收
//! - 🎨 富文本和卡片消息
//! - 📁 文件和媒体消息
//! - 👥 群聊管理
//! - 🔔 消息推送和通知
//!
//! # 快速开始
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 发送文本消息
//! let message = CreateMessageRequestBody::builder()
//!     .receive_id("ou_xxx")
//!     .msg_type("text")
//!     .content("{\"text\":\"Hello!\"}")
//!     .build();
//!
//! let request = CreateMessageRequest::builder()
//!     .receive_id_type("open_id")
//!     .request_body(message)
//!     .build();
//!
//! // let result = client.im.v1.message.create(request, None).await?;
//! ```

use crate::{
    core::config::Config,
    service::im::{v1::V1, v2::V2},
};

/// IM API v1版本
pub mod v1;
/// IM API v2版本
pub mod v2;

/// 即时消息服务
///
/// 聚合所有IM相关的API版本，提供统一的访问接口。
/// 通过不同版本的子服务访问具体的API功能。
pub struct ImService {
    /// IM API v1版本服务
    pub v1: V1,
    /// IM API v2版本服务
    pub v2: V2,
}

impl ImService {
    /// 创建新的IM服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置
    pub fn new(config: Config) -> Self {
        Self {
            v1: V1::new(config.clone()),
            v2: V2::new(config.clone()),
        }
    }
}
