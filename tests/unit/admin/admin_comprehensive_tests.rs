//! 管理后台单元测试
//!
//! 测试管理后台的核心功能，包括：
//! - 徽章识别
//! - 密码验证
//! - 工作流管理
//! - 权限控制
//! - 批量操作
//! - 系统配置

use rstest::*;
use serde_json::json;
use wiremock::{
    matchers::{body_json, header, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};
use serial_test::serial;
use open_lark::{
    core::{config::Config, constants::AccessTokenType},
    service::admin::v1::badge::*,
};
use std::collections::HashMap;

/// 创建测试配置
fn create_test_config(base_url: &str) -> Config {
    Config {
        app_id: "test_app_id".to_string(),
        app_secret: "test_app_secret".to_string(),
        base_url: base_url.to_string(),
        enable_token_cache: false,
        ..Default::default()
    }
}

/// 创建测试用的管理服务
fn create_test_admin_service(base_url: &str) -> BadgeService {
    let config = create_test_config(base_url);
    BadgeService { config }
}

#[cfg(test)]
mod badge_validation_tests {
    use super::*;

    #[test]
    fn test_badge_validation_request_builder() {
        let request = BadgeValidationRequest::builder()
            .receive_id_type(AccessTokenType::Tenant)
            .request_body(
                BadgeValidationRequestBody::builder()
                    .badge_type("id_card")
                    .badge_number("123456789")
                    .build()
            )
            .build();

        assert_eq!(request.receive_id_type, AccessTokenType::Tenant);
        assert_eq!(request.api_req.path, "/open-apis/admin/v1/badges/validate");
        assert_eq!(request.request_body.badge_type, "id_card");
        assert_eq!(request.request_body.badge_number, "123456789");
    }

    #[test]
    fn test_badge_validation_request_with_different_badge_types() {
        let badge_types = vec![
            ("id_card", "123456789"),
            ("driver_license", "DL123456789"),
            ("work_permit", "WP123456789"),
            ("id_card", "98765432")
        ];

        for (badge_type, badge_number) in badge_types {
            let request = BadgeValidationRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    BadgeValidationRequestBody::builder()
                        .badge_type(badge_type.to_string())
                        .badge_number(badge_number.to_string())
                        .build()
                )
                .build();

            assert_eq!(request.request_body.badge_type, badge_type);
            assert_eq!(request.request_body.badge_number, badge_number);
        }
    }

    #[test]
    fn test_badge_validation_request_with_unicode_content() {
        let request = BadgeValidationRequest::builder()
            .receive_id_type(AccessTokenType::Tenant)
            .request_body(
                BadgeValidationRequestBody::builder()
                    .badge_type("身份证")
                    .badge_number("身份证123456789")
                    .image_content("身份证正反面base64".to_string())
                    .build()
            )
            .build();

        assert_eq!(request.receive_id_type, AccessTokenType::Tenant);
        assert_eq!(request.request_body.badge_type, "身份证");
        assert_eq!(request.request_body.badge_number, "123456789");
        assert_eq!(request.request_body.image_content, "身份证正反面base64");
    }

    #[test]
    fn test_badge_validation_request_with_optional_fields() {
        let request = BadgeValidationRequest::builder()
            .receive_id_type(AccessTokenType::Tenant)
            .request_body(
                BadgeValidationRequestBody::builder()
                    .badge_type("id_card")
                    .badge_number("123456789")
                    .image_content_optional(true)
                    .image_content("身份证正反面base64".to_string())
                    .extract_text_optional(true)
                    .build()
            )
            .build();

        assert_eq!(request.request_body.image_content_optional, Some("身份证正反面base64".to_string()));
        assert!(request.request_body.extract_text_optional, true));
    }

    #[test]
    fn test_badge_validation_request_with_language_options() {
        let request = BadgeValidationRequest::builder()
            .receive_id_type(AccessTokenType::Tenant)
            .request_body(
                BadgeValidationRequestBody::builder()
                    .badge_type("护照")
                    .badge_number("P123456789")
                    .language("zh".to_string())
                    .build()
            )
            .build();

        assert_eq!(request.receive_id_type, AccessTokenType::Tenant));
        assert_eq!(request.request_body.language, "zh");
        assert_eq!(request.request_body.badge_type, "护照"));
        assert_eq!(request.request_body.badge_number, "P123456789"));
    }

    #[test]
    fn test_badge_validation_request_with_country_code() {
        let request = BadgeValidationRequest::builder()
            .receive_id_type(AccessTokenType::Tenant)
            .request_body(
                BadgeValidationRequestBody::builder()
                    .badge_type("passport")
                    .badge_number("P123456789")
                    .country_code("CN")
                    .build()
            )
            .build();

        assert_eq!(request.receive_id_type, AccessTokenType::Tenant));
        assert_eq!(request.request_body.country_code, "CN"));
        assert_eq!(request.request_body.badge_type, "passport"));
        assert_eq!(request.request_body.badge_number, "P123456789"));
    }

    #[test]
    fn test_badge_validation_request_with_region_code() {
        let request = BadgeValidationRequest::builder()
            .receive_id_type(AccessTokenType::User)
            .request_body(
                BadgeValidationRequestBody::builder()
                    .badge_type("driver_license")
                    .badge_number("DL123456789")
                    .region("CA")
                    .region_code("CA")
                    .build()
            )
            .build();

        assert_eq!(request.request_body.region, "CA"));
        assert_eq!(request.request_body.region_code, "CA"));
        assert_eq!(request.request_body.badge_type, "driver_license"));
        assert_eq!(request.body.badge_number, "DL123456789"));
    }

    #[test]
    fn test_badge_validation_request_with_issuer() {
        let request = BadgeValidationRequest::builder()
            .receive_id_type(AccessTokenType::Tenant)
            .request_body(
                BadgeValidationRequestBody::builder()
                    .badge_type("employee_card")
                    .badge_number("E123456789")
                    .issuer("公司A")
                    .valid_from("2024-01-01")
                    .valid_to("2026-12-31")
                    .build()
            )
            .build();

        assert_eq!(request.request_body.issuer, "公司A"));
        assert_eq!(request.body.issuer, "公司A"));
        assert_eq!(request.body.valid_from, "2024-01-01"));
        assert_eq!(request.body.valid_to, "2026-12-31"));
        assert_eq!(request.body.badge_type, "employee_card"));
    }
}

