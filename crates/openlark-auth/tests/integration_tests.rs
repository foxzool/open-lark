//! openlark-auth 集成测试
//!
//! 测试整个认证服务的端到端功能

use openlark_auth::prelude::*;
use openlark_auth::AuthServices;

/// 创建测试配置
fn create_test_config() -> AuthConfig {
    AuthConfig::new("test_app_id", "test_app_secret").with_base_url("https://open.feishu.cn")
}

#[tokio::test]
async fn test_auth_services_creation() {
    let config = create_test_config();
    let auth_services = AuthServices::new(config);

    // 验证服务创建成功
    assert_eq!(auth_services.config.app_id, "test_app_id");
    assert_eq!(auth_services.config.app_secret, "test_app_secret");
    assert_eq!(auth_services.config.base_url, "https://open.feishu.cn");
}

#[tokio::test]
async fn test_auth_project_structure() {
    let config = create_test_config();
    let auth_services = AuthServices::new(config);

    // 测试 auth 项目结构
    let auth_v3 = auth_services.auth.v3();

    // 验证可以访问各个服务
    let _tenant_service = auth_v3.tenant_access_token();
    let _app_service = auth_v3.app_access_token();
    let _ticket_service = auth_v3.app_ticket();
}

#[tokio::test]
async fn test_authen_project_structure() {
    let config = create_test_config();
    let auth_services = AuthServices::new(config);

    // 测试 authen 项目结构
    let authen_v1 = auth_services.authen.v1;

    // 验证可以访问各个服务
    let _user_info_service = authen_v1.user_info();
    let _oidc_service = authen_v1.oidc();
    let _access_token_service = authen_v1.access_token();
}

#[tokio::test]
async fn test_oauth_project_structure() {
    let config = create_test_config();
    let auth_services = AuthServices::new(config);

    // 测试 oauth 项目结构
    let _authorization_service = auth_services.oauth.old.authorization();
}

#[tokio::test]
async fn test_builder_patterns() {
    let config = create_test_config();
    let auth_services = AuthServices::new(config);

    // 测试构建器模式不会 panic
    let _tenant_builder = auth_services.auth.v3().tenant_access_token().internal();
    let _app_builder = auth_services.auth.v3().app_access_token().internal();
    let _ticket_builder = auth_services.auth.v3().app_ticket().resend();

    let _user_info_builder = auth_services.authen.v1.user_info().get();
    let _oidc_builder = auth_services.authen.v1.oidc().create_refresh_access_token();
    let _access_token_builder = auth_services.authen.v1.access_token().create();

    let _oauth_builder = auth_services.oauth.old.authorization().get_index();
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_config_handling() {
        // 测试空配置
        let config = AuthConfig::new("", "");
        let auth_services = AuthServices::new(config);

        // 应该能够创建服务，即使配置无效
        assert_eq!(auth_services.config.app_id, "");
        assert_eq!(auth_services.config.app_secret, "");
    }

    #[tokio::test]
    async fn test_default_config() {
        let config = AuthConfig::default();
        let auth_services = AuthServices::new(config);

        // 验证默认配置
        assert_eq!(auth_services.config.app_id, "");
        assert_eq!(auth_services.config.app_secret, "");
        assert_eq!(auth_services.config.base_url, "https://open.feishu.cn");
    }
}

#[cfg(test)]
mod serialization_tests {
    use super::*;
    use openlark_auth::models::*;

    #[test]
    fn test_auth_config_creation() {
        let config = AuthConfig::new("test_app", "test_secret");

        assert_eq!(config.app_id, "test_app");
        assert_eq!(config.app_secret, "test_secret");
        assert_eq!(config.base_url, "https://open.feishu.cn");
    }

    #[test]
    fn test_user_status_serialization() {
        let status = UserStatus::Activated;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"activated\"");

        let deserialized: UserStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, UserStatus::Activated);
    }

    #[test]
    fn test_gender_serialization() {
        let gender = Gender::Male;
        let json = serde_json::to_string(&gender).unwrap();
        assert_eq!(json, "\"male\"");

        let deserialized: Gender = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Gender::Male);
    }
}

#[cfg(test)]
mod concurrent_tests {
    use super::*;
    use std::sync::Arc;
    use tokio::task::JoinSet;

    #[tokio::test]
    async fn test_concurrent_service_creation() {
        let config = create_test_config();
        let auth_services = Arc::new(AuthServices::new(config));

        let mut join_set = JoinSet::new();

        // 并发创建多个服务实例
        for i in 0..10 {
            let services = auth_services.clone();
            join_set.spawn(async move {
                // 模拟并发访问
                let _tenant = services.auth.v3().tenant_access_token();
                let _user_info = services.authen.v1.user_info();
                format!("Task {} completed", i)
            });
        }

        // 等待所有任务完成
        let mut results = Vec::new();
        while let Some(result) = join_set.join_next().await {
            results.push(result.unwrap());
        }

        // 验证所有任务都成功完成
        assert_eq!(results.len(), 10);
    }
}
