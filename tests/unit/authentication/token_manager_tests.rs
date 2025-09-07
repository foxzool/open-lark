//! TokenManager 测试
//!
//! 测试令牌管理器的核心功能，包括：
//! - 令牌缓存和刷新机制
//! - 并发访问安全性
//! - 性能指标统计
//! - 预热机制
//! - 错误处理和恢复

use open_lark::core::{
    config::Config,
    constants::AppType,
    token_manager::{PreheatingConfig, TokenManager, TokenMetrics},
};
use proptest::prelude::*;
use rstest::rstest;
use serde_json::json;
use std::{
    collections::HashMap,
    sync::{
        atomic::Ordering,
        Arc,
    },
    time::Duration,
};
use tokio::{
    sync::Mutex,
    time::{sleep, Instant},
};
use wiremock::{
    matchers::{header, method, path},
    Mock, MockServer, ResponseTemplate,
};

/// 创建测试配置
fn create_test_config(base_url: String, app_type: AppType) -> Config {
    Config {
        app_id: "test_app_id".to_string(),
        app_secret: "test_app_secret".to_string(),
        app_type,
        base_url,
        http_client: reqwest::Client::new(),
        enable_token_cache: true,
        req_timeout: Some(Duration::from_secs(10)),
        header: HashMap::new(),
        token_manager: Arc::new(Mutex::new(TokenManager::new())),
        app_ticket_manager: Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        )),
    }
}

/// 创建成功的 App Access Token 响应
fn create_app_token_response(token: &str, expire: i32) -> serde_json::Value {
    json!({
        "code": 0,
        "msg": "success",
        "app_access_token": token,
        "expire": expire
    })
}

