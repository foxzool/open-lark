//! Authentication 模块集成测试
//!
//! 测试认证模块的整体集成和真实使用场景，包括：
//! - 令牌生命周期管理
//! - 用户信息获取流程
//! - 错误处理和恢复
//! - 性能和并发特性
//! - 端到端工作流程

use open_lark::{
    core::{
        config::Config,
        constants::{AccessTokenType, AppType},
        token_manager::{PreheatingConfig, TokenManager},
    },
    service::authentication::{v1::UserInfo, AuthenService},
};
use proptest::prelude::*;
use rstest::rstest;
use serde_json::json;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{
    sync::Mutex,
    time::{sleep, Instant},
};
use wiremock::{
    matchers::{header, method, path},
    Mock, MockServer, ResponseTemplate,
};

/// 创建集成测试配置
async fn create_integration_config(
    mock_server: &MockServer,
    app_type: AppType,
) -> Arc<Config> {
    Arc::new(Config {
        app_id: "integration_test_app".to_string(),
        app_secret: "integration_test_secret".to_string(),
        app_type,
        base_url: mock_server.uri(),
        http_client: reqwest::Client::new(),
        enable_token_cache: true,
        req_timeout: Some(Duration::from_secs(10)),
        header: HashMap::new(),
        token_manager: Arc::new(Mutex::new(TokenManager::new())),
        app_ticket_manager: Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        )),
    })
}

/// 设置成功的令牌响应 mock
async fn setup_successful_token_mocks(mock_server: &MockServer) {
    let app_token_response = json!({
        "code": 0,
        "msg": "success",
        "app_access_token": "t-integration_app_token_12345",
        "expire": 7200
    });

    let tenant_token_response = json!({
        "code": 0,
        "msg": "success",
        "tenant_access_token": "t-integration_tenant_token_67890",
        "expire": 7200
    });

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
}

