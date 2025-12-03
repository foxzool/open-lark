//! 商店应用获取 app_access_token API
//!
//! 对应CSV记录: 6995779366223708164
//! 应用商店应用通过此接口获取 app_access_token，调用接口获取应用资源时，
//! 需要使用 app_access_token 作为授权凭证。

use openlark_core::{
    config::Config,
    api::{ApiRequest, RequestData},
    prelude::Transport,
    error::SDKResult,
};
use crate::models::auth::*;
use crate::utils::error_handler::handle_api_response_owned;

/// 应用访问令牌构建器（商店应用）
#[derive(Debug)]
pub struct AppAccessTokenBuilder {
    config: Config,
    request: AppAccessTokenRequest,
}

impl AppAccessTokenBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: AppAccessTokenRequest {
                app_id: String::new(),
                app_secret: String::new(),
            },
        }
    }

    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.request.app_id = app_id.into();
        self
    }

    /// 设置应用密钥
    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.request.app_secret = app_secret.into();
        self
    }

    /// 发送请求获取 app_access_token
    pub async fn send(self) -> SDKResult<AccessTokenResponse> {
        let url = format!("{}/open-apis/auth/v3/app_access_token", self.config.base_url);

        let request: ApiRequest<AccessTokenResponse> = ApiRequest::post(&url)
            .body(RequestData::Json(serde_json::to_value(&self.request)?))
            .header("Content-Type", "application/json");

        let response = Transport::request(request, &self.config, None).await?;

        // 使用优化的错误处理器，大幅简化错误处理逻辑（从73行减少到3行）
        handle_api_response_owned(response, "/open-apis/auth/v3/app_access_token".to_string())
    }
}