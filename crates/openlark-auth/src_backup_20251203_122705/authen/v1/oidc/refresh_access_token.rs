//! OIDC刷新访问令牌创建API
//!
//! 刷新 user_access_token (OIDC 方式)，通过 refresh_token 获取新的用户访问令牌。
//!
//! API文档: https://open.feishu.cn/document/historic-version/authen/create-4

use crate::models::{OidcRefreshAccessTokenResponse, AuthConfig};
use openlark_core::SDKResult;
use crate::utils::authen_auth_config_to_core;
use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// OIDC刷新访问令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcRefreshAccessTokenRequest {
    /// 刷新令牌
    pub refresh_token: String,
    /// 授权类型，固定为"refresh_token"
    pub grant_type: String,
}

/// OIDC刷新访问令牌构建器
pub struct OidcRefreshAccessTokenBuilder {
    config: Config,
    request: OidcRefreshAccessTokenRequest,
}

impl OidcRefreshAccessTokenBuilder {
    /// 创建新的OIDC刷新访问令牌构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: OidcRefreshAccessTokenRequest {
                refresh_token: String::new(),
                grant_type: "refresh_token".to_string(),
            },
        }
    }

    /// 设置刷新令牌
    pub fn refresh_token(mut self, refresh_token: impl Into<String>) -> Self {
        self.request.refresh_token = refresh_token.into();
        self
    }

    /// 发送请求刷新OIDC访问令牌
    pub async fn send(self) -> SDKResult<OidcRefreshAccessTokenResponse> {
        // 验证必要参数
        if self.request.refresh_token.is_empty() {
            return Err(crate::error::AuthErrorBuilder::validation_error(
                "refresh_token",
                "刷新令牌不能为空",
                Some(&self.request.refresh_token),
            ));
        }

        // 构建API请求
        let url = format!("{}/open-apis/authen/v1/oidc/refresh_access_token", self.config.base_url);
        let api_request = ApiRequest::<OidcRefreshAccessTokenResponse>::post(&url)
            .body(RequestData::Json(serde_json::to_value(&self.request)?));

        // OIDC刷新令牌API不需要特殊的令牌认证
        let request_option = RequestOption::builder().build();

        // 使用 openlark-core 的传输层发送请求
        let response = Transport::request(api_request, &self.config, Some(request_option)).await?;

        // 检查响应状态
        if response.code() != 0 {
            return Err(crate::error::map_feishu_auth_error(
                response.code(),
                response.raw().msg.as_str(),
                response.raw().request_id.as_deref(),
            ));
        }

        // 构建响应对象
        let token_response = response.data.unwrap();
        Ok(token_response)
    }
}

/// OIDC刷新访问令牌服务
pub struct OidcRefreshAccessTokenService {
    config: Config,
}

impl OidcRefreshAccessTokenService {
    /// 创建新的服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 从 AuthConfig 创建服务实例
    pub fn from_auth_config(auth_config: AuthConfig) -> Self {
        Self::new(authen_auth_config_to_core(auth_config))
    }

    /// 创建OIDC刷新访问令牌（需要设置refresh_token）
    pub fn create(&self) -> OidcRefreshAccessTokenBuilder {
        OidcRefreshAccessTokenBuilder::new(self.config.clone())
    }

    /// 使用指定的刷新令牌创建OIDC刷新访问令牌
    pub fn with_refresh_token(&self, refresh_token: impl Into<String>) -> OidcRefreshAccessTokenBuilder {
        OidcRefreshAccessTokenBuilder::new(self.config.clone()).refresh_token(refresh_token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oidc_refresh_access_token_request_creation() {
        let request = OidcRefreshAccessTokenRequest {
            refresh_token: "test_refresh_token".to_string(),
            grant_type: "refresh_token".to_string(),
        };

        assert_eq!(request.refresh_token, "test_refresh_token");
        assert_eq!(request.grant_type, "refresh_token");
    }

    #[test]
    fn test_oidc_refresh_builder_token_setting() {
        let config = Config::builder().build();
        let builder = OidcRefreshAccessTokenBuilder::new(config)
            .refresh_token("test_token");

        assert_eq!(builder.request.refresh_token, "test_token");
        assert_eq!(builder.request.grant_type, "refresh_token");
    }

    #[test]
    fn test_oidc_refresh_service_creation() {
        let config = Config::builder().build();
        let service = OidcRefreshAccessTokenService::new(config);

        let builder = service.create();
        assert_eq!(builder.request.grant_type, "refresh_token");
    }

    #[test]
    fn test_oidc_refresh_service_with_token() {
        let config = Config::builder().build();
        let service = OidcRefreshAccessTokenService::new(config);

        let builder = service.with_refresh_token("refresh_token");
        assert_eq!(builder.request.refresh_token, "refresh_token");
        assert_eq!(builder.request.grant_type, "refresh_token");
    }

    #[test]
    fn test_oidc_refresh_service_from_auth_config() {
        let auth_config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = OidcRefreshAccessTokenService::from_auth_config(auth_config);

        // 验证转换成功
        let builder = service.create();
        assert_eq!(builder.config.base_url, "https://open.feishu.cn");
    }
}