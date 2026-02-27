//! API特征定义 - 独立版本

pub use super::responses::RawResponse;
use super::ApiRequest;
use crate::error::SDKResult;

/// 异步API客户端特征
pub trait AsyncApiClient: Send + Sync {
    async fn execute_raw(&self, request: ApiRequest<()>) -> SDKResult<RawResponse>;
}

/// 同步API客户端特征
pub trait SyncApiClient: Send + Sync {
    fn execute_raw(&self, request: ApiRequest<()>) -> SDKResult<RawResponse>;
}
