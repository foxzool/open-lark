//! API特征定义 - 简化版本

use crate::{api_resp::RawResponse, error::SDKResult};
use async_trait::async_trait;

use super::ApiRequest;

/// 异步API客户端特征
#[async_trait]
pub trait AsyncApiClient: Send + Sync {
    async fn execute_raw(&self, request: ApiRequest<()>) -> SDKResult<RawResponse>;
}

/// 同步API客户端特征
pub trait SyncApiClient: Send + Sync {
    fn execute_raw(&self, request: ApiRequest<()>) -> SDKResult<RawResponse>;
}
