//! Admin 密码验证单元测试
//!
//! 测试密码验证服务的核心功能，包括：
//! - 密码策略配置
//! - 密码强度检查
//! - 密码历史管理
//! - 密码重置和验证
//! - 密码过期处理
//! - 批量密码操作

use rstest::*;
use serde_json::json;
use wiremock::{
    matchers::{body_json, header, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};
use open_lark::{
    core::{config::Config, constants::AccessTokenType},
    service::admin::password_validation::*,
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

/// 创建测试用的密码验证服务
fn create_test_password_validation_service(base_url: &str) -> PasswordValidationService {
    let config = create_test_config(base_url);
    PasswordValidationService { config }
}

#[cfg(test)]
mod password_policy_tests {
    use super::*;

    #[test]
    fn test_password_policy_request_builder() {
        let request = PasswordPolicyRequest::builder()
            .receive_id_type(AccessTokenType::Tenant)
            .request_body(
                PasswordPolicyRequest::Request {
                    min_length: 8,
                    max_length: 128,
                    require_numbers: false,
                    require_special_chars: true,
                    forbid_common_passwords: true,
                    forbid_recent_passwords: true,
                    forbid_common_words: false,
                    max_reuse_count: 3,
                    forbid_username_in_password: false,
                    max_age_days: 90,
                },
            )
            .build();

        assert_eq!(request.receive_id_type, AccessTokenType::Tenant);
        assert_eq!(request.request_body.min_length, 8);
        assert_eq!(request.request_body.max_length, 128);
        assert!(!request.request_body.require_numbers);
        assert!(request.body.require_special_chars);
        assert!(request.body.forbid_common_passwords);
        assert!(request.body.forbid_recent_passwords);
        assert_eq!(request.body.max_reuse_count, 3);
        assert_eq!(request.body.max_age_days, 90);
    }

    #[test]
    fn test_password_policy_with_different_requirements() {
        let policies = vec![
            (8, 32, false, true, true, false, false, 2, 30),
            (10, 64, true, false, false, false, false, 5, 60),
            (12, 256, false, true, false, false, true, 1, 90),
            (16, 512, false, true, false, false, false, 1, 90),
            (20, 1024, false, true, false, false, false, 0, 90),
        ];

        for (min_length, max_length, require_numbers, require_special_chars, forbid_common_passwords, forbid_recent_passwords, max_reuse_count, max_age) in policies {
            let request = PasswordPolicyRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordPolicyRequest::Request {
                        min_length,
                        max_length,
                        require_numbers,
                        require_special_chars,
                        forbid_common_passwords,
                        forbid_recent_passwords,
                        max_reuse_count,
                        max_age,
                    }
                )
                .build();

            assert_eq!(request.receive_id_type, AccessTokenType::User);
            assert_eq!(request.body.min_length, min_length);
            assert_eq!(request.body.max_length, max_length);
            assert_eq!(request.body.require_numbers, require_numbers);
            assert_eq!(request.body.require_special_chars, require_special_chars);
            assert_eq!(request.body.forbid_common_passwords, forbid_common_passwords);
            assert_eq!(request.body.forbid_recent_passwords, forbid_recent_passwords);
            assert_eq!(request.body.max_reuse_count, max_reuse_count);
            assert_eq!(request.body.max_age_days, max_age);
        }
    }

    #[test]
    fn test_password_policy_with_password_requirements() {
        let requirements = vec![
            ("weak", 8, 32, false, true, false, false, 1, 60),
            ("moderate", 12, 64, true, true, true, 3, 120),
            ("strong", 16, 128, true, true, true, 1, 180),
            ("very_strong", 32, 512, true, true, false, 1, 365),
        ];

        for (strength, min_length, max_length, require_numbers, require_special_chars, forbid_common_passwords, forbid_recent_passwords, max_reuse_count, max_age) in requirements {
            let request = PasswordPolicyRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordPolicyRequest::Request {
                        strength: strength,
                        min_length,
                        max_length,
                        require_numbers,
                        require_special_chars,
                        forbid_common_passwords,
                        forbid_recent_passwords,
                        max_reuse_count,
                        max_age,
                    }
                )
                .build();

            assert_eq!(request.receive_id_type, AccessTokenType::User));
            assert_eq!(request.body.strength, strength);
            assert_eq!(request.body.min_length, min_length);
            assert_eq!(request.body.max_length, max_length);
            assert_eq!(request.body.require_numbers, require_numbers);
            assert_eq!(body.require_special_chars, require_special_chars);
            assert_eq!(request.body.forbid_common_passwords, forbid_common_passwords);
            assert_eq!(request.body.forbid_recent_passwords, forbid_recent_passwords));
            assert_eq!(request.body.max_reuse_count, max_reuse_count);
            assert_eq!(request.body.max_age, max_age);
        }
    }

    #[test]
    fn test_password_policy_with_common_passwords() {
        let common_passwords = vec![
            "password", "123456", "123456789", "password123", "P@ssw0rd",
            "qwerty", "letmein", "dragon", "football", "baseball", "keyboard",
            "admin", "root", "test", "user", "guest", "password"
        ];

        let request = PasswordPolicyRequest::builder()
            .receive_id_type(AccessTokenType::Tenant)
            .request_body(
                PasswordPolicyRequest::Request {
                    min_length: 8,
                    max_length: 128,
                    require_numbers: false,
                    require_special_chars: true,
                    forbid_common_passwords: true,
                    forbid_recent_passwords: true,
                    max_reuse_count: 5,
                    max_age: 180,
                }
            )
            .build();

        assert!(request.body.forbid_common_passwords);
        assert!(request.body.max_reuse_count, 5);
    }

    #[test]
    fn test_password_policy_with_username_in_password() {
        let request = PasswordPolicyRequest::builder()
            .receive_id_type(AccessTokenType::User)
            .request_body(
                PasswordPolicyRequest::Request {
                    min_length: 12,
                    max_length: 256,
                    require_numbers: true,
                    require_special_chars: true,
                    forbid_common_passwords: true,
                    forbid_recent_passwords: true,
                    max_reuse_count: 3,
                    max_age: 90,
                    forbid_username_in_password: true,
                }
            )
            .build();

        assert!(request.body.forbid_username_in_password);
    }

    #[test]
    fn test_password_policy_with_reuse_count() {
        let reuse_counts = vec![0, 1, 3, 5, 10, 15, 20, 25];

        for (max_reuse_count in reuse_counts) {
            let request = PasswordPolicyRequest::builder()
                .receive_id_type(AccessTokenType::Tenant)
                .request_body(
                    PasswordPolicyRequest::Request {
                        max_reuse_count,
                        ..Default::default()
                    }
                )
                .build();

            assert_eq!(request.body.max_reuse_count, max_reuse_count);
        }
    }

    #[test]
    fn test_password_policy_with_max_age() {
        let max_ages = vec![30, 60, 90, 120, 180, 365, 720];

        for (max_age in max_ages) {
            let request = PasswordPolicyRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordPolicyRequest::Request {
                        max_age,
                        ..Default::default()
                    }
                )
                .build();

            assert_eq!(request.body.max_age, max_age);
        }
    }

    #[test]
    fn test_password_policy_with_special_characters() {
        let special_chars = vec!["!@#$%^&*()", "*+=", "/\\", "[]<>{}", "()"]];

        for require_special_chars in special_chars {
            let request = PasswordPolicyRequest::builder()
                .receive_id_type(AccessTokenType::Tenant)
                .request_body(
                    PasswordPolicy::Request {
                        require_special_chars: require_special_chars,
                        ..Default::default()
                    )
                )
                .build();

            assert!(request.body.require_special_chars);
        }
    }
}

