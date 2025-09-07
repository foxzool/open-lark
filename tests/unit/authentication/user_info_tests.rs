//! UserInfoService 测试
//!
//! 测试用户信息服务的所有功能，包括：
//! - API 请求构建和执行
//! - 响应数据反序列化
//! - 错误处理和边界条件
//! - 并发安全性
//! - 性能特性

use open_lark::{
    core::{
        config::Config,
        constants::{AccessTokenType, AppType},
        req_option::RequestOption,
    },
    service::authentication::v1::{UserInfo, UserInfoService},
};
use proptest::prelude::*;
use reqwest::StatusCode;
use rstest::rstest;
use serde_json::json;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::Instant};
use wiremock::{
    matchers::{header, method, path},
    Mock, MockServer, ResponseTemplate,
};

/// 创建测试用的 UserInfoService 实例
fn create_test_service(base_url: String) -> UserInfoService {
    let config = Config {
        app_id: "test_app_id".to_string(),
        app_secret: "test_app_secret".to_string(),
        app_type: AppType::SelfBuild,
        base_url,
        http_client: reqwest::Client::new(),
        enable_token_cache: true,
        req_timeout: Some(Duration::from_secs(10)),
        header: HashMap::new(),
        token_manager: Arc::new(Mutex::new(
            open_lark::core::token_manager::TokenManager::new(),
        )),
        app_ticket_manager: Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        )),
    };

    UserInfoService::new(config)
}

