//! 自建应用获取应用访问令牌API
//!
//! 实现自建应用通过此接口获取 app_access_token，调用接口获取应用资源时，
//! 需要使用 app_access_token 作为授权凭证。
//!
//! API文档: https://open.feishu.cn/document/server-docs/authentication-management/access-token/app_access_token_internal

use crate::models::{AppAccessTokenResponse, AuthConfig, AuthResult};
use crate::utils::self_build_auth_config_to_core;
use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 自建应用访问令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppAccessTokenInternalRequest {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
}

/// 自建应用访问令牌构建器
pub struct AppAccessTokenInternalBuilder {
    config: Config,
    request: AppAccessTokenInternalRequest,
}

impl AppAccessTokenInternalBuilder {
    /// 创建新的自建应用访问令牌构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: AppAccessTokenInternalRequest {
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

    /// 从配置设置应用ID和应用密钥
    pub fn from_config(mut self) -> Self {
        self.request.app_id = self.config.app_id.clone();
        self.request.app_secret = self.config.app_secret.clone();
        self
    }

    /// 发送请求获取自建应用访问令牌
    pub async fn send(self) -> AuthResult<AppAccessTokenResponse> {
        // 验证必要参数
        if self.request.app_id.is_empty() {
            return Err(crate::error::AuthErrorBuilder::validation_error(
                "app_id",
                "应用ID不能为空",
                Some(&self.request.app_id),
            ));
        }

        if self.request.app_secret.is_empty() {
            return Err(crate::error::AuthErrorBuilder::validation_error(
                "app_secret",
                "应用密钥不能为空",
                Some("***"), // 不记录敏感信息
            ));
        }

        // 构建API请求
        let url = format!("{}/open-apis/auth/v3/app_access_token/internal", self.config.base_url);
        let api_request = ApiRequest::<AppAccessTokenResponse>::post(&url)
            .body(RequestData::Json(serde_json::to_value(&self.request)?));

        // 使用 openlark-core 的传输层发送请求
        let response = Transport::request(api_request, &self.config, None).await?;

        // 检查响应状态
        if response.code() != 0 {
            return Err(crate::error::map_feishu_auth_error(
                response.code(),
                response.raw().msg.as_str(),
                response.raw().request_id.as_deref(),
            ));
        }

        // 构建响应对象
        let token_response = AppAccessTokenResponse {
            code: response.code(),
            app_access_token: response.data.unwrap_or_default(),
            expire: 7200, // 默认2小时有效期，实际以服务器返回为准
        };

        Ok(token_response)
    }
}

/// 自建应用访问令牌服务
pub struct AppAccessTokenInternalService {
    config: Config,
}

impl AppAccessTokenInternalService {
    /// 创建新的服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 从 AuthConfig 创建服务实例
    pub fn from_auth_config(auth_config: AuthConfig) -> Self {
        Self::new(self_build_auth_config_to_core(auth_config))
    }

    /// 获取自建应用访问令牌（使用配置中的凭证）
    pub fn from_config(&self) -> AppAccessTokenInternalBuilder {
        AppAccessTokenInternalBuilder::new(self.config.clone())
            .from_config()
    }

    /// 自定义应用凭证获取自建应用访问令牌
    pub fn with_credentials(
        &self,
        app_id: impl Into<String>,
        app_secret: impl Into<String>,
    ) -> AppAccessTokenInternalBuilder {
        AppAccessTokenInternalBuilder::new(self.config.clone())
            .app_id(app_id)
            .app_secret(app_secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_access_token_internal_request_creation() {
        let request = AppAccessTokenInternalRequest {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
        };

        assert_eq!(request.app_id, "test_app_id");
        assert_eq!(request.app_secret, "test_app_secret");
    }

    #[test]
    fn test_internal_builder_from_config() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(openlark_core::constants::AppType::SelfBuild)
            .build();

        let builder = AppAccessTokenInternalBuilder::new(config.clone())
            .from_config();

        assert_eq!(builder.request.app_id, "test_app_id");
        assert_eq!(builder.request.app_secret, "test_app_secret");
    }

    #[test]
    fn test_internal_builder_custom_credentials() {
        let config = Config::builder().build();
        let builder = AppAccessTokenInternalBuilder::new(config.clone())
            .app_id("custom_app_id")
            .app_secret("custom_app_secret");

        assert_eq!(builder.request.app_id, "custom_app_id");
        assert_eq!(builder.request.app_secret, "custom_app_secret");
    }

    #[test]
    fn test_internal_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(openlark_core::constants::AppType::SelfBuild)
            .build();

        let service = AppAccessTokenInternalService::new(config.clone());
        let builder = service.from_config();

        assert_eq!(builder.request.app_id, "test_app_id");
        assert_eq!(builder.request.app_secret, "test_app_secret");
    }

    #[test]
    fn test_internal_service_custom_credentials() {
        let config = Config::builder().build();
        let service = AppAccessTokenInternalService::new(config);
        let builder = service.with_credentials("custom_app", "custom_secret");

        assert_eq!(builder.request.app_id, "custom_app");
        assert_eq!(builder.request.app_secret, "custom_secret");
    }

    #[test]
    fn test_internal_service_from_auth_config() {
        let auth_config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = AppAccessTokenInternalService::from_auth_config(auth_config);

        // 验证转换成功
        let builder = service.from_config();
        assert_eq!(builder.request.app_id, "test_app_id");
        assert_eq!(builder.request.app_secret, "test_app_secret");
    }
}