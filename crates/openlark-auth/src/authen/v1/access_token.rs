//! 用户访问令牌服务 (Resource: access_token)
//!
//! 提供用户访问令牌的获取功能。
//!
//! API文档: https://open.feishu.cn/document/server-docs/user-authentication/access-token/obtain_user_access_token

use crate::models::{UserAccessTokenResponse, AuthConfig, AuthResult};
use crate::utils::marketplace_auth_config_to_core;
use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 用户访问令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenRequest {
    /// 授权类型，固定为 "authorization_code"
    pub grant_type: String,
    /// 授权码，通过用户授权获取
    pub code: String,
}

/// 用户访问令牌构建器
pub struct AccessTokenBuilder {
    config: Config,
    request: AccessTokenRequest,
}

impl AccessTokenBuilder {
    /// 创建新的用户访问令牌构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: AccessTokenRequest {
                grant_type: String::new(),
                code: String::new(),
            },
        }
    }

    /// 设置授权类型
    pub fn grant_type(mut self, grant_type: impl Into<String>) -> Self {
        self.request.grant_type = grant_type.into();
        self
    }

    /// 设置授权码
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.request.code = code.into();
        self
    }

    /// 发送请求获取用户访问令牌
    pub async fn send(self) -> AuthResult<UserAccessTokenResponse> {
        // 验证必要参数
        if self.request.grant_type.is_empty() {
            return Err(crate::error::AuthErrorBuilder::validation_error(
                "grant_type",
                "授权类型不能为空",
                Some(&self.request.grant_type),
            ));
        }

        if self.request.code.is_empty() {
            return Err(crate::error::AuthErrorBuilder::validation_error(
                "code",
                "授权码不能为空",
                Some(&self.request.code),
            ));
        }

        // 构建API请求
        let url = format!("{}/open-apis/authen/v1/access_token", self.config.base_url);
        let api_request = ApiRequest::<UserAccessTokenResponse>::post(&url)
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
        let token_response: UserAccessTokenResponse = response.data.ok_or_else(|| {
            crate::error::AuthErrorBuilder::network_error("服务器响应数据为空", Some("access_token"))
        })?;

        Ok(token_response)
    }
}

/// 用户访问令牌服务
pub struct AccessTokenService {
    config: Config,
}

impl AccessTokenService {
    /// 创建新的服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 从 AuthConfig 创建服务实例
    pub fn from_auth_config(auth_config: AuthConfig) -> Self {
        Self::new(marketplace_auth_config_to_core(auth_config))
    }

    /// 标准方式获取用户访问令牌（使用授权码）
    pub fn create(&self) -> AccessTokenBuilder {
        AccessTokenBuilder::new(self.config.clone())
    }

    /// 从配置获取用户访问令牌（使用默认授权码）
    pub fn with_credentials(
        &self,
        grant_type: impl Into<String>,
        code: impl Into<String>,
    ) -> AccessTokenBuilder {
        AccessTokenBuilder::new(self.config.clone())
            .grant_type(grant_type)
            .code(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_token_request_creation() {
        let request = AccessTokenRequest {
            grant_type: "authorization_code".to_string(),
            code: "test_code".to_string(),
        };

        assert_eq!(request.grant_type, "authorization_code");
        assert_eq!(request.code, "test_code");
    }

    #[test]
    fn test_access_token_builder() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let builder = AccessTokenBuilder::new(config.clone())
            .grant_type("authorization_code")
            .code("test_code");

        assert_eq!(builder.request.grant_type, "authorization_code");
        assert_eq!(builder.request.code, "test_code");
    }

    #[test]
    fn test_access_token_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = AccessTokenService::new(config.clone());
        let builder = service.create();

        // 验证服务创建成功
        assert_eq!(service.config.app_id, "test_app_id");
    }

    #[test]
    fn test_access_token_service_with_credentials() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = AccessTokenService::new(config);
        let builder = service.with_credentials("authorization_code", "test_code");

        assert_eq!(builder.request.grant_type, "authorization_code");
        assert_eq!(builder.request.code, "test_code");
    }

    #[test]
    fn test_access_token_service_from_auth_config() {
        let auth_config = crate::models::AuthConfig::new("test_app_id", "test_app_secret");
        let service = AccessTokenService::from_auth_config(auth_config);

        // 验证转换成功
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_app_secret");
    }

    #[test]
    fn test_access_token_validation_empty_grant_type() {
        let config = openlark_core::config::Config::builder().build();
        let builder = AccessTokenBuilder::new(config)
            .code("test_code");

        // 这里只验证构建逻辑，实际发送请求时会验证参数
        assert_eq!(builder.request.code, "test_code");
        assert!(builder.request.grant_type.is_empty());
    }

    #[test]
    fn test_access_token_validation_empty_code() {
        let config = openlark_core::config::Config::builder().build();
        let builder = AccessTokenBuilder::new(config)
            .grant_type("authorization_code");

        // 这里只验证构建逻辑，实际发送请求时会验证参数
        assert_eq!(builder.request.grant_type, "authorization_code");
        assert!(builder.request.code.is_empty());
    }

    #[test]
    fn test_access_token_builder_multiple_calls() {
        let config = openlark_core::config::Config::builder().build();
        let builder = AccessTokenBuilder::new(config)
            .grant_type("first_type")
            .code("first_code")
            .grant_type("authorization_code")  // 覆盖之前的值
            .code("final_code");  // 覆盖之前的值

        assert_eq!(builder.request.grant_type, "authorization_code");
        assert_eq!(builder.request.code, "final_code");
    }

    #[test]
    fn test_access_token_config_clone() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://test.example.com")
            .build();

        let service = AccessTokenService::new(config.clone());
        let builder = service.create();

        // 验证配置被正确克隆
        assert_eq!(builder.config.app_id, "test_app_id");
        assert_eq!(builder.config.base_url, "https://test.example.com");
        assert!(!builder.config.app_id.is_empty());
    }

    #[test]
    fn test_access_token_request_serialization() {
        let request = AccessTokenRequest {
            grant_type: "authorization_code".to_string(),
            code: "test_code_123".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let parsed: AccessTokenRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.grant_type, request.grant_type);
        assert_eq!(parsed.code, request.code);
    }

    #[test]
    fn test_access_token_request_deserialization() {
        let json = r#"{
            "grant_type": "authorization_code",
            "code": "test_code_456"
        }"#;

        let request: AccessTokenRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.grant_type, "authorization_code");
        assert_eq!(request.code, "test_code_456");
    }

    // ==================== 异步测试单元 ====================
    // 注意：由于这些测试需要HTTP Mock，我们简化为单元测试
    // 实际的HTTP集成测试应该在测试套件中单独实现

    #[tokio::test]
    async fn test_access_token_builder_send_validation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://invalid.url")  // 故意使用无效URL
            .build();

        // 测试空 grant_type 验证
        let result = AccessTokenBuilder::new(config.clone())
            .code("test_code")
            .send()
            .await;
        assert!(result.is_err());

        // 测试空 code 验证
        let result = AccessTokenBuilder::new(config.clone())
            .grant_type("authorization_code")
            .send()
            .await;
        assert!(result.is_err());
    }
}