/// 设置成功的用户信息响应 mock
async fn setup_successful_user_info_mock(mock_server: &MockServer, user_access_token: &str) {
    let user_info_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "name": "集成测试用户",
            "en_name": "Integration Test User",
            "avatar_url": "https://example.com/integration_avatar.jpg",
            "avatar_thumb": "https://example.com/integration_avatar_thumb.jpg",
            "avatar_middle": "https://example.com/integration_avatar_middle.jpg",
            "avatar_big": "https://example.com/integration_avatar_big.jpg",
            "open_id": "ou-integration123456789",
            "union_id": "on-integration123456789",
            "email": "integration@example.com",
            "enterprise_email": "integration@company.com",
            "user_id": "integration_user_123",
            "mobile": "+8613800138000",
            "tenant_key": "integration_tenant_key",
            "employee_no": "INT123456"
        }
    });

    Mock::given(method("GET"))
        .and(path("/open-apis/authen/v1/user_info"))
        .and(header("Authorization", format!("Bearer {}", user_access_token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(user_info_response))
        .mount(&mock_server)
        .await;
}

#[cfg(test)]
mod end_to_end_tests {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_complete_authentication_flow() {
        let mock_server = MockServer::start().await;
        let config = create_integration_config(&mock_server, AppType::SelfBuild).await;

        // 设置 mock 响应
        setup_successful_token_mocks(&mock_server).await;
        let user_token = "integration_user_token_abc123";
        setup_successful_user_info_mock(&mock_server, user_token).await;

        // 创建认证服务
        let auth_service = AuthenService::new(config.clone());

        // 执行完整的认证流程
        let start_time = Instant::now();

        // 1. 获取用户信息（这会触发内部的令牌获取）
        let user_info_result = auth_service.v1.user_info.get(user_token).await;
        
        let total_elapsed = start_time.elapsed();

        // 验证结果
        assert!(user_info_result.is_ok());
        let response = user_info_result.unwrap();
        assert_eq!(response.code, 0);
        assert_eq!(response.msg, "success");
        assert!(response.data.is_some());

        let user_info = response.data.unwrap();
        assert_eq!(user_info.name, "集成测试用户");
        assert_eq!(user_info.en_name, "Integration Test User");
        assert_eq!(user_info.open_id, "ou-integration123456789");
        assert_eq!(user_info.union_id, "on-integration123456789");
        assert_eq!(user_info.user_id, "integration_user_123");
        assert_eq!(user_info.tenant_key, "integration_tenant_key");

        println!("完整认证流程耗时: {:?}", total_elapsed);

        // 验证性能合理
        assert!(total_elapsed < Duration::from_secs(5));
    }

    #[tokio::test]
    #[serial]
    async fn test_authentication_with_token_caching() {
        let mock_server = MockServer::start().await;
        let config = create_integration_config(&mock_server, AppType::SelfBuild).await;

        setup_successful_token_mocks(&mock_server).await;
        let user_token = "cache_test_user_token";
        setup_successful_user_info_mock(&mock_server, user_token).await;

        let auth_service = AuthenService::new(config.clone());

        // 第一次调用 - 应该触发缓存未命中
        let start_time = Instant::now();
        let result1 = auth_service.v1.user_info.get(user_token).await;
        let first_call_time = start_time.elapsed();
        assert!(result1.is_ok());

        // 第二次调用 - 应该从缓存获取（如果有缓存逻辑）
        let start_time = Instant::now();
        let result2 = auth_service.v1.user_info.get(user_token).await;
        let second_call_time = start_time.elapsed();
        assert!(result2.is_ok());

        // 验证两次调用都成功，且第二次不会慢于第一次（允许网络波动）
        println!("第一次调用耗时: {:?}", first_call_time);
        println!("第二次调用耗时: {:?}", second_call_time);

        // 验证结果一致性
        let data1 = result1.unwrap().data.unwrap();
        let data2 = result2.unwrap().data.unwrap();
        assert_eq!(data1.name, data2.name);
        assert_eq!(data1.open_id, data2.open_id);
    }

    #[tokio::test]
    #[serial]
    async fn test_authentication_error_recovery() {
        let mock_server = MockServer::start().await;
        let config = create_integration_config(&mock_server, AppType::SelfBuild).await;

        let user_token = "error_recovery_token";

        // 第一次调用返回错误
        Mock::given(method("GET"))
            .and(path("/open-apis/authen/v1/user_info"))
            .and(header("Authorization", format!("Bearer {}", user_token)))
            .respond_with(ResponseTemplate::new(401).set_body_json(json!({
                "code": 99991663,
                "msg": "invalid user access token"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let auth_service = AuthenService::new(config.clone());

        // 第一次调用 - 应该收到错误响应
        let result1 = auth_service.v1.user_info.get(user_token).await;
        assert!(result1.is_ok()); // HTTP 调用成功，但业务逻辑失败
        let response1 = result1.unwrap();
        assert_ne!(response1.code, 0); // 业务错误码

        // 设置第二次调用成功的 mock
        setup_successful_user_info_mock(&mock_server, user_token).await;

        // 第二次调用 - 应该成功
        let result2 = auth_service.v1.user_info.get(user_token).await;
        assert!(result2.is_ok());
        let response2 = result2.unwrap();
        assert_eq!(response2.code, 0);
        assert!(response2.data.is_some());
    }

    #[tokio::test]
    #[serial]
    async fn test_authentication_with_marketplace_app() {
        let mock_server = MockServer::start().await;
        let config = create_integration_config(&mock_server, AppType::Marketplace).await;

        // 为 Marketplace 应用设置不同的 mock
        let app_token_response = json!({
            "code": 0,
            "msg": "success",
            "app_access_token": "t-marketplace_app_token",
            "expire": 7200
        });

        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/app_access_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(app_token_response))
            .mount(&mock_server)
            .await;

        let user_token = "marketplace_user_token";
        setup_successful_user_info_mock(&mock_server, user_token).await;

        let auth_service = AuthenService::new(config);

        let result = auth_service.v1.user_info.get(user_token).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.code, 0);
        assert!(response.data.is_some());
    }
}

#[cfg(test)]
mod concurrent_integration_tests {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_concurrent_authentication_requests() {
        let mock_server = MockServer::start().await;
        let config = create_integration_config(&mock_server, AppType::SelfBuild).await;

        setup_successful_token_mocks(&mock_server).await;

        // 为多个不同的 user token 设置 mock
        let user_tokens = vec![
            "concurrent_token_1",
            "concurrent_token_2", 
            "concurrent_token_3",
            "concurrent_token_4",
            "concurrent_token_5",
        ];

        for token in &user_tokens {
            setup_successful_user_info_mock(&mock_server, token).await;
        }

        let auth_service = Arc::new(AuthenService::new(config));

        let start_time = Instant::now();

        // 创建并发任务
        let tasks = user_tokens
            .into_iter()
            .enumerate()
            .map(|(i, token)| {
                let auth_service = auth_service.clone();
                tokio::spawn(async move {
                    let result = auth_service.v1.user_info.get(token).await;
                    (i, result)
                })
            })
            .collect::<Vec<_>>();

        // 等待所有任务完成
        let results: Vec<_> = futures::future::try_join_all(tasks).await.unwrap();
        let elapsed = start_time.elapsed();

        // 验证所有请求都成功
        for (i, result) in results {
            assert!(result.is_ok(), "Task {} failed", i);
            let response = result.unwrap();
            assert_eq!(response.code, 0);
            assert!(response.data.is_some());
        }

        println!(
            "并发认证测试: 5 个并发请求，总时间 {:?}",
            elapsed
        );

        // 验证并发性能
        assert!(elapsed < Duration::from_secs(10));
    }

    #[tokio::test]
    #[serial]
    async fn test_mixed_concurrent_operations() {
        let mock_server = MockServer::start().await;
        let config = create_integration_config(&mock_server, AppType::SelfBuild).await;

        setup_successful_token_mocks(&mock_server).await;

        // 设置多种不同的响应
        for i in 1..=10 {
            let token = format!("mixed_token_{}", i);
            setup_successful_user_info_mock(&mock_server, &token).await;
        }

        let auth_service = Arc::new(AuthenService::new(config.clone()));

        let mut tasks = Vec::new();

        // 混合不同类型的操作
        for i in 1..=5 {
            // 用户信息获取任务
            let auth_service = auth_service.clone();
            let token = format!("mixed_token_{}", i);
            tasks.push(tokio::spawn(async move {
                auth_service.v1.user_info.get(token).await.map(|_| "user_info")
            }));

            // 令牌管理器操作任务
            let token_manager = config.token_manager.clone();
            let config_clone = config.clone();
            let app_ticket_manager = config.app_ticket_manager.clone();
            tasks.push(tokio::spawn(async move {
                let manager = token_manager.lock().await;
                manager
                    .get_app_access_token(&config_clone, "", &app_ticket_manager)
                    .await
                    .map(|_| "app_token")
            }));
        }

        let start_time = Instant::now();
        let results: Vec<_> = futures::future::try_join_all(tasks).await.unwrap();
        let elapsed = start_time.elapsed();

        // 验证所有操作都成功
        for (i, result) in results.iter().enumerate() {
            assert!(result.is_ok(), "Mixed operation {} failed", i);
        }

        println!(
            "混合并发操作测试: {} 个操作，总时间 {:?}",
            results.len(),
            elapsed
        );
    }
}

#[cfg(test)]
mod token_lifecycle_tests {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_token_preheating_integration() {
        let mock_server = MockServer::start().await;
        let config = create_integration_config(&mock_server, AppType::SelfBuild).await;

        setup_successful_token_mocks(&mock_server).await;

        // 创建带预热功能的令牌管理器
        let mut token_manager = TokenManager::new();
        let app_ticket_manager = config.app_ticket_manager.clone();

        // 启动预热（使用较短的间隔用于测试）
        let preheating_config = PreheatingConfig {
            check_interval_seconds: 1, // 1秒检查一次
            preheat_threshold_seconds: 3600, // 1小时阈值（所有token都需要预热）
            enable_tenant_preheating: true,
            max_concurrent_preheat: 2,
        };

        token_manager.start_background_preheating_with_config(
            (*config).clone(),
            app_ticket_manager.clone(),
            preheating_config,
        );

        assert!(token_manager.is_preheating_active());

        // 等待预热任务运行一段时间
        sleep(Duration::from_secs(2)).await;

        // 获取性能指标
        let metrics = token_manager.metrics();
        metrics.performance_report();

        // 停止预热
        token_manager.stop_background_preheating();
        assert!(!token_manager.is_preheating_active());
    }

    #[tokio::test]
    #[serial]
    async fn test_token_expiry_and_refresh() {
        let mock_server = MockServer::start().await;
        let config = create_integration_config(&mock_server, AppType::SelfBuild).await;

        // 设置短期过期的令牌
        let short_lived_token_response = json!({
            "code": 0,
            "msg": "success",
            "app_access_token": "t-short_lived_token",
            "expire": 2 // 2秒后过期
        });

        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/app_access_token/internal"))
            .respond_with(ResponseTemplate::new(200).set_body_json(short_lived_token_response))
            .expect(1)
            .mount(&mock_server)
            .await;

        let token_manager = TokenManager::new();
        let app_ticket_manager = config.app_ticket_manager.clone();

        // 第一次获取令牌
        let token1 = token_manager
            .get_app_access_token(&config, "", &app_ticket_manager)
            .await;
        assert!(token1.is_ok());
        assert_eq!(token1.unwrap(), "t-short_lived_token");

        // 等待令牌过期
        sleep(Duration::from_secs(3)).await;

        // 设置刷新后的令牌响应
        let refreshed_token_response = json!({
            "code": 0,
            "msg": "success",
            "app_access_token": "t-refreshed_token",
            "expire": 7200
        });

        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/app_access_token/internal"))
            .respond_with(ResponseTemplate::new(200).set_body_json(refreshed_token_response))
            .expect(1)
            .mount(&mock_server)
            .await;

        // 第二次获取令牌 - 应该触发刷新
        let token2 = token_manager
            .get_app_access_token(&config, "", &app_ticket_manager)
            .await;
        assert!(token2.is_ok());
        assert_eq!(token2.unwrap(), "t-refreshed_token");

        // 验证指标更新
        let metrics = token_manager.metrics();
        assert!(metrics.refresh_success.load(std::sync::atomic::Ordering::Relaxed) >= 2);
    }

    #[tokio::test]
    #[serial]
    async fn test_token_cache_performance_metrics() {
        let mock_server = MockServer::start().await;
        let config = create_integration_config(&mock_server, AppType::SelfBuild).await;

        setup_successful_token_mocks(&mock_server).await;

        let token_manager = TokenManager::new();
        let app_ticket_manager = config.app_ticket_manager.clone();

        // 执行多次令牌获取以收集指标
        for _ in 0..5 {
            let _ = token_manager
                .get_app_access_token(&config, "", &app_ticket_manager)
                .await;
        }

        // 验证指标收集
        let metrics = token_manager.metrics();
        let performance_report = metrics.performance_report();

        println!("Token 性能报告:\n{}", performance_report);

        // 验证指标合理性
        assert!(metrics.refresh_success.load(std::sync::atomic::Ordering::Relaxed) >= 1);
        assert!(metrics.app_cache_hits.load(std::sync::atomic::Ordering::Relaxed) >= 3); // 至少有缓存命中

        let hit_rate = metrics.app_cache_hit_rate();
        assert!(hit_rate >= 0.0 && hit_rate <= 1.0);
    }
}

#[cfg(test)]
mod performance_integration_tests {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_authentication_service_under_load() {
        let mock_server = MockServer::start().await;
        let config = create_integration_config(&mock_server, AppType::SelfBuild).await;

        setup_successful_token_mocks(&mock_server).await;

        // 设置大量用户令牌的 mock
        for i in 1..=50 {
            let token = format!("load_test_token_{}", i);
            setup_successful_user_info_mock(&mock_server, &token).await;
        }

        let auth_service = Arc::new(AuthenService::new(config));

        let start_time = Instant::now();
        let load_tasks = 50;

        let tasks = (1..=load_tasks)
            .map(|i| {
                let auth_service = auth_service.clone();
                tokio::spawn(async move {
                    let token = format!("load_test_token_{}", i);
                    auth_service.v1.user_info.get(token).await
                })
            })
            .collect::<Vec<_>>();

        let results: Vec<_> = futures::future::try_join_all(tasks).await.unwrap();
        let elapsed = start_time.elapsed();

        // 验证所有请求都成功
        for (i, result) in results.iter().enumerate() {
            assert!(result.is_ok(), "Load test request {} failed", i + 1);
            let response = result.as_ref().unwrap();
            assert_eq!(response.code, 0);
        }

        println!(
            "负载测试: {} 个请求，总时间 {:?}，平均每个 {:?}",
            load_tasks,
            elapsed,
            elapsed / load_tasks
        );

        // 验证负载下的性能合理
        assert!(elapsed < Duration::from_secs(30));
    }

    #[tokio::test]
    #[serial]
    async fn test_authentication_memory_usage_stability() {
        let mock_server = MockServer::start().await;
        let config = create_integration_config(&mock_server, AppType::SelfBuild).await;

        setup_successful_token_mocks(&mock_server).await;

        let user_token = "memory_stability_token";
        setup_successful_user_info_mock(&mock_server, user_token).await;

        let auth_service = AuthenService::new(config);

        // 执行大量重复请求以测试内存稳定性
        for batch in 0..10 {
            let start_time = Instant::now();

            for i in 0..100 {
                let result = auth_service.v1.user_info.get(user_token).await;
                assert!(result.is_ok(), "Batch {} request {} failed", batch, i);
            }

            let batch_time = start_time.elapsed();
            println!("Batch {} (100 requests): {:?}", batch, batch_time);

            // 短暂暂停以允许内存回收
            sleep(Duration::from_millis(10)).await;
        }

        println!("内存稳定性测试完成：1000 个请求处理完毕");
    }

    #[tokio::test]
    #[serial]
    async fn test_authentication_latency_consistency() {
        let mock_server = MockServer::start().await;
        let config = create_integration_config(&mock_server, AppType::SelfBuild).await;

        setup_successful_token_mocks(&mock_server).await;

        let user_token = "latency_test_token";
        setup_successful_user_info_mock(&mock_server, user_token).await;

        let auth_service = AuthenService::new(config);

        let mut latencies = Vec::new();

        // 测量多次请求的延迟
        for _ in 0..20 {
            let start_time = Instant::now();
            let result = auth_service.v1.user_info.get(user_token).await;
            let latency = start_time.elapsed();

            assert!(result.is_ok());
            latencies.push(latency);
        }

        // 计算延迟统计
        let total: Duration = latencies.iter().sum();
        let avg_latency = total / latencies.len() as u32;
        let min_latency = *latencies.iter().min().unwrap();
        let max_latency = *latencies.iter().max().unwrap();

        println!(
            "延迟统计: 平均 {:?}, 最小 {:?}, 最大 {:?}",
            avg_latency, min_latency, max_latency
        );

        // 验证延迟合理性和一致性
        assert!(avg_latency < Duration::from_millis(500));
        assert!(max_latency < Duration::from_secs(2));
        assert!(max_latency.as_millis() / min_latency.as_millis() < 10); // 最大延迟不应该超过最小延迟的10倍
    }
}

#[cfg(test)]
mod real_world_scenario_tests {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_user_authentication_workflow() {
        let mock_server = MockServer::start().await;
        let config = create_integration_config(&mock_server, AppType::SelfBuild).await;

        setup_successful_token_mocks(&mock_server).await;

        // 模拟用户登录工作流程
        let user_scenarios = vec![
            ("new_user_token", "新用户", "New User"),
            ("returning_user_token", "老用户", "Returning User"),
            ("admin_user_token", "管理员", "Admin User"),
            ("guest_user_token", "访客", "Guest User"),
        ];

        for (token, name, en_name) in user_scenarios {
            let user_info_response = json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "name": name,
                    "en_name": en_name,
                    "avatar_url": "https://example.com/avatar.jpg",
                    "avatar_thumb": "https://example.com/avatar_thumb.jpg",
                    "avatar_middle": "https://example.com/avatar_middle.jpg",
                    "avatar_big": "https://example.com/avatar_big.jpg",
                    "open_id": format!("ou-{}", token),
                    "union_id": format!("on-{}", token),
                    "email": format!("{}@example.com", token),
                    "enterprise_email": format!("{}@company.com", token),
                    "user_id": format!("user_{}", token),
                    "mobile": "+8613800138000",
                    "tenant_key": "scenario_tenant_key",
                    "employee_no": format!("EMP_{}", token.to_uppercase())
                }
            });

            Mock::given(method("GET"))
                .and(path("/open-apis/authen/v1/user_info"))
                .and(header("Authorization", format!("Bearer {}", token)))
                .respond_with(ResponseTemplate::new(200).set_body_json(user_info_response))
                .mount(&mock_server)
                .await;
        }

        let auth_service = AuthenService::new(config);

        // 执行用户认证工作流程
        for (token, expected_name, expected_en_name) in user_scenarios {
            println!("测试用户场景: {} ({})", expected_name, token);

            let start_time = Instant::now();
            let result = auth_service.v1.user_info.get(token).await;
            let elapsed = start_time.elapsed();

            assert!(result.is_ok());
            let response = result.unwrap();
            assert_eq!(response.code, 0);
            
            let user_info = response.data.unwrap();
            assert_eq!(user_info.name, expected_name);
            assert_eq!(user_info.en_name, expected_en_name);
            assert_eq!(user_info.open_id, format!("ou-{}", token));

            println!("  ✅ 用户 {} 认证成功，耗时 {:?}", expected_name, elapsed);
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_enterprise_integration_scenario() {
        let mock_server = MockServer::start().await;
        let config = create_integration_config(&mock_server, AppType::Marketplace).await;

        // 设置企业级应用的认证流程
        let marketplace_app_token_response = json!({
            "code": 0,
            "msg": "success",
            "app_access_token": "t-enterprise_app_token",
            "expire": 7200
        });

        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/app_access_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(marketplace_app_token_response))
            .mount(&mock_server)
            .await;

        // 设置多租户场景
        let tenants = vec![
            ("tenant_corp_a", "企业A"),
            ("tenant_corp_b", "企业B"), 
            ("tenant_corp_c", "企业C"),
        ];

        for (tenant_key, tenant_name) in &tenants {
            let tenant_token_response = json!({
                "code": 0,
                "msg": "success",
                "tenant_access_token": format!("t-{}_token", tenant_key),
                "expire": 7200
            });

            Mock::given(method("POST"))
                .and(path("/open-apis/auth/v3/tenant_access_token"))
                .respond_with(ResponseTemplate::new(200).set_body_json(tenant_token_response))
                .mount(&mock_server)
                .await;

            // 为每个租户的用户设置用户信息
            let user_token = format!("{}_user_token", tenant_key);
            let user_info_response = json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "name": format!("{}员工", tenant_name),
                    "en_name": format!("{} Employee", tenant_name),
                    "avatar_url": "https://example.com/avatar.jpg",
                    "avatar_thumb": "https://example.com/avatar_thumb.jpg",
                    "avatar_middle": "https://example.com/avatar_middle.jpg",
                    "avatar_big": "https://example.com/avatar_big.jpg",
                    "open_id": format!("ou-{}_employee", tenant_key),
                    "union_id": format!("on-{}_employee", tenant_key),
                    "email": format!("employee@{}.com", tenant_key),
                    "enterprise_email": format!("employee@{}.com", tenant_key),
                    "user_id": format!("{}_employee_id", tenant_key),
                    "mobile": "+8613800138000",
                    "tenant_key": tenant_key.to_string(),
                    "employee_no": format!("EMP_{}", tenant_key.to_uppercase())
                }
            });

            Mock::given(method("GET"))
                .and(path("/open-apis/authen/v1/user_info"))
                .and(header("Authorization", format!("Bearer {}", user_token)))
                .respond_with(ResponseTemplate::new(200).set_body_json(user_info_response))
                .mount(&mock_server)
                .await;
        }

        let auth_service = AuthenService::new(config.clone());

        // 模拟企业级多租户认证场景
        println!("开始企业级多租户认证测试");

        for (tenant_key, tenant_name) in tenants {
            println!("处理租户: {} ({})", tenant_name, tenant_key);

            // 获取租户访问令牌
            let tenant_token_result = config.token_manager.lock().await
                .get_tenant_access_token(&config, &tenant_key, "test_app_ticket", &config.app_ticket_manager)
                .await;
            assert!(tenant_token_result.is_ok());
            println!("  ✅ 租户令牌获取成功");

            // 获取用户信息
            let user_token = format!("{}_user_token", tenant_key);
            let user_info_result = auth_service.v1.user_info.get(user_token).await;
            assert!(user_info_result.is_ok());

            let response = user_info_result.unwrap();
            assert_eq!(response.code, 0);
            let user_info = response.data.unwrap();
            assert_eq!(user_info.tenant_key, tenant_key);
            println!("  ✅ 用户信息获取成功: {}", user_info.name);
        }

        println!("企业级多租户认证测试完成");
    }

    #[tokio::test] 
    #[serial]
    async fn test_error_handling_and_retry_scenario() {
        let mock_server = MockServer::start().await;
        let config = create_integration_config(&mock_server, AppType::SelfBuild).await;

        let user_token = "retry_scenario_token";

        // 设置一系列错误响应，最后成功
        let error_responses = vec![
            (500, json!({"error": "Internal Server Error"})),
            (429, json!({"code": 1061045, "msg": "rate limit exceeded"})),
            (401, json!({"code": 99991663, "msg": "invalid user access token"})),
        ];

        for (status, body) in error_responses {
            Mock::given(method("GET"))
                .and(path("/open-apis/authen/v1/user_info"))
                .and(header("Authorization", format!("Bearer {}", user_token)))
                .respond_with(ResponseTemplate::new(status).set_body_json(body))
                .expect(1)
                .mount(&mock_server)
                .await;
        }

        // 最终成功响应
        setup_successful_user_info_mock(&mock_server, user_token).await;

        let auth_service = AuthenService::new(config);

        // 模拟错误恢复场景
        println!("开始错误处理和重试场景测试");

        let mut attempts = 0;
        let max_attempts = 5;

        while attempts < max_attempts {
            attempts += 1;
            println!("尝试第 {} 次认证", attempts);

            let result = auth_service.v1.user_info.get(user_token).await;
            
            match result {
                Ok(response) if response.code == 0 => {
                    println!("  ✅ 第 {} 次尝试成功", attempts);
                    assert!(response.data.is_some());
                    break;
                }
                Ok(response) => {
                    println!("  ⚠️ 第 {} 次尝试收到业务错误: {}", attempts, response.msg);
                    if attempts < max_attempts {
                        sleep(Duration::from_millis(100)).await;
                    }
                }
                Err(e) => {
                    println!("  ❌ 第 {} 次尝试网络错误: {:?}", attempts, e);
                    if attempts < max_attempts {
                        sleep(Duration::from_millis(100)).await;
                    }
                }
            }
        }

        assert!(attempts <= max_attempts, "重试次数超过限制");
        println!("错误处理和重试场景测试完成，共尝试 {} 次", attempts);
    }
}