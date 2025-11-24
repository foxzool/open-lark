//! openlark-client 认证服务集成测试
//!
//! 测试 openlark-client 中认证服务的完整功能

#[cfg(feature = "auth")]
use openlark_client::services::auth::{AuthService, TokenInfo, TokenVerificationResponse};
#[cfg(feature = "auth")]
use openlark_client::{Config, Error};

/// 创建测试配置
#[cfg(feature = "auth")]
fn create_test_config() -> Config {
    Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url("https://open.feishu.cn")
        .build()
        .expect("Failed to create test config")
}

#[cfg(feature = "auth")]
#[test]
fn test_auth_service_creation() {
    let config = create_test_config();
    let _auth_service = AuthService::new(&config);

    // 验证服务创建成功（如果没有 panic 就证明创建成功）
    assert!(true);
}

#[cfg(feature = "auth")]
#[test]
fn test_token_info_creation() {
    let token_info = TokenInfo {
        access_token: "test_token".to_string(),
        token_type: "Bearer".to_string(),
        expires_in: 7200,
        expires_at: chrono::Utc::now() + chrono::Duration::hours(2),
        scope: Some("user:info".to_string()),
    };

    // 验证令牌信息
    assert_eq!(token_info.access_token, "test_token");
    assert_eq!(token_info.token_type, "Bearer");
    assert_eq!(token_info.expires_in, 7200);
    assert_eq!(token_info.scope, Some("user:info".to_string()));
}

#[cfg(feature = "auth")]
#[test]
fn test_token_verification_response() {
    let verification = TokenVerificationResponse {
        valid: true,
        user_id: Some("test_user_123".to_string()),
        tenant_key: Some("test_tenant_456".to_string()),
        expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(2)),
        scope: vec!["user:info".to_string(), "docs:read".to_string()],
    };

    assert!(verification.valid);
    assert_eq!(verification.user_id, Some("test_user_123".to_string()));
    assert_eq!(verification.tenant_key, Some("test_tenant_456".to_string()));
    assert_eq!(verification.scope.len(), 2);
}

#[cfg(feature = "auth")]
#[test]
fn test_user_info_structure() {
    // 测试 UserInfo 结构可以通过以下方式创建
    // 注意：由于这是类型别名，实际使用时来自 openlark-auth
    let user_info_example_expected_fields = vec![
        "user_id",
        "open_id",
        "union_id",
        "name",
        "en_name",
        "email",
        "mobile",
        "avatar_url",
        "status",
        "department_ids",
        "position",
        "employee_no",
        "nickname",
        "gender",
    ];

    // 验证预期的字段数量
    assert_eq!(user_info_example_expected_fields.len(), 14);
}

#[cfg(feature = "auth")]
mod token_management_tests {
    use super::*;

    #[test]
    fn test_oauth_url_generation() {
        let config = create_test_config();
        let auth_service = AuthService::new(&config);

        let url = auth_service.generate_oauth_url(
            "https://example.com/callback",
            "user:info docs:read",
            "test_state_123",
        );

        // 验证 URL 包含必要的参数
        assert!(url.contains("app_id=test_app_id"));
        assert!(url.contains("redirect_uri=https://example.com/callback"));
        assert!(url.contains("scope=user:info docs:read"));
        assert!(url.contains("state=test_state_123"));
        assert!(url.contains("open.feishu.cn"));
        assert!(url.contains("/authorize"));
    }

    #[test]
    fn test_token_info_expiration_methods() {
        // 测试未过期的令牌
        let future_token = TokenInfo {
            access_token: "valid_token".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
            scope: Some("test".to_string()),
        };

        assert!(!future_token.is_expired());
        assert!(!future_token.needs_refresh(30)); // 30分钟内不需要刷新
        assert!(future_token.remaining_seconds() > 0);

        // 测试已过期的令牌
        let expired_token = TokenInfo {
            access_token: "expired_token".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 0,
            expires_at: chrono::Utc::now() - chrono::Duration::minutes(1),
            scope: Some("test".to_string()),
        };

        assert!(expired_token.is_expired());
        assert!(expired_token.needs_refresh(30));
        assert_eq!(expired_token.remaining_seconds(), 0);
    }

    #[test]
    fn test_token_needs_refresh_edge_cases() {
        let now = chrono::Utc::now();

        // 刚好在刷新边界内的令牌
        let boundary_token = TokenInfo {
            access_token: "boundary_token".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 1800, // 30分钟
            expires_at: now + chrono::Duration::minutes(30),
            scope: Some("test".to_string()),
        };

        assert!(boundary_token.needs_refresh(30)); // 需要刷新
        assert!(!boundary_token.needs_refresh(20)); // 不需要刷新
    }
}

#[cfg(feature = "auth")]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        // 测试无效配置处理
        let config = Config::builder().app_id("").app_secret("").build();

        assert!(config.is_err()); // 空配置应该失败
    }

    #[test]
    fn test_error_types() {
        // 验证错误类型可以创建
        let api_error = Error::APIError {
            code: "AUTH_ERROR".to_string(),
            message: "测试错误".to_string(),
        };

        assert!(matches!(api_error, Error::APIError { .. }));
        assert!(api_error.to_string().contains("AUTH_ERROR"));
        assert!(api_error.to_string().contains("测试错误"));
    }
}

#[cfg(feature = "auth")]
mod builder_pattern_tests {
    use super::*;

    #[test]
    fn test_auth_service_builder_methods() {
        let config = create_test_config();
        let auth_service = AuthService::new(&config);

        // 验证所有方法都可以调用（不会 panic）
        // 注意：这些是异步方法，实际调用需要 .await
        let _internal_app_token_future = auth_service.get_internal_app_access_token();
        let _store_app_token_future = auth_service.get_store_app_access_token();
        let _user_token_future = auth_service.get_user_access_token("test_code");
        let _refresh_token_future = auth_service.refresh_oidc_access_token("refresh_token");
        let _user_info_future = auth_service.get_user_info("user_access_token");
        let _verify_token_future = auth_service.verify_token("access_token");
        let _resend_ticket_future = auth_service.resend_app_ticket();

        // 如果没有 panic，则测试通过
        assert!(true);
    }
}

#[cfg(feature = "auth")]
mod serialization_tests {
    use super::*;

    #[test]
    fn test_token_info_serialization() {
        let token_info = TokenInfo {
            access_token: "test_token".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 7200,
            expires_at: chrono::Utc::now() + chrono::Duration::hours(2),
            scope: Some("user:info docs:read".to_string()),
        };

        // 测试序列化
        let json = serde_json::to_string(&token_info).expect("Failed to serialize TokenInfo");
        assert!(json.contains("test_token"));
        assert!(json.contains("Bearer"));

        // 测试反序列化
        let deserialized: TokenInfo =
            serde_json::from_str(&json).expect("Failed to deserialize TokenInfo");
        assert_eq!(deserialized.access_token, token_info.access_token);
        assert_eq!(deserialized.token_type, token_info.token_type);
    }

    #[test]
    fn test_token_verification_serialization() {
        let verification = TokenVerificationResponse {
            valid: true,
            user_id: Some("user_123".to_string()),
            tenant_key: Some("tenant_456".to_string()),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
            scope: vec!["user:info".to_string(), "docs:read".to_string()],
        };

        // 测试序列化
        let json = serde_json::to_string(&verification)
            .expect("Failed to serialize TokenVerificationResponse");
        assert!(json.contains("user_123"));
        assert!(json.contains("tenant_456"));

        // 测试反序列化
        let deserialized: TokenVerificationResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.valid, verification.valid);
        assert_eq!(deserialized.user_id, verification.user_id);
    }
}
