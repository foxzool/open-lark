//! 应用票据服务 (Resource: app_ticket)
//!
//! 提供应用票据的重新推送功能。

use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Arc;

use crate::models::{map_feishu_auth_error, AuthErrorBuilder};
use crate::models::{AppTicketResponse, AuthConfig, AuthResult};

/// 应用票据服务
#[derive(Debug)]
pub struct AppTicketService {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
}

impl AppTicketService {
    /// 创建新的应用票据服务
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// 重新推送应用票据
    pub fn resend(&self) -> AppTicketResendBuilder {
        AppTicketResendBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            app_id: self.config.app_id.clone(),
            app_secret: self.config.app_secret.clone(),
        }
    }
}

/// 应用票据重新推送构建器
#[derive(Debug)]
pub struct AppTicketResendBuilder {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
    app_id: String,
    app_secret: String,
}

impl AppTicketResendBuilder {
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

    /// 发送请求重新推送票据
    pub async fn send(self) -> AuthResult<AppTicketResponse> {
        let url = format!(
            "{}/open-apis/auth/v3/app_ticket/resend",
            self.config.base_url
        );

        let request_body = AppTicketResendRequest {
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
            let ticket_response: AppTicketResponse = response.json().await?;
            Ok(ticket_response)
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

/// 应用票据重新推送请求
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppTicketResendRequest {
    app_id: String,
    app_secret: String,
}

/// 应用票据重新推送响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppTicketResendResponse {
    /// 应用票据
    pub app_ticket: String,
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
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

        /// 设置 app_ticket/resend 成功响应
        async fn setup_app_ticket_resend_success(&self) {
            self.server
                .register(
                    Mock::given(wiremock::matchers::method("POST"))
                        .and(wiremock::matchers::path(
                            "/open-apis/auth/v3/app_ticket/resend",
                        ))
                        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                            "code": 0,
                            "msg": "success",
                            "app_ticket": format!("ticket-{}", uuid::Uuid::new_v4())
                        }))),
                )
                .await;
        }

        /// 设置 app_ticket/resend 错误响应
        async fn setup_app_ticket_resend_error(&self, error_code: i32, error_msg: &str) {
            self.server
                .register(
                    Mock::given(wiremock::matchers::method("POST"))
                        .and(wiremock::matchers::path(
                            "/open-apis/auth/v3/app_ticket/resend",
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
                            "/open-apis/auth/v3/app_ticket/resend",
                        ))
                        .respond_with(ResponseTemplate::new(500).set_body_json(json!({
                            "error": "Internal Server Error"
                        }))),
                )
                .await;
        }
    }

    #[test]
    fn test_app_ticket_service_creation() {
        let config = crate::models::AuthConfig::new("test_app_id", "test_app_secret");
        let service = AppTicketService::new(std::sync::Arc::new(config));

        // 测试构建器创建
        let _builder = service.resend();
    }

    #[test]
    fn test_app_ticket_resend_builder() {
        let config = crate::models::AuthConfig::new("test_app_id", "test_app_secret");
        let service = AppTicketService::new(std::sync::Arc::new(config));

        let _builder = service
            .resend()
            .app_id("custom_app_id")
            .app_secret("custom_app_secret");

        // 构建器应该正确设置参数
        // 注意：这里不实际发送请求，只测试构建器的设置
    }

    // ==================== 完整的HTTP请求/响应测试 ====================

    #[tokio::test]
    async fn test_app_ticket_resend_successful_request() {
        // Given: 设置Mock服务器
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_app_ticket_resend_success().await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送应用票据重新推送请求
        let result = auth_services.auth.v3().app_ticket().resend().send().await;

        // Then: 验证成功响应
        let ticket_response = assert_ok!(result);
        assert!(!ticket_response.app_ticket.is_empty());
        assert!(ticket_response.app_ticket.starts_with("ticket-"));

        println!(
            "✅ 应用票据重新推送测试通过: {}...",
            &ticket_response.app_ticket[..16]
        );
    }

    #[tokio::test]
    async fn test_app_ticket_resend_invalid_credentials() {
        // Given: 设置Mock服务器返回认证失败
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .setup_app_ticket_resend_error(401, "invalid app credentials")
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 使用无效凭据发送请求
        let result = auth_services.auth.v3().app_ticket().resend().send().await;

        // Then: 验证返回错误
        assert_err!(result);
        println!("✅ 无效凭据错误处理测试通过");
    }

    #[tokio::test]
    async fn test_app_ticket_resend_app_not_found() {
        // Given: 设置Mock服务器返回应用未找到
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .setup_app_ticket_resend_error(40001, "app not found")
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送请求到不存在的应用
        let result = auth_services.auth.v3().app_ticket().resend().send().await;

        // Then: 验证返回错误
        assert_err!(result);
        println!("✅ 应用未找到错误处理测试通过");
    }

    #[tokio::test]
    async fn test_app_ticket_resend_network_timeout() {
        // Given: 设置Mock服务器返回网络错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_network_error().await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送请求遇到网络错误
        let result = auth_services.auth.v3().app_ticket().resend().send().await;

        // Then: 验证网络错误处理
        assert_err!(result);
        println!("✅ 网络错误处理测试通过");
    }

    #[tokio::test]
    async fn test_app_ticket_resend_concurrent_requests() {
        // Given: 设置Mock服务器
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_app_ticket_resend_success().await;

        let auth_services = std::sync::Arc::new(create_test_auth_services(&mock_helper.base_url()));

        // When: 并发发送多个请求
        let mut handles = vec![];
        for _i in 0..5 {
            let services = auth_services.clone();
            let handle =
                tokio::spawn(async move { services.auth.v3().app_ticket().resend().send().await });
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
    async fn test_app_ticket_resend_custom_app_credentials() {
        // Given: 设置Mock服务器
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_app_ticket_resend_success().await;

        let config = create_test_auth_config(&mock_helper.base_url());
        let auth_services = AuthServices::new(config);

        // When: 使用自定义应用凭据
        let result = auth_services
            .auth
            .v3()
            .app_ticket()
            .resend()
            .app_id("custom_app_id_123")
            .app_secret("custom_app_secret_456")
            .send()
            .await;

        // Then: 验证请求成功（Mock服务器不验证具体凭据）
        let ticket_response = assert_ok!(result);
        assert!(!ticket_response.app_ticket.is_empty());

        println!(
            "✅ 自定义凭据测试通过: {}...",
            &ticket_response.app_ticket[..16]
        );
    }

    #[tokio::test]
    async fn test_app_ticket_resend_response_structure() {
        // Given: 设置Mock服务器返回特定结构的响应
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .server
            .register(
                Mock::given(wiremock::matchers::method("POST"))
                    .and(wiremock::matchers::path(
                        "/open-apis/auth/v3/app_ticket/resend",
                    ))
                    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                        "code": 0,
                        "msg": "success",
                        "app_ticket": "ticket-specific-test-ticket-12345"
                    }))),
            )
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送请求
        let result = auth_services.auth.v3().app_ticket().resend().send().await;

        // Then: 验证响应结构
        let ticket_response = assert_ok!(result);
        assert_eq!(
            ticket_response.app_ticket,
            "ticket-specific-test-ticket-12345"
        );

        println!("✅ 响应结构验证测试通过");
    }

    #[tokio::test]
    async fn test_app_ticket_resend_rate_limit_error() {
        // Given: 设置Mock服务器返回频率限制错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .setup_app_ticket_resend_error(429, "rate limit exceeded")
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送请求超过频率限制
        let result = auth_services.auth.v3().app_ticket().resend().send().await;

        // Then: 验证频率限制错误处理
        assert_err!(result);
        println!("✅ 频率限制错误处理测试通过");
    }

    #[tokio::test]
    async fn test_app_ticket_resend_server_error() {
        // Given: 设置Mock服务器返回服务器内部错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .setup_app_ticket_resend_error(500, "internal server error")
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 服务器内部错误
        let result = auth_services.auth.v3().app_ticket().resend().send().await;

        // Then: 验证服务器错误处理
        assert_err!(result);
        println!("✅ 服务器错误处理测试通过");
    }
}
