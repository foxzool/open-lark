//! OpenLark 官方入口 crate。
//!
//! 普通用户应优先使用 `openlark`，通过业务 feature 开启所需能力：
//!
//! ```toml
//! [dependencies]
//! openlark = { version = "0.15", features = ["essential"] }
//! ```
//!
//! - 统一客户端入口：[`Client`]
//! - 高级业务模块入口：[`auth`]、[`communication`]、[`docs`]、[`workflow`] 等
//! - 统一预导出：[`prelude`]
//!
//! 若只想要单一业务域的最小依赖，再直接使用 `openlark-{domain}` 子 crate。

// 允许测试模块中的未使用导入（测试桩代码常见模式）
#![allow(unused_imports)]

pub use openlark_client::{Client, ClientBuilder, Config, Error, Result};
pub use openlark_core as core;
pub use openlark_core::config::Config as CoreConfig;
pub use openlark_core::error::{CoreError, ErrorCode, ErrorSeverity, ErrorTrait, ErrorType};
pub use openlark_core::req_option::RequestOption;
pub use openlark_core::SDKResult;

#[cfg(feature = "websocket")]
/// WebSocket 客户端相关类型导出。
pub mod ws_client {
    pub use openlark_client::ws_client::*;
}

#[cfg(feature = "auth")]
pub use openlark_auth as auth;

#[cfg(feature = "communication")]
pub use openlark_communication as communication;

#[cfg(feature = "docs")]
pub use openlark_docs as docs;

#[cfg(feature = "hr")]
pub use openlark_hr as hr;

#[cfg(feature = "ai")]
pub use openlark_ai as ai;

#[cfg(feature = "helpdesk")]
pub use openlark_helpdesk as helpdesk;

#[cfg(feature = "mail")]
pub use openlark_mail as mail;

#[cfg(feature = "meeting")]
pub use openlark_meeting as meeting;

#[cfg(feature = "application")]
pub use openlark_application as application;

#[cfg(feature = "security")]
pub use openlark_security as security;

#[cfg(feature = "workflow")]
pub use openlark_workflow as workflow;

#[cfg(feature = "platform")]
pub use openlark_platform as platform;

#[cfg(feature = "analytics")]
pub use openlark_analytics as analytics;

#[cfg(feature = "user")]
pub use openlark_user as user;

#[cfg(feature = "webhook")]
pub use openlark_webhook as webhook;

#[cfg(feature = "cardkit")]
pub use openlark_cardkit as cardkit;

#[cfg(feature = "auth")]
pub use openlark_client::AuthClient;

#[cfg(feature = "docs")]
pub use openlark_client::DocsClient;

#[cfg(feature = "communication")]
pub use openlark_client::CommunicationClient;

#[cfg(feature = "meeting")]
pub use openlark_client::MeetingClient;

#[cfg(feature = "cardkit")]
pub use openlark_client::CardkitClient;

/// 面向 `openlark` 用户的统一预导出。
pub mod prelude {
    pub use crate::{Client, ClientBuilder, Config, CoreConfig, Error, Result};
    pub use crate::{CoreError, ErrorCode, ErrorSeverity, ErrorTrait, ErrorType, RequestOption};
    pub use crate::SDKResult;
    pub use openlark_core::prelude::*;
}