/// 创建测试用的 UserInfo 数据
fn create_test_user_info() -> UserInfo {
    UserInfo {
        name: "张三".to_string(),
        en_name: "Zhang San".to_string(),
        avatar_url: "https://example.com/avatar.jpg".to_string(),
        avatar_thumb: "https://example.com/avatar_thumb.jpg".to_string(),
        avatar_middle: "https://example.com/avatar_middle.jpg".to_string(),
        avatar_big: "https://example.com/avatar_big.jpg".to_string(),
        open_id: "ou-test123456789".to_string(),
        union_id: "on-test123456789".to_string(),
        email: Some("zhangsan@example.com".to_string()),
        enterprise_email: Some("zhangsan@company.com".to_string()),
        user_id: "user123456".to_string(),
        mobile: Some("+8613800138000".to_string()),
        tenant_key: "tenant_key_123".to_string(),
        employee_no: "E123456".to_string(),
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_user_info_service_creation() {
        let service = create_test_service("https://open.feishu.cn".to_string());
        // 验证服务创建成功，不应该 panic
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_app_secret");
        assert_eq!(service.config.app_type, AppType::SelfBuild);
    }

    #[test]
    fn test_user_info_deserialization() {
        let json_data = json!({
            "name": "张三",
            "en_name": "Zhang San",
            "avatar_url": "https://example.com/avatar.jpg",
            "avatar_thumb": "https://example.com/avatar_thumb.jpg",
            "avatar_middle": "https://example.com/avatar_middle.jpg",
            "avatar_big": "https://example.com/avatar_big.jpg",
            "open_id": "ou-test123456789",
            "union_id": "on-test123456789",
            "email": "zhangsan@example.com",
            "enterprise_email": "zhangsan@company.com",
            "user_id": "user123456",
            "mobile": "+8613800138000",
            "tenant_key": "tenant_key_123",
            "employee_no": "E123456"
        });

        let user_info: Result<UserInfo, _> = serde_json::from_value(json_data);
        assert!(user_info.is_ok());

        let user_info = user_info.unwrap();
        assert_eq!(user_info.name, "张三");
        assert_eq!(user_info.en_name, "Zhang San");
        assert_eq!(user_info.open_id, "ou-test123456789");
        assert_eq!(user_info.union_id, "on-test123456789");
        assert_eq!(user_info.email, Some("zhangsan@example.com".to_string()));
        assert_eq!(user_info.mobile, Some("+8613800138000".to_string()));
        assert_eq!(user_info.tenant_key, "tenant_key_123");
        assert_eq!(user_info.employee_no, "E123456");
    }

    #[test]
    fn test_user_info_deserialization_with_optional_fields_none() {
        let json_data = json!({
            "name": "李四",
            "en_name": "Li Si",
            "avatar_url": "https://example.com/avatar2.jpg",
            "avatar_thumb": "https://example.com/avatar2_thumb.jpg",
            "avatar_middle": "https://example.com/avatar2_middle.jpg",
            "avatar_big": "https://example.com/avatar2_big.jpg",
            "open_id": "ou-test987654321",
            "union_id": "on-test987654321",
            "user_id": "user987654",
            "tenant_key": "tenant_key_456",
            "employee_no": "E987654"
        });

        let user_info: Result<UserInfo, _> = serde_json::from_value(json_data);
        assert!(user_info.is_ok());

        let user_info = user_info.unwrap();
        assert_eq!(user_info.name, "李四");
        assert_eq!(user_info.email, None);
        assert_eq!(user_info.enterprise_email, None);
        assert_eq!(user_info.mobile, None);
    }

    #[test]
    fn test_user_info_deserialization_invalid_data() {
        // 缺少必需字段的情况
        let json_data = json!({
            "name": "王五",
            // 缺少其他必需字段
        });

        let user_info: Result<UserInfo, _> = serde_json::from_value(json_data);
        assert!(user_info.is_err());
    }

    #[rstest]
    #[case("张三", "ou-123", "on-456", "tenant_789")]
    #[case("李四", "ou-abc", "on-def", "tenant_ghi")]
    #[case("王五", "ou-xyz", "on-uvw", "tenant_rst")]
    fn test_user_info_various_names(
        #[case] name: &str,
        #[case] open_id: &str,
        #[case] union_id: &str,
        #[case] tenant_key: &str,
    ) {
        let json_data = json!({
            "name": name,
            "en_name": format!("{} EN", name),
            "avatar_url": "https://example.com/avatar.jpg",
            "avatar_thumb": "https://example.com/avatar_thumb.jpg",
            "avatar_middle": "https://example.com/avatar_middle.jpg",
            "avatar_big": "https://example.com/avatar_big.jpg",
            "open_id": open_id,
            "union_id": union_id,
            "user_id": "user123",
            "tenant_key": tenant_key,
            "employee_no": "E123"
        });

        let user_info: UserInfo = serde_json::from_value(json_data).unwrap();
        assert_eq!(user_info.name, name);
        assert_eq!(user_info.open_id, open_id);
        assert_eq!(user_info.union_id, union_id);
        assert_eq!(user_info.tenant_key, tenant_key);
    }
}

#[cfg(test)]
mod api_tests {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_get_user_info_success() {
        let mock_server = MockServer::start().await;
        let service = create_test_service(mock_server.uri());

        let response_data = json!({
            "code": 0,
            "msg": "success",
            "data": {
                "name": "张三",
                "en_name": "Zhang San",
                "avatar_url": "https://example.com/avatar.jpg",
                "avatar_thumb": "https://example.com/avatar_thumb.jpg",
                "avatar_middle": "https://example.com/avatar_middle.jpg",
                "avatar_big": "https://example.com/avatar_big.jpg",
                "open_id": "ou-test123456789",
                "union_id": "on-test123456789",
                "email": "zhangsan@example.com",
                "enterprise_email": "zhangsan@company.com",
                "user_id": "user123456",
                "mobile": "+8613800138000",
                "tenant_key": "tenant_key_123",
                "employee_no": "E123456"
            }
        });

        Mock::given(method("GET"))
            .and(path("/open-apis/authen/v1/user_info"))
            .and(header("Authorization", "Bearer test_user_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_data))
            .mount(&mock_server)
            .await;

        let result = service.get("test_user_token").await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.code, 0);
        assert_eq!(response.msg, "success");
        assert!(response.data.is_some());

        let user_info = response.data.unwrap();
        assert_eq!(user_info.name, "张三");
        assert_eq!(user_info.open_id, "ou-test123456789");
        assert_eq!(user_info.email, Some("zhangsan@example.com".to_string()));
    }

    #[tokio::test]
    #[serial]
    async fn test_get_user_info_with_empty_token() {
        let mock_server = MockServer::start().await;
        let service = create_test_service(mock_server.uri());

        Mock::given(method("GET"))
            .and(path("/open-apis/authen/v1/user_info"))
            .respond_with(ResponseTemplate::new(401).set_body_json(json!({
                "code": 99991663,
                "msg": "invalid user access token"
            })))
            .mount(&mock_server)
            .await;

        let result = service.get("").await;
        // 期望请求仍然会发出，但会收到401错误
        assert!(result.is_ok()); // API调用成功，但业务逻辑失败
        let response = result.unwrap();
        assert_ne!(response.code, 0);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_user_info_network_error() {
        // 使用一个不存在的地址来模拟网络错误
        let service = create_test_service("http://nonexistent-server:12345".to_string());

        let result = service.get("test_token").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[serial]
    async fn test_get_user_info_server_error() {
        let mock_server = MockServer::start().await;
        let service = create_test_service(mock_server.uri());

        Mock::given(method("GET"))
            .and(path("/open-apis/authen/v1/user_info"))
            .respond_with(ResponseTemplate::new(500).set_body_json(json!({
                "error": "Internal Server Error"
            })))
            .mount(&mock_server)
            .await;

        let result = service.get("test_token").await;
        assert!(result.is_err());
    }

    #[rstest]
    #[case("valid_token_123")]
    #[case("another_valid_token_456")]
    #[case("special_token_with_symbols_!@#")]
    #[tokio::test]
    #[serial]
    async fn test_get_user_info_with_various_tokens(#[case] token: &str) {
        let mock_server = MockServer::start().await;
        let service = create_test_service(mock_server.uri());

        let user_info = create_test_user_info();
        let response_data = json!({
            "code": 0,
            "msg": "success",
            "data": user_info
        });

        Mock::given(method("GET"))
            .and(path("/open-apis/authen/v1/user_info"))
            .and(header("Authorization", format!("Bearer {}", token)))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_data))
            .mount(&mock_server)
            .await;

        let result = service.get(token).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.code, 0);
        assert!(response.data.is_some());
    }
}

#[cfg(test)]
mod property_based_tests {
    use super::*;

    proptest! {
        #[test]
        fn test_user_info_roundtrip_serialization(
            name in "[\\p{Han}\\p{Latin}]{1,50}",
            en_name in "[a-zA-Z ]{1,50}",
            open_id in "ou-[a-zA-Z0-9]{10,30}",
            union_id in "on-[a-zA-Z0-9]{10,30}",
            user_id in "[a-zA-Z0-9]{5,20}",
            tenant_key in "[a-zA-Z0-9]{5,20}",
            employee_no in "[A-Z][0-9]{3,8}",
        ) {
            let original_user_info = UserInfo {
                name: name.clone(),
                en_name: en_name.clone(),
                avatar_url: "https://example.com/avatar.jpg".to_string(),
                avatar_thumb: "https://example.com/avatar_thumb.jpg".to_string(),
                avatar_middle: "https://example.com/avatar_middle.jpg".to_string(),
                avatar_big: "https://example.com/avatar_big.jpg".to_string(),
                open_id: open_id.clone(),
                union_id: union_id.clone(),
                email: Some("test@example.com".to_string()),
                enterprise_email: Some("test@company.com".to_string()),
                user_id: user_id.clone(),
                mobile: Some("+8613800138000".to_string()),
                tenant_key: tenant_key.clone(),
                employee_no: employee_no.clone(),
            };

            // 序列化到JSON
            let json_value = serde_json::to_value(&original_user_info)?;
            // 从JSON反序列化
            let deserialized_user_info: UserInfo = serde_json::from_value(json_value)?;

            // 验证往返序列化保持数据一致性
            assert_eq!(original_user_info.name, deserialized_user_info.name);
            assert_eq!(original_user_info.en_name, deserialized_user_info.en_name);
            assert_eq!(original_user_info.open_id, deserialized_user_info.open_id);
            assert_eq!(original_user_info.union_id, deserialized_user_info.union_id);
            assert_eq!(original_user_info.user_id, deserialized_user_info.user_id);
            assert_eq!(original_user_info.tenant_key, deserialized_user_info.tenant_key);
            assert_eq!(original_user_info.employee_no, deserialized_user_info.employee_no);
        }
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[tokio::test]
    async fn test_user_info_deserialization_performance() {
        let json_data = json!({
            "name": "性能测试用户",
            "en_name": "Performance Test User",
            "avatar_url": "https://example.com/avatar.jpg",
            "avatar_thumb": "https://example.com/avatar_thumb.jpg",
            "avatar_middle": "https://example.com/avatar_middle.jpg",
            "avatar_big": "https://example.com/avatar_big.jpg",
            "open_id": "ou-performance123456789",
            "union_id": "on-performance123456789",
            "email": "perf@example.com",
            "enterprise_email": "perf@company.com",
            "user_id": "perfuser123456",
            "mobile": "+8613800138000",
            "tenant_key": "perf_tenant_123",
            "employee_no": "PERF123456"
        });

        let start_time = Instant::now();
        let iterations = 1000;

        for _ in 0..iterations {
            let _user_info: UserInfo = serde_json::from_value(json_data.clone()).unwrap();
        }

        let elapsed = start_time.elapsed();
        let per_operation = elapsed / iterations;

        println!(
            "反序列化性能测试: {} 次操作，总时间 {:?}，平均每次 {:?}",
            iterations, elapsed, per_operation
        );

        // 确保反序列化性能合理 (每次操作应该在1ms以内)
        assert!(per_operation < Duration::from_millis(1));
    }

    #[tokio::test]
    #[serial]
    async fn test_multiple_concurrent_requests() {
        let mock_server = MockServer::start().await;
        let service = Arc::new(create_test_service(mock_server.uri()));

        let response_data = json!({
            "code": 0,
            "msg": "success",
            "data": create_test_user_info()
        });

        Mock::given(method("GET"))
            .and(path("/open-apis/authen/v1/user_info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_data))
            .mount(&mock_server)
            .await;

        let start_time = Instant::now();
        let concurrent_requests = 10;

        let tasks = (0..concurrent_requests)
            .map(|i| {
                let service = service.clone();
                tokio::spawn(async move {
                    let token = format!("test_token_{}", i);
                    service.get(token).await
                })
            })
            .collect::<Vec<_>>();

        let results: Vec<_> = futures::future::try_join_all(tasks).await.unwrap();
        let elapsed = start_time.elapsed();

        // 验证所有请求都成功
        for result in results {
            assert!(result.is_ok());
            let response = result.unwrap();
            assert_eq!(response.code, 0);
        }

        println!(
            "并发性能测试: {} 个并发请求，总时间 {:?}，平均每个 {:?}",
            concurrent_requests,
            elapsed,
            elapsed / concurrent_requests
        );

        // 并发请求应该能够在合理时间内完成
        assert!(elapsed < Duration::from_secs(5));
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[tokio::test]
    #[serial]
    async fn test_malformed_json_response() {
        let mock_server = MockServer::start().await;
        let service = create_test_service(mock_server.uri());

        Mock::given(method("GET"))
            .and(path("/open-apis/authen/v1/user_info"))
            .respond_with(ResponseTemplate::new(200).set_body_string("invalid json"))
            .mount(&mock_server)
            .await;

        let result = service.get("test_token").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[serial]
    async fn test_timeout_handling() {
        let mock_server = MockServer::start().await;
        let service = create_test_service(mock_server.uri());

        Mock::given(method("GET"))
            .and(path("/open-apis/authen/v1/user_info"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_delay(Duration::from_secs(15)) // 超过默认超时时间
                    .set_body_json(json!({
                        "code": 0,
                        "msg": "success",
                        "data": create_test_user_info()
                    })),
            )
            .mount(&mock_server)
            .await;

        let result = service.get("test_token").await;
        assert!(result.is_err());
    }

    #[rstest]
    #[case(400, "Bad Request")]
    #[case(401, "Unauthorized")]
    #[case(403, "Forbidden")]
    #[case(404, "Not Found")]
    #[case(429, "Too Many Requests")]
    #[case(500, "Internal Server Error")]
    #[case(502, "Bad Gateway")]
    #[case(503, "Service Unavailable")]
    #[tokio::test]
    #[serial]
    async fn test_various_http_error_codes(#[case] status_code: u16, #[case] _reason: &str) {
        let mock_server = MockServer::start().await;
        let service = create_test_service(mock_server.uri());

        Mock::given(method("GET"))
            .and(path("/open-apis/authen/v1/user_info"))
            .respond_with(
                ResponseTemplate::new(status_code).set_body_json(json!({
                    "code": status_code,
                    "msg": format!("HTTP Error {}", status_code)
                })),
            )
            .mount(&mock_server)
            .await;

        let result = service.get("test_token").await;
        if status_code >= 400 {
            assert!(result.is_err());
        }
    }
}

#[cfg(test)]
mod boundary_tests {
    use super::*;

    #[tokio::test]
    async fn test_user_info_with_extreme_string_lengths() {
        // 测试极长字符串
        let very_long_string = "a".repeat(1000);
        let json_data = json!({
            "name": very_long_string,
            "en_name": very_long_string,
            "avatar_url": format!("https://example.com/{}.jpg", very_long_string),
            "avatar_thumb": format!("https://example.com/{}_thumb.jpg", very_long_string),
            "avatar_middle": format!("https://example.com/{}_middle.jpg", very_long_string),
            "avatar_big": format!("https://example.com/{}_big.jpg", very_long_string),
            "open_id": format!("ou-{}", very_long_string),
            "union_id": format!("on-{}", very_long_string),
            "email": format!("{}@example.com", very_long_string),
            "enterprise_email": format!("{}@company.com", very_long_string),
            "user_id": very_long_string,
            "mobile": format!("+86{}", "1".repeat(20)),
            "tenant_key": very_long_string,
            "employee_no": very_long_string
        });

        let user_info_result: Result<UserInfo, _> = serde_json::from_value(json_data);
        // 应该能够处理极长字符串而不panic
        assert!(user_info_result.is_ok());
    }

    #[tokio::test]
    async fn test_user_info_with_special_characters() {
        // 测试特殊字符
        let special_chars = "🎉🚀中文العربية🌟";
        let json_data = json!({
            "name": special_chars,
            "en_name": "Special User",
            "avatar_url": "https://example.com/special.jpg",
            "avatar_thumb": "https://example.com/special_thumb.jpg",
            "avatar_middle": "https://example.com/special_middle.jpg",
            "avatar_big": "https://example.com/special_big.jpg",
            "open_id": "ou-special123",
            "union_id": "on-special123",
            "user_id": "special123",
            "tenant_key": "special_tenant",
            "employee_no": "SP123"
        });

        let user_info: UserInfo = serde_json::from_value(json_data).unwrap();
        assert_eq!(user_info.name, special_chars);
    }

    #[tokio::test]
    async fn test_user_info_with_empty_strings() {
        // 测试空字符串（如果模型允许的话）
        let json_data = json!({
            "name": "",
            "en_name": "",
            "avatar_url": "",
            "avatar_thumb": "",
            "avatar_middle": "",
            "avatar_big": "",
            "open_id": "",
            "union_id": "",
            "user_id": "",
            "tenant_key": "",
            "employee_no": ""
        });

        let user_info_result: Result<UserInfo, _> = serde_json::from_value(json_data);
        assert!(user_info_result.is_ok());
    }
}