/// 创建成功的 Tenant Access Token 响应
fn create_tenant_token_response(token: &str, expire: i32) -> serde_json::Value {
    json!({
        "code": 0,
        "msg": "success",  
        "tenant_access_token": token,
        "expire": expire
    })
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_token_manager_creation() {
        let manager = TokenManager::new();
        // 验证 TokenManager 创建不会 panic
        let metrics = manager.metrics();
        assert_eq!(metrics.app_cache_hit_rate(), 0.0);
        assert_eq!(metrics.tenant_cache_hit_rate(), 0.0);
        assert_eq!(metrics.refresh_success_rate(), 0.0);
    }

    #[test]
    fn test_token_metrics_initial_state() {
        let metrics = TokenMetrics::new();
        
        // 验证初始状态
        assert_eq!(metrics.app_cache_hits.load(Ordering::Relaxed), 0);
        assert_eq!(metrics.app_cache_misses.load(Ordering::Relaxed), 0);
        assert_eq!(metrics.tenant_cache_hits.load(Ordering::Relaxed), 0);
        assert_eq!(metrics.tenant_cache_misses.load(Ordering::Relaxed), 0);
        assert_eq!(metrics.refresh_success.load(Ordering::Relaxed), 0);
        assert_eq!(metrics.refresh_failures.load(Ordering::Relaxed), 0);
        assert_eq!(metrics.read_lock_acquisitions.load(Ordering::Relaxed), 0);
        assert_eq!(metrics.write_lock_acquisitions.load(Ordering::Relaxed), 0);
    }

    #[rstest]
    #[case(0, 0, 0.0)]
    #[case(10, 0, 1.0)]
    #[case(0, 10, 0.0)]
    #[case(8, 2, 0.8)]
    #[case(7, 3, 0.7)]
    #[case(1, 9, 0.1)]
    fn test_app_cache_hit_rate_calculation(
        #[case] hits: u64,
        #[case] misses: u64,
        #[case] expected_rate: f64,
    ) {
        let metrics = TokenMetrics::new();
        metrics.app_cache_hits.store(hits, Ordering::Relaxed);
        metrics.app_cache_misses.store(misses, Ordering::Relaxed);
        
        let rate = metrics.app_cache_hit_rate();
        assert!((rate - expected_rate).abs() < f64::EPSILON);
    }

    #[rstest]
    #[case(0, 0, 0.0)]
    #[case(10, 0, 1.0)]
    #[case(0, 10, 0.0)]
    #[case(9, 1, 0.9)]
    #[case(5, 5, 0.5)]
    fn test_tenant_cache_hit_rate_calculation(
        #[case] hits: u64,
        #[case] misses: u64,
        #[case] expected_rate: f64,
    ) {
        let metrics = TokenMetrics::new();
        metrics.tenant_cache_hits.store(hits, Ordering::Relaxed);
        metrics.tenant_cache_misses.store(misses, Ordering::Relaxed);
        
        let rate = metrics.tenant_cache_hit_rate();
        assert!((rate - expected_rate).abs() < f64::EPSILON);
    }

    #[rstest]
    #[case(0, 0, 0.0)]
    #[case(10, 0, 1.0)]
    #[case(0, 10, 0.0)]
    #[case(95, 5, 0.95)]
    #[case(80, 20, 0.8)]
    fn test_refresh_success_rate_calculation(
        #[case] success: u64,
        #[case] failures: u64,
        #[case] expected_rate: f64,
    ) {
        let metrics = TokenMetrics::new();
        metrics.refresh_success.store(success, Ordering::Relaxed);
        metrics.refresh_failures.store(failures, Ordering::Relaxed);
        
        let rate = metrics.refresh_success_rate();
        assert!((rate - expected_rate).abs() < f64::EPSILON);
    }

    #[test]
    fn test_token_metrics_performance_report() {
        let metrics = TokenMetrics::new();
        
        // 设置测试数据
        metrics.app_cache_hits.store(80, Ordering::Relaxed);
        metrics.app_cache_misses.store(20, Ordering::Relaxed);
        metrics.tenant_cache_hits.store(90, Ordering::Relaxed);
        metrics.tenant_cache_misses.store(10, Ordering::Relaxed);
        metrics.refresh_success.store(95, Ordering::Relaxed);
        metrics.refresh_failures.store(5, Ordering::Relaxed);
        metrics.read_lock_acquisitions.store(150, Ordering::Relaxed);
        metrics.write_lock_acquisitions.store(25, Ordering::Relaxed);
        
        let report = metrics.performance_report();
        
        // 验证报告包含预期内容
        assert!(report.contains("80.00%")); // App cache hit rate
        assert!(report.contains("90.00%")); // Tenant cache hit rate  
        assert!(report.contains("95.00%")); // Refresh success rate
        assert!(report.contains("150")); // Read locks
        assert!(report.contains("25")); // Write locks
        assert!(report.contains("80 hits, 20 misses")); // App cache details
        assert!(report.contains("90 hits, 10 misses")); // Tenant cache details
        assert!(report.contains("95 success, 5 failures")); // Refresh details
    }

    #[test]
    fn test_preheating_config_default() {
        let config = PreheatingConfig::default();
        assert_eq!(config.check_interval_seconds, 1800); // 30分钟
        assert_eq!(config.preheat_threshold_seconds, 900); // 15分钟
        assert!(config.enable_tenant_preheating);
        assert_eq!(config.max_concurrent_preheat, 3);
    }

    #[test]
    fn test_preheating_config_custom() {
        let config = PreheatingConfig {
            check_interval_seconds: 600,
            preheat_threshold_seconds: 300,
            enable_tenant_preheating: false,
            max_concurrent_preheat: 5,
        };
        
        assert_eq!(config.check_interval_seconds, 600);
        assert_eq!(config.preheat_threshold_seconds, 300);
        assert!(!config.enable_tenant_preheating);
        assert_eq!(config.max_concurrent_preheat, 5);
    }

    #[test]
    fn test_token_manager_is_preheating_active_initially_false() {
        let manager = TokenManager::new();
        assert!(!manager.is_preheating_active());
    }
}

#[cfg(test)]
mod cache_tests {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    async fn test_cache_operations() {
        let manager = TokenManager::new();
        let cache = manager.get_cache();
        
        // 测试设置和获取缓存
        {
            let mut cache_write = cache.write().await;
            cache_write.set("test_key", "test_value".to_string(), 3600);
        }
        
        {
            let cache_read = cache.read().await;
            let value = cache_read.get("test_key");
            assert_eq!(value, Some("test_value".to_string()));
        }
    }

    #[tokio::test]
    async fn test_cache_expiry() {
        let manager = TokenManager::new();
        let cache = manager.get_cache();
        
        // 设置一个很短的过期时间
        {
            let mut cache_write = cache.write().await;
            cache_write.set("short_lived_key", "value".to_string(), 1);
        }
        
        // 等待过期
        sleep(Duration::from_secs(2)).await;
        
        {
            let cache_read = cache.read().await;
            let value = cache_read.get("short_lived_key");
            // 应该返回 None 或空字符串（取决于实现）
            assert!(value.is_none() || value == Some(String::new()));
        }
    }

