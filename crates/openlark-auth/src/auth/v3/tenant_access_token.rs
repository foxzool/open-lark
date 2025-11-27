//! 租户访问令牌服务 (Resource: tenant_access_token)
//!
//! 提供租户访问令牌的获取功能，支持自建应用和商店应用两种模式。

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::models::{map_feishu_auth_error, AuthErrorBuilder};
use crate::models::{AuthConfig, AuthResult, TenantAccessTokenResponse};

/// 租户访问令牌服务
#[derive(Debug)]
pub struct TenantAccessTokenService {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
}

impl TenantAccessTokenService {
    /// 创建新的租户访问令牌服务
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// 获取自建应用租户访问令牌
    pub fn internal(&self) -> TenantAccessTokenInternalBuilder {
        TenantAccessTokenInternalBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            app_id: self.config.app_id.clone(),
            app_secret: self.config.app_secret.clone(),
        }
    }

    /// 获取商店应用租户访问令牌
    pub fn store(&self) -> TenantAccessTokenStoreBuilder {
        TenantAccessTokenStoreBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            app_access_token: String::new(),
            tenant_key: String::new(),
        }
    }
}

/// 自建应用租户访问令牌构建器
#[derive(Debug)]
pub struct TenantAccessTokenInternalBuilder {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
    app_id: String,
    app_secret: String,
}

impl TenantAccessTokenInternalBuilder {
    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 设置应用密钥
    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.app_secret = app_secret.into();
        self
    }

    /// 发送请求获取令牌
    pub async fn send(self) -> AuthResult<TenantAccessTokenResponse> {
        let url = format!(
            "{}/open-apis/auth/v3/tenant_access_token/internal",
            self.config.base_url
        );

        let request_body = TenantAccessTokenInternalRequest {
            app_id: self.app_id,
            app_secret: self.app_secret,
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("User-Agent", "openlark-rust-sdk/0.1.0")
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response: TenantAccessTokenResponse = response.json().await?;
            Ok(token_response)
        } else {
            let status = response.status().as_u16() as i32;
            let error_text = response.text().await.unwrap_or_default();

            // 尝试解析飞书错误响应
            if let Ok(feishu_response) = serde_json::from_str::<serde_json::Value>(&error_text) {
                if let (Some(code), Some(message)) = (
                    feishu_response.get("code").and_then(|v| v.as_i64()),
                    feishu_response
                        .get("msg")
                        .or_else(|| feishu_response.get("message"))
                        .and_then(|v| v.as_str()),
                ) {
                    let request_id = feishu_response.get("log_id").and_then(|v| v.as_str());
                    return Err(map_feishu_auth_error(code as i32, message, request_id));
                }
            }

            // 回退到基于 HTTP 状态码的错误处理
            Err(AuthErrorBuilder::credentials_invalid(format!(
                "HTTP {} 错误: {}",
                status, error_text
            )))
        }
    }
}

/// 商店应用租户访问令牌构建器
#[derive(Debug)]
pub struct TenantAccessTokenStoreBuilder {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
    app_access_token: String,
    tenant_key: String,
}

impl TenantAccessTokenStoreBuilder {
    /// 设置应用访问令牌
    pub fn app_access_token(mut self, app_access_token: impl Into<String>) -> Self {
        self.app_access_token = app_access_token.into();
        self
    }

    /// 设置租户标识
    pub fn tenant_key(mut self, tenant_key: impl Into<String>) -> Self {
        self.tenant_key = tenant_key.into();
        self
    }

    /// 发送请求获取令牌
    pub async fn send(self) -> AuthResult<TenantAccessTokenResponse> {
        let url = format!(
            "{}/open-apis/auth/v3/tenant_access_token",
            self.config.base_url
        );

        let app_access_token_clone = self.app_access_token.clone();
        let request_body = TenantAccessTokenStoreRequest {
            app_access_token: self.app_access_token,
            tenant_key: self.tenant_key,
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("User-Agent", "openlark-rust-sdk/0.1.0")
            .header(
                "Authorization",
                format!("Bearer {}", app_access_token_clone),
            )
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response: TenantAccessTokenResponse = response.json().await?;
            Ok(token_response)
        } else {
            let status = response.status().as_u16() as i32;
            let error_text = response.text().await.unwrap_or_default();

            // 尝试解析飞书错误响应
            if let Ok(feishu_response) = serde_json::from_str::<serde_json::Value>(&error_text) {
                if let (Some(code), Some(message)) = (
                    feishu_response.get("code").and_then(|v| v.as_i64()),
                    feishu_response
                        .get("msg")
                        .or_else(|| feishu_response.get("message"))
                        .and_then(|v| v.as_str()),
                ) {
                    let request_id = feishu_response.get("log_id").and_then(|v| v.as_str());
                    return Err(map_feishu_auth_error(code as i32, message, request_id));
                }
            }

            // 回退到基于 HTTP 状态码的错误处理
            Err(AuthErrorBuilder::credentials_invalid(format!(
                "HTTP {} 错误: {}",
                status, error_text
            )))
        }
    }
}