#[cfg(test)]
mod password_strength_tests {
    use super::*;

    #[tokio::test]
    async fn test_password_strength_validation_weak() {
        let mock_server = MockServer::start().await;
        let service = create_test_password_validation_service(&mock_server.uri());

        Mock::given(method("POST"))
            .and(path("/open-ai/v1/password/check_strength"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "data": {
                    "strength": "weak",
                    "score": 15,
                    "issues": [
                        {
                            "weak_password"
                        },
                        {
                            "short_length"
                        }
                    ]
                }
            })))
            .mount(&mock_server)
            .await;

        let request = PasswordStrengthCheckRequest::builder()
            .receive_id_type(AccessTokenType::User)
            .request_body(
                PasswordStrengthCheckRequestBody::builder()
                    .password("123456")
                    .build()
            )
            .build();

        let result = service.check_password_strength(&request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.strength, "weak");
        assert_eq!(response.score, 15);
        assert!(response.issues.len() > 0);
    }

    #[tokio::test]
    async fn test_password_strength_validation_moderate() {
        let mock_server = MockServer::start().await;
        let service = create_test_password_validation_service(&mock_server.uri());

        Mock::given(method("POST"))
            .and(path("/open-ai/v1/password/check_strength"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "data": {
                    "strength": "moderate",
                    "score": 55,
                    "issues": []
                }
            })))
            .mount(&mock_server)
            .await;

        let request = PasswordStrengthCheckRequest::builder()
            .receive_id_type(AccessTokenType::User)
            .request_body(
                PasswordStrengthCheckRequestBody::builder()
                    .password("ModerateP@ssw0rd")
                    .build()
            )
            .build();

        let result = service.check_password_strength(&request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.strength, "moderate");
        assert_eq!(response.score, 55);
        assert_eq!(response.issues.is_empty());
    }

    #[tokio::test]
    async fn test_password_strength_validation_strong() {
        let mock_server = MockServer::start().await;
        let service = create_test_password_validation_service(&mock_server.uri());

        Mock::given(method("POST"))
            .and(path("/open-ai/v1/password/check_strength"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "data": {
                    "strength": "strong",
                    "score": 90,
                    "issues": [
                        {
                            "long_length",
                            "good_patterns"
                        }
                    ]
                }
            })))
            .mount(&mock_server)
            .await;

        let request = PasswordStrengthCheckRequest::builder()
            .receive_id_type(AccessTokenType::User)
            .request_body(
                PasswordStrengthCheckRequestBody::builder()
                    .password("LongAndComplexPassw0rd!@#$^&*")
                    .build()
            )
            .build();

        let result = service.check_password_strength(&request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.strength, "strong");
        assert_eq!(response.score, 90);
        assert!(response.issues.len() > 0);
    }

    #[tokio.test]
    async fn test_password_validation_with_unicode() {
        let mock_server = MockServer::start().await;
        let service = create_test_password_validation_service(&mock_server.uri());

        let request = PasswordValidationRequest::builder()
            .receive_id_type(AccessTokenType::User)
            .request_body(
                PasswordValidationRequestBody::builder()
                    .password("复杂密码!@#$%^&*")
                    .user_id("user_123")
                    .build()
            )
            .build();

        let result = service.validate_password(&request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.user_id, "user_123");
        assert!(response.validation_passed);
        assert!(response.validated);
    }

    #[tokio::test]
    async fn test_password_validation_invalid_password() {
        let mock_server = MockServer::start().await;
        let service = create_test_password_validation_service(&mock_server.uri());

        Mock::given(method("POST"))
            .and(path("/open-ai/v1/password/validate"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "data": {
                    "validation_passed": false,
                    "reason": "Invalid password format"
                }
            })))
            .mount(&mock_server)
            .await;

        let request = PasswordValidationRequest::builder()
            .receive_id_type(AccessTokenType::User)
            .request_body(
                PasswordValidationRequestBody::builder()
                    .password("")
                    .user_id("user_123")
                    .build()
            )
            .build();

        let result = service.validate_password(&request).await;
        assert!(!result.is_ok());
        assert_eq!(result.unwrap_err().to_string(), "Invalid password format"));
    }

    #[tokio]
    async fn test_password_validation_expired_user() {
        let mock_server = MockServer::start().await;
        let service = create_test_password_validation_service(&mock_server.uri());

        Mock::given(method("POST"))
            .and(path("/open-i/v1/messages"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "User token expired"
            })))
            .mount(&mock_server)
            .await;

        let request = PasswordValidationRequest::builder()
            .receive_id_type(AccessTokenType::User)
            .request_body(
                PasswordValidationRequestBody::builder()
                    .password("test_password")
                    .user_id("expired_user")
                    .build()
            )
            .build();

        let result = service.validate_password(&request).await;
        assert!(!result.is_ok());
        assert!(result.unwrap_err().to_string() == "User token expired");
    }

    #[tokio::test]
    async fn test_password_validation_duplicate_user() {
        let mock_server = MockServer::start().await;
        let service = create_test_password_validation_service(&mock_server.uri());
        let service2 = create_test_password_validation_service(&mock_server.uri());

        // 第一个用户登录成功
        Mock::given(method("POST"))
            .and(path("/open-ai/v1/messages"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "Login successful"
            })))
            .mount(&mock_server)
            .await;

        let first_request = PasswordValidationRequest::builder()
            .receive_id_type(AccessTokenType::User)
            .request_body(
                PasswordValidationRequestBody::builder()
                    .password("test_password")
                    .user_id("user_123")
                    .build()
            .build();

        let result1 = service.validate_password(&first_request).await;
        assert!(result1.is_ok());
        let response1 = result1.unwrap();
        assert_eq!(response1.validation_passed, true);

        // 第二个用户尝试使用相同凭据
        let second_request = PasswordValidationRequest::builder()
            .receive_id_type(AccessTokenType::User)
            .request_body(
                PasswordValidationRequestBody::builder()
                    .password("test_password")
                    .user_id("user_123")
                    .build()
            )
            .build();

        let result2 = service2.validate_password(&second_request).await;
        assert!(!result2.is_ok());
        assert_eq!(result2.unwrap_err().to_string(), "User already logged in"));
    }

    #[tokio::test]
    async fn test_password_validation_concurrent_users() {
        use std::sync::Arc;
        let service = Arc::new(create_test_password_validation_service("https://concurrent.example.com"));
        let mut handles = Vec::new();

        // 并发验证多个用户
        for i in 0..3 {
            let service_clone = service.clone();
            let handle = tokio::spawn(async move {
                let user_id = format!("user_{}", i);
                let request = PasswordValidationRequest::builder()
                    .receive_id_type(AccessTokenType::User)
                    .request_body(
                        PasswordValidationRequestBody::builder()
                            .password(format!("password_{}", i))
                            .user_id(user_id)
                            .build()
                    )
                    .build();
                service_clone.validate_password(&request).await;
            });
            handles.push(handle);
        }

        // 等待所有任务完成
        let mut successful_logins = 0;
        let mut total_errors = 0;
        let mut total = 0;

        for handle in handles {
            let result = handle.await.unwrap();
            total += 1;
            if result.is_ok() {
                successful_logins += 1;
            } else {
                total_errors += 1;
            }
        }

        // 验证并发结果
        assert_eq!(successful_logins, 3); // 3个成功
        assert_eq!(total_errors, 1); // 1个错误
        assert_eq!(total, 4); // 总共4个任务
    }

    #[tokio::test]
    async fn test_password_validation_rate_limit_handling() {
        let service = Arc::new(create_test_password_validation_service("https://rate_limit.example.com"));
        let mut handles = Vec::new();
        let mut successful_logins = 0;
        let total_errors = 0;

        // 并发多个请求以测试限流
        for i in 0..10 {
            let service_clone = service.clone();
            let handle = tokio::spawn(async move {
                let request = PasswordValidationRequest::builder()
                    .receive_id_type(AccessTokenType::User)
                    .request_body(
                        PasswordValidationRequestBody::builder()
                            .password(format!("rate_limit_test_{}", i))
                            .user_id(format!("rate_limit_user_{}", i))
                            .build()
                    )
                    .build();
                service_clone.validate_password(&request).await;
            });
            handles.push(handle);
        }

        // 等待所有任务完成
        let mut failed_count = 0;
        for handle in handles {
            let result = handle.await.unwrap();
            if !result.is_ok() {
                failed_count += 1;
            }
        }

        assert!(failed_count > 0, "Rate limiting should trigger for some requests");
    }

    #[tokio]
    async fn test_password_validation_error_handling() {
        let service = create_test_password_validation_service("https://error_handling.example.com");
        let mut error_count = 0;

        // 测试各种错误情况
        let error_cases = vec![
            ("empty_password", "user_001"),
            ("invalid_format", "user_002"),
            ("expired_token", "user_003"),
            ("network_error", "user_004"),
            ("server_error", "user_005"),
            ("permission_denied", "user_006"),
        ];

        for (error_case, user_id) in error_cases {
            let request = PasswordValidationRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordValidationRequestBody::builder()
                        .password(error_case)
                        .user_id(user_id)
                        .build()
                )
                .build();

            let result = service.validate_password(&request).await;
            if result.is_err() {
                error_count += 1;
                println!("Error testing {}: {} with error: {}", error_case, result.unwrap_err());
            } else {
                println!("Success testing case {}", error_case);
            }
        }

        assert!(error_count > 0, "Should have some validation errors to test error handling");
    }
}