    #[tokio::test]
    async fn test_cache_get_with_expiry() {
        let manager = TokenManager::new();
        let cache = manager.get_cache();
        
        {
            let mut cache_write = cache.write().await;
            cache_write.set("expiry_test_key", "expiry_value".to_string(), 3600);
        }
        
        {
            let cache_read = cache.read().await;
            if let Some(expiry_info) = cache_read.get_with_expiry("expiry_test_key") {
                assert!(expiry_info.expiry_seconds() > 0);
                assert!(expiry_info.expiry_seconds() <= 3600);
            }
        }
    }
}

#[cfg(test)]
mod app_access_token_tests {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_get_app_access_token_self_build_success() {
        let mock_server = MockServer::start().await;
        let config = create_test_config(mock_server.uri(), AppType::SelfBuild);
        let manager = TokenManager::new();
        let app_ticket_manager = Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        ));

        let expected_token = "t-test_app_access_token_12345";
        let response = create_app_token_response(expected_token, 7200);

        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/app_access_token/internal"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&mock_server)
            .await;

        let result = manager
            .get_app_access_token(&config, "", &app_ticket_manager)
            .await;
        
        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token, expected_token);

        // 验证指标更新
        let metrics = manager.metrics();
        assert_eq!(metrics.refresh_success.load(Ordering::Relaxed), 1);
        assert!(metrics.app_cache_misses.load(Ordering::Relaxed) > 0);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_app_access_token_cache_hit() {
        let mock_server = MockServer::start().await;
        let config = create_test_config(mock_server.uri(), AppType::SelfBuild);
        let manager = TokenManager::new();
        let app_ticket_manager = Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        ));

        let expected_token = "t-cached_app_access_token_67890";
        let response = create_app_token_response(expected_token, 7200);

        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/app_access_token/internal"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&mock_server)
            .await;

        // 第一次调用 - 缓存未命中
        let result1 = manager
            .get_app_access_token(&config, "", &app_ticket_manager)
            .await;
        assert!(result1.is_ok());

        let initial_cache_misses = manager.metrics().app_cache_misses.load(Ordering::Relaxed);

        // 第二次调用 - 应该从缓存获取
        let result2 = manager
            .get_app_access_token(&config, "", &app_ticket_manager)
            .await;
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), expected_token);

        // 验证缓存命中
        let metrics = manager.metrics();
        assert!(metrics.app_cache_hits.load(Ordering::Relaxed) > 0);
        assert_eq!(metrics.app_cache_misses.load(Ordering::Relaxed), initial_cache_misses);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_app_access_token_api_error() {
        let mock_server = MockServer::start().await;
        let config = create_test_config(mock_server.uri(), AppType::SelfBuild);
        let manager = TokenManager::new();
        let app_ticket_manager = Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        ));

        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/app_access_token/internal"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 10013,
                "msg": "invalid app_id or app_secret"
            })))
            .mount(&mock_server)
            .await;

        let result = manager
            .get_app_access_token(&config, "", &app_ticket_manager)
            .await;
        
        assert!(result.is_err());

        // 验证失败指标更新
        let metrics = manager.metrics();
        assert!(metrics.refresh_failures.load(Ordering::Relaxed) > 0);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_app_access_token_marketplace_success() {
        let mock_server = MockServer::start().await;
        let config = create_test_config(mock_server.uri(), AppType::Marketplace);
        let manager = TokenManager::new();
        let app_ticket_manager = Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        ));

        let expected_token = "t-marketplace_app_token_abc123";
        let response = create_app_token_response(expected_token, 7200);

        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/app_access_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&mock_server)
            .await;

        let result = manager
            .get_app_access_token(&config, "test_app_ticket", &app_ticket_manager)
            .await;
        
        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token, expected_token);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_app_access_token_network_error() {
        let config = create_test_config("http://nonexistent:12345".to_string(), AppType::SelfBuild);
        let manager = TokenManager::new();
        let app_ticket_manager = Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        ));

        let result = manager
            .get_app_access_token(&config, "", &app_ticket_manager)
            .await;
        
        assert!(result.is_err());

        // 验证失败指标更新
        let metrics = manager.metrics();
        assert!(metrics.refresh_failures.load(Ordering::Relaxed) > 0);
    }
}

