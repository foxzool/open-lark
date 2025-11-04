//! 集成测试模块
//!
//! 包含端到端工作流测试、WebSocket实时功能测试等
//! 验证整个SDK的集成性和稳定性

pub mod end_to_end_workflows;
#[cfg(feature = "websocket")]
pub mod websocket_integration;

use std::env;
use std::time::Duration;
use wiremock::MockServer;
use serde_json::json;
use open_lark::prelude::*;

/// 集成测试配置
pub struct IntegrationTestConfig {
    pub app_id: String,
    pub app_secret: String,
    pub base_url: String,
    pub user_access_token: Option<String>,
    pub test_chat_id: String,
    pub test_user_id: String,
    pub test_department_id: String,
}

impl IntegrationTestConfig {
    /// 从环境变量创建测试配置
    pub fn from_env() -> Option<Self> {
        let app_id = env::var("APP_ID").ok()?;
        let app_secret = env::var("APP_SECRET").ok()?;

        Some(Self {
            app_id,
            app_secret,
            base_url: env::var("BASE_URL").unwrap_or_else(|_| "https://open.feishu.cn".to_string()),
            user_access_token: env::var("USER_ACCESS_TOKEN").ok(),
            test_chat_id: env::var("TEST_CHAT_ID").unwrap_or_else(|_| "oc_test_chat_123".to_string()),
            test_user_id: env::var("TEST_USER_ID").unwrap_or_else(|_| "ou_test_user_456".to_string()),
            test_department_id: env::var("TEST_DEPARTMENT_ID").unwrap_or_else(|_| "od_test_dept_789".to_string()),
        })
    }

    /// 创建默认测试配置
    pub fn default() -> Self {
        Self {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            user_access_token: None,
            test_chat_id: "oc_test_chat_123".to_string(),
            test_user_id: "ou_test_user_456".to_string(),
            test_department_id: "od_test_dept_789".to_string(),
        }
    }

    /// 检查是否有真实的凭证（用于真实API测试）
    pub fn has_real_credentials(&self) -> bool {
        !self.app_id.starts_with("test_") && !self.app_secret.starts_with("test_")
    }
}

