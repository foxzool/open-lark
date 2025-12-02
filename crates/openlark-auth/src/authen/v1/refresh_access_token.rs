/// 用户访问令牌刷新服务 (Resource: refresh_access_token)
///
/// 提供用户访问令牌的刷新功能。

use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Arc;

use crate::models::{map_feishu_auth_error, AuthErrorBuilder};
use crate::models::{AuthConfig, AuthResult};

/// 用户访问令牌刷新服务
#[derive(Debug)]
pub struct RefreshTokenService {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
}

impl RefreshTokenService {
    /// 创建新的用户访问令牌刷新服务
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// 刷新用户访问令牌
    pub fn create(&self) -> RefreshTokenBuilder {
        RefreshTokenBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            grant_type: String::new(),
            refresh_token: String::new(),
        }
    }
}

/// 用户访问令牌刷新构建器
#[derive(Debug)]
pub struct RefreshTokenBuilder {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
    grant_type: String,
    refresh_token: String,
}

impl RefreshTokenBuilder {
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

    /// 发送请求刷新用户访问令牌
    pub async fn send(self) -> AuthResult<UserAccessTokenResponse> {
        let url = format!(
            "{}/open-apis/authen/v1/refresh_access_token",
            self.config.base_url
        );

        let request_body = RefreshTokenRequest {
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
            let token_response: UserAccessTokenResponse = response.json().await?;
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

/// 刷新令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RefreshTokenRequest {
    grant_type: String,
    refresh_token: String,
}

/// 用户访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccessTokenResponse {
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间（秒）
    pub expires_in: u64,
    /// 刷新令牌
    pub refresh_token: String,
    /// 授权范围
    pub scope: String,
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

        /// 设置 refresh_access_token 成功响应
        async fn setup_refresh_token_success(&self) {
            self.server
                .register(
                    Mock::given(wiremock::matchers::method("POST"))
                        .and(wiremock::matchers::path(
                            "/open-apis/authen/v1/refresh_access_token",
                        ))
                        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                            "access_token": format!("access_{}", uuid::Uuid::new_v4()),
                            "token_type": "Bearer",
                            "expires_in": 7200,
                            "refresh_token": format!("refresh_{}", uuid::Uuid::new_v4()),
                            "scope": "userinfo:profile"
                        }))),
                )
                .await;
        }

        /// 设置 refresh_access_token 错误响应
        async fn setup_refresh_token_error(&self, error_code: i32, error_msg: &str) {
            self.server
                .register(
                    Mock::given(wiremock::matchers::method("POST"))
                        .and(wiremock::matchers::path(
                            "/open-apis/authen/v1/refresh_access_token",
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
                            "/open-apis/authen/v1/refresh_access_token",
                        ))
                        .respond_with(ResponseTemplate::new(500).set_body_json(json!({
                            "error": "Internal Server Error"
                        }))),
                )
                .await;
        }
    }

    #[test]
    fn test_refresh_token_service_creation() {
        let config = crate::models::AuthConfig::new("test_app_id", "test_app_secret");
        let service = RefreshTokenService::new(std::sync::Arc::new(config));

        // 测试构建器创建
        let _builder = service.create();
    }

    #[test]
    fn test_refresh_token_builder() {
        let config = crate::models::AuthConfig::new("test_app_id", "test_app_secret");
        let service = RefreshTokenService::new(std::sync::Arc::new(config));

        let _builder = service
            .create()
            .grant_type("refresh_token")
            .refresh_token("test_refresh_token");

        // 构建器应该正确设置参数
        // 注意：这里不实际发送请求，只测试构建器的设置
    }

    // ==================== 完整的HTTP请求/响应测试 ====================

    #[tokio::test]
    async fn test_refresh_token_successful_request() {
        // Given: 设置Mock服务器
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_refresh_token_success().await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送用户访问令牌刷新请求
        let result = auth_services
            .authen
            .v1
            .refresh_access_token()
            .create()
            .grant_type("refresh_token")
            .refresh_token("test_refresh_token")
            .send()
            .await;

        // Then: 验证成功响应
        let token_response = assert_ok!(result);
        assert!(!token_response.access_token.is_empty());
        assert_eq!(token_response.token_type, "Bearer");
        assert_eq!(token_response.expires_in, 7200);
        assert!(!token_response.refresh_token.is_empty());

        println!(
            "✅ 用户访问令牌刷新测试通过: {}...",
            &token_response.access_token[..16]
        );
    }

    #[tokio::test]
    async fn test_refresh_token_invalid_refresh_token() {
        // Given: 设置Mock服务器返回刷新令牌无效错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .setup_refresh_token_error(401, "invalid refresh token")
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 使用无效刷新令牌发送请求
        let result = auth_services
            .authen
            .v1
            .refresh_access_token()
            .create()
            .grant_type("refresh_token")
            .refresh_token("invalid_refresh_token")
            .send()
            .await;

        // Then: 验证返回错误
        assert_err!(result);
        println!("✅ 无效刷新令牌错误处理测试通过");
    }

    #[tokio::test]
    async fn test_refresh_token_expired() {
        // Given: 设置Mock服务器返回刷新令牌过期错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .setup_refresh_token_error(40001, "refresh token expired")
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 刷新令牌已过期
        let result = auth_services
            .authen
            .v1
            .refresh_access_token()
            .create()
            .grant_type("refresh_token")
            .refresh_token("expired_refresh_token")
            .send()
            .await;

        // Then: 验证刷新令牌过期错误处理
        assert_err!(result);
        println!("✅ 刷新令牌过期错误处理测试通过");
    }

    #[tokio::test]
    async fn test_refresh_token_network_timeout() {
        // Given: 设置Mock服务器返回网络错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_network_error().await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送请求遇到网络错误
        let result = auth_services
            .authen
            .v1
            .refresh_access_token()
            .create()
            .grant_type("refresh_token")
            .refresh_token("test_refresh_token")
            .send()
            .await;

        // Then: 验证网络错误处理
        assert_err!(result);
        println!("✅ 网络错误处理测试通过");
    }

    #[tokio::test]
    async fn test_refresh_token_concurrent_requests() {
        // Given: 设置Mock服务器
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_refresh_token_success().await;

        let auth_services = std::sync::Arc::new(create_test_auth_services(&mock_helper.base_url()));

        // When: 并发发送多个请求
        let mut handles = vec![];
        for _i in 0..5 {
            let services = auth_services.clone();
            let handle = tokio::spawn(async move {
                services
                    .authen
                    .v1
                    .refresh_access_token()
                    .create()
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
                Err(_) => panic!("并发请求失败"),
            }
        }

        assert_eq!(success_count, 5);
        println!("✅ 并发请求测试通过，成功处理 {} 个请求", success_count);
    }

    #[tokio::test]
    async fn test_refresh_token_custom_refresh_token() {
        // Given: 设置Mock服务器
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_refresh_token_success().await;

        let config = create_test_auth_config(&mock_helper.base_url());
        let auth_services = AuthServices::new(config);

        // When: 使用自定义刷新令牌
        let result = auth_services
            .authen
            .v1
            .refresh_access_token()
            .create()
            .grant_type("refresh_token")
            .refresh_token("custom_refresh_token_123")
            .send()
            .await;

        // Then: 验证请求成功（Mock服务器不验证具体令牌）
        let token_response = assert_ok!(result);
        assert!(!token_response.access_token.is_empty());

        println!(
            "✅ 自定义刷新令牌测试通过: {}...",
            &token_response.access_token[..16]
        );
    }

    #[tokio::test]
    async fn test_refresh_token_response_structure() {
        // Given: 设置Mock服务器返回特定结构的响应
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .server
            .register(
                Mock::given(wiremock::matchers::method("POST"))
                    .and(wiremock::matchers::path(
                        "/open-apis/authen/v1/refresh_access_token",
                    ))
                    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                        "access_token": "access-specific-test-token-12345",
                        "token_type": "Bearer",
                        "expires_in": 3600,
                        "refresh_token": "refresh-specific-test-token-67890",
                        "scope": "userinfo:profile email"
                    }))),
            )
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送请求
        let result = auth_services
            .authen
            .v1
            .refresh_access_token()
            .create()
            .grant_type("refresh_token")
            .refresh_token("test_refresh_token")
            .send()
            .await;

        // Then: 验证响应结构
        let token_response = assert_ok!(result);
        assert_eq!(
            token_response.access_token,
            "access-specific-test-token-12345"
        );
        assert_eq!(token_response.token_type, "Bearer");
        assert_eq!(token_response.expires_in, 3600);
        assert_eq!(
            token_response.refresh_token,
            "refresh-specific-test-token-67890"
        );
        assert_eq!(token_response.scope, "userinfo:profile email");

        println!("✅ 响应结构验证测试通过");
    }

    #[tokio::test]
    async fn test_refresh_token_rate_limit_error() {
        // Given: 设置Mock服务器返回频率限制错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .setup_refresh_token_error(429, "rate limit exceeded")
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送请求超过频率限制
        let result = auth_services
            .authen
            .v1
            .refresh_access_token()
            .create()
            .grant_type("refresh_token")
            .refresh_token("test_refresh_token")
            .send()
            .await;

        // Then: 验证频率限制错误处理
        assert_err!(result);
        println!("✅ 频率限制错误处理测试通过");
    }

    #[tokio::test]
    async fn test_refresh_token_server_error() {
        // Given: 设置Mock服务器返回服务器内部错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .setup_refresh_token_error(500, "internal server error")
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 服务器内部错误
        let result = auth_services
            .authen
            .v1
            .refresh_access_token()
            .create()
            .grant_type("refresh_token")
            .refresh_token("test_refresh_token")
            .send()
            .await;

        // Then: 验证服务器错误处理
        assert_err!(result);
        println!("✅ 服务器错误处理测试通过");
    }
}