#[cfg(test)]
mod tenant_access_token_tests {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_get_tenant_access_token_self_build_success() {
        let mock_server = MockServer::start().await;
        let config = create_test_config(mock_server.uri(), AppType::SelfBuild);
        let manager = TokenManager::new();
        let app_ticket_manager = Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        ));

        let expected_token = "t-test_tenant_access_token_xyz789";
        let response = create_tenant_token_response(expected_token, 7200);

        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/tenant_access_token/internal"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&mock_server)
            .await;

        let result = manager
            .get_tenant_access_token(&config, "test_tenant_key", "", &app_ticket_manager)
            .await;
        
        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token, expected_token);

        // 验证指标更新
        let metrics = manager.metrics();
        assert_eq!(metrics.refresh_success.load(Ordering::Relaxed), 1);
        assert!(metrics.tenant_cache_misses.load(Ordering::Relaxed) > 0);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_tenant_access_token_cache_hit() {
        let mock_server = MockServer::start().await;
        let config = create_test_config(mock_server.uri(), AppType::SelfBuild);
        let manager = TokenManager::new();
        let app_ticket_manager = Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        ));

        let expected_token = "t-cached_tenant_token_123abc";
        let response = create_tenant_token_response(expected_token, 7200);

        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/tenant_access_token/internal"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&mock_server)
            .await;

        let tenant_key = "test_tenant_cache";

        // 第一次调用
        let result1 = manager
            .get_tenant_access_token(&config, tenant_key, "", &app_ticket_manager)
            .await;
        assert!(result1.is_ok());

        let initial_cache_misses = manager.metrics().tenant_cache_misses.load(Ordering::Relaxed);

        // 第二次调用 - 应该从缓存获取
        let result2 = manager
            .get_tenant_access_token(&config, tenant_key, "", &app_ticket_manager)
            .await;
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), expected_token);

        // 验证缓存命中
        let metrics = manager.metrics();
        assert!(metrics.tenant_cache_hits.load(Ordering::Relaxed) > 0);
        assert_eq!(metrics.tenant_cache_misses.load(Ordering::Relaxed), initial_cache_misses);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_tenant_access_token_marketplace() {
        let mock_server = MockServer::start().await;
        let config = create_test_config(mock_server.uri(), AppType::Marketplace);
        let manager = TokenManager::new();
        let app_ticket_manager = Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        ));

        // 先设置 app access token 的 mock
        let app_token_response = create_app_token_response("app_token_for_tenant", 7200);
        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/app_access_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(app_token_response))
            .mount(&mock_server)
            .await;

        // 设置 tenant access token 的 mock
        let expected_tenant_token = "t-marketplace_tenant_token_def456";
        let tenant_response = create_tenant_token_response(expected_tenant_token, 7200);
        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/tenant_access_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(tenant_response))
            .mount(&mock_server)
            .await;

        let result = manager
            .get_tenant_access_token(&config, "marketplace_tenant", "test_ticket", &app_ticket_manager)
            .await;
        
        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token, expected_tenant_token);
    }

    #[tokio::test]  
    #[serial]
    async fn test_get_tenant_access_token_api_error() {
        let mock_server = MockServer::start().await;
        let config = create_test_config(mock_server.uri(), AppType::SelfBuild);
        let manager = TokenManager::new();
        let app_ticket_manager = Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        ));

        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/tenant_access_token/internal"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 99991671,
                "msg": "invalid tenant access token"
            })))
            .mount(&mock_server)
            .await;

        let result = manager
            .get_tenant_access_token(&config, "invalid_tenant", "", &app_ticket_manager)
            .await;
        
        assert!(result.is_err());

        // 验证失败指标更新
        let metrics = manager.metrics();
        assert!(metrics.refresh_failures.load(Ordering::Relaxed) > 0);
    }
}

