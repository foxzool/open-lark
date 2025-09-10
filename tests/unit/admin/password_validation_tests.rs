use open_lark::core::validation::{
    validate_and_sanitize_password, validate_password_strength, ValidationResult,
};
use open_lark::service::admin::password::PasswordResetRequestBuilder;

#[test]
fn test_password_validation_integration() {
    // 测试有效密码
    let valid_passwords = vec![
        "SecurePass123!",
        "MyP@ssw0rd",
        "Complex!123",
        "Str0ng!P@ss",
    ];

    for password in valid_passwords {
        let result = validate_password_strength(password);
        assert!(
            matches!(result, ValidationResult::Valid),
            "Password '{}' should be valid",
            password
        );
    }

    // 测试无效密码
    let invalid_passwords = vec![
        ("short", "too short"),
        ("no_special_chars123", "missing special character"),
        ("NO_LOWERCASE123!", "missing lowercase"),
        ("no_uppercase123!", "missing uppercase"),
        ("NoDigitsHere!", "missing digits"),
        ("   spaces around   ", "should be trimmed"),
    ];

    for (password, _) in invalid_passwords {
        let result = validate_password_strength(password);
        assert!(
            matches!(result, ValidationResult::Invalid(_)),
            "Password '{}' should be invalid",
            password
        );
    }
}

#[test]
fn test_password_builder_integration() {
    // 测试构建器创建有效请求
    let result = PasswordResetRequestBuilder::new()
        .user_id("test_user_123")
        .password("SecurePass123!")
        .build();

    assert!(result.is_ok());
    let request = result.unwrap();
    assert_eq!(request.user_id, "test_user_123");
    assert_eq!(request.password, "SecurePass123!");

    // 测试构建器拒绝无效密码
    let result = PasswordResetRequestBuilder::new()
        .user_id("test_user_123")
        .password("weak")
        .build();

    assert!(result.is_err());

    // 测试构建器拒绝空用户ID
    let result = PasswordResetRequestBuilder::new()
        .user_id("")
        .password("SecurePass123!")
        .build();

    assert!(result.is_err());

    // 测试密码清理
    let result = PasswordResetRequestBuilder::new()
        .user_id("test_user_123")
        .password("  SecurePass123!  ")
        .build();

    assert!(result.is_ok());
    let request = result.unwrap();
    assert_eq!(request.password, "SecurePass123!"); // 空格被去除
}

#[test]
fn test_password_sanitization() {
    let test_cases = vec![
        ("  password123  ", "password123"),
        ("password123  ", "password123"),
        ("  password123", "password123"),
        ("password123", "password123"),
    ];

    for (input, expected) in test_cases {
        let (sanitized, result) = validate_and_sanitize_password(input.to_string(), "test");
        assert_eq!(sanitized, expected);
        
        // 注意：仅长度检查，密码强度检查在单独的函数中
        if input.trim().len() >= 8 {
            assert!(matches!(result, ValidationResult::Valid | ValidationResult::Warning(_)));
        }
    }
}