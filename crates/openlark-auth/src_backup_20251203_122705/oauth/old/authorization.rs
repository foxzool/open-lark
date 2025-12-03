//! OAuth授权服务 (Resource: authorization)
//!
//! 提供向后兼容的OAuth授权功能，用于获取登录预授权码。
//!
//! 应用请求用户身份验证时，需构造登录链接，并引导用户跳转至此链接。
//! 用户登录成功后会生成登录预授权码 code，并作为参数追加到重定向URL。
//!
//! API文档: https://open.feishu.cn/document/server-docs/authentication-management/login-state-management/obtain-code

use crate::models::{AuthConfig};
use openlark_core::SDKResult;
use crate::utils::marketplace_auth_config_to_core;
use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 登录预授权码请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationIndexRequest {
    /// 应用ID
    pub app_id: String,
    /// 重定向URI
    pub redirect_uri: String,
    /// 状态参数，用于防止CSRF攻击
    pub state: String,
}

/// 登录预授权码响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationIndexResponse {
    /// 错误码，0表示成功
    pub code: i32,
    /// 登录预授权码
    pub data: Option<String>,
}

/// OAuth授权构建器
pub struct AuthorizationBuilder {
    config: Config,
    request: AuthorizationIndexRequest,
}

impl AuthorizationBuilder {
    /// 创建新的OAuth授权构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: AuthorizationIndexRequest {
                app_id: String::new(),
                redirect_uri: String::new(),
                state: String::new(),
            },
        }
    }

    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.request.app_id = app_id.into();
        self
    }

    /// 设置重定向URI
    pub fn redirect_uri(mut self, redirect_uri: impl Into<String>) -> Self {
        self.request.redirect_uri = redirect_uri.into();
        self
    }

    /// 设置状态参数
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.request.state = state.into();
        self
    }

    /// 从配置设置应用ID
    pub fn from_config(mut self) -> Self {
        self.request.app_id = self.config.app_id.clone();
        self
    }

    /// 构建授权URL（向后兼容方法）
    pub fn build_url(self) -> String {
        // URL编码参数
        let encoded_redirect_uri = urlencoding::encode(&self.request.redirect_uri);
        let encoded_state = urlencoding::encode(&self.request.state);

        format!(
            "{}/open-apis/authen/v1/index?app_id={}&redirect_uri={}&state={}",
            self.config.base_url, self.request.app_id, encoded_redirect_uri, encoded_state
        )
    }

    /// 发送请求获取登录预授权码
    pub async fn send(self) -> SDKResult<AuthorizationIndexResponse> {
        // 验证必要参数
        if self.request.app_id.is_empty() {
            return Err(crate::error::AuthErrorBuilder::validation_error(
                "app_id",
                "应用ID不能为空".to_string(),
                Some(self.request.app_id.clone()),
            ));
        }

        if self.request.redirect_uri.is_empty() {
            return Err(crate::error::AuthErrorBuilder::validation_error(
                "redirect_uri",
                "重定向URI不能为空".to_string(),
                Some(self.request.redirect_uri.clone()),
            ));
        }

        // 构建API请求
        let url = format!("{}/open-apis/authen/v1/index", self.config.base_url);
        let api_request = ApiRequest::<AuthorizationIndexResponse>::get(&url)
            .body(RequestData::Json(serde_json::to_value(&self.request)?));

        // OAuth API 不需要特殊的令牌认证
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
        let auth_response = AuthorizationIndexResponse {
            code: response.code(),
            data: response.data,
        };

        Ok(auth_response)
    }
}

/// OAuth授权服务
pub struct AuthorizationService {
    config: Config,
}

impl AuthorizationService {
    /// 创建新的服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 从 AuthConfig 创建服务实例
    pub fn from_auth_config(auth_config: AuthConfig) -> Self {
        Self::new(marketplace_auth_config_to_core(auth_config))
    }

    /// 获取登录预授权码（使用配置中的应用ID）
    pub fn get_index(&self) -> AuthorizationBuilder {
        AuthorizationBuilder::new(self.config.clone())
            .from_config()
    }

    /// 使用自定义应用ID获取登录预授权码
    pub fn with_app_id(&self, app_id: impl Into<String>) -> AuthorizationBuilder {
        AuthorizationBuilder::new(self.config.clone()).app_id(app_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorization_request_creation() {
        let request = AuthorizationIndexRequest {
            app_id: "test_app_id".to_string(),
            redirect_uri: "https://example.com/callback".to_string(),
            state: "test_state".to_string(),
        };

        assert_eq!(request.app_id, "test_app_id");
        assert_eq!(request.redirect_uri, "https://example.com/callback");
        assert_eq!(request.state, "test_state");
    }

    #[test]
    fn test_authorization_builder_from_config() {
        let config = Config::builder()
            .app_id("test_app_id")
            .build();

        let builder = AuthorizationBuilder::new(config.clone())
            .from_config();

        assert_eq!(builder.request.app_id, "test_app_id");
    }

    #[test]
    fn test_authorization_builder_custom() {
        let config = Config::builder().build();
        let builder = AuthorizationBuilder::new(config)
            .app_id("custom_app")
            .redirect_uri("https://custom.com/callback")
            .state("custom_state");

        assert_eq!(builder.request.app_id, "custom_app");
        assert_eq!(builder.request.redirect_uri, "https://custom.com/callback");
        assert_eq!(builder.request.state, "custom_state");
    }

    #[test]
    fn test_authorization_build_url() {
        let config = Config::builder()
            .app_id("test_app_id")
            .base_url("https://open.feishu.cn")
            .build();

        let url = AuthorizationBuilder::new(config.clone())
            .from_config()
            .redirect_uri("https://example.com/callback")
            .state("test_state")
            .build_url();

        assert!(url.contains("app_id=test_app_id"));
        assert!(url.contains("redirect_uri=https://example.com/callback"));
        assert!(url.contains("state=test_state"));
    }

    #[test]
    fn test_authorization_url_encoding() {
        let config = Config::builder().build();
        let url = AuthorizationBuilder::new(config)
            .app_id("test_app")
            .redirect_uri("https://example.com/callback?param=value")
            .state("state with spaces")
            .build_url();

        // 验证URL编码
        assert!(url.contains("param=value")); // URL应该包含编码后的参数
        assert!(url.contains("state%20with%20spaces")); // 空格应该被编码
    }

    #[test]
    fn test_authorization_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .build();

        let service = AuthorizationService::new(config.clone());
        let builder = service.get_index();

        assert_eq!(builder.request.app_id, "test_app_id");
    }

    #[test]
    fn test_authorization_service_custom_app_id() {
        let config = Config::builder().build();
        let service = AuthorizationService::new(config);
        let builder = service.with_app_id("custom_app");

        assert_eq!(builder.request.app_id, "custom_app");
    }

    #[test]
    fn test_authorization_service_from_auth_config() {
        let auth_config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = AuthorizationService::from_auth_config(auth_config);

        let builder = service.get_index();
        assert_eq!(builder.request.app_id, "test_app_id");
    }
}

