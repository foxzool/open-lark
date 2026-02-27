//! OpenLark SDK - 飞书开放平台 Rust SDK
//!
//! 这个 crate 是 OpenLark SDK 的统一入口，重新导出了各个功能模块。
//! 用户可以根据需要开启对应的 feature 来使用特定功能。

pub use openlark_core;

#[cfg(feature = "client")]
pub use openlark_client;

#[cfg(feature = "protocol")]
pub use openlark_protocol;

#[cfg(feature = "auth")]
pub use openlark_auth;

#[cfg(feature = "communication")]
pub use openlark_communication;

#[cfg(feature = "docs")]
pub use openlark_docs;

#[cfg(feature = "hr")]
pub use openlark_hr;

#[cfg(feature = "ai")]
pub use openlark_ai;

#[cfg(feature = "helpdesk")]
pub use openlark_helpdesk;

#[cfg(feature = "mail")]
pub use openlark_mail;

#[cfg(feature = "meeting")]
pub use openlark_meeting;

#[cfg(feature = "application")]
pub use openlark_application;

#[cfg(feature = "security")]
pub use openlark_security;

#[cfg(feature = "workflow")]
pub use openlark_workflow;

#[cfg(feature = "platform")]
pub use openlark_platform;

#[cfg(feature = "analytics")]
pub use openlark_analytics;

#[cfg(feature = "user")]
pub use openlark_user;
