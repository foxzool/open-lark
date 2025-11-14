//! OpenLark Docs Prelude
//!
//! 重新导出常用的类型和trait，便于其他模块使用。

pub use openlark_core::{
    config::Config,
    error::LarkAPIError,
    constants::AccessTokenType,
    api_req::ApiRequest,
    http::Transport,
    SDKResult,
};

pub use serde::{Deserialize, Serialize};
pub use std::sync::Arc;

pub type LarkClient = openlark_core::client::LarkClient;