#[cfg(test)]
mod concurrent_tests {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_concurrent_app_access_token_requests() {
        let mock_server = MockServer::start().await;
        let config = Arc::new(create_test_config(mock_server.uri(), AppType::SelfBuild));
        let manager = Arc::new(TokenManager::new());
        let app_ticket_manager = Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        ));

        let expected_token = "t-concurrent_app_token_123";
        let response = create_app_token_response(expected_token, 7200);

        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/app_access_token/internal"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&mock_server)
            .await;

        let start_time = Instant::now();
        let concurrent_tasks = 10;

        let tasks = (0..concurrent_tasks)
            .map(|_| {
                let manager = manager.clone();
                let config = config.clone();
                let app_ticket_manager = app_ticket_manager.clone();
                tokio::spawn(async move {
                    manager
                        .get_app_access_token(&config, "", &app_ticket_manager)
                        .await
                })
            })
            .collect::<Vec<_>>();

        let results: Vec<_> = futures::future::try_join_all(tasks).await.unwrap();
        let elapsed = start_time.elapsed();

        // 验证所有请求都成功且返回相同的 token
        for result in results {
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected_token);
        }

        // 验证并发性能
        println!(
            "并发 App Token 测试: {} 个并发请求，总时间 {:?}",
            concurrent_tasks, elapsed
        );

        // 验证至少有一些缓存命中（除了第一个请求）
        let metrics = manager.metrics();
        assert!(metrics.app_cache_hits.load(Ordering::Relaxed) > 0);
    }

    #[tokio::test]
    #[serial]
    async fn test_concurrent_tenant_access_token_requests() {
        let mock_server = MockServer::start().await;
        let config = Arc::new(create_test_config(mock_server.uri(), AppType::SelfBuild));
        let manager = Arc::new(TokenManager::new());
        let app_ticket_manager = Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        ));

        let expected_token = "t-concurrent_tenant_token_456";
        let response = create_tenant_token_response(expected_token, 7200);

        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/tenant_access_token/internal"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&mock_server)
            .await;

        let tenant_key = "concurrent_tenant";
        let concurrent_tasks = 8;

        let tasks = (0..concurrent_tasks)
            .map(|_| {
                let manager = manager.clone();
                let config = config.clone();
                let app_ticket_manager = app_ticket_manager.clone();
                tokio::spawn(async move {
                    manager
                        .get_tenant_access_token(&config, tenant_key, "", &app_ticket_manager)
                        .await
                })
            })
            .collect::<Vec<_>>();

        let results: Vec<_> = futures::future::try_join_all(tasks).await.unwrap();

        // 验证所有请求都成功且返回相同的 token
        for result in results {
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected_token);
        }

        // 验证至少有一些缓存命中
        let metrics = manager.metrics();
        assert!(metrics.tenant_cache_hits.load(Ordering::Relaxed) > 0);
    }

    #[tokio::test]
    #[serial] 
    async fn test_mixed_concurrent_token_requests() {
        let mock_server = MockServer::start().await;
        let config = Arc::new(create_test_config(mock_server.uri(), AppType::SelfBuild));
        let manager = Arc::new(TokenManager::new());
        let app_ticket_manager = Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        ));

        // 设置 app token mock
        let app_token_response = create_app_token_response("app_token_mixed", 7200);
        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/app_access_token/internal"))
            .respond_with(ResponseTemplate::new(200).set_body_json(app_token_response))
            .mount(&mock_server)
            .await;

        // 设置 tenant token mock  
        let tenant_token_response = create_tenant_token_response("tenant_token_mixed", 7200);
        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/tenant_access_token/internal"))
            .respond_with(ResponseTemplate::new(200).set_body_json(tenant_token_response))
            .mount(&mock_server)
            .await;

        let mut tasks = Vec::new();

        // 混合 app token 和 tenant token 请求
        for i in 0..5 {
            // App token 任务
            let manager = manager.clone();
            let config = config.clone();
            let app_ticket_manager = app_ticket_manager.clone();
            tasks.push(tokio::spawn(async move {
                manager
                    .get_app_access_token(&config, "", &app_ticket_manager)
                    .await
                    .map(|_| "app")
            }));

            // Tenant token 任务
            let manager = manager.clone();
            let config = config.clone();
            let app_ticket_manager = app_ticket_manager.clone();
            let tenant_key = format!("tenant_{}", i);
            tasks.push(tokio::spawn(async move {
                manager
                    .get_tenant_access_token(&config, &tenant_key, "", &app_ticket_manager)
                    .await
                    .map(|_| "tenant")
            }));
        }

        let results: Vec<_> = futures::future::try_join_all(tasks).await.unwrap();

        // 验证所有请求都成功
        for result in results {
            assert!(result.is_ok());
        }

        // 验证指标正常更新
        let metrics = manager.metrics();
        assert!(metrics.read_lock_acquisitions.load(Ordering::Relaxed) > 0);
        assert!(metrics.write_lock_acquisitions.load(Ordering::Relaxed) > 0);
    }
}

