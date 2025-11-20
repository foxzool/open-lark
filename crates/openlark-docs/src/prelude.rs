//! OpenLark Docs Prelude
//!
//! 重新导出常用的类型和trait，便于其他模块使用。

pub use openlark_core::prelude::{ApiResponseTrait, ResponseFormat, StandardResponse};
pub use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

// 为了向后兼容性，保留类型别名（但不依赖legacy_client_adapter）
pub type APIResult<T> = SDKResult<T>;

// 注意：LarkClient, LegacyClientAdapter, RequestBuilder 已被移除
// 如果需要使用HTTP客户端，请直接使用 openlark_core::http::Transport
// 或者使用新的基于Config的服务架构

pub use serde::{Deserialize, Serialize};
pub use std::collections::HashMap;
pub use std::sync::Arc;
