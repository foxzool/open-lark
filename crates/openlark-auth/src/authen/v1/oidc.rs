//! OIDC认证服务 (Resource: oidc)
//!
//! 提供OIDC认证功能，包括访问令牌的获取和刷新。

use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Arc;

use crate::models::{map_feishu_auth_error, AuthErrorBuilder};
use crate::models::{AuthConfig, AuthResult};

/// OIDC认证服务
#[derive(Debug)]
pub struct OidcService {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
}

impl OidcService {
    /// 创建新的OIDC认证服务
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// 获取OIDC访问令牌
    pub fn create_access_token(&self) -> OidcAccessTokenBuilder {
        OidcAccessTokenBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            grant_type: String::new(),
            code: String::new(),
        }
    }

    /// 刷新OIDC访问令牌
    pub fn create_refresh_access_token(&self) -> OidcRefreshTokenBuilder {
        OidcRefreshTokenBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            grant_type: String::new(),
            refresh_token: String::new(),
        }
    }
}

/// OIDC访问令牌构建器
#[derive(Debug)]
pub struct OidcAccessTokenBuilder {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
    grant_type: String,
    code: String,
}

impl OidcAccessTokenBuilder {
    /// 设置授权类型
    pub fn grant_type(mut self, grant_type: impl Into<String>) -> Self {
        self.grant_type = grant_type.into();
        self
    }

