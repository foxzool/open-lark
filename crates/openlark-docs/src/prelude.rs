//! OpenLark Docs Prelude
//!
//! 重新导出常用的类型和trait，便于其他模块使用。

pub use openlark_core::{
    api_req::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

pub use serde::{Deserialize, Serialize};
pub use std::sync::Arc;

pub type LarkClient = openlark_core::client::LarkClient;