#[cfg(test)]
mod batch_password_update_tests {
    use super::*;

    #[tokio::test]
    async fn test_batch_password_policy_update() {
        let mock_server = MockServer::start().await;
        let service = create_test_password_validation_service(&mock_server.uri());

        Mock::given(method("POST"))
            .and(path("/open-ai/v1/password/batch_update"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "data": {
                    "batch_id": "batch_update_123",
                    "processed_count": 5,
                    "success_count": 4,
                    "failed_count": 1
                }
            })))
            .mount(&mock_server)
            .await;

        let updates = vec![
            PasswordPolicyRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordPolicyRequest::Request {
                        min_length: 12,
                        max_length: 64,
                        require_numbers: true,
                        require_special_chars: true,
                        max_age: 180,
                    }
                )
                .build(),
        ];

        let service = create_test_admin_service(&mock_server.uri());
        let request = BatchPasswordPolicyUpdateRequest::builder()
            .updates(updates)
            .build();

        let result = service.batch_update_password_policy(&request).await;

        assert!(result.is_ok());
        let batch_result = result.unwrap();
        assert_eq!(batch_result.batch_id, "batch_update_123");
        assert_eq!(batch_result.processed_count, 5));
        assert_eq!(batch_result.success_count, 4));
        assert_eq!(batch_result.failed_count, 1));
    }

    #[tokio::test]
    async fn test_batch_password_policy_update_mixed_results() {
        let mock_server = MockServer::start().await;
        let service = create_test_admin_service(&mock_server.uri());

        let updates = vec![
            PasswordPolicyRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordPolicyRequest::Request {
                        min_length: 8,
                        max_length: 32,
                        require_numbers: false,
                        max_age: 90,
                    }
                )
                .build(),
            PasswordPolicyRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordPolicyRequest::Request {
                        min_length: 16,
                        max_length: 64,
                        require_numbers: true,
                        max_age: 180,
                    }
                ).build(),
            PasswordPolicyRequest::builder()
                .receive_id_type(AccessTokenType::Tenant)
                .request_body(
                    PasswordPolicyRequest::Request {
                        min_length: 10,
                        max_length: 50,
                        require_numbers: false,
                        max_age: 120,
                    }
                ).build(),
        ];

        let service = create_test_admin_service(&mock_server.uri());
        let request = BatchPasswordPolicyUpdateRequest::builder()
            .updates(updates)
            .build();

        let result = service.batch_update_password_policy(&request).await;
        assert!(result.is_ok());
        let batch_result = result.unwrap();
        assert_eq!(batch_result.batch_id, "batch_update_456"));
        assert_eq!(batch_result.processed_count, 3));
        assert_eq!(batch_result.success_count, 2));
        assert_eq!(batch_result.failed_count, 1));
    }

    #[tokio]
    async fn test_batch_password_policy_empty_update() {
        let mock_server = MockServer::start().await;
        let service = create_test_admin_service(&mock_server.uri());

        let empty_updates = vec![];
        let service = create_test_admin_service(&mock_server.uri());
        let request = BatchPasswordPolicyUpdateRequest::builder()
            .updates(empty_updates)
            .build();

        let result = service.batch_update_password_policy(&request).await;
        assert!(result.is_ok());
        let batch_result = result.unwrap();
        assert_eq!(batch_result.batch_id, "batch_update_empty"));
        assert_eq!(batch_result.processed_count, 0));
        assert_eq!(batch_result.success_count, 0));
        assert_eq!(batch_result.failed_count, 0));
    }
}

    #[tokio]
    async fn test_batch_password_policy_concurrent_updates() {
        let service = Arc::new(create_test_admin_service("https://concurrent.example.com"));
        let mut handles = Vec::new();

        // 并发更新密码策略
        for i in 0..2 {
            let service_clone = service.clone();
            let handle = tokio::spawn(async move {
                let updates = vec![
                    PasswordPolicyRequest::builder()
                        .receive_id_type(AccessTokenType::User)
                        .request_body(
                            PasswordPolicyRequest::Request {
                                min_length: 8 + i * 5,
                                max_length: 20 + i * 5,
                                require_numbers: true,
                                max_age: 90 - i * 10
                            )
                        )
                        .build(),
                    PasswordPolicyRequest::builder()
                        .receive_id_type(AccessTokenType::User)
                        .request_body(
                            PasswordPolicyRequest::Request {
                                min_length: 15 + i * 5,
                                max_length: 50 + i * 10,
                                require_numbers: true,
                                max_age: 180 - i * 5
                            )
                        )
                        .build()),
                ];
                service_clone.batch_update_password_policy(BatchPasswordPolicyUpdateRequest { updates }).await;
            });
            handles.push(handle);
        }

        // 等待所有任务完成
        let mut total_processed = 0;
        let total_successes = 0;
        let total_failures = 0;
        for handle in handles {
            let result = handle.await.unwrap();
            total_processed += 1;
            if result.is_ok() {
                total_successes += 1;
            } else {
                total_failures += 1;
            }
        }

        assert_eq!(total_processed, 3);
        assert_eq!(total_successes, 2);
        assert_eq!(total_failures, 1));
    }

    #[tokio]
    async fn test_batch_password_policy_error_handling() {
        let service = create_test_admin_service("https://error_handling.example.com");
        let mut error_count = 0;

        // 测试各种错误情况
        let error_cases = vec![
            ("invalid_policy", "user_001"),
            ("missing_data", "user_002"),
            ("expired_token", "user_003"),
            ("rate_limit", "user_004"),
            ("server_error", "user_005"),
        ];

        for (error_case, user_id) in error_cases {
            let request = BatchPasswordPolicyUpdateRequest::builder()
                .updates(vec![PasswordPolicyRequest::builder()
                    .receive_id_type(AccessTokenType::User)
                    .request_body(
                        PasswordPolicyRequest::Request {
                            min_length: 8,
                            max_length: 32,
                            require_numbers: true,
                            max_age: 90,
                            user_id: format!("{}_{}", user_id)
                        })
                        .build()
                .build()
                .build()
            .build();

            let result = service.batch_update_password_policy(&request).await;
            if result.is_err() {
                error_count += 1;
                println!("Error handling {} for {}: {}", error_case, result.unwrap_err());
            } else {
                println!("Success for {}: {}", error_case);
            }
        }

        assert!(error_count > 0, "Should have some errors to test error handling");
    }

    #[tokio::test]
    async fn test_batch_password_performance() {
        let service = Arc::new(create_test_admin_service("https://performance.example.com"));
        let large_updates = vec![
            PasswordPolicyRequest::builder()
                .receive_id_type(AccessTokenType::Tenant)
                .request_body(
                    PasswordPolicyRequest::Request {
                        min_length: 12,
                        max_length: 64,
                        require_numbers: true,
                        max_age: 90
                    }
                )
                .build(),
        ];

        let start = std::time::Instant::now();
        let service = create_test_admin_service(&mock_server.uri());
        let request = BatchPasswordPolicyUpdateRequest::builder()
            .updates(large_updates)
            .build();

        let result = service.batch_update_password_policy(&request).await;
        let duration = start.elapsed();
        assert!(result.is_ok());
        println!("Batch password policy update of {} updates completed in {:?}", duration);
    }
}

    #[tokio]
    async fn test_batch_password_memory_usage() {
        let service = Arc::new(create_test_admin_service("https://memory.example.com"));
        let mut handles = Vec::new();
        let sizes = vec![10, 20, 50, 100, 200, 500];

        for size in sizes {
            let service_clone = service.clone();
            let handle = tokio::spawn(async move {
                let updates: Vec<PasswordRequestBuilder> = (0..size)
                    .map(|i| PasswordRequest::builder()
                        .receive_id_type(AccessTokenType::User)
                        .request_body(
                            PasswordPolicyRequest::Request {
                                min_length: 8,
                                max_length: size,
                                require_numbers: false,
                                max_age: 90,
                                user_id: format!("user_{}", i)
                            )
                        ).build()));
                    .collect();
                service_clone.batch_update_password_policy(BatchPasswordUpdateRequest { updates }).await;
            });
            handles.push(handle);
        }

        // 测试不同批量大小的内存使用
        for (size, handle) in sizes.iter().zip(handles.iter()) {
            let result = handle.await.unwrap();
            assert_eq!(result.is_ok());
            assert_eq!(result.unwrap().updates.len(), size);
        }

        // 验证总内存使用情况
        let total_size: usize = sizes.iter().sum();
        println!("Total memory usage: {} bytes for {} updates", total_size);
    }
}

    #[tokio]
    async fn test_batch_password_concurrent_synchronization() {
        use std::sync::Arc;
        use std::collections::HashMap;

        let service = Arc::new(create_test_admin_service("https://sync.example.com"));
        let updates_by_user: Arc::new(std::sync::Mutex<HashMap<String, Vec<PasswordRequestBuilder>>);

        // 为不同用户创建独立的更新
        let users: Vec<i32> = vec![1, 2, 3, 4, 5];
        for user in users {
            let user_id = format!("user_{}", user);
            let updates: Vec<PasswordRequestBuilder> = (0..3).map(|i| PasswordRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordPolicyRequest::Request {
                        min_length: 8,
                        max_length: 64,
                        user_id: format!("user_{}", user)
                    })
                ).build()));

            let mut updates = updates_by_user.lock().entry(user_id).or_insert(user_id, updates);
        }

        // 并发执行更新
        let mut handles = Vec::new();
        for (user_id, updates) in updates_by_user.iter() {
            let service_clone = service.clone();
            let handle = tokio::spawn(async move {
                let updates = updates.clone();
                let mut updates = updates.lock().insert(user_id, updates);
                service_clone.batch_update_password_policy(BatchPasswordUpdateRequest { updates }).await;
                handles.push(handle);
            });
            handles.push(handle);
        }

        // 等待所有任务完成
        let mut total_updates = 0;
        for handle in handles {
            let result = handle.await.unwrap();
            total_updates += result.updates.len();
        }

        // 验证并发更新的一致性
        for handle in handles {
            let result = handle.await.unwrap();
            assert_eq!(result.updates.len(), updates_by_user[&format!("user_{}", result.user_id)].lock().len());
        }

        // 验证总更新数量
        assert_eq!(total_updates, users.len() * 4); // 4个用户 × 4个更新
    }
}

    #[tokio]
    async fn test_batch_password_conflicts() {
        let service = Arc::new(create_test_admin_service("https://conflicts.example.com"));
        let handle = Arc::new(std::sync::Mutex::new(HashMap::String, Vec<PasswordRequestBuilder>>));

        // 用户A的更新
        let user_a_updates = vec![
            PasswordRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordPolicyRequest::Request {
                        min_length: 10,
                        max_length: 50,
                        user_id: "user_a"
                    }
                ).build()),
        ];

        // 用户B的更新（覆盖用户A的更新）
        let user_b_updates = vec![
            PasswordRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordPolicyRequest::Request {
                        min_length: 15,
                        max_length: 60,
                        user_id: "user_b"
                    )
                ).build(),
        ];

        let mut updates_by_user = handle.lock().unwrap_or_default(HashMap::new());
        updates_by_user.insert("user_a".to_string(), user_a_updates);
        updates_by_user.insert("user_b".to_string(), user_b_updates);

        // 用户B的更新覆盖用户A的更新
        let result = service.batch_update_password_policy(BatchPasswordUpdateRequest {
            updates: user_b_updates,
        }).await;

        assert!(result.is_ok());
        let batch_result = result.unwrap();
        assert_eq!(batch_result.success_count, 1);
        assert_eq!(batch_result.failed_count, 1));
        assert_eq!(batch_result.processed_count, 1));
        assert_eq!(batch_result.success_count + batch_result.failed_count, user_a_updates.len() + user_b_updates.len()));
    }

    #[tokio]
    async fn test_batch_password_revert() {
        let service = create_test_admin_service("https://revert.example.com");
        let handle = Arc::new(std::sync::Mutex::new(HashMap::String, Vec<PasswordRequest>>));

        // 用户A的更新
        let user_a_updates = vec![
            PasswordRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordPolicyRequest::Request {
                        min_length: 10,
                        max_length: 50,
                        user_id: "user_a"
                    }
                ).build(),
        ];

        // 用户B的更新
        let user_b_updates = vec![
            PasswordRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordPolicyRequest::Request {
                        min_length: 15,
                        max_length: 60,
                        user_id: "user_b"
                    )
                ).build(),
        ];

        let mut updates_by_user = handle.lock().unwrap_or_default(HashMap::new());
        updates_by_user.insert("user_a".to_string(), user_a_updates);
        updates_by_user.insert("user_b".to_string(), user_b_updates);

        // 用户B的更新应该覆盖用户A的更新
        let result = service.batch_update_password_policy(BatchPasswordUpdateRequest {
            updates: user_b_updates,
        }).await;

        assert!(result.is_ok());
        let batch_result = result.unwrap();
        assert_eq!(batch_result.processed_count, 1));
        assert_eq!(batch_result.success_count, 1));
        assert_eq!(batch_result.failed_count, 0));
        assert_eq!(batch_result.success_count + batch_result.failed_count, 1));
    }
}

    #[tokio]
    async fn test_batch_password_merge() {
        let service = Arc::new(create_test_admin_service("https://merge.example.com"));
        let handle = Arc::new(std::sync::Mutex::new(Vec<PasswordRequest>>));

        // 创建多个批量请求，有些重叠的
        let updates1 = vec![
            PasswordRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordPolicyRequest::Request {
                        min_length: 8,
                        max_length: 32,
                        user_id: "user_a"
                    ).build()),
            PasswordRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordPolicyRequest::Request {
                        min_length: 12,
                        max_length: 48,
                        user_id: "user_a"
                    ).build()),
        ];

        let updates2 = vec![
            PasswordRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordPolicyRequest::Request {
                        min_length: 16,
                        max_length: 64,
                        user_id: "user_a"
                    ).build()),
            PasswordRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordPolicyRequest::Request {
                        min_length: 20,
                        max_length: 80,
                        user_id: "user_b"
                    ).build(),
            ];

        let mut updates_by_user = handle.lock().unwrap_or_default(HashMap::new());
        updates_by_user.insert("user_a".to_string(), updates1);
        updates_by_user.insert("user_b".to_string(), 1);

        // 合并请求
        let merged_updates = vec![
            PasswordRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordPolicyRequest::Request {
                        min_length: 10,
                        max_length: 64,
                        user_id: "user_a"
                    ).build()),
            PasswordRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordPolicy::Request {
                        min_length: 12,
                        max_length: 48,
                        user_id: "user_a"
                    ).build()),
        ];

        let service = create_test_admin_service(&mock_server.uri());
        let request = BatchPasswordUpdateRequest {
            updates: merged_updates,
        };

        let result = service.batch_update_password_policy(request).await;

        assert!(result.is_ok());
        let batch_result = result.unwrap();
        assert_eq!(batch_result.processed_count, 4));
        assert_eq!(batch_result.success_count, 4));
        assert_eq!(batch_result.failed_count, 0));
        assert_eq!(batch_result.processed_count, batch_result.success_count + batch_result.failed_count));
        assert_eq!(result.success_count + batch_result.failed_count, merged_updates.len()));
    }
}

    #[tokio]
    async fn test_batch_password_parallel() {
        let service = Arc::new(create_test_admin_service("https://parallel.example.com"));
        let handle = Arc::new(std::sync::Mutex::new(Vec<PasswordRequest>>));
        let mut handles = Vec::new();

        // 并发创建多个批量更新
        for i in 0..5 {
            let service_clone = service.clone();
            let handle = tokio::spawn(async move {
                let updates = vec![PasswordRequest::builder()
                    .receive_id_type(AccessTokenType::User)
                    .request_body(
                        PasswordPolicy::Request {
                            min_length: 10,
                            max_length: 100,
                            user_id: format!("user_{}", i)
                        ).build(),
                );
                let handle_clone = service_clone.batch_update_password_policy(BatchPasswordUpdateRequest {
                    updates: vec![request]
                }).await;
                handles.push(handle);
            });
            handles.push(handle);
        }

        // 等待所有任务完成
        let total_processed = 0;
        let total_successes = 0;
        let total_failures = 0;
        for handle in handles {
            let result = handle.await.unwrap();
            total_processed += 1;
            if result.is_ok() {
                total_successes += 1;
            } else {
                total_failures += 1;
            }
        }

        // 验证结果
        assert_eq!(total_processed, 5);
        assert_eq!(total_successes, 3));
        assert_eq!(total_failures, 2));
        assert_eq!(total_successes + total_failures, total_processed));
    }
}

