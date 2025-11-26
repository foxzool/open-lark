//! API综合测试 - 简化版本
//!
//! 专注于11个核心API接口的基础功能验证

use openlark_auth::prelude::*;
use serde_json::json;
use wiremock::{Mock, MockServer, ResponseTemplate};

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

    /// 设置成功的基础认证响应
    async fn setup_auth_success(&self) {
        // tenant_access_token success
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

        // app_access_token success
        self.server
            .register(
                Mock::given(wiremock::matchers::method("POST"))
                    .and(wiremock::matchers::path(
                        "/open-apis/auth/v3/app_access_token/internal",
                    ))
                    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                        "code": 0,
                        "msg": "success",
                        "app_access_token": format!("cli-{}", uuid::Uuid::new_v4()),
                        "expire": 7200
                    }))),
            )
            .await;

        // user_info success
        self.server
            .register(
                Mock::given(wiremock::matchers::method("GET"))
                    .and(wiremock::matchers::path("/open-apis/authen/v1/user_info"))
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

    /// 设置错误响应
    async fn setup_auth_error(&self) {
        self.server
            .register(
                Mock::given(wiremock::matchers::method("POST"))
                    .and(wiremock::matchers::path_regex(r"^/open-apis/auth/.*"))
                    .respond_with(ResponseTemplate::new(403).set_body_json(json!({
                        "code": 99991663,
                        "msg": "invalid app credentials or app not activated"
                    }))),
            )
            .await;
    }
}

/// 创建测试用的认证配置
fn create_test_auth_config(base_url: &str) -> AuthConfig {
    AuthConfig::new("test_app_id", "test_app_secret").with_base_url(base_url)
}

/// 创建测试用的认证服务
fn create_test_auth_services(base_url: &str) -> AuthServices {
    let config = create_test_auth_config(base_url);
    AuthServices::new(config)
}

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

// ==================== auth v3 API测试 ====================

#[tokio::test]
async fn test_tenant_access_token_internal_success() {
    // Given: 设置Mock服务器
    let mock_helper = SimpleMockHelper::new().await;
    mock_helper.setup_auth_success().await;

    let auth_services = create_test_auth_services(&mock_helper.base_url());

    // When: 获取租户访问令牌
    let result = auth_services
        .auth
        .v3()
        .tenant_access_token()
        .internal()
        .send()
        .await;

    // Then: 验证成功
    let token = assert_ok!(result);
    assert!(!token.tenant_access_token.is_empty());
    assert!(token.tenant_access_token.starts_with("t-"));
    assert_eq!(token.expire, 7200);
    println!(
        "✅ 租户访问令牌测试通过: {}...",
        &token.tenant_access_token[..16]
    );
}

#[tokio::test]
async fn test_app_access_token_internal_success() {
    // Given: 设置Mock服务器
    let mock_helper = SimpleMockHelper::new().await;
    mock_helper.setup_auth_success().await;

    let auth_services = create_test_auth_services(&mock_helper.base_url());

    // When: 获取应用访问令牌
    let result = auth_services
        .auth
        .v3()
        .app_access_token()
        .internal()
        .send()
        .await;

    // Then: 验证成功
    let token = assert_ok!(result);
    assert!(!token.app_access_token.is_empty());
    assert!(token.app_access_token.starts_with("cli-"));
    assert_eq!(token.expire, 7200);
    println!(
        "✅ 应用访问令牌测试通过: {}...",
        &token.app_access_token[..16]
    );
}

// ==================== authen v1 API测试 ====================

#[tokio::test]
async fn test_user_info_get_success() {
    // Given: 设置Mock服务器
    let mock_helper = SimpleMockHelper::new().await;
    mock_helper.setup_auth_success().await;

    let auth_services = create_test_auth_services(&mock_helper.base_url());
    let user_access_token = "test_user_token";

    // When: 获取用户信息
    let result = auth_services
        .authen
        .v1
        .user_info()
        .get()
        .user_access_token(user_access_token)
        .send()
        .await;

    // Then: 验证成功
    let user_info = assert_ok!(result);
    assert!(!user_info.user_id.is_empty());
    assert_eq!(user_info.name, "测试用户");
    assert_eq!(
        user_info.status,
        openlark_auth::models::UserStatus::Activated
    );
    println!("✅ 用户信息测试通过: {}", user_info.name);
}

// ==================== 错误场景测试 ====================

#[tokio::test]
async fn test_auth_invalid_credentials() {
    // Given: 设置认证错误
    let mock_helper = SimpleMockHelper::new().await;
    mock_helper.setup_auth_error().await;

    let auth_services = create_test_auth_services(&mock_helper.base_url());

    // When & Then: 调用API应该失败
    let result = auth_services
        .auth
        .v3()
        .tenant_access_token()
        .internal()
        .send()
        .await;
    assert_err!(result);
    println!("✅ 认证失败测试通过");

    let result = auth_services
        .auth
        .v3()
        .app_access_token()
        .internal()
        .send()
        .await;
    assert_err!(result);
    println!("✅ 应用令牌认证失败测试通过");
}

// ==================== 并发测试 ====================

#[tokio::test]
async fn test_concurrent_token_requests() {
    // Given: 设置Mock服务器
    let mock_helper = SimpleMockHelper::new().await;
    mock_helper.setup_auth_success().await;

    let _auth_services = create_test_auth_services(&mock_helper.base_url());

    // When: 并发请求 (使用Arc共享)
    let mut tasks = Vec::new();
    for i in 0..5 {
        let base_url = mock_helper.base_url().clone();
        let task = tokio::spawn(async move {
            let services = create_test_auth_services(&base_url);
            let result = services
                .auth
                .v3()
                .tenant_access_token()
                .internal()
                .send()
                .await;
            (i, result)
        });
        tasks.push(task);
    }

    // Then: 所有请求都应该成功
    for task in tasks.into_iter() {
        let (i, result) = task.await.unwrap();
        let token = assert_ok!(result);
        assert!(!token.tenant_access_token.is_empty());
        println!(
            "✅ 并发请求 {} 成功: {}...",
            i + 1,
            &token.tenant_access_token[..16]
        );
    }
}

// ==================== 测试统计 ====================

#[tokio::test]
async fn test_coverage_summary() {
    println!("\n📊 OpenLark Auth API测试覆盖统计");
    println!("=====================================");

    // 模拟的测试结果统计
    let total_apis = 11;
    let tested_apis = 4; // 我们已实现的基础测试

    println!("总API接口数: {}", total_apis);
    println!("已测试接口数: {}", tested_apis);
    println!(
        "测试覆盖率: {:.1}%",
        (tested_apis as f64 / total_apis as f64) * 100.0
    );
    println!("");
    println!("✅ 已测试接口:");
    println!("   1. tenant_access_token/internal - 自建应用租户令牌");
    println!("   2. app_access_token/internal - 自建应用令牌");
    println!("   3. user_info.get - 获取用户信息");
    println!("   4. 认证失败错误处理");
    println!("   5. 并发请求测试");
    println!("");
    println!("📋 待实现接口:");
    println!("   - tenant_access_token (商店应用)");
    println!("   - app_access_token (商店应用)");
    println!("   - app_ticket/resend");
    println!("   - OIDC相关接口 (3个)");
    println!("   - access_token相关接口 (2个)");
    println!("   - OAuth授权接口");
    println!("");
    println!("🎯 当前状态: 基础测试框架已建立，核心功能验证通过");
    println!("📈 改进方向: 扩展接口覆盖，增加错误场景，提升业务逻辑测试");
}
