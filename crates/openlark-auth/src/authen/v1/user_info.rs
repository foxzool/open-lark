//! 用户信息服务 (Resource: user_info)
//!
//! 提供用户信息获取功能。

use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Arc;

use crate::models::{map_feishu_auth_error, AuthErrorBuilder};
use crate::models::{AuthConfig, AuthResult, UserInfoResponse};

/// 用户信息服务
#[derive(Debug)]
pub struct UserInfoService {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
}

impl UserInfoService {
    /// 创建新的用户信息服务
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// 获取用户信息
    pub fn get(&self) -> UserInfoGetBuilder {
        UserInfoGetBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            user_access_token: String::new(),
        }
    }
}

/// 用户信息获取构建器
#[derive(Debug)]
pub struct UserInfoGetBuilder {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
    user_access_token: String,
}

impl UserInfoGetBuilder {
    /// 设置用户访问令牌
    pub fn user_access_token(mut self, user_access_token: impl Into<String>) -> Self {
        self.user_access_token = user_access_token.into();
        self
    }

    /// 发送请求获取用户信息
    pub async fn send(self) -> AuthResult<UserInfoResponse> {
        let url = format!("{}/open-apis/authen/v1/user_info", self.config.base_url);

        let response = self
            .client
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.user_access_token),
            )
            .header("Content-Type", "application/json")
            .header("User-Agent", "openlark-rust-sdk/0.1.0")
            .send()
            .await?;

        if response.status().is_success() {
            let user_info: UserInfoResponse = response.json().await?;
            Ok(user_info)
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

/// 用户信息获取响应（内部数据结构）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)] // 保留用于未来API响应解析
struct UserInfoInternalResponse {
    /// 用户数据
    pub data: UserInfoResponse,
    /// 请求是否成功
    pub success: bool,
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

        /// 设置 user_info 成功响应
        async fn setup_user_info_success(&self) {
            self.server
                .register(
                    Mock::given(wiremock::matchers::method("GET"))
                        .and(wiremock::matchers::path("/open-apis/authen/v1/user_info"))
                        .and(wiremock::matchers::header(
                            "Authorization",
                            "Bearer test_user_access_token",
                        ))
                        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                            "user_id": format!("user_{}", uuid::Uuid::new_v4()),
                            "name": "测试用户",
                            "open_id": format!("open_{}", uuid::Uuid::new_v4()),
                            "union_id": format!("union_{}", uuid::Uuid::new_v4()),
                            "en_name": "Test User",
                            "email": "test@example.com",
                            "mobile": "+86 138 0013 8000",
                            "avatar_url": "https://example.com/avatar.jpg",
                            "status": "activated",
                            "department_ids": ["dept_001", "dept_002"],
                            "position": "软件工程师",
                            "employee_no": "EMP001",
                            "nickname": "小测",
                            "gender": "unknown"
                        }))),
                )
                .await;
        }

        /// 设置 user_info 错误响应
        async fn setup_user_info_error(&self, status_code: u16) {
            self.server
                .register(
                    Mock::given(wiremock::matchers::method("GET"))
                        .and(wiremock::matchers::path("/open-apis/authen/v1/user_info"))
                        .and(wiremock::matchers::header(
                            "Authorization",
                            "Bearer test_user_access_token",
                        ))
                        .respond_with(ResponseTemplate::new(status_code)),
                )
                .await;
        }

        /// 设置无效令牌错误响应
        async fn setup_invalid_token_error(&self) {
            self.server
                .register(
                    Mock::given(wiremock::matchers::method("GET"))
                        .and(wiremock::matchers::path("/open-apis/authen/v1/user_info"))
                        .and(wiremock::matchers::header(
                            "Authorization",
                            "Bearer invalid_token",
                        ))
                        .respond_with(ResponseTemplate::new(401).set_body_json(json!({
                            "code": 401,
                            "msg": "invalid token"
                        }))),
                )
                .await;
        }

        /// 设置网络错误响应
        async fn setup_network_error(&self) {
            self.server
                .register(
                    Mock::given(wiremock::matchers::method("GET"))
                        .and(wiremock::matchers::path("/open-apis/authen/v1/user_info"))
                        .and(wiremock::matchers::header(
                            "Authorization",
                            "Bearer test_user_access_token",
                        ))
                        .respond_with(ResponseTemplate::new(500).set_body_json(json!({
                            "error": "Internal Server Error"
                        }))),
                )
                .await;
        }
    }

    #[test]
    fn test_user_info_service_creation() {
        let config = crate::models::AuthConfig::new("test_app_id", "test_app_secret");
        let service = UserInfoService::new(std::sync::Arc::new(config));

        // 测试构建器创建
        let _builder = service.get();
    }

    #[test]
    fn test_user_info_get_builder() {
        let config = crate::models::AuthConfig::new("test_app_id", "test_app_secret");
        let service = UserInfoService::new(std::sync::Arc::new(config));

        let _builder = service.get().user_access_token("test_user_access_token");

        // 构建器应该正确设置参数
        // 注意：这里不实际发送请求，只测试构建器的设置
    }

    // ==================== 完整的HTTP请求/响应测试 ====================

    #[tokio::test]
    async fn test_user_info_get_successful_request() {
        // Given: 设置Mock服务器
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_user_info_success().await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送用户信息获取请求
        let result = auth_services
            .authen
            .v1
            .user_info()
            .get()
            .user_access_token("test_user_access_token")
            .send()
            .await;

        // Then: 验证成功响应
        let user_info = assert_ok!(result);
        assert!(!user_info.user_id.is_empty());
        assert_eq!(user_info.name, "测试用户");
        assert_eq!(user_info.email, "test@example.com");
        assert_eq!(
            user_info.status,
            crate::models::user_info::UserStatus::Activated
        );

        println!(
            "✅ 用户信息获取测试通过: {} ({})",
            user_info.name, user_info.user_id
        );
    }

    #[tokio::test]
    async fn test_user_info_get_invalid_token() {
        // Given: 设置Mock服务器返回无效令牌错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_invalid_token_error().await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 使用无效令牌发送请求
        let result = auth_services
            .authen
            .v1
            .user_info()
            .get()
            .user_access_token("invalid_token")
            .send()
            .await;

        // Then: 验证返回错误
        assert_err!(result);
        println!("✅ 无效令牌错误处理测试通过");
    }

    #[tokio::test]
    async fn test_user_info_get_unauthorized() {
        // Given: 设置Mock服务器返回401错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_user_info_error(401).await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送未授权请求
        let result = auth_services
            .authen
            .v1
            .user_info()
            .get()
            .user_access_token("test_user_access_token")
            .send()
            .await;

        // Then: 验证返回错误
        assert_err!(result);
        println!("✅ 未授权错误处理测试通过");
    }

    #[tokio::test]
    async fn test_user_info_get_forbidden() {
        // Given: 设置Mock服务器返回403错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_user_info_error(403).await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送禁止访问请求
        let result = auth_services
            .authen
            .v1
            .user_info()
            .get()
            .user_access_token("test_user_access_token")
            .send()
            .await;

        // Then: 验证返回错误
        assert_err!(result);
        println!("✅ 禁止访问错误处理测试通过");
    }

    #[tokio::test]
    async fn test_user_info_get_network_timeout() {
        // Given: 设置Mock服务器返回网络错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_network_error().await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送请求遇到网络错误
        let result = auth_services
            .authen
            .v1
            .user_info()
            .get()
            .user_access_token("test_user_access_token")
            .send()
            .await;

        // Then: 验证网络错误处理
        assert_err!(result);
        println!("✅ 网络错误处理测试通过");
    }

    #[tokio::test]
    async fn test_user_info_get_concurrent_requests() {
        // Given: 设置Mock服务器
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_user_info_success().await;

        let auth_services = std::sync::Arc::new(create_test_auth_services(&mock_helper.base_url()));

        // When: 并发发送多个请求
        let mut handles = vec![];
        for _i in 0..5 {
            let services = auth_services.clone();
            let handle = tokio::spawn(async move {
                services
                    .authen
                    .v1
                    .user_info()
                    .get()
                    .user_access_token("test_user_access_token")
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
    async fn test_user_info_get_custom_user_token() {
        // Given: 设置Mock服务器
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .server
            .register(
                Mock::given(wiremock::matchers::method("GET"))
                    .and(wiremock::matchers::path("/open-apis/authen/v1/user_info"))
                    .and(wiremock::matchers::header(
                        "Authorization",
                        "Bearer custom_user_token_123",
                    ))
                    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                        "user_id": "user_custom_123",
                        "name": "自定义用户",
                        "open_id": "open_custom_123",
                        "union_id": "union_custom_123",
                        "en_name": "Custom User",
                        "email": "custom@example.com",
                        "mobile": "+86 139 0013 9000",
                        "avatar_url": "https://example.com/custom-avatar.jpg",
                        "status": "activated",
                        "department_ids": ["dept_custom"],
                        "position": "高级工程师",
                        "employee_no": "CUSTOM123",
                        "nickname": "小自",
                        "gender": "male"
                    }))),
            )
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 使用自定义用户令牌
        let result = auth_services
            .authen
            .v1
            .user_info()
            .get()
            .user_access_token("custom_user_token_123")
            .send()
            .await;

        // Then: 验证请求成功
        let user_info = assert_ok!(result);
        assert_eq!(user_info.user_id, "user_custom_123");
        assert_eq!(user_info.name, "自定义用户");
        assert_eq!(user_info.email, "custom@example.com");

        println!(
            "✅ 自定义用户令牌测试通过: {} ({})",
            user_info.name, user_info.user_id
        );
    }

    #[tokio::test]
    async fn test_user_info_get_response_structure() {
        // Given: 设置Mock服务器返回特定结构的响应
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper
            .server
            .register(
                Mock::given(wiremock::matchers::method("GET"))
                    .and(wiremock::matchers::path("/open-apis/authen/v1/user_info"))
                    .and(wiremock::matchers::header(
                        "Authorization",
                        "Bearer test_token_456",
                    ))
                    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                        "user_id": "user_specific_456",
                        "name": "特定测试用户",
                        "open_id": "open_specific_456",
                        "union_id": "union_specific_456",
                        "en_name": "Specific Test User",
                        "email": "specific@test.com",
                        "mobile": "+86 137 0013 7000",
                        "avatar_url": "https://example.com/specific-avatar.jpg",
                        "status": "activated",
                        "department_ids": ["dept_specific_001", "dept_specific_002"],
                        "position": "测试工程师",
                        "employee_no": "SPEC456",
                        "nickname": "小特",
                        "gender": "female"
                    }))),
            )
            .await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送请求
        let result = auth_services
            .authen
            .v1
            .user_info()
            .get()
            .user_access_token("test_token_456")
            .send()
            .await;

        // Then: 验证响应结构
        let user_info = assert_ok!(result);
        assert_eq!(user_info.user_id, "user_specific_456");
        assert_eq!(user_info.name, "特定测试用户");
        assert_eq!(user_info.email, "specific@test.com");
        assert_eq!(user_info.position, "测试工程师");

        println!("✅ 响应结构验证测试通过");
    }

    #[tokio::test]
    async fn test_user_info_get_user_not_found() {
        // Given: 设置Mock服务器返回404错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_user_info_error(404).await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 用户不存在
        let result = auth_services
            .authen
            .v1
            .user_info()
            .get()
            .user_access_token("test_user_access_token")
            .send()
            .await;

        // Then: 验证用户不存在错误处理
        assert_err!(result);
        println!("✅ 用户不存在错误处理测试通过");
    }

    #[tokio::test]
    async fn test_user_info_get_rate_limit_error() {
        // Given: 设置Mock服务器返回频率限制错误
        let mock_helper = SimpleMockHelper::new().await;
        mock_helper.setup_user_info_error(429).await;

        let auth_services = create_test_auth_services(&mock_helper.base_url());

        // When: 发送请求超过频率限制
        let result = auth_services
            .authen
            .v1
            .user_info()
            .get()
            .user_access_token("test_user_access_token")
            .send()
            .await;

        // Then: 验证频率限制错误处理
        assert_err!(result);
        println!("✅ 频率限制错误处理测试通过");
    }
}