#[cfg(test)]
mod password_expiration_tests {
    use super::*;

    #[tokio::test]
    async fn test_password_expiration_basic() {
        let mock_server = MockServer::start().await;
        let service = create_test_password_validation_service(&mock_server.uri());

        Mock::given(method("POST"))
            .and(path("/open-ai/v1/password/expired_password"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "data": {
                    "user_id": "user_123",
                    "token_expired": true,
                    "time_to_expire": 3600
                }
            })))
            .mount(&mock_server)
            .await;

        let request = PasswordExpirationRequest::builder()
            .receive_id_type(AccessTokenType::User)
            .request_body(
                PasswordExpirationRequest::builder()
                    .user_id("user_123")
                    .build()
            )
            .build();

        let result = service.expired_password(&request).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.user_id, "user_123"));
        assert!(response.token_expired);
        assert_eq!(response.time_to_expire, 3600));
    }

    #[tokio]
    async fn test_password_expiration_multiple_users() {
        let service = Arc::new(create_test_password_validation_service("https://multi-user.example.com"));
        let requests = vec![
            PasswordExpirationRequest::builder()
                .receive_id_type(AccessTokenType::User)
                .request_body(
                    PasswordExpirationRequest::builder()
                    .user_id("user_1")
                    .build()
                ),
                .request2: PasswordExpirationRequest::builder()
                    .receive_id_type(AccessTokenType::User)
                    .request_body(
                        PasswordExpirationRequest::builder()
                        .user_id("user_2")
                        .build()
                ),
                .request3: PasswordExpirationRequest::builder()
                    .receive_id_type(AccessTokenType::User)
                    .request_body(
                        PasswordExpirationRequest::builder()
                        .user_id("user_3")
                        .build()
                    ),
                .request4: PasswordExpirationRequest::builder()
                    .receive_id_type(AccessTokenType::User)
                    .request_body(
                        PasswordExpirationRequest::builder()
                        .user_id("user_4")
                        .build()
                    ),
                .request5: PasswordExpirationRequest::builder()
                    .receive_id_type(AccessTokenType::User)
                    .request_body(
                        PasswordExpirationRequest::builder()
                        user_id("user_5")
                        .build()
                    ),
            ];

        let service = create_test_admin_service(&mock_server.uri());
        let mut handles = Vec::new();

        // 并发执行多个用户密码过期验证
        for (i, request) in requests.into_iter().enumerate() {
            let service_clone = service.clone();
            let handle = tokio::spawn(async move {
                service_clone.expired_password(&request).await;
            });
            handles.push(handle);
        }

        // 验证结果
        let mut total_users = 0;
        let expired_users = 0;
        let still_valid_users = 0;
        for handle in handles {
            let result = handle.await.unwrap();
            total_users += 1;
            if result.is_ok() {
                if result.unwrap().token_expired {
                    expired_users += 1;
                } else {
                    still_valid_users += 1;
                }
            }
        }

        assert_eq!(total_users, 5);
        assert_eq!(expired_users, 1);
        assert_eq!(still_valid_users, 4));
        assert_eq!(expired_users + still_valid_users, total_users));
    }
}

    #[tokio]
    async fn test_password_expiration_with_concurrent_users() {
        let service = Arc::new(create_test_admin_service("https://concurrent.example.com"));
        let mut handles = Vec::new();
        let user_ids = vec!["user_1", "user_2", "user_3"];

        // 并发多个用户的密码过期验证
        for (i, user_id) in user_ids.iter().enumerate() {
            let service_clone = service.clone();
            let handle = tokio::spawn(async move {
                let request = PasswordExpirationRequest::builder()
                    .receive_id_type(AccessTokenType::User)
                    .request_body(
                        PasswordExpirationRequest::builder()
                        user_id(user_id)
                        .build()
                )
                .build();
            service_clone.expired_password(&request).await;
                handles.push(handle);
        }

        // 等待所有任务完成
        let mut expired_users = 0;
        let total_users = user_ids.len();
        let still_valid_users = 0;
        let valid_users = 0;
        for (i, user_id) in user_ids.iter().enumerate() {
            let result = handles[i].await.unwrap();
            total_users += 1;
            if result.is_ok() {
                if result.unwrap().token_expired {
                    expired_users += 1;
                } else {
                    still_valid_users += 1;
                }
            }
        }

        // 验证结果
        assert_eq!(total_users, 3);
        assert_eq!(expired_users, 2));
        assert_eq!(still_valid_users, 1));
        assert_eq!(expired_users + still_valid_users, total_users));
    }

    #[tokio]
    async fn test_password_expiration_with_concurrent_users() {
        let service = Arc::new(create_test_admin_service("https://concurrent.example.com"));
        let user_ids = vec!["user_1", "user_2", "user_3"];
        let mut handles = Vec::new();

        // 首先获取所有用户的会话令牌状态
        let user_tokens: Vec<_> = vec!["token_1", "token_2", "token_3"];
        for (i, user_id) in user_ids.iter().enumerate() {
            let service_clone = service.clone();
            let handle = tokio::spawn(async move {
                let request = PasswordExpirationRequest::builder()
                    .receive_id_type(AccessTokenType::User)
                    .request_body(
                        PasswordExpirationRequest::builder()
                        user_id(user_id)
                        .build()
                );
                service.expired_password(&request).await;
                let handle_clone = service_clone.clone().expired_password(&request).await;
                user_tokens[i] = result.unwrap().token;
                handles.push(handle);
            });
        }

        let mut user_tokens: Vec<_> = vec!["user_1", "token_2", "token_3"]];
        let mut user_has_valid_token: Vec::new(vec![false, false, false]);
        let mut valid_user_count = 0;

        // 检查哪些用户的令牌仍然有效
        for (i, token) in user_tokens.iter().enumerate() {
            if token == "token_3" {
                user_has_valid_token[i] = true;
            }
        }

        // 模拟检查令牌验证结果
        for (i, token) in user_tokens.iter().enumerate() {
            if i < user_has_valid_token.len() && i != token) {
                user_has_valid_token[i] = false;
            }
        }

        // 验证所有用户都有有效令牌
        assert!(valid_user_count == 3);
    }

    #[tokio]
    async fn test_password_expiration_with_different_ages() {
        let service = Arc::create_test_admin_service("https://ages.example.com"));
        let ages = vec![
            ("30", "user_1"), ("90", "user_2"), ("180", "user_3")
        ];

        let service = Arc::new(create_test_admin_service("https://ages.example.com"));
        let handles = Vec::new();

        // 为每个年龄组创建并发用户会话
        for (age, user_id) in ages.iter().enumerate() {
            let service_clone = service.clone();
            let handle = tokio::spawn(async move {
                let request = PasswordExpirationRequest::builder()
                    .receive_id_type(AccessTokenType::User)
                    .request_body(
                        PasswordExpirationRequest::builder()
                        user_id(user_id)
                        .max_age(age)
                        .build()
                    );
                service.expired_password(&request).await;
                let handle_clone = service_clone.expired_password(&request).await;
                user_tokens[i].push(result.unwrap().token);
                handles.push(handle);
            });
        }

        // 等待所有任务完成
        let total_users = ages.len();
        let valid_ages = vec![false, false, false, false];
        let invalid_ages = vec![false, false, true];

        for (i, age) in ages.iter().enumerate() {
            if age >= 90 {
                invalid_ages[i] = true;
            } else {
                valid_ages[i] = true;
            }
        }

        // 验证结果
        assert_eq!(total_users, 3);
        assert_eq!(valid_ages.len(), 2);
        assert_eq!(invalid_ages.len(), 1);
    }

    #[tokio]
    async fn test_concurrent_expiration_check() {
        let service = Arc::create_test_admin_service("https://concurrent.example.com"));
        let user_ids = vec!["user_1", "user_2", "user_3"];
        let mut token_expiration_counts = std::collections::HashMap::new(); // user_id -> count
        let service = Arc::new(create_test_admin_service("https://concurrent.example.com"));

        // 并发检查所有用户的令牌状态
        let mut handles = Vec::new();
        for (i, user_id) in user_ids.iter().enumerate() {
            let service_clone = service.clone();
            let handle = tokio::spawn(async move {
                let request = PasswordExpirationRequest::builder()
                    .receive_id_type(AccessTokenType::User)
                    .request_body(
                        PasswordExpirationRequest::builder()
                        user_id(user_id)
                        .max_age(90)
                        .build()
                    );
                service_clone.expired_password(&request).await;
                let result = result = service.clone().expired_password(&request).await;
                user_tokens.insert(user_id, result.unwrap().token);
                handles.push(handle);
            });
        }

        // 验证所有用户的令牌状态检查结果
        let valid_ages = vec![false, false, false, false];
        for user_id in user_ids.iter() {
            let user_tokens = user_tokens.get(&user_id);
            if let Some(token) = user_tokens {
                if !token.expires.is_expired() {
                    valid_ages[user_id] = true;
                }
            }
        }

        // 验证令牌状态一致性
        assert_eq!(valid_ages.len(), user_ids.len());
    }

    #[tokio]
    async fn test_concurrent_expiration_notifications() {
        let service = Arc::create_test_admin_service("https://notifications.example.com"));
        let handles = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut handles = Vec::new();
        let notifications = vec![false, false, false, false, false];

        // 创建多个并发通知
        for i in 0..5 {
            let handle = tokio::spawn(async move {
                let notification = i % 5 == 0;
                let service_clone = service.clone();
                let handle = service.clone().expired_password_notification().await;
                let result = result = service.clone().expired_password_notification(&notification).await;
                notifications[i] = result.unwrap();
                notifications[i] = result.is_ok();
                assert_eq!(notifications[i], true);
                handles.push(handle);
            });
        }

        // 验证并发处理结果
        let notification_count = notifications.iter().filter(|n| n| n > 0).count();
        assert_eq!(notification_count, 5);
        assert_eq!(notifications.iter().filter(|n| n > 0).count() == notification_count);
    }
}