#[cfg(test)]
mod preheating_tests {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    async fn test_preheating_config_custom_values() {
        let config = PreheatingConfig {
            check_interval_seconds: 300,
            preheat_threshold_seconds: 120,
            enable_tenant_preheating: false,
            max_concurrent_preheat: 1,
        };

        assert_eq!(config.check_interval_seconds, 300);
        assert_eq!(config.preheat_threshold_seconds, 120);
        assert!(!config.enable_tenant_preheating);
        assert_eq!(config.max_concurrent_preheat, 1);
    }

    #[tokio::test]
    async fn test_token_manager_preheating_lifecycle() {
        let config = create_test_config("https://example.com".to_string(), AppType::SelfBuild);
        let mut manager = TokenManager::new();
        let app_ticket_manager = Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        ));

        // 初始状态：预热未启动
        assert!(!manager.is_preheating_active());

        // 启动预热
        manager.start_background_preheating(config.clone(), app_ticket_manager.clone());
        assert!(manager.is_preheating_active());

        // 停止预热
        manager.stop_background_preheating();
        assert!(!manager.is_preheating_active());
    }

    #[tokio::test]
    async fn test_token_manager_preheating_with_custom_config() {
        let config = create_test_config("https://example.com".to_string(), AppType::SelfBuild);
        let mut manager = TokenManager::new();
        let app_ticket_manager = Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        ));

        let custom_config = PreheatingConfig {
            check_interval_seconds: 60, // 1分钟检查一次
            preheat_threshold_seconds: 30, // 30秒阈值
            enable_tenant_preheating: true,
            max_concurrent_preheat: 2,
        };

        // 启动带自定义配置的预热
        manager.start_background_preheating_with_config(
            config,
            app_ticket_manager,
            custom_config,
        );
        assert!(manager.is_preheating_active());

        // 停止预热
        manager.stop_background_preheating();
        assert!(!manager.is_preheating_active());
    }

    #[tokio::test]
    async fn test_token_manager_restart_preheating() {
        let config = create_test_config("https://example.com".to_string(), AppType::SelfBuild);
        let mut manager = TokenManager::new();
        let app_ticket_manager = Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        ));

        // 启动第一次预热
        manager.start_background_preheating(config.clone(), app_ticket_manager.clone());
        assert!(manager.is_preheating_active());

        // 用不同配置重新启动预热（应该自动停止旧的）
        let new_config = PreheatingConfig {
            check_interval_seconds: 120,
            preheat_threshold_seconds: 60,
            enable_tenant_preheating: false,
            max_concurrent_preheat: 1,
        };

        manager.start_background_preheating_with_config(
            config,
            app_ticket_manager,
            new_config,
        );
        assert!(manager.is_preheating_active());

        manager.stop_background_preheating();
        assert!(!manager.is_preheating_active());
    }
}

#[cfg(test)]
mod property_based_tests {
    use super::*;

    proptest! {
        #[test]
        fn test_token_metrics_calculations_property(
            app_hits in 0u64..1000,
            app_misses in 0u64..1000,
            tenant_hits in 0u64..1000,
            tenant_misses in 0u64..1000,
            refresh_success in 0u64..1000,
            refresh_failures in 0u64..1000,
        ) {
            let metrics = TokenMetrics::new();
            
            metrics.app_cache_hits.store(app_hits, Ordering::Relaxed);
            metrics.app_cache_misses.store(app_misses, Ordering::Relaxed);
            metrics.tenant_cache_hits.store(tenant_hits, Ordering::Relaxed);
            metrics.tenant_cache_misses.store(tenant_misses, Ordering::Relaxed);
            metrics.refresh_success.store(refresh_success, Ordering::Relaxed);
            metrics.refresh_failures.store(refresh_failures, Ordering::Relaxed);
            
            // 验证命中率计算的属性
            let app_rate = metrics.app_cache_hit_rate();
            let tenant_rate = metrics.tenant_cache_hit_rate();
            let refresh_rate = metrics.refresh_success_rate();
            
            // 命中率应该在 0.0 到 1.0 之间
            prop_assert!(app_rate >= 0.0 && app_rate <= 1.0);
            prop_assert!(tenant_rate >= 0.0 && tenant_rate <= 1.0);
            prop_assert!(refresh_rate >= 0.0 && refresh_rate <= 1.0);
            
            // 如果没有缓存操作，命中率应该为 0
            if app_hits == 0 && app_misses == 0 {
                prop_assert_eq!(app_rate, 0.0);
            }
            if tenant_hits == 0 && tenant_misses == 0 {
                prop_assert_eq!(tenant_rate, 0.0);
            }
            if refresh_success == 0 && refresh_failures == 0 {
                prop_assert_eq!(refresh_rate, 0.0);
            }
            
            // 如果只有命中，命中率应该为 1.0
            if app_hits > 0 && app_misses == 0 {
                prop_assert_eq!(app_rate, 1.0);
            }
            if tenant_hits > 0 && tenant_misses == 0 {
                prop_assert_eq!(tenant_rate, 1.0);
            }
            if refresh_success > 0 && refresh_failures == 0 {
                prop_assert_eq!(refresh_rate, 1.0);
            }
            
            // 如果只有未命中，命中率应该为 0.0
            if app_hits == 0 && app_misses > 0 {
                prop_assert_eq!(app_rate, 0.0);
            }
            if tenant_hits == 0 && tenant_misses > 0 {
                prop_assert_eq!(tenant_rate, 0.0);
            }
            if refresh_success == 0 && refresh_failures > 0 {
                prop_assert_eq!(refresh_rate, 0.0);
            }
        }
    }

