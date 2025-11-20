//! 密码验证模块

use super::core::ValidationResult;

/// 密码限制常量
pub mod password_limits {
    /// 密码最小长度
    pub const MIN_LENGTH: usize = 8;
    /// 密码最大长度
    pub const MAX_LENGTH: usize = 128;
    /// 必须包含数字
    pub const REQUIRE_DIGIT: bool = true;
    /// 必须包含小写字母
    pub const REQUIRE_LOWERCASE: bool = true;
    /// 必须包含大写字母
    pub const REQUIRE_UPPERCASE: bool = true;
    /// 必须包含特殊字符
    pub const REQUIRE_SPECIAL_CHAR: bool = true;
}

/// 验证密码强度
///
/// # 参数
/// - `password`: 要验证的密码
///
/// # 返回
/// 验证结果
pub fn validate_password_strength(password: &str) -> ValidationResult {
    if password.len() < password_limits::MIN_LENGTH {
        return ValidationResult::Invalid(format!(
            "密码长度不能少于 {} 位",
            password_limits::MIN_LENGTH
        ));
    }

    if password.len() > password_limits::MAX_LENGTH {
        return ValidationResult::Invalid(format!(
            "密码长度不能超过 {} 位",
            password_limits::MAX_LENGTH
        ));
    }

    let mut has_digit = false;
    let mut has_lowercase = false;
    let mut has_uppercase = false;
    let mut has_special_char = false;
    let mut has_chinese = false;

    for ch in password.chars() {
        if ch.is_ascii_digit() {
            has_digit = true;
        } else if ch.is_ascii_lowercase() {
            has_lowercase = true;
        } else if ch.is_ascii_uppercase() {
            has_uppercase = true;
        } else if ch.is_ascii() && !ch.is_alphanumeric() {
            has_special_char = true;
        } else if super::core::is_chinese_char(ch) {
            has_chinese = true;
        }
    }

    let mut errors = Vec::new();

    if password_limits::REQUIRE_DIGIT && !has_digit {
        errors.push("密码必须包含至少一个数字");
    }

    if password_limits::REQUIRE_LOWERCASE && !has_lowercase {
        errors.push("密码必须包含至少一个小写字母");
    }

    if password_limits::REQUIRE_UPPERCASE && !has_uppercase {
        errors.push("密码必须包含至少一个大写字母");
    }

    if password_limits::REQUIRE_SPECIAL_CHAR && !has_special_char {
        errors.push("密码必须包含至少一个特殊字符");
    }

    if has_chinese {
        errors.push("密码不能包含中文字符");
    }

    if errors.is_empty() {
        ValidationResult::Valid
    } else {
        ValidationResult::Invalid(errors.join("; "))
    }
}

/// 验证并清理密码
///
/// # 参数
/// - `password`: 原始密码
/// - `field_name`: 字段名称
///
/// # 返回
/// 验证结果
pub fn validate_and_sanitize_password(password: String, field_name: &str) -> ValidationResult {
    // 去除首尾空白字符
    let password = password.trim();

    if password.is_empty() {
        return ValidationResult::Invalid(format!("{} 不能为空", field_name));
    }

    let validation_result = validate_password_strength(password);
    match validation_result {
        ValidationResult::Valid => ValidationResult::Valid,
        ValidationResult::Invalid(msg) => {
            ValidationResult::Invalid(format!("{} 验证失败: {}", field_name, msg))
        }
        ValidationResult::Sanitized(sanitized) => {
            ValidationResult::Sanitized(format!("{} 已清理: {}", field_name, sanitized))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_validation() {
        // 有效密码（包含所有必需的字符类型）
        assert!(validate_password_strength("Test123!@#").is_valid());
        assert!(validate_password_strength("mYp@ssw0rd").is_valid());

        // 太短
        assert!(!validate_password_strength("Test1").is_valid());

        // 太长
        let long_password = "a".repeat(150);
        assert!(!validate_password_strength(&long_password).is_valid());

        // 缺少数字
        assert!(!validate_password_strength("Password!").is_valid());

        // 缺少小写字母
        assert!(!validate_password_strength("PASSWORD123!").is_valid());

        // 缺少大写字母
        assert!(!validate_password_strength("password123!").is_valid());

        // 缺少特殊字符
        assert!(!validate_password_strength("Password123").is_valid());

        // 包含中文
        assert!(!validate_password_strength("密码123!").is_valid());
    }

    #[test]
    fn test_validate_and_sanitize_password() {
        // 正常情况
        let result = validate_and_sanitize_password("Test123!@#".to_string(), "密码");
        assert!(result.is_valid());

        // 包含空白字符
        let result = validate_and_sanitize_password("  Test123!@#  ".to_string(), "密码");
        assert!(result.is_valid());

        // 空密码
        let result = validate_and_sanitize_password("".to_string(), "密码");
        assert!(!result.is_valid());

        // 只有空白字符
        let result = validate_and_sanitize_password("   ".to_string(), "密码");
        assert!(!result.is_valid());
    }

    #[test]
    fn test_password_limits() {
        assert_eq!(password_limits::MIN_LENGTH, 8);
        assert_eq!(password_limits::MAX_LENGTH, 128);
        assert!(password_limits::REQUIRE_DIGIT);
        assert!(password_limits::REQUIRE_LOWERCASE);
        assert!(password_limits::REQUIRE_UPPERCASE);
        assert!(password_limits::REQUIRE_SPECIAL_CHAR);
    }
}