/// 自建应用租户访问令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TenantAccessTokenInternalRequest {
    app_id: String,
    app_secret: String,
}

/// 商店应用租户访问令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TenantAccessTokenStoreRequest {
    app_access_token: String,
    tenant_key: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AuthServices;
    use serde_json::json;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// 简单的断言宏
    macro_rules! assert_ok {
        ($result:expr) => {
            match $result {
                Ok(value) => value,
                Err(e) => panic!("Expected Ok, got Err: {:?}", e),
            }
        };
    }

    macro_rules! assert_err {
        ($result:expr) => {
            match $result {
                Ok(_) => panic!("Expected Err, got Ok"),
                Err(_) => true,
            }
        };
    }

    /// 创建测试用的认证配置
    fn create_test_auth_config(base_url: &str) -> crate::models::AuthConfig {
        crate::models::AuthConfig::new("test_app_id", "test_app_secret").with_base_url(base_url)
    }

    /// 创建测试用的认证服务
    fn create_test_auth_services(base_url: &str) -> AuthServices {
        let config = create_test_auth_config(base_url);
        AuthServices::new(config)
    }

    /// 简化的测试用HTTP Mock助手
    struct SimpleMockHelper {
        server: MockServer,
    }

    impl SimpleMockHelper {
        async fn new() -> Self {
            let server = MockServer::start().await;
            Self { server }
        }

        fn base_url(&self) -> String {
            self.server.uri()
        }

        /// 设置 tenant_access_token 成功响应
        async fn setup_tenant_token_success(&self) {
            self.server
                .register(
                    Mock::given(wiremock::matchers::method("POST"))
                        .and(wiremock::matchers::path(
                            "/open-apis/auth/v3/tenant_access_token/internal",
                        ))
                        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                            "code": 0,
                            "msg": "success",
                            "tenant_access_token": format!("t-{}", uuid::Uuid::new_v4()),
                            "expire": 7200
                        }))),
                )
                .await;
        }

        /// 设置 tenant_access_token 错误响应
        async fn setup_tenant_token_error(&self, error_code: i32, error_msg: &str) {
            self.server
                .register(
                    Mock::given(wiremock::matchers::method("POST"))
                        .and(wiremock::matchers::path(
                            "/open-apis/auth/v3/tenant_access_token/internal",
                        ))
                        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                            "code": error_code,
                            "msg": error_msg
                        }))),
                )
                .await;
        }

        /// 设置网络错误响应
        async fn setup_network_error(&self) {
            self.server
                .register(
                    Mock::given(wiremock::matchers::method("POST"))
                        .and(wiremock::matchers::path(
                            "/open-apis/auth/v3/tenant_access_token/internal",
                        ))
                        .respond_with(ResponseTemplate::new(500).set_body_json(json!({
                            "error": "Internal Server Error"
                        }))),
                )
                .await;
        }
    }

    #[test]
    fn test_tenant_access_token_service_creation() {
        let config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = TenantAccessTokenService::new(std::sync::Arc::new(config));

        // 测试构建器创建
        let _builder = service.internal();
        let _builder = service.store();
    }

    #[test]
    fn test_tenant_access_token_internal_builder() {
        let config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = TenantAccessTokenService::new(std::sync::Arc::new(config));

        let _builder = service
            .internal()
            .app_id("custom_app_id")
            .app_secret("custom_app_secret");

        // 构建器应该正确设置参数
        // 注意：这里不实际发送请求，只测试构建器的设置
    }

    // ==================== 完整的HTTP请求/响应测试 ====================

    #[tokio::test]
    async fn test_tenant_access_token_successful_request() {
        // Given: 设置Mock服务器
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_tenant_token_success().await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送租户访问令牌请求
        let result = auth_services
            .auth
            .v3()
            .tenant_access_token()
            .internal()
            .send()
            .await;

        // Then: 验证成功响应
        let token_response = assert_ok!(result);
        assert!(!token_response.tenant_access_token.is_empty());
        assert!(token_response.tenant_access_token.starts_with("t-"));
        assert_eq!(token_response.expire, 7200);

        println!(
            "✅ 租户访问令牌测试通过: {}...",
            &token_response.tenant_access_token[..16]
        );
    }

    #[tokio::test]
    async fn test_tenant_access_token_invalid_credentials() {
        // Given: 设置Mock服务器返回认证失败
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .setup_tenant_token_error(401, "invalid app credentials")
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 使用无效凭据发送请求
        let result = auth_services
            .auth
            .v3()
            .tenant_access_token()
            .internal()
            .send()
            .await;

        // Then: 验证返回错误
        assert_err!(result);
        println!("✅ 无效凭据错误处理测试通过");
    }

    #[tokio::test]
    async fn test_tenant_access_token_app_not_found() {
        // Given: 设置Mock服务器返回应用未找到
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .setup_tenant_token_error(40001, "app not found")
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送请求到不存在的应用
        let result = auth_services
            .auth
            .v3()
            .tenant_access_token()
            .internal()
            .send()
            .await;

        // Then: 验证返回错误
        assert_err!(result);
        println!("✅ 应用未找到错误处理测试通过");
    }

    #[tokio::test]
    async fn test_tenant_access_token_network_timeout() {
        // Given: 设置Mock服务器返回网络错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_network_error().await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送请求遇到网络错误
        let result = auth_services
            .auth
            .v3()
            .tenant_access_token()
            .internal()
            .send()
            .await;

        // Then: 验证网络错误处理
        assert_err!(result);
        println!("✅ 网络错误处理测试通过");
    }

    #[tokio::test]
    async fn test_tenant_access_token_concurrent_requests() {
        // Given: 设置Mock服务器
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_tenant_token_success().await;

        let auth_services = std::sync::Arc::new(create_test_auth_services(&mock_helper.base_url()));

        // When: 并发发送多个请求
        let mut handles = vec![];
        for _i in 0..5 {
            let services = auth_services.clone();
            let handle = tokio::spawn(async move {
                services
                    .auth
                    .v3()
                    .tenant_access_token()
                    .internal()
                    .send()
                    .await
            });
            handles.push(handle);
        }

        // Then: 验证所有请求都成功
        let mut success_count = 0;
        for handle in handles {
            match handle.await.unwrap() {
                Ok(_) => success_count += 1,
                Err(_) => panic!("并发请求失败"),
            }
        }

        assert_eq!(success_count, 5);
        println!("✅ 并发请求测试通过，成功处理 {} 个请求", success_count);
    }

    #[tokio::test]
    async fn test_tenant_access_token_custom_app_credentials() {
        // Given: 设置Mock服务器
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_tenant_token_success().await;

        let config = create_test_auth_config(&mock_helper.base_url());
        let auth_services = AuthServices::new(config);

        // When: 使用自定义应用凭据
        let result = auth_services
            .auth
            .v3()
            .tenant_access_token()
            .internal()
            .app_id("custom_app_id_123")
            .app_secret("custom_app_secret_456")
            .send()
            .await;

        // Then: 验证请求成功（Mock服务器不验证具体凭据）
        let token_response = assert_ok!(result);
        assert!(!token_response.tenant_access_token.is_empty());

        println!(
            "✅ 自定义凭据测试通过: {}...",
            &token_response.tenant_access_token[..16]
        );
    }

    #[tokio::test]
    async fn test_tenant_access_token_response_structure() {
        // Given: 设置Mock服务器返回特定结构的响应
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .server
            .register(
                Mock::given(wiremock::matchers::method("POST"))
                    .and(wiremock::matchers::path(
                        "/open-apis/auth/v3/tenant_access_token/internal",
                    ))
                    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                        "code": 0,
                        "msg": "success",
                        "tenant_access_token": "t-specific-test-token-12345",
                        "expire": 3600
                    }))),
            )
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送请求
        let result = auth_services
            .auth
            .v3()
            .tenant_access_token()
            .internal()
            .send()
            .await;

        // Then: 验证响应结构
        let token_response = assert_ok!(result);
        assert_eq!(
            token_response.tenant_access_token,
            "t-specific-test-token-12345"
        );
        assert_eq!(token_response.expire, 3600);

        println!("✅ 响应结构验证测试通过");
    }
}
