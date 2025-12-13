//! 基本功能测试
//!
//! 验证 openlark-auth 模块的核心功能

use openlark_auth::{AuthService, AuthenService, OAuthService};
use openlark_core::config::Config;

#[test]
fn test_service_creation() {
    // 测试配置创建
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url("https://open.feishu.cn")
        .build();

    // 测试服务创建
    let auth_service = AuthService::new(config.clone());
    let authen_service = AuthenService::new(config.clone());
    let oauth_service = OAuthService::new(config);

    // 验证服务创建成功
    assert!(!format!("{:?}", auth_service).is_empty());
    assert!(!format!("{:?}", authen_service).is_empty());
    assert!(!format!("{:?}", oauth_service).is_empty());
}

#[test]
fn test_api_builder_creation() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url("https://open.feishu.cn")
        .build();

    let auth_service = AuthService::new(config.clone());
    let authen_service = AuthenService::new(config.clone());

    // 测试 API 构建器创建
    let _app_token_builder = auth_service.v3().app_access_token();
    let _tenant_token_builder = auth_service.v3().tenant_access_token();
    let _user_info_builder = authen_service.v1().user_info();
    let _access_token_builder = authen_service.v1().access_token();

    // 如果到这里没有 panic，说明构建器创建成功
    assert!(true);
}

#[test]
fn test_models_serialization() {
    use openlark_auth::models::auth::*;
    use serde_json;

    // 测试 AccessTokenResponse 序列化
    let token_response = AccessTokenResponse {
        app_access_token: "test_token".to_string(),
        expires_in: 3600,
        tenant_key: "test_tenant".to_string(),
        token_type: Some("Bearer".to_string()),
    };

    let json_str = serde_json::to_string(&token_response);
    assert!(json_str.is_ok());
    assert!(json_str.unwrap().contains("test_token"));
}

#[test]
fn test_version_info() {
    // 测试版本信息导出
    let version = env!("CARGO_PKG_VERSION");
    assert!(!version.is_empty());
    assert!(version.contains('.'));
}

#[test]
fn test_debug_implementations() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url("https://open.feishu.cn")
        .build();

    let auth_service = AuthService::new(config.clone());
    let authen_service = AuthenService::new(config.clone());
    let oauth_service = OAuthService::new(config);

    // 测试 Debug trait 实现
    let _debug_auth = format!("{:?}", auth_service);
    let _debug_authen = format!("{:?}", authen_service);
    let _debug_oauth = format!("{:?}", oauth_service);

    // 如果能正常调用 Debug 格式化，说明实现正确
    assert!(true);
}
