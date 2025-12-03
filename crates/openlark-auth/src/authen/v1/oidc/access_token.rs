//! OIDC访问令牌创建API
//!
//! 获取 user_access_token (OIDC 方式)，支持OpenID Connect协议的授权码流程。
//!
//! API文档: https://open.feishu.cn/document/historic-version/authen/create-3

use crate::models::{OidcAccessTokenResponse, AuthConfig, AuthResult};
use crate::utils::marketplace_auth_config_to_core;
use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// OIDC访问令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcAccessTokenRequest {
    /// 授权码
    pub code: String,
    /// 授权码重定向URI
    pub redirect_uri: String,
    /// 客户端ID
    pub client_id: String,
    /// 客户端密钥
    pub client_secret: String,
    /// 授权类型，固定为"authorization_code"
    pub grant_type: String,
}

/// OIDC访问令牌构建器
pub struct OidcAccessTokenBuilder {
    config: Config,
    request: OidcAccessTokenRequest,
}

impl OidcAccessTokenBuilder {
    /// 创建新的OIDC访问令牌构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: OidcAccessTokenRequest {
                code: String::new(),
                redirect_uri: String::new(),
                client_id: String::new(),
                client_secret: String::new(),
                grant_type: "authorization_code".to_string(),
            },
        }
    }

    /// 设置授权码
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.request.code = code.into();
        self
    }

    /// 设置重定向URI
    pub fn redirect_uri(mut self, redirect_uri: impl Into<String>) -> Self {
        self.request.redirect_uri = redirect_uri.into();
        self
    }

    /// 设置客户端ID
    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.request.client_id = client_id.into();
        self
    }

    /// 设置客户端密钥
    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.request.client_secret = client_secret.into();
        self
    }

    /// 从配置设置客户端ID和客户端密钥
    pub fn from_config(mut self) -> Self {
        self.request.client_id = self.config.app_id.clone();
        self.request.client_secret = self.config.app_secret.clone();
        self
    }

    /// 发送请求获取OIDC访问令牌
    pub async fn send(self) -> AuthResult<OidcAccessTokenResponse> {
        // 验证必要参数
        if self.request.code.is_empty() {
            return Err(crate::error::AuthErrorBuilder::validation_error(
                "code",
                "授权码不能为空",
                Some(&self.request.code),
            ));
        }

        if self.request.client_id.is_empty() {
            return Err(crate::error::AuthErrorBuilder::validation_error(
                "client_id",
                "客户端ID不能为空",
                Some(&self.request.client_id),
            ));
        }

        if self.request.client_secret.is_empty() {
            return Err(crate::error::AuthErrorBuilder::validation_error(
                "client_secret",
                "客户端密钥不能为空",
                Some("***"), // 不记录敏感信息
            ));
        }

        // 构建API请求
        let url = format!("{}/open-apis/authen/v1/oidc/access_token", self.config.base_url);
        let api_request = ApiRequest::<OidcAccessTokenResponse>::post(&url)
            .body(RequestData::Json(serde_json::to_value(&self.request)?));

        // OIDC API 不需要特殊的令牌认证，使用应用凭证即可
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

/// OIDC访问令牌服务
pub struct OidcAccessTokenService {
    config: Config,
}

impl OidcAccessTokenService {
    /// 创建新的服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 从 AuthConfig 创建服务实例
    pub fn from_auth_config(auth_config: AuthConfig) -> Self {
        Self::new(marketplace_auth_config_to_core(auth_config))
    }

    /// 创建OIDC访问令牌（使用配置中的凭证）
    pub fn create(&self) -> OidcAccessTokenBuilder {
        OidcAccessTokenBuilder::new(self.config.clone())
            .from_config()
    }

    /// 使用自定义凭证创建OIDC访问令牌
    pub fn with_credentials(
        &self,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> OidcAccessTokenBuilder {
        OidcAccessTokenBuilder::new(self.config.clone())
            .client_id(client_id)
            .client_secret(client_secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oidc_access_token_request_creation() {
        let request = OidcAccessTokenRequest {
            code: "test_code".to_string(),
            redirect_uri: "https://example.com/callback".to_string(),
            client_id: "test_client_id".to_string(),
            client_secret: "test_client_secret".to_string(),
            grant_type: "authorization_code".to_string(),
        };

        assert_eq!(request.code, "test_code");
        assert_eq!(request.grant_type, "authorization_code");
    }

    #[test]
    fn test_oidc_builder_from_config() {
        let config = Config::builder()
            .app_id("test_client_id")
            .app_secret("test_client_secret")
            .build();

        let builder = OidcAccessTokenBuilder::new(config.clone())
            .from_config();

        assert_eq!(builder.request.client_id, "test_client_id");
        assert_eq!(builder.request.client_secret, "test_client_secret");
        assert_eq!(builder.request.grant_type, "authorization_code");
    }

    #[test]
    fn test_oidc_builder_custom_credentials() {
        let config = Config::builder().build();
        let builder = OidcAccessTokenBuilder::new(config)
            .client_id("custom_client_id")
            .client_secret("custom_client_secret")
            .code("auth_code");

        assert_eq!(builder.request.client_id, "custom_client_id");
        assert_eq!(builder.request.client_secret, "custom_client_secret");
        assert_eq!(builder.request.code, "auth_code");
    }

    #[test]
    fn test_oidc_service_creation() {
        let config = Config::builder()
            .app_id("test_client_id")
            .app_secret("test_client_secret")
            .build();

        let service = OidcAccessTokenService::new(config.clone());
        let builder = service.create();

        assert_eq!(builder.request.client_id, "test_client_id");
        assert_eq!(builder.request.client_secret, "test_client_secret");
    }

    #[test]
    fn test_oidc_service_custom_credentials() {
        let config = Config::builder().build();
        let service = OidcAccessTokenService::new(config);
        let builder = service.with_credentials("custom_client", "custom_secret");

        assert_eq!(builder.request.client_id, "custom_client");
        assert_eq!(builder.request.client_secret, "custom_secret");
    }

    #[test]
    fn test_oidc_service_from_auth_config() {
        let auth_config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = OidcAccessTokenService::from_auth_config(auth_config);

        // 验证转换成功
        let builder = service.create();
        assert_eq!(builder.request.client_id, "test_app_id");
        assert_eq!(builder.request.client_secret, "test_app_secret");
    }
}