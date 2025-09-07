//! Authentication 服务测试
//!
//! 测试 Authentication 服务的核心功能，包括：
//! - 服务创建和初始化
//! - V1 版本服务集成
//! - 配置传递和管理
//! - 服务生命周期

use open_lark::{
    core::{config::Config, constants::AppType},
    service::authentication::AuthenService,
};
use rstest::rstest;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::Mutex;

/// 创建测试配置
fn create_test_config(app_type: AppType) -> Config {
    Config {
        app_id: "test_auth_app_id".to_string(),
        app_secret: "test_auth_app_secret".to_string(),
        app_type,
        base_url: "https://open.feishu.cn".to_string(),
        http_client: reqwest::Client::new(),
        enable_token_cache: true,
        req_timeout: Some(Duration::from_secs(30)),
        header: HashMap::new(),
        token_manager: Arc::new(Mutex::new(
            open_lark::core::token_manager::TokenManager::new(),
        )),
        app_ticket_manager: Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        )),
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_authentication_service_creation() {
        let config = create_test_config(AppType::SelfBuild);
        let auth_service = AuthenService::new(Arc::new(config));

        // 验证服务创建成功
        // 注意：由于 AuthenService 的字段是公共的，我们可以直接访问
        assert_eq!(auth_service.v1.user_info.config.app_id, "test_auth_app_id");
        assert_eq!(
            auth_service.v1.user_info.config.app_secret,
            "test_auth_app_secret"
        );
        assert_eq!(auth_service.v1.user_info.config.app_type, AppType::SelfBuild);
    }

    #[rstest]
    #[case(AppType::SelfBuild)]
    #[case(AppType::Marketplace)]
    fn test_authentication_service_with_different_app_types(#[case] app_type: AppType) {
        let config = create_test_config(app_type);
        let auth_service = AuthenService::new(Arc::new(config));

        // 验证不同类型的应用都能正确创建认证服务
        assert_eq!(auth_service.v1.user_info.config.app_type, app_type);
    }

    #[test]
    fn test_authentication_service_config_propagation() {
        let mut config = create_test_config(AppType::SelfBuild);
        config.base_url = "https://custom.feishu.cn".to_string();
        config.enable_token_cache = false;
        config.req_timeout = Some(Duration::from_secs(60));

        let auth_service = AuthenService::new(Arc::new(config));

        // 验证配置正确传播到子服务
        assert_eq!(
            auth_service.v1.user_info.config.base_url,
            "https://custom.feishu.cn"
        );
        assert!(!auth_service.v1.user_info.config.enable_token_cache);
        assert_eq!(
            auth_service.v1.user_info.config.req_timeout,
            Some(Duration::from_secs(60))
        );
    }

    #[test]
    fn test_authentication_service_with_custom_headers() {
        let mut config = create_test_config(AppType::SelfBuild);
        config
            .header
            .insert("X-Custom-Header".to_string(), "custom-value".to_string());
        config.header.insert(
            "X-Client-Version".to_string(),
            "open-lark-test-1.0".to_string(),
        );

        let auth_service = AuthenService::new(Arc::new(config));

        // 验证自定义头部正确传播
        assert_eq!(
            auth_service
                .v1
                .user_info
                .config
                .header
                .get("X-Custom-Header"),
            Some(&"custom-value".to_string())
        );
        assert_eq!(
            auth_service
                .v1
                .user_info
                .config
                .header
                .get("X-Client-Version"),
            Some(&"open-lark-test-1.0".to_string())
        );
    }

    #[test]
    fn test_authentication_service_v1_structure() {
        let config = create_test_config(AppType::SelfBuild);
        let auth_service = AuthenService::new(Arc::new(config));

        // 验证 V1 服务结构
        // 目前只有 user_info 服务，未来可能会添加更多服务
        let _user_info_service = &auth_service.v1.user_info;

        // 验证 user_info 服务可用
        assert_eq!(
            auth_service.v1.user_info.config.app_id,
            "test_auth_app_id"
        );
    }

    #[test]
    fn test_multiple_authentication_service_instances() {
        let config1 = create_test_config(AppType::SelfBuild);
        let config2 = create_test_config(AppType::Marketplace);

        let auth_service1 = AuthenService::new(Arc::new(config1));
        let auth_service2 = AuthenService::new(Arc::new(config2));

        // 验证可以创建多个独立的服务实例
        assert_eq!(auth_service1.v1.user_info.config.app_type, AppType::SelfBuild);
        assert_eq!(
            auth_service2.v1.user_info.config.app_type,
            AppType::Marketplace
        );

        // 验证两个实例是独立的
        assert_ne!(
            auth_service1.v1.user_info.config.app_type,
            auth_service2.v1.user_info.config.app_type
        );
    }

    #[test]
    fn test_authentication_service_with_shared_config() {
        let config = Arc::new(create_test_config(AppType::SelfBuild));

        // 使用相同的配置创建多个服务
        let auth_service1 = AuthenService::new(config.clone());
        let auth_service2 = AuthenService::new(config.clone());

        // 验证配置共享
        assert_eq!(
            auth_service1.v1.user_info.config.app_id,
            auth_service2.v1.user_info.config.app_id
        );
        assert_eq!(
            auth_service1.v1.user_info.config.app_secret,
            auth_service2.v1.user_info.config.app_secret
        );
    }
}