/// 创建集成测试客户端
pub fn create_integration_client(config: &IntegrationTestConfig) -> LarkClient {
    LarkClient::builder(&config.app_id, &config.app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_base_url(&config.base_url)
        .enable_token_cache(true)
        .req_timeout(Duration::from_secs(30))
        .build()
}

/// 设置基础Mock服务器
pub async fn setup_base_mocks(mock_server: &MockServer) {
    // Mock应用访问令牌获取
    Mock::given(wiremock::matchers::method("POST"))
        .and(wiremock::matchers::path("/open-apis/auth/v3/tenant_access_token/internal"))
        .respond_with(
            wiremock::ResponseTemplate::new(200)
                .set_body_json(json!({
                    "code": 0,
                    "msg": "success",
                    "expire": 7200,
                    "tenant_access_token": "mock_tenant_token_integration"
                }))
        )
        .mount(mock_server)
        .await;

    // Mock应用信息获取
    Mock::given(wiremock::matchers::method("GET"))
        .and(wiremock::matchers::path("/open-apis/auth/v3/app_info"))
        .respond_with(
            wiremock::ResponseTemplate::new(200)
                .set_body_json(json!({
                    "code": 0,
                    "msg": "success",
                    "data": {
                        "app": {
                            "app_id": "test_app_id",
                            "app_name": "集成测试应用",
                            "app_description": "用于SDK集成测试的应用",
                            "app_type": "self_build",
                            "app_logo": "https://example.com/logo.png",
                            "is_link_authorized": true,
                            "is_share_data": false,
                            "is_link_audit_log": false,
                            "app_status": "enabled",
                            "homepage_url": "https://example.com",
                            "privacy_policy_url": "https://example.com/privacy",
                            "service_term_url": "https://example.com/terms"
                        },
                        "tenant": {
                            "tenant_key": "test_tenant",
                            "tenant_name": "测试租户"
                        }
                    }
                }))
        )
        .mount(mock_server)
        .await;
}

/// 设置IM服务Mock
pub async fn setup_im_mocks(mock_server: &MockServer, config: &IntegrationTestConfig) {
    let tenant_token = "mock_tenant_token_integration";

    // Mock发送消息
    Mock::given(wiremock::matchers::method("POST"))
        .and(wiremock::matchers::path("/open-apis/im/v1/messages"))
        .and(wiremock::matchers::header("Authorization", format!("Bearer {}", tenant_token)))
        .respond_with(
            wiremock::ResponseTemplate::new(200)
                .set_body_json(json!({
                    "code": 0,
                    "msg": "success",
                    "data": {
                        "message_id": format!("om_msg_{}", chrono::Utc::now().timestamp()),
                        "root_id": null,
                        "parent_id": null,
                        "thread_id": null,
                        "msg_type": "text",
                        "create_time": format!("{}", chrono::Utc::now().timestamp_millis()),
                        "update_time": format!("{}", chrono::Utc::now().timestamp_millis()),
                        "deleted": false,
                        "updated": false,
                        "chat_id": config.test_chat_id,
                        "sender": {
                            "id": "cli_test_app",
                            "id_type": "app_id",
                            "sender_type": "app",
                            "tenant_key": "test_tenant"
                        },
                        "body": {
                            "content": r#"{"text":"集成测试消息"}"#
                        }
                    }
                }))
        )
        .mount(mock_server)
        .await;

    // Mock获取聊天列表
    Mock::given(wiremock::matchers::method("GET"))
        .and(wiremock::matchers::path("/open-apis/im/v1/chats"))
        .and(wiremock::matchers::header("Authorization", format!("Bearer {}", tenant_token)))
        .respond_with(
            wiremock::ResponseTemplate::new(200)
                .set_body_json(json!({
                    "code": 0,
                    "msg": "success",
                    "data": {
                        "items": [{
                            "chat_id": config.test_chat_id,
                            "avatar": "https://example.com/chat_avatar.jpg",
                            "name": "集成测试群组",
                            "description": "用于SDK集成测试的群组",
                            "owner_id": config.test_user_id,
                            "owner_id_type": "open_id",
                            "external": false,
                            "tenant_key": "test_tenant",
                            "chat_status": "active",
                            "add_member_permission": "all",
                            "share_card_permission": "all",
                            "at_all_permission": "all",
                            "edit_permission": "all",
                            "need_approval": false
                        }],
                        "page_token": "",
                        "has_more": false
                    }
                }))
        )
        .mount(mock_server)
        .await;
}

/// 设置联系人服务Mock
pub async fn setup_contact_mocks(mock_server: &MockServer, config: &IntegrationTestConfig) {
    let tenant_token = "mock_tenant_token_integration";

    // Mock获取用户列表
    Mock::given(wiremock::matchers::method("GET"))
        .and(wiremock::matchers::path("/open-apis/contact/v3/users"))
        .and(wiremock::matchers::header("Authorization", format!("Bearer {}", tenant_token)))
        .respond_with(
            wiremock::ResponseTemplate::new(200)
                .set_body_json(json!({
                    "code": 0,
                    "msg": "success",
                    "data": {
                        "items": [{
                            "user_id": config.test_user_id,
                            "union_id": format!("on_{}", config.test_user_id),
                            "open_id": config.test_user_id,
                            "name": "集成测试用户",
                            "en_name": "Integration Test User",
                            "email": "test@example.com",
                            "mobile": "+86 138 0000 0000",
                            "avatar": "https://example.com/user_avatar.jpg",
                            "status": {
                                "is_activated": true,
                                "is_frozen": false,
                                "is_resigned": false
                            },
                            "department_ids": [config.test_department_id],
                            "leader_user_id": format!("ou_leader_{}", config.test_user_id),
                            "position": "测试工程师",
                            "orders": [1],
                            "custom_attrs": [],
                            "employee_type": 1,
                            "join_time": 1609459200,
                            "employee_no": "TEST001"
                        }]
                    }
                }))
        )
        .mount(mock_server)
        .await;

    // Mock获取部门列表
    Mock::given(wiremock::matchers::method("GET"))
        .and(wiremock::matchers::path("/open-apis/contact/v3/departments"))
        .and(wiremock::matchers::header("Authorization", format!("Bearer {}", tenant_token)))
        .respond_with(
            wiremock::ResponseTemplate::new(200)
                .set_body_json(json!({
                    "code": 0,
                    "msg": "success",
                    "data": {
                        "items": [{
                            "department_id": config.test_department_id,
                            "open_department_id": config.test_department_id,
                            "name": "集成测试部门",
                            "name_en": "Integration Test Department",
                            "department_type": "department",
                            "parent_department_id": "0",
                            "leader_user_id": config.test_user_id,
                            "chat_id": format!("oc_dept_{}", config.test_department_id),
                            "member_user_id_count": 5,
                            "order": 1,
                            "status": {
                                "is_deleted": false
                            },
                            "unit_ids": []
                        }]
                    }
                }))
        )
        .mount(mock_server)
        .await;
}

/// 运行集成测试的辅助函数
#[macro_export]
macro_rules! run_integration_test {
    ($test_name:expr, $test_fn:expr) => {
        println!("\n🧪 开始集成测试: {}", $test_name);

        let _ = dotenvy::dotenv();
        let config = IntegrationTestConfig::from_env()
            .unwrap_or_else(IntegrationTestConfig::default);

        if config.has_real_credentials() {
            println!("🔑 检测到真实凭证，将进行真实API测试");
        } else {
            println!("🔧 使用模拟环境进行测试");
        }

        match $test_fn(&config).await {
            Ok(_) => {
                println!("✅ 集成测试通过: {}", $test_name);
            }
            Err(e) => {
                println!("❌ 集成测试失败: {} - {}", $test_name, e);
                panic!("集成测试失败: {}", $test_name);
            }
        }
    };
}

/// 测试结果统计
#[derive(Debug, Default)]
pub struct TestResults {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub skipped_tests: usize,
    pub test_duration: Duration,
}

impl TestResults {
    pub fn success_rate(&self) -> f64 {
        if self.total_tests == 0 {
            0.0
        } else {
            self.passed_tests as f64 / self.total_tests as f64 * 100.0
        }
    }

    pub fn print_summary(&self) {
        println!("\n📊 集成测试结果汇总:");
        println!("   总测试数: {}", self.total_tests);
        println!("   通过: {} ({:.1}%)", self.passed_tests, self.success_rate());
        println!("   失败: {}", self.failed_tests);
        println!("   跳过: {}", self.skipped_tests);
        println!("   总耗时: {:?}", self.test_duration);

        if self.failed_tests == 0 {
            println!("🎉 所有集成测试都通过了！");
        } else {
            println!("⚠️  有 {} 个测试失败，请检查日志", self.failed_tests);
        }
    }
}

/// 健康检查测试
pub async fn test_sdk_health_check(config: &IntegrationTestConfig) -> Result<(), Box<dyn std::error::Error>> {
    let client = create_integration_client(config);

    // 基础配置验证
    assert!(!client.config.app_id.is_empty());
    assert!(!client.config.app_secret.is_empty());
    assert!(!client.config.base_url.is_empty());

    // 服务可用性检查（如果使用真实凭证）
    if config.has_real_credentials() {
        match client.auth.v3.get_app_access_token(None).await {
            Ok(token_response) => {
                assert_eq!(token_response.code, 0);
                assert!(!token_response.tenant_access_token.is_empty());
                println!("✅ SDK健康检查通过：认证服务正常");
            }
            Err(e) => {
                println!("⚠️  SDK健康检查警告：认证服务异常 - {}", e.user_friendly_message());
                return Err(e.into());
            }
        }
    } else {
        println!("✅ SDK健康检查通过：模拟环境配置正常");
    }

    Ok(())
}

#[cfg(test)]
mod integration_test_utils {
    use super::*;

    #[test]
    fn test_integration_config_creation() {
        // 测试默认配置
        let default_config = IntegrationTestConfig::default();
        assert_eq!(default_config.app_id, "test_app_id");
        assert_eq!(default_config.app_secret, "test_app_secret");
        assert!(!default_config.has_real_credentials());

        // 测试环境变量配置（如果有）
        if let Some(env_config) = IntegrationTestConfig::from_env() {
            if env_config.has_real_credentials() {
                assert!(!env_config.app_id.starts_with("test_"));
                assert!(!env_config.app_secret.starts_with("test_"));
            }
        }
    }

    #[test]
    fn test_client_creation() {
        let config = IntegrationTestConfig::default();
        let client = create_integration_client(&config);

        assert_eq!(client.config.app_id, config.app_id);
        assert_eq!(client.config.app_secret, config.app_secret);
        assert_eq!(client.config.base_url, config.base_url);
    }

    #[test]
    fn test_results_calculation() {
        let mut results = TestResults::default();
        results.total_tests = 10;
        results.passed_tests = 8;
        results.failed_tests = 1;
        results.skipped_tests = 1;

        assert_eq!(results.success_rate(), 80.0);
    }
}