    proptest! {
        #[test]
        fn test_preheating_config_values_property(
            check_interval in 1u64..86400, // 1秒到24小时
            preheat_threshold in 1u64..7200, // 1秒到2小时
            max_concurrent in 1usize..100,
        ) {
            let config = PreheatingConfig {
                check_interval_seconds: check_interval,
                preheat_threshold_seconds: preheat_threshold,
                enable_tenant_preheating: true,
                max_concurrent_preheat: max_concurrent,
            };
            
            // 配置值应该与设置的值匹配
            prop_assert_eq!(config.check_interval_seconds, check_interval);
            prop_assert_eq!(config.preheat_threshold_seconds, preheat_threshold);
            prop_assert_eq!(config.max_concurrent_preheat, max_concurrent);
            
            // 应该能够正常创建配置而不panic
            prop_assert!(config.enable_tenant_preheating);
        }
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_token_manager_cache_performance() {
        let manager = TokenManager::new();
        let cache = manager.get_cache();
        
        let iterations = 1000;
        let start_time = Instant::now();
        
        // 批量写入操作
        {
            let mut cache_write = cache.write().await;
            for i in 0..iterations {
                cache_write.set(
                    &format!("perf_key_{}", i),
                    format!("perf_value_{}", i),
                    3600,
                );
            }
        }
        
        let write_elapsed = start_time.elapsed();
        
        // 批量读取操作
        let read_start = Instant::now();
        {
            let cache_read = cache.read().await;
            for i in 0..iterations {
                let _ = cache_read.get(&format!("perf_key_{}", i));
            }
        }
        let read_elapsed = read_start.elapsed();
        
        println!(
            "缓存性能测试: {} 次写入 {:?}, {} 次读取 {:?}",
            iterations, write_elapsed, iterations, read_elapsed
        );
        
        // 写入和读取操作都应该在合理时间内完成
        assert!(write_elapsed < Duration::from_millis(1000));
        assert!(read_elapsed < Duration::from_millis(100));
    }

    #[tokio::test]  
    async fn test_token_metrics_performance() {
        let metrics = TokenMetrics::new();
        let iterations = 10000;
        
        let start_time = Instant::now();
        
        // 并发更新指标
        let tasks = (0..iterations)
            .map(|_| {
                let metrics = &metrics;
                tokio::spawn(async move {
                    metrics.app_cache_hits.fetch_add(1, Ordering::Relaxed);
                    metrics.tenant_cache_misses.fetch_add(1, Ordering::Relaxed);
                    metrics.refresh_success.fetch_add(1, Ordering::Relaxed);
                })
            })
            .collect::<Vec<_>>();
            
        futures::future::try_join_all(tasks).await.unwrap();
        let elapsed = start_time.elapsed();
        
        // 验证所有更新都成功
        assert_eq!(metrics.app_cache_hits.load(Ordering::Relaxed), iterations as u64);
        assert_eq!(metrics.tenant_cache_misses.load(Ordering::Relaxed), iterations as u64);
        assert_eq!(metrics.refresh_success.load(Ordering::Relaxed), iterations as u64);
        
        println!(
            "指标更新性能测试: {} 次并发更新，总时间 {:?}",
            iterations, elapsed
        );
        
        // 原子操作应该很快
        assert!(elapsed < Duration::from_secs(1));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_token_manager_full_lifecycle() {
        let mock_server = MockServer::start().await;
        let config = create_test_config(mock_server.uri(), AppType::SelfBuild);
        let manager = TokenManager::new();
        let app_ticket_manager = Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        ));

        // 设置 mock 响应
        let app_token_response = create_app_token_response("lifecycle_app_token", 7200);
        let tenant_token_response = create_tenant_token_response("lifecycle_tenant_token", 7200);

        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/app_access_token/internal"))
            .respond_with(ResponseTemplate::new(200).set_body_json(app_token_response))
            .mount(&mock_server)
            .await;

        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/tenant_access_token/internal"))
            .respond_with(ResponseTemplate::new(200).set_body_json(tenant_token_response))
            .mount(&mock_server)
            .await;

        // 1. 获取 app access token (应该触发网络请求)
        let app_result1 = manager
            .get_app_access_token(&config, "", &app_ticket_manager)
            .await;
        assert!(app_result1.is_ok());
        assert_eq!(app_result1.unwrap(), "lifecycle_app_token");

        // 2. 再次获取 app access token (应该从缓存获取)
        let app_result2 = manager
            .get_app_access_token(&config, "", &app_ticket_manager)
            .await;
        assert!(app_result2.is_ok());
        assert_eq!(app_result2.unwrap(), "lifecycle_app_token");

        // 3. 获取 tenant access token (应该触发网络请求)
        let tenant_result1 = manager
            .get_tenant_access_token(&config, "test_tenant", "", &app_ticket_manager)
            .await;
        assert!(tenant_result1.is_ok());
        assert_eq!(tenant_result1.unwrap(), "lifecycle_tenant_token");

        // 4. 再次获取 tenant access token (应该从缓存获取)
        let tenant_result2 = manager
            .get_tenant_access_token(&config, "test_tenant", "", &app_ticket_manager)
            .await;
        assert!(tenant_result2.is_ok());
        assert_eq!(tenant_result2.unwrap(), "lifecycle_tenant_token");

        // 5. 验证指标统计
        let metrics = manager.metrics();
        assert!(metrics.app_cache_hits.load(Ordering::Relaxed) >= 1);
        assert!(metrics.tenant_cache_hits.load(Ordering::Relaxed) >= 1);
        assert!(metrics.refresh_success.load(Ordering::Relaxed) >= 2); // 至少2次成功刷新
        assert_eq!(metrics.refresh_failures.load(Ordering::Relaxed), 0);

        // 6. 生成性能报告
        let report = metrics.performance_report();
        assert!(report.contains("App Cache Hit Rate"));
        assert!(report.contains("Tenant Cache Hit Rate"));
        assert!(report.contains("Refresh Success Rate"));

        manager.log_performance_metrics();
    }

