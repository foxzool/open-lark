//! API特征定义 - 独立版本
#![allow(async_fn_in_trait)]

use super::ApiRequest;
pub use super::responses::RawResponse;
use crate::error::SDKResult;

/// 异步API客户端特征
pub trait AsyncApiClient: Send + Sync {
    /// 执行原始 API 请求
    async fn execute_raw(&self, request: ApiRequest<()>) -> SDKResult<RawResponse>;
}

/// 同步API客户端特征
pub trait SyncApiClient: Send + Sync {
    /// 执行原始 API 请求
    fn execute_raw(&self, request: ApiRequest<()>) -> SDKResult<RawResponse>;
}
