//! openlark-security 模块基础测试

use openlark_security::prelude::*;
use openlark_security::service::SecurityServiceManager;

#[tokio::test]
async fn test_security_service_manager_creation() {
    // 测试安全管理器创建
    let config = SecurityServiceConfig {
        app_id: "test_app_id".to_string(),
        app_secret: "test_app_secret".to_string(),
        base_url: "https://open.feishu.cn".to_string(),
        session_timeout: 3600,
        token_timeout: 7200,
        enable_mfa: false,
        enable_ip_restriction: false,
        allowed_ips: vec![],
    };

    let manager = SecurityServiceManager::new(config);

    // 测试健康检查
    let health = manager.health_check().await;
    assert!(health.is_ok());

    let status = health.unwrap();
    assert_eq!(status["service"], "security_manager");
    assert_eq!(status["status"], "healthy");
    assert!(status.contains_key("timestamp"));
    assert!(status.contains_key("version"));
}

#[tokio::test]
async fn test_security_service_manager_from_env() {
    // 设置测试环境变量
    std::env::set_var("APP_ID", "test_app_id");
    std::env::set_var("APP_SECRET", "test_app_secret");

    let manager = SecurityServiceManager::from_env();
    assert!(manager.is_ok());

    let manager = manager.unwrap();
    let config = manager.config();
    assert_eq!(config.app_id, "test_app_id");
    assert_eq!(config.app_secret, "test_app_secret");
    assert_eq!(config.base_url, "https://open.feishu.cn");

    // 清理环境变量
    std::env::remove_var("APP_ID");
    std::env::remove_var("APP_SECRET");
}

#[tokio::test]
#[cfg(feature = "auth")]
async fn test_authentication_service() {
    let auth_service = DefaultAuthService::new();

    let login_request = LoginRequest {
        username: "test_user".to_string(),
        password: "test_password".to_string(),
        captcha: None,
        login_type: Some(LoginType::Password),
        client_info: Some(ClientInfo {
            client_type: ClientType::Web,
            client_version: "1.0.0".to_string(),
            os: "test_os".to_string(),
            device_id: "test_device".to_string(),
            user_agent: "test_agent".to_string(),
        }),
        remember_me: Some(false),
    };

    let result = auth_service.login(login_request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(response.success);
    assert!(!response.access_token.is_empty());
    assert!(!response.refresh_token.is_empty());
    assert_eq!(response.token_type, TokenType::AccessToken);
}

#[tokio::test]
#[cfg(feature = "acs")]
async fn test_access_control_service() {
    let acs_service = DefaultAccessControlService::new();

    let permission_request = PermissionCheckRequest {
        user_id: "admin_user".to_string(),
        resource_type: "document".to_string(),
        resource_id: Some("doc_123".to_string()),
        action: "read".to_string(),
        context: None,
    };

    let result = acs_service.check_permission(permission_request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(response.allowed);
    assert!(response.reason.is_some());
}

#[tokio::test]
#[cfg(feature = "token")]
async fn test_token_service() {
    let token_service = DefaultTokenService::with_default_config();

    let create_request = CreateTokenRequest {
        user_id: "test_user".to_string(),
        app_id: Some("test_app".to_string()),
        scopes: vec!["user_info".to_string(), "message.send".to_string()],
        permissions: Some(vec!["read".to_string()]),
        expires_in: Some(3600),
        client_info: None,
    };

    let result = token_service.create_access_token(create_request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(!response.access_token.is_empty());
    assert!(response.refresh_token.is_some());
    assert_eq!(response.token_type, TokenType::AccessToken);
    assert_eq!(response.expires_in, 3600);
}

#[tokio::test]
#[cfg(feature = "audit")]
async fn test_audit_service() {
    let audit_service = DefaultAuditService::new();

    let audit_log = AuditLog {
        log_id: "test_log_001".to_string(),
        timestamp: chrono::Utc::now(),
        action: "test_action".to_string(),
        resource_type: "test_resource".to_string(),
        resource_id: Some("test_resource_001".to_string()),
        user_id: Some("test_user".to_string()),
        username: Some("test_user".to_string()),
        ip_address: Some("127.0.0.1".to_string()),
        result: ActionResult::Success,
        error_message: None,
        request_details: Some(serde_json::json!({
            "test": "data"
        })),
        response_details: Some(serde_json::json!({
            "status": "success"
        })),
    };

    let result = audit_service.log_audit_event(audit_log).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_service_statistics() {
    let config = SecurityServiceConfig {
        app_id: "test_app_id".to_string(),
        app_secret: "test_app_secret".to_string(),
        base_url: "https://open.feishu.cn".to_string(),
        session_timeout: 3600,
        token_timeout: 7200,
        enable_mfa: false,
        enable_ip_restriction: false,
        allowed_ips: vec![],
    };

    let manager = SecurityServiceManager::new(config);
    let stats = manager.get_statistics().await;

    assert!(stats.is_ok());
    let statistics = stats.unwrap();
    assert_eq!(statistics.uptime_seconds, 0);
    assert_eq!(statistics.total_requests, 0);
    assert_eq!(statistics.successful_authentications, 0);
    assert_eq!(statistics.failed_authentications, 0);
    assert_eq!(statistics.active_sessions, 0);
    assert_eq!(statistics.total_tokens_issued, 0);
    assert!(!statistics.services_enabled.is_empty());
}
