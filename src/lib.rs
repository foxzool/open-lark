//! OpenLark SDK - 飞书开放平台 Rust SDK
//!
//! 这个 crate 是 OpenLark SDK 的统一入口，重新导出了各个功能模块。
//! 用户可以根据需要开启对应的 feature 来使用特定功能。

pub use openlark_core;

#[cfg(feature = "openlark-client")]
pub use openlark_client;

#[cfg(feature = "openlark-protocol")]
pub use openlark_protocol;

#[cfg(feature = "openlark-auth")]
pub use openlark_auth;

#[cfg(feature = "openlark-communication")]
pub use openlark_communication;

#[cfg(feature = "openlark-docs")]
pub use openlark_docs;

#[cfg(feature = "openlark-hr")]
pub use openlark_hr;

#[cfg(feature = "openlark-ai")]
pub use openlark_ai;

#[cfg(feature = "openlark-helpdesk")]
pub use openlark_helpdesk;

#[cfg(feature = "openlark-mail")]
pub use openlark_mail;

#[cfg(feature = "openlark-meeting")]
pub use openlark_meeting;

#[cfg(feature = "openlark-application")]
pub use openlark_application;

#[cfg(feature = "openlark-security")]
pub use openlark_security;