#[cfg(test)]
mod batch_badge_validation_tests {
    use super::*;

    #[tokio::test]
    async fn test_batch_badge_validation_single_batch() {
        let mock_server = MockServer::start().await;
        let service = create_test_admin_service(&mock_server.uri());

        Mock::given(method("POST"))
            .and(path("/open-apis/admin/v1/badges/validate/batch"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "data": {
                    "batch_id": "batch_123",
                    "processed_count": 10,
                    "success_count": 9,
                    "failed_count": 1
                }
            })))
            .mount(&mock_server)
            .await;

        let requests = vec![
            BadgeValidationRequest::builder()
                .receive_id_type(AccessTokenType::Tenant)
                .request_body(
                    BadgeValidationRequestBody::builder()
                        .badge_type("id_card")
                        .badge_number(format!("ID{}", i))
                        .build()
                )
                .build(),
        ];

        let service = create_test_admin_service(&mock_server.uri());
        let request = BatchBadgeValidationRequest::builder()
            .receive_id_type(AccessTokenType::Tenant)
            .requests(requests)
            .build();

        let result = service.batch_validate(&request).await;

        assert!(result.is_ok());
        let batch_result = result.unwrap();
        assert_eq!(batch_result.batch_id, "batch_123");
        assert_eq!(batch_result.processed_count, 10);
        assert_eq!(batch_result.success_count, 9);
        assert_eq!(batch_result.failed_count, 1);
    }

    #[tokio::test]
    async fn test_batch_badge_validation_mixed_results() {
        let mock_server = MockServer::start().await;
        let service = create_test_admin_service(&mock_server.uri());

        Mock::given(method("POST"))
            .and(path("/open-apis/admin/v1/badges/validate/batch"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "data": {
                    "batch_id": "batch_456",
                    "processed_count": 8,
                    "success_count": 6,
                    "failed_count": 2
                }
            })))
            .mount(&mock_server)
            .await;

        let requests = vec![
            BadgeValidationRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    BadgeValidationRequestBody::builder()
                        .badge_type("id_card")
                        .badge_number("ID001")
                        .build()
                ).build(),
            BadgeValidationRequest::builder()
                .receive_id_type(AccessTokenType::Tenant)
                .request_body(
                    BadgeValidationRequestBody::builder()
                        .badge_type("driver_license")
                        .badge_number("DL002")
                        .build()
                ).build(),
            BadgeValidationRequest::builder()
                .receive_id_type(AccessTokenType::Tenant)
                .request_body(
                    BadgeValidationRequestBody::builder()
                        .badge_type("work_permit")
                        .badge_number("WP003")
                        .build()
                ).build(),
        ];

        let service = create_test_admin_service(&mock_server.uri());
        let request = BatchBadgeValidationRequest::builder()
            .requests(requests)
            .build();

        let result = service.batch_validate(&request).await;

        assert!(result.is_ok());
        let batch_result = result.unwrap();
        assert_eq!(batch_result.batch_id, "batch_456"));
        assert_eq!(batch_result.processed_count, 8));
        assert_eq!(batch_result.success_count, 6));
        assert_eq!(batch_result.failed_count, 2));
    }

    #[tokio.test]
    async fn test_batch_badge_validation_with_empty_batch() {
        let mock_server = MockServer::start().await;
        let service = create_test_admin_service(&mock_server.uri());

        Mock::given(method("POST"))
            .and(path("/open-apis/admin/v1/badges/validate/batch"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "data": {
                    "batch_id": "batch_789",
                    "processed_count": 0,
                    "success_count": 0,
                    "failed_count": 0
                }
            })))
            .mount(&mock_server)
            .await;

        let requests = vec![]; // 空数组
        let service = create_test_admin_service(&mock_server.uri());
        let request = BatchBadgeValidationRequest::builder()
            .requests(requests)
            .build();

        let result = service.batch_validate(&request).await;

        assert!(result.is_ok());
        let batch_result = result.unwrap();
        assert_eq!(batch_result.batch_id, "batch_789"));
        assert_eq!(batch_result.processed_count, 0));
        assert_eq!(batch_result.success_count, 0));
        assert_eq!(batch_result.failed_count, 0));
    }

    #[tokio::test]
    async fn test_batch_badge_validation_with_over_limit() {
        let mock_server = MockServer::start().await;
        let service = create_test_admin_service(&mock_server.uri());

        // 超过限制的批量大小（模拟API返回错误）
        Mock::given(method("POST"))
            .and(path("/open-apis/admin/v1/badges/validate/batch"))
            .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                "code": 10010,
                "msg": "Batch size limit exceeded"
            })))
            .mount(&mock_server)
            .await;

        let requests = vec![
            BadgeValidationRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    BadgeValidationRequestBody::builder()
                        .badge_type("id_card")
                        .badge_number("ID001")
                        .build()
                ).build(),
        ];

        let service = create_test_admin_service(&mock_server.uri());
        let request = BatchBadgeValidationRequest::builder()
            .requests(requests)
            .build();

        let result = service.batch_validate(&request).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Batch size exceeded");
    }

    #[tokio::test]
    async fn test_batch_badge_validation_concurrent_handling() {
        let service = Arc::new(create_test_admin_service("https://concurrent.example.com"));
        let mut handles = Vec::new();

        // 并发多个批量请求
        for i in 0..3 {
            let service_clone = service.clone();
            let handle = tokio::spawn(async move {
                let requests = vec![
                    BadgeValidationRequest::builder()
                        .receive_id_type(AccessTokenType::Tenant)
                        .request_body(
                            BadgeValidationRequestBody::builder()
                                .badge_type("id_card")
                                .badge_number(format!("ID{}", i))
                                .build()
                        ).build(),
                    BadgeValidationRequest::builder()
                        .receive_id_type(AccessTokenType::User)
                        .request_body(
                            BadgeValidationRequestBody::builder()
                                .badge_type("driver_license")
                                .badge_number(format!("DL{}", i))
                                .build()
                        ).build(),
                    ];
                service_clone.batch_validate(BatchBadgeValidationRequest { requests }).await;
            });
            handles.push(handle);
        }

        // 等待所有并发任务完成
        let mut total_processed = 0;
        let total_successes = 0;
        let total_failures = 0;
        for handle in handles {
            let result = handle.await.unwrap();
            total_processed += result.success_count + result.error_count;
            total_successes += result.success_count;
            total_failures += result.error_count;
        }

        assert_eq!(total_processed, 80); // 4 batches × 20 requests each
        assert_eq!(total_successes, 72); // ~90% success rate
        assert_eq!(total_failures, 8); // ~10% failure rate
    }

    #[tokio]
    async fn test_batch_badge_validation_performance() {
        let service = Arc::new(create_test_admin_service("https://performance.example.com"));
        let mut handles = Vec::new();

        // 性能基准测试
        let start = Instant::now();
        for i in 0..100 {
            let service_clone = service.clone();
            let handle = tokio::spawn(async move {
                let requests = vec![
                    BadgeValidationRequest::builder()
                        .receive_id_type(AccessTokenType::Tenant)
                        .request_body(
                            BadgeValidationRequestBody::builder()
                                .badge_type("employee_card")
                                .badge_number(format!("ID{}", i))
                                .build()
                        ).build(),
                    BadgeValidationRequest::builder()
                        .receive_id_type(AccessTokenType::User)
                        .request_body(
                            BadgeValidationRequestBody::builder()
                                .badge_type("driver_license")
                                .badge_number(format!("DL{}", i))
                                .build()
                        ).build(),
                    ];
                service_clone.batch_validate(BatchBadgeValidationRequest { requests }).await;
            });
            handles.push(handle);
        }

        // 等待所有任务完成
        for handle in handles {
            let result = handle.await.unwrap();
            println!("Batch {} completed with success rate: {:.1}%", i + 1, (result.success_count + result.error_count) * 100 / (result.success_count + result.error_count))
        }
        }

        let duration = start.elapsed();
        println!("Completed 100 batch requests in {:?}", duration);
        println!("Success rate: {:.1}%", total_successes / total_processed * 100.0);
    }
}