    #[tokio::test]
    #[serial] 
    async fn test_token_manager_error_recovery() {
        let mock_server = MockServer::start().await;
        let config = create_test_config(mock_server.uri(), AppType::SelfBuild);
        let manager = TokenManager::new();
        let app_ticket_manager = Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        ));

        // 第一次调用返回错误
        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/app_access_token/internal"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 10013,
                "msg": "invalid app_id or app_secret"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        // 第二次调用返回成功
        let success_response = create_app_token_response("recovery_app_token", 7200);
        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/app_access_token/internal"))
            .respond_with(ResponseTemplate::new(200).set_body_json(success_response))
            .expect(1)
            .mount(&mock_server)
            .await;

        // 第一次调用 - 应该失败
        let result1 = manager
            .get_app_access_token(&config, "", &app_ticket_manager)
            .await;
        assert!(result1.is_err());

        // 验证失败指标
        assert!(manager.metrics().refresh_failures.load(Ordering::Relaxed) > 0);

        // 第二次调用 - 应该成功
        let result2 = manager
            .get_app_access_token(&config, "", &app_ticket_manager)
            .await;
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), "recovery_app_token");

        // 验证恢复后的指标
        let metrics = manager.metrics();
        assert!(metrics.refresh_failures.load(Ordering::Relaxed) > 0);
        assert!(metrics.refresh_success.load(Ordering::Relaxed) > 0);
    }
}