#[cfg(test)]
mod configuration_tests {
    use super::*;

    #[test]
    fn test_authentication_service_with_minimal_config() {
        let config = Config {
            app_id: "minimal_app".to_string(),
            app_secret: "minimal_secret".to_string(),
            app_type: AppType::SelfBuild,
            base_url: "https://open.feishu.cn".to_string(),
            http_client: reqwest::Client::new(),
            enable_token_cache: false,
            req_timeout: None,
            header: HashMap::new(),
            token_manager: Arc::new(Mutex::new(
                open_lark::core::token_manager::TokenManager::new(),
            )),
            app_ticket_manager: Arc::new(Mutex::new(
                open_lark::core::app_ticket_manager::AppTicketManager::new(),
            )),
        };

        let auth_service = AuthenService::new(Arc::new(config));

        // 验证最小配置能够正常工作
        assert_eq!(auth_service.v1.user_info.config.app_id, "minimal_app");
        assert_eq!(
            auth_service.v1.user_info.config.app_secret,
            "minimal_secret"
        );
        assert!(!auth_service.v1.user_info.config.enable_token_cache);
        assert!(auth_service.v1.user_info.config.req_timeout.is_none());
        assert!(auth_service.v1.user_info.config.header.is_empty());
    }

    #[test]
    fn test_authentication_service_with_comprehensive_config() {
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), "open-lark-test".to_string());
        headers.insert("Accept".to_string(), "application/json".to_string());
        headers.insert(
            "X-Request-ID".to_string(),
            "test-request-12345".to_string(),
        );

        let config = Config {
            app_id: "comprehensive_app_id_12345".to_string(),
            app_secret: "comprehensive_app_secret_67890".to_string(),
            app_type: AppType::Marketplace,
            base_url: "https://custom.feishu-api.cn".to_string(),
            http_client: reqwest::Client::new(),
            enable_token_cache: true,
            req_timeout: Some(Duration::from_secs(120)),
            header: headers,
            token_manager: Arc::new(Mutex::new(
                open_lark::core::token_manager::TokenManager::new(),
            )),
            app_ticket_manager: Arc::new(Mutex::new(
                open_lark::core::app_ticket_manager::AppTicketManager::new(),
            )),
        };

        let auth_service = AuthenService::new(Arc::new(config));

        // 验证全面配置正确传播
        assert_eq!(
            auth_service.v1.user_info.config.app_id,
            "comprehensive_app_id_12345"
        );
        assert_eq!(
            auth_service.v1.user_info.config.app_secret,
            "comprehensive_app_secret_67890"
        );
        assert_eq!(
            auth_service.v1.user_info.config.app_type,
            AppType::Marketplace
        );
        assert_eq!(
            auth_service.v1.user_info.config.base_url,
            "https://custom.feishu-api.cn"
        );
        assert!(auth_service.v1.user_info.config.enable_token_cache);
        assert_eq!(
            auth_service.v1.user_info.config.req_timeout,
            Some(Duration::from_secs(120))
        );
        assert_eq!(auth_service.v1.user_info.config.header.len(), 3);
        assert_eq!(
            auth_service
                .v1
                .user_info
                .config
                .header
                .get("User-Agent"),
            Some(&"open-lark-test".to_string())
        );
        assert_eq!(
            auth_service.v1.user_info.config.header.get("Accept"),
            Some(&"application/json".to_string())
        );
    }

    #[rstest]
    #[case("")]
    #[case("test_app")]
    #[case("app_id_with_numbers_123")]
    #[case("app-id-with-dashes")]
    #[case("app_id_with_underscores")]
    #[case("很长的应用ID名称用于测试极端情况")]
    fn test_authentication_service_with_various_app_ids(#[case] app_id: &str) {
        let mut config = create_test_config(AppType::SelfBuild);
        config.app_id = app_id.to_string();

        let auth_service = AuthenService::new(Arc::new(config));

        // 验证各种 app_id 都能正确处理
        assert_eq!(auth_service.v1.user_info.config.app_id, app_id);
    }

    #[rstest]
    #[case("")]
    #[case("secret")]
    #[case("very_long_secret_key_for_testing_purposes_123456789")]
    #[case("secret-with-special-chars-!@#$%")]
    fn test_authentication_service_with_various_app_secrets(#[case] app_secret: &str) {
        let mut config = create_test_config(AppType::SelfBuild);
        config.app_secret = app_secret.to_string();

        let auth_service = AuthenService::new(Arc::new(config));

        // 验证各种 app_secret 都能正确处理
        assert_eq!(auth_service.v1.user_info.config.app_secret, app_secret);
    }

    #[rstest]
    #[case("https://open.feishu.cn")]
    #[case("https://open.larksuite.com")]
    #[case("https://custom-api.example.com")]
    #[case("http://localhost:8080")]
    #[case("https://test-env.feishu-api.internal")]
    fn test_authentication_service_with_various_base_urls(#[case] base_url: &str) {
        let mut config = create_test_config(AppType::SelfBuild);
        config.base_url = base_url.to_string();

        let auth_service = AuthenService::new(Arc::new(config));

        // 验证各种 base_url 都能正确处理
        assert_eq!(auth_service.v1.user_info.config.base_url, base_url);
    }

    #[rstest]
    #[case(None)]
    #[case(Some(Duration::from_secs(1)))]
    #[case(Some(Duration::from_secs(30)))]
    #[case(Some(Duration::from_secs(300)))]
    #[case(Some(Duration::from_millis(500)))]
    fn test_authentication_service_with_various_timeouts(
        #[case] timeout: Option<Duration>,
    ) {
        let mut config = create_test_config(AppType::SelfBuild);
        config.req_timeout = timeout;

        let auth_service = AuthenService::new(Arc::new(config));

        // 验证各种超时设置都能正确处理
        assert_eq!(auth_service.v1.user_info.config.req_timeout, timeout);
    }

    #[rstest]
    #[case(true)]
    #[case(false)]
    fn test_authentication_service_with_token_cache_options(#[case] enable_cache: bool) {
        let mut config = create_test_config(AppType::SelfBuild);
        config.enable_token_cache = enable_cache;

        let auth_service = AuthenService::new(Arc::new(config));

        // 验证令牌缓存设置正确传播
        assert_eq!(
            auth_service.v1.user_info.config.enable_token_cache,
            enable_cache
        );
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use tokio::time::Instant;

    #[tokio::test]
    async fn test_authentication_service_initialization_performance() {
        let start_time = Instant::now();

        let config = create_test_config(AppType::SelfBuild);
        let _auth_service = AuthenService::new(Arc::new(config));

        let elapsed = start_time.elapsed();

        // 验证服务初始化性能合理（应该很快）
        assert!(elapsed < Duration::from_millis(100));
        println!("AuthenService 初始化时间: {:?}", elapsed);
    }

    #[tokio::test]
    async fn test_multiple_authentication_services_performance() {
        let start_time = Instant::now();
        let mut services = Vec::new();

        // 创建多个认证服务实例
        for i in 0..100 {
            let mut config = create_test_config(AppType::SelfBuild);
            config.app_id = format!("test_app_{}", i);
            services.push(AuthenService::new(Arc::new(config)));
        }

        let elapsed = start_time.elapsed();

        // 验证批量创建性能合理
        assert!(elapsed < Duration::from_secs(1));
        assert_eq!(services.len(), 100);

        println!("创建 100 个 AuthenService 实例时间: {:?}", elapsed);
    }

    #[test]
    fn test_authentication_service_memory_efficiency() {
        // 创建多个服务实例并检查内存使用（简单测试）
        let mut services = Vec::new();

        for i in 0..1000 {
            let mut config = create_test_config(AppType::SelfBuild);
            config.app_id = format!("memory_test_{}", i);
            services.push(AuthenService::new(Arc::new(config)));
        }

        // 验证能够创建大量实例而不出错
        assert_eq!(services.len(), 1000);

        // 验证第一个和最后一个服务的配置正确
        assert_eq!(services[0].v1.user_info.config.app_id, "memory_test_0");
        assert_eq!(services[999].v1.user_info.config.app_id, "memory_test_999");
    }

    #[tokio::test]
    async fn test_authentication_service_with_real_http_client_config() {
        // 测试使用真实 HTTP 客户端配置的认证服务
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("open-lark-test/1.0")
            .build()
            .unwrap();

        let config = Config {
            app_id: "real_http_test".to_string(),
            app_secret: "real_http_secret".to_string(),
            app_type: AppType::SelfBuild,
            base_url: "https://open.feishu.cn".to_string(),
            http_client,
            enable_token_cache: true,
            req_timeout: Some(Duration::from_secs(30)),
            header: HashMap::new(),
            token_manager: Arc::new(Mutex::new(
                open_lark::core::token_manager::TokenManager::new(),
            )),
            app_ticket_manager: Arc::new(Mutex::new(
                open_lark::core::app_ticket_manager::AppTicketManager::new(),
            )),
        };

        let auth_service = AuthenService::new(Arc::new(config));

        // 验证服务创建成功
        assert_eq!(auth_service.v1.user_info.config.app_id, "real_http_test");
        assert_eq!(
            auth_service.v1.user_info.config.req_timeout,
            Some(Duration::from_secs(30))
        );
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_authentication_service_with_extreme_values() {
        let mut config = create_test_config(AppType::SelfBuild);

        // 设置极端值
        config.app_id = "a".repeat(1000); // 极长的 app_id
        config.app_secret = "s".repeat(1000); // 极长的 app_secret
        config.base_url = format!("https://{}.com", "x".repeat(100)); // 极长的 URL
        config.req_timeout = Some(Duration::from_secs(3600)); // 1小时超时

        let auth_service = AuthenService::new(Arc::new(config));

        // 验证极端值不会导致创建失败
        assert_eq!(auth_service.v1.user_info.config.app_id.len(), 1000);
        assert_eq!(auth_service.v1.user_info.config.app_secret.len(), 1000);
        assert_eq!(
            auth_service.v1.user_info.config.req_timeout,
            Some(Duration::from_secs(3600))
        );
    }

    #[test]
    fn test_authentication_service_with_special_characters() {
        let mut config = create_test_config(AppType::SelfBuild);

        // 设置包含特殊字符的值
        config.app_id = "app🚀测试_id-123".to_string();
        config.app_secret = "secret!@#$%^&*()测试密钥".to_string();
        config.base_url = "https://测试.feishu.cn".to_string();

        let auth_service = AuthenService::new(Arc::new(config));

        // 验证特殊字符正确处理
        assert_eq!(auth_service.v1.user_info.config.app_id, "app🚀测试_id-123");
        assert_eq!(
            auth_service.v1.user_info.config.app_secret,
            "secret!@#$%^&*()测试密钥"
        );
        assert_eq!(
            auth_service.v1.user_info.config.base_url,
            "https://测试.feishu.cn"
        );
    }

    #[test]
    fn test_authentication_service_with_empty_strings() {
        let config = Config {
            app_id: String::new(),
            app_secret: String::new(),
            app_type: AppType::SelfBuild,
            base_url: String::new(),
            http_client: reqwest::Client::new(),
            enable_token_cache: false,
            req_timeout: None,
            header: HashMap::new(),
            token_manager: Arc::new(Mutex::new(
                open_lark::core::token_manager::TokenManager::new(),
            )),
            app_ticket_manager: Arc::new(Mutex::new(
                open_lark::core::app_ticket_manager::AppTicketManager::new(),
            )),
        };

        let auth_service = AuthenService::new(Arc::new(config));

        // 验证空字符串不会导致创建失败
        assert!(auth_service.v1.user_info.config.app_id.is_empty());
        assert!(auth_service.v1.user_info.config.app_secret.is_empty());
        assert!(auth_service.v1.user_info.config.base_url.is_empty());
    }

    #[test]
    fn test_authentication_service_with_many_headers() {
        let mut config = create_test_config(AppType::SelfBuild);

        // 添加大量头部
        for i in 0..100 {
            config.header.insert(
                format!("X-Custom-Header-{}", i),
                format!("value-{}", i),
            );
        }

        let auth_service = AuthenService::new(Arc::new(config));

        // 验证大量头部正确传播
        assert_eq!(auth_service.v1.user_info.config.header.len(), 100);
        assert_eq!(
            auth_service
                .v1
                .user_info
                .config
                .header
                .get("X-Custom-Header-0"),
            Some(&"value-0".to_string())
        );
        assert_eq!(
            auth_service
                .v1
                .user_info
                .config
                .header
                .get("X-Custom-Header-99"),
            Some(&"value-99".to_string())
        );
    }
}