//! 用户访问令牌刷新服务 (Resource: refresh_access_token)
//!
//! 提供用户访问令牌的刷新功能。
//!
//! API文档: https://open.feishu.cn/document/server-docs/user-authentication/access-token/refresh_user_access_token

use crate::models::{RefreshUserAccessTokenResponse, AuthConfig, AuthResult};
use crate::utils::marketplace_auth_config_to_core;
use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 用户访问令牌刷新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    /// 授权类型，固定为 "refresh_token"
    pub grant_type: String,
    /// 刷新令牌
    pub refresh_token: String,
}

/// 用户访问令牌刷新构建器
pub struct RefreshTokenBuilder {
    config: Config,
    request: RefreshTokenRequest,
}

impl RefreshTokenBuilder {
    /// 创建新的用户访问令牌刷新构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: RefreshTokenRequest {
                grant_type: String::new(),
                refresh_token: String::new(),
            },
        }
    }

    /// 设置授权类型
    pub fn grant_type(mut self, grant_type: impl Into<String>) -> Self {
        self.request.grant_type = grant_type.into();
        self
    }

    /// 设置刷新令牌
    pub fn refresh_token(mut self, refresh_token: impl Into<String>) -> Self {
        self.request.refresh_token = refresh_token.into();
        self
    }

    /// 发送请求刷新用户访问令牌
    pub async fn send(self) -> AuthResult<RefreshUserAccessTokenResponse> {
        // 验证必要参数
        if self.request.grant_type.is_empty() {
            return Err(crate::error::AuthErrorBuilder::validation_error(
                "grant_type",
                "授权类型不能为空",
                Some(&self.request.grant_type),
            ));
        }

        if self.request.refresh_token.is_empty() {
            return Err(crate::error::AuthErrorBuilder::validation_error(
                "refresh_token",
                "刷新令牌不能为空",
                Some("***"), // 不记录敏感信息
            ));
        }

        // 构建API请求
        let url = format!("{}/open-apis/authen/v1/refresh_access_token", self.config.base_url);
        let api_request = ApiRequest::<RefreshUserAccessTokenResponse>::post(&url)
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

        // 解析响应数据
        let token_response: RefreshUserAccessTokenResponse = response.data.ok_or_else(|| {
            crate::error::AuthErrorBuilder::network_error("服务器响应数据为空", Some("refresh_access_token"))
        })?;

        Ok(token_response)
    }
}

/// 用户访问令牌刷新服务
pub struct RefreshTokenService {
    config: Config,
}

impl RefreshTokenService {
    /// 创建新的服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 从 AuthConfig 创建服务实例
    pub fn from_auth_config(auth_config: AuthConfig) -> Self {
        Self::new(marketplace_auth_config_to_core(auth_config))
    }

    /// 标准方式刷新用户访问令牌（使用刷新令牌）
    pub fn create(&self) -> RefreshTokenBuilder {
        RefreshTokenBuilder::new(self.config.clone())
    }

    /// 从配置刷新用户访问令牌（使用默认刷新令牌）
    pub fn with_credentials(
        &self,
        grant_type: impl Into<String>,
        refresh_token: impl Into<String>,
    ) -> RefreshTokenBuilder {
        RefreshTokenBuilder::new(self.config.clone())
            .grant_type(grant_type)
            .refresh_token(refresh_token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refresh_token_request_creation() {
        let request = RefreshTokenRequest {
            grant_type: "refresh_token".to_string(),
            refresh_token: "test_refresh_token".to_string(),
        };

        assert_eq!(request.grant_type, "refresh_token");
        assert_eq!(request.refresh_token, "test_refresh_token");
    }

    #[test]
    fn test_refresh_token_builder() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let builder = RefreshTokenBuilder::new(config.clone())
            .grant_type("refresh_token")
            .refresh_token("test_refresh_token");

        assert_eq!(builder.request.grant_type, "refresh_token");
        assert_eq!(builder.request.refresh_token, "test_refresh_token");
    }

    #[test]
    fn test_refresh_token_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = RefreshTokenService::new(config.clone());
        let builder = service.create();

        // 验证服务创建成功
        assert_eq!(service.config.app_id, "test_app_id");
    }

    #[test]
    fn test_refresh_token_service_with_credentials() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = RefreshTokenService::new(config);
        let builder = service.with_credentials("refresh_token", "test_refresh_token");

        assert_eq!(builder.request.grant_type, "refresh_token");
        assert_eq!(builder.request.refresh_token, "test_refresh_token");
    }

    #[test]
    fn test_refresh_token_service_from_auth_config() {
        let auth_config = crate::models::AuthConfig::new("test_app_id", "test_app_secret");
        let service = RefreshTokenService::from_auth_config(auth_config);

        // 验证转换成功
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_app_secret");
    }

    #[test]
    fn test_refresh_token_validation_empty_grant_type() {
        let config = openlark_core::config::Config::builder().build();
        let builder = RefreshTokenBuilder::new(config)
            .refresh_token("test_refresh_token");

        // 这里只验证构建逻辑，实际发送请求时会验证参数
        assert_eq!(builder.request.refresh_token, "test_refresh_token");
        assert!(builder.request.grant_type.is_empty());
    }

    #[test]
    fn test_refresh_token_validation_empty_refresh_token() {
        let config = openlark_core::config::Config::builder().build();
        let builder = RefreshTokenBuilder::new(config)
            .grant_type("refresh_token");

        // 这里只验证构建逻辑，实际发送请求时会验证参数
        assert_eq!(builder.request.grant_type, "refresh_token");
        assert!(builder.request.refresh_token.is_empty());
    }

    #[test]
    fn test_refresh_token_builder_multiple_calls() {
        let config = openlark_core::config::Config::builder().build();
        let builder = RefreshTokenBuilder::new(config)
            .grant_type("first_type")
            .refresh_token("first_token")
            .grant_type("refresh_token")  // 覆盖之前的值
            .refresh_token("final_token");  // 覆盖之前的值

        assert_eq!(builder.request.grant_type, "refresh_token");
        assert_eq!(builder.request.refresh_token, "final_token");
    }

    #[test]
    fn test_refresh_token_config_clone() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://test.example.com")
            .build();

        let service = RefreshTokenService::new(config.clone());
        let builder = service.create();

        // 验证配置被正确克隆
        assert_eq!(builder.config.app_id, "test_app_id");
        assert_eq!(builder.config.base_url, "https://test.example.com");
        assert!(!builder.config.app_id.is_empty());
    }

    #[test]
    fn test_refresh_token_request_serialization() {
        let request = RefreshTokenRequest {
            grant_type: "refresh_token".to_string(),
            refresh_token: "test_refresh_token_123".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let parsed: RefreshTokenRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.grant_type, request.grant_type);
        assert_eq!(parsed.refresh_token, request.refresh_token);
    }

    #[test]
    fn test_refresh_token_request_deserialization() {
        let json = r#"{
            "grant_type": "refresh_token",
            "refresh_token": "test_refresh_token_456"
        }"#;

        let request: RefreshTokenRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.grant_type, "refresh_token");
        assert_eq!(request.refresh_token, "test_refresh_token_456");
    }

    // ==================== 异步测试单元 ====================
    // 注意：由于这些测试需要HTTP Mock，我们简化为单元测试
    // 实际的HTTP集成测试应该在测试套件中单独实现

    #[tokio::test]
    async fn test_refresh_token_builder_send_validation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://invalid.url")  // 故意使用无效URL
            .build();

        // 测试空 grant_type 验证
        let result = RefreshTokenBuilder::new(config.clone())
            .refresh_token("test_refresh_token")
            .send()
            .await;
        assert!(result.is_err());

        // 测试空 refresh_token 验证
        let result = RefreshTokenBuilder::new(config.clone())
            .grant_type("refresh_token")
            .send()
            .await;
        assert!(result.is_err());
    }
}