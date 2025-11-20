//! OpenLark Docs Prelude
//!
//! 重新导出常用的类型和trait，便于其他模块使用。

pub use openlark_core::prelude::{ApiResponseTrait, ResponseFormat, StandardResponse};
pub use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

// 导出兼容性适配器
pub use crate::legacy_client_adapter::{LarkClient, LegacyClientAdapter, RequestBuilder, APIResult};

pub use serde::{Deserialize, Serialize};
pub use std::collections::HashMap;
pub use std::sync::Arc;