    /// 设置授权码
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = code.into();
        self
    }

    /// 发送请求获取OIDC访问令牌
    pub async fn send(self) -> AuthResult<OidcAccessTokenResponse> {
        let url = format!(
            "{}/open-apis/authen/v1/oidc/access_token",
            self.config.base_url
        );

        let request_body = OidcAccessTokenRequest {
            grant_type: self.grant_type,
            code: self.code,
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
            let token_response: OidcAccessTokenResponse = response.json().await?;
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

/// OIDC刷新令牌构建器
#[derive(Debug)]
pub struct OidcRefreshTokenBuilder {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
    grant_type: String,
    refresh_token: String,
}

impl OidcRefreshTokenBuilder {
    /// 设置授权类型
    pub fn grant_type(mut self, grant_type: impl Into<String>) -> Self {
        self.grant_type = grant_type.into();
        self
    }

    /// 设置刷新令牌
    pub fn refresh_token(mut self, refresh_token: impl Into<String>) -> Self {
        self.refresh_token = refresh_token.into();
        self
    }

    /// 发送请求刷新OIDC访问令牌
    pub async fn send(self) -> AuthResult<OidcAccessTokenResponse> {
        let url = format!(
            "{}/open-apis/authen/v1/oidc/refresh_access_token",
            self.config.base_url
        );

        let request_body = OidcRefreshTokenRequest {
            grant_type: self.grant_type,
            refresh_token: self.refresh_token,
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
            let token_response: OidcAccessTokenResponse = response.json().await?;
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

/// OIDC访问令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OidcAccessTokenRequest {
    grant_type: String,
    code: String,
}

/// OIDC刷新令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OidcRefreshTokenRequest {
    grant_type: String,
    refresh_token: String,
}

/// OIDC访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcAccessTokenResponse {
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间（秒）
    pub expires_in: i32,
    /// 权限范围
    pub scope: Option<String>,
    /// 刷新令牌
    pub refresh_token: Option<String>,
    /// ID令牌
    pub id_token: Option<String>,
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

        /// 设置 OIDC access_token 成功响应
        async fn setup_oidc_access_token_success(&self) {
            self.server
                .register(
                    Mock::given(wiremock::matchers::method("POST"))
                        .and(wiremock::matchers::path(
                            "/open-apis/authen/v1/oidc/access_token",
                        ))
                        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                            "access_token": format!("oidc_access_{}", uuid::Uuid::new_v4()),
                            "token_type": "Bearer",
                            "expires_in": 3600,
                            "scope": "openid profile email",
                            "refresh_token": format!("oidc_refresh_{}", uuid::Uuid::new_v4()),
                            "id_token": format!("oidc_id_{}", uuid::Uuid::new_v4())
                        }))),
                )
                .await;
        }

        /// 设置 OIDC refresh_access_token 成功响应
        async fn setup_oidc_refresh_access_token_success(&self) {
            self.server
                .register(
                    Mock::given(wiremock::matchers::method("POST"))
                        .and(wiremock::matchers::path(
                            "/open-apis/authen/v1/oidc/refresh_access_token",
                        ))
                        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                            "access_token": format!("oidc_new_access_{}", uuid::Uuid::new_v4()),
                            "token_type": "Bearer",
                            "expires_in": 7200,
                            "scope": "openid profile email",
                            "refresh_token": format!("oidc_new_refresh_{}", uuid::Uuid::new_v4()),
                            "id_token": format!("oidc_new_id_{}", uuid::Uuid::new_v4())
                        }))),
                )
                .await;
        }

        /// 设置 OIDC 错误响应
        async fn setup_oidc_error(&self, endpoint: &str, status_code: u16) {
            self.server
                .register(
                    Mock::given(wiremock::matchers::method("POST"))
                        .and(wiremock::matchers::path(endpoint))
                        .respond_with(ResponseTemplate::new(status_code)),
                )
                .await;
        }

        /// 设置无效授权码错误响应
        async fn setup_invalid_code_error(&self) {
            self.server
                .register(
                    Mock::given(wiremock::matchers::method("POST"))
                        .and(wiremock::matchers::path(
                            "/open-apis/authen/v1/oidc/access_token",
                        ))
                        .and(wiremock::matchers::body_json(json!({
                            "grant_type": "authorization_code",
                            "code": "invalid_code"
                        })))
                        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                            "error": "invalid_grant",
                            "error_description": "Invalid authorization code"
                        }))),
                )
                .await;
        }

        /// 设置无效刷新令牌错误响应
        async fn setup_invalid_refresh_token_error(&self) {
            self.server
                .register(
                    Mock::given(wiremock::matchers::method("POST"))
                        .and(wiremock::matchers::path(
                            "/open-apis/authen/v1/oidc/refresh_access_token",
                        ))
                        .and(wiremock::matchers::body_json(json!({
                            "grant_type": "refresh_token",
                            "refresh_token": "invalid_refresh_token"
                        })))
                        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                            "error": "invalid_grant",
                            "error_description": "Invalid refresh token"
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
                            "/open-apis/authen/v1/oidc/access_token",
                        ))
                        .respond_with(ResponseTemplate::new(500).set_body_json(json!({
                            "error": "Internal Server Error"
                        }))),
                )
                .await;
        }
    }

    // ==================== 基础构建器测试 ====================

    #[test]
    fn test_oidc_service_creation() {
        let config = crate::models::AuthConfig::new("test_app_id", "test_app_secret");
        let service = OidcService::new(std::sync::Arc::new(config));

        // 测试构建器创建
        let _builder = service.create_access_token();
        let _refresh_builder = service.create_refresh_access_token();
    }

    #[test]
    fn test_oidc_access_token_builder() {
        let config = crate::models::AuthConfig::new("test_app_id", "test_app_secret");
        let service = OidcService::new(std::sync::Arc::new(config));

        let _builder = service
            .create_access_token()
            .grant_type("authorization_code")
            .code("test_code");

        // 构建器应该正确设置参数
        // 注意：这里不实际发送请求，只测试构建器的设置
    }

    #[test]
    fn test_oidc_refresh_access_token_builder() {
        let config = crate::models::AuthConfig::new("test_app_id", "test_app_secret");
        let service = OidcService::new(std::sync::Arc::new(config));

        let _builder = service
            .create_refresh_access_token()
            .grant_type("refresh_token")
            .refresh_token("test_refresh_token");

        // 构建器应该正确设置参数
        // 注意：这里不实际发送请求，只测试构建器的设置
    }

    // ==================== OIDC Access Token 完整HTTP请求/响应测试 ====================

    #[tokio::test]
    async fn test_oidc_access_token_successful_request() {
        // Given: 设置Mock服务器
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_oidc_access_token_success().await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送OIDC访问令牌请求
        let result = auth_services
            .authen
            .v1
            .oidc()
            .create_access_token()
            .grant_type("authorization_code")
            .code("test_authorization_code")
            .send()
            .await;

        // Then: 验证成功响应
        let token_response = assert_ok!(result);
        assert!(!token_response.access_token.is_empty());
        assert!(token_response.access_token.starts_with("oidc_access_"));
        assert_eq!(token_response.token_type, "Bearer");
        assert_eq!(token_response.expires_in, 3600);
        assert!(token_response.scope.is_some());
        assert_eq!(token_response.scope.unwrap(), "openid profile email");
        assert!(token_response.refresh_token.is_some());
        assert!(token_response.id_token.is_some());

        println!(
            "✅ OIDC访问令牌测试通过: {}...",
            &token_response.access_token[..16]
        );
    }

    #[tokio::test]
    async fn test_oidc_access_token_invalid_code() {
        // Given: 设置Mock服务器返回无效授权码错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_invalid_code_error().await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 使用无效授权码发送请求
        let result = auth_services
            .authen
            .v1
            .oidc()
            .create_access_token()
            .grant_type("authorization_code")
            .code("invalid_code")
            .send()
            .await;

        // Then: 验证返回错误
        assert_err!(result);
        println!("✅ 无效授权码错误处理测试通过");
    }

    #[tokio::test]
    async fn test_oidc_access_token_unauthorized() {
        // Given: 设置Mock服务器返回401错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .setup_oidc_error("/open-apis/authen/v1/oidc/access_token", 401)
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送未授权请求
        let result = auth_services
            .authen
            .v1
            .oidc()
            .create_access_token()
            .grant_type("authorization_code")
            .code("test_code")
            .send()
            .await;

        // Then: 验证返回错误
        assert_err!(result);
        println!("✅ 未授权错误处理测试通过");
    }

    #[tokio::test]
    async fn test_oidc_access_token_forbidden() {
        // Given: 设置Mock服务器返回403错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .setup_oidc_error("/open-apis/authen/v1/oidc/access_token", 403)
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送禁止访问请求
        let result = auth_services
            .authen
            .v1
            .oidc()
            .create_access_token()
            .grant_type("authorization_code")
            .code("test_code")
            .send()
            .await;

        // Then: 验证返回错误
        assert_err!(result);
        println!("✅ 禁止访问错误处理测试通过");
    }

    #[tokio::test]
    async fn test_oidc_access_token_network_timeout() {
        // Given: 设置Mock服务器返回网络错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_network_error().await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送请求遇到网络错误
        let result = auth_services
            .authen
            .v1
            .oidc()
            .create_access_token()
            .grant_type("authorization_code")
            .code("test_code")
            .send()
            .await;

        // Then: 验证网络错误处理
        assert_err!(result);
        println!("✅ 网络错误处理测试通过");
    }

    #[tokio::test]
    async fn test_oidc_access_token_concurrent_requests() {
        // Given: 设置Mock服务器
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_oidc_access_token_success().await;

        let auth_services = std::sync::Arc::new(create_test_auth_services(&mock_helper.base_url()));

        // When: 并发发送多个请求
        let mut handles = vec![];
        for _i in 0..5 {
            let services = auth_services.clone();
            let handle = tokio::spawn(async move {
                services
                    .authen
                    .v1
                    .oidc()
                    .create_access_token()
                    .grant_type("authorization_code")
                    .code("test_code")
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
    async fn test_oidc_access_token_custom_grant_type() {
        // Given: 设置Mock服务器
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .server
            .register(
                Mock::given(wiremock::matchers::method("POST"))
                    .and(wiremock::matchers::path(
                        "/open-apis/authen/v1/oidc/access_token",
                    ))
                    .and(wiremock::matchers::body_json(json!({
                        "grant_type": "client_credentials",
                        "code": "custom_code_123"
                    })))
                    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                        "access_token": "oidc_custom_access_123",
                        "token_type": "Bearer",
                        "expires_in": 1800,
                        "scope": "custom_scope",
                        "refresh_token": "oidc_custom_refresh_123",
                        "id_token": "oidc_custom_id_123"
                    }))),
            )
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 使用自定义授权类型
        let result = auth_services
            .authen
            .v1
            .oidc()
            .create_access_token()
            .grant_type("client_credentials")
            .code("custom_code_123")
            .send()
            .await;

        // Then: 验证请求成功
        let token_response = assert_ok!(result);
        assert_eq!(token_response.access_token, "oidc_custom_access_123");
        assert_eq!(token_response.expires_in, 1800);

        println!("✅ 自定义授权类型测试通过: {}", token_response.access_token);
    }

    // ==================== OIDC Refresh Access Token 完整HTTP请求/响应测试 ====================

    #[tokio::test]
    async fn test_oidc_refresh_access_token_successful_request() {
        // Given: 设置Mock服务器
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_oidc_refresh_access_token_success().await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送OIDC刷新访问令牌请求
        let result = auth_services
            .authen
            .v1
            .oidc()
            .create_refresh_access_token()
            .grant_type("refresh_token")
            .refresh_token("test_refresh_token")
            .send()
            .await;

        // Then: 验证成功响应
        let token_response = assert_ok!(result);
        assert!(!token_response.access_token.is_empty());
        assert!(token_response.access_token.starts_with("oidc_new_access_"));
        assert_eq!(token_response.token_type, "Bearer");
        assert_eq!(token_response.expires_in, 7200);
        assert!(token_response.scope.is_some());
        assert_eq!(token_response.scope.unwrap(), "openid profile email");
        assert!(token_response.refresh_token.is_some());
        assert!(token_response.id_token.is_some());

        println!(
            "✅ OIDC刷新访问令牌测试通过: {}...",
            &token_response.access_token[..16]
        );
    }

    #[tokio::test]
    async fn test_oidc_refresh_access_token_invalid_token() {
        // Given: 设置Mock服务器返回无效刷新令牌错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_invalid_refresh_token_error().await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 使用无效刷新令牌发送请求
        let result = auth_services
            .authen
            .v1
            .oidc()
            .create_refresh_access_token()
            .grant_type("refresh_token")
            .refresh_token("invalid_refresh_token")
            .send()
            .await;

        // Then: 验证返回错误
        assert_err!(result);
        println!("✅ 无效刷新令牌错误处理测试通过");
    }

    #[tokio::test]
    async fn test_oidc_refresh_access_token_expired_token() {
        // Given: 设置Mock服务器返回401错误（令牌过期）
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .setup_oidc_error("/open-apis/authen/v1/oidc/refresh_access_token", 401)
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 使用过期刷新令牌发送请求
        let result = auth_services
            .authen
            .v1
            .oidc()
            .create_refresh_access_token()
            .grant_type("refresh_token")
            .refresh_token("expired_refresh_token")
            .send()
            .await;

        // Then: 验证返回错误
        assert_err!(result);
        println!("✅ 过期刷新令牌错误处理测试通过");
    }

    #[tokio::test]
    async fn test_oidc_refresh_access_token_forbidden() {
        // Given: 设置Mock服务器返回403错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .setup_oidc_error("/open-apis/authen/v1/oidc/refresh_access_token", 403)
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送禁止访问的刷新令牌请求
        let result = auth_services
            .authen
            .v1
            .oidc()
            .create_refresh_access_token()
            .grant_type("refresh_token")
            .refresh_token("forbidden_refresh_token")
            .send()
            .await;

        // Then: 验证返回错误
        assert_err!(result);
        println!("✅ 禁止访问刷新令牌错误处理测试通过");
    }

    #[tokio::test]
    async fn test_oidc_refresh_access_token_network_error() {
        // Given: 设置Mock服务器返回网络错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .setup_oidc_error("/open-apis/authen/v1/oidc/refresh_access_token", 500)
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送请求遇到网络错误
        let result = auth_services
            .authen
            .v1
            .oidc()
            .create_refresh_access_token()
            .grant_type("refresh_token")
            .refresh_token("test_refresh_token")
            .send()
            .await;

        // Then: 验证网络错误处理
        assert_err!(result);
        println!("✅ 网络错误处理测试通过");
    }

    #[tokio::test]
    async fn test_oidc_refresh_access_token_concurrent_requests() {
        // Given: 设置Mock服务器
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_oidc_refresh_access_token_success().await;

        let auth_services = std::sync::Arc::new(create_test_auth_services(&mock_helper.base_url()));

        // When: 并发发送多个刷新请求
        let mut handles = vec![];
        for _i in 0..5 {
            let services = auth_services.clone();
            let handle = tokio::spawn(async move {
                services
                    .authen
                    .v1
                    .oidc()
                    .create_refresh_access_token()
                    .grant_type("refresh_token")
                    .refresh_token("test_refresh_token")
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
                Err(_) => panic!("并发刷新请求失败"),
            }
        }

        assert_eq!(success_count, 5);
        println!("✅ 并发刷新请求测试通过，成功处理 {} 个请求", success_count);
    }

    #[tokio::test]
    async fn test_oidc_refresh_access_token_custom_response() {
        // Given: 设置Mock服务器返回特定结构的响应
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .server
            .register(
                Mock::given(wiremock::matchers::method("POST"))
                    .and(wiremock::matchers::path(
                        "/open-apis/authen/v1/oidc/refresh_access_token",
                    ))
                    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                        "access_token": "oidc_specific_refresh_token_12345",
                        "token_type": "Bearer",
                        "expires_in": 5400,
                        "scope": "openid profile",
                        "refresh_token": "oidc_specific_refresh_new_12345",
                        "id_token": "oidc_specific_id_new_12345"
                    }))),
            )
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送刷新令牌请求
        let result = auth_services
            .authen
            .v1
            .oidc()
            .create_refresh_access_token()
            .grant_type("refresh_token")
            .refresh_token("specific_refresh_token")
            .send()
            .await;

        // Then: 验证响应结构
        let token_response = assert_ok!(result);
        assert_eq!(
            token_response.access_token,
            "oidc_specific_refresh_token_12345"
        );
        assert_eq!(token_response.expires_in, 5400);
        assert_eq!(token_response.scope.unwrap(), "openid profile");

        println!("✅ 刷新令牌响应结构验证测试通过");
    }

    #[tokio::test]
    async fn test_oidc_access_token_rate_limit_error() {
        // Given: 设置Mock服务器返回频率限制错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .setup_oidc_error("/open-apis/authen/v1/oidc/access_token", 429)
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送请求超过频率限制
        let result = auth_services
            .authen
            .v1
            .oidc()
            .create_access_token()
            .grant_type("authorization_code")
            .code("test_code")
            .send()
            .await;

        // Then: 验证频率限制错误处理
        assert_err!(result);
        println!("✅ 频率限制错误处理测试通过");
    }
}
