//! 分页验证模块

use super::core::ValidationResult;

/// 分页限制常量
pub mod pagination_limits {
    /// 默认页大小
    pub const DEFAULT_PAGE_SIZE: u32 = 20;
    /// 最小页大小
    pub const MIN_PAGE_SIZE: u32 = 1;
    /// 最大页大小
    pub const MAX_PAGE_SIZE: u32 = 100;
    /// 分页令牌最大长度
    pub const MAX_PAGE_TOKEN_LENGTH: usize = 1024;
    /// 分页令牌格式
    pub const PAGE_TOKEN_REGEX: &str = r"^[a-zA-Z0-9+/=_-]+$";
}

/// 验证页大小
///
/// # 参数
/// - `page_size`: 页大小
/// - `field_name`: 字段名称
///
/// # 返回
/// 验证结果
pub fn validate_page_size(page_size: u32, field_name: &str) -> ValidationResult {
    if page_size < pagination_limits::MIN_PAGE_SIZE {
        ValidationResult::Invalid(format!(
            "{} 不能小于 {}",
            field_name,
            pagination_limits::MIN_PAGE_SIZE
        ))
    } else if page_size > pagination_limits::MAX_PAGE_SIZE {
        ValidationResult::Invalid(format!(
            "{} 不能超过 {}",
            field_name,
            pagination_limits::MAX_PAGE_SIZE
        ))
    } else {
        ValidationResult::Valid
    }
}

/// 验证分页令牌
///
/// # 参数
/// - `page_token`: 分页令牌
/// - `field_name`: 字段名称
///
/// # 返回
/// 验证结果
pub fn validate_page_token(page_token: &str, field_name: &str) -> ValidationResult {
    let trimmed_token = page_token.trim();

    if trimmed_token.is_empty() {
        // 空令牌表示第一页，这是有效的
        return ValidationResult::Valid;
    }

    if trimmed_token.len() > pagination_limits::MAX_PAGE_TOKEN_LENGTH {
        return ValidationResult::Invalid(format!(
            "{} 长度不能超过 {}",
            field_name,
            pagination_limits::MAX_PAGE_TOKEN_LENGTH
        ));
    }

    // 验证分页令牌格式（只允许base64字符）
    let regex = regex::Regex::new(pagination_limits::PAGE_TOKEN_REGEX).unwrap();
    if !regex.is_match(trimmed_token) {
        return ValidationResult::Invalid(format!(
            "{} 包含无效字符，只允许字母、数字和 +/=_-",
            field_name
        ));
    }

    ValidationResult::Valid
}

/// 验证分页参数
///
/// # 参数
/// - `page_size`: 页大小
/// - `page_token`: 分页令牌
/// - `page_size_field`: 页大小字段名
/// - `page_token_field`: 分页令牌字段名
///
/// # 返回
/// 验证结果
pub fn validate_pagination_params(
    page_size: u32,
    page_token: &str,
    page_size_field: &str,
    page_token_field: &str,
) -> ValidationResult {
    // 验证页大小
    let size_result = validate_page_size(page_size, page_size_field);
    if !size_result.is_valid() {
        return size_result;
    }

    // 验证分页令牌
    let token_result = validate_page_token(page_token, page_token_field);
    if !token_result.is_valid() {
        return token_result;
    }

    ValidationResult::Valid
}

/// 清理标签字符串
///
/// # 参数
/// - `tag`: 原始标签
///
/// # 返回
/// 清理后的标签
pub fn sanitize_tag(tag: &str) -> String {
    tag.trim()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect::<String>()
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_page_size() {
        // 有效页大小
        assert!(validate_page_size(20, "页大小").is_valid());
        assert!(validate_page_size(1, "页大小").is_valid());
        assert!(validate_page_size(100, "页大小").is_valid());

        // 无效页大小
        assert!(!validate_page_size(0, "页大小").is_valid());
        assert!(!validate_page_size(101, "页大小").is_valid());
    }

    #[test]
    fn test_validate_page_token() {
        // 空令牌（第一页）
        assert!(validate_page_token("", "分页令牌").is_valid());
        assert!(validate_page_token("   ", "分页令牌").is_valid());

        // 有效令牌
        assert!(validate_page_token("abc123", "分页令牌").is_valid());
        assert!(validate_page_token("ZXhhbXBsZXRva2Vu", "分页令牌").is_valid()); // base64
        assert!(validate_page_token("abc123+/=_-", "分页令牌").is_valid());

        // 无效令牌（包含特殊字符）
        assert!(!validate_page_token("abc@123", "分页令牌").is_valid());
        assert!(!validate_page_token("abc 123", "分页令牌").is_valid());

        // 令牌过长
        let long_token = "a".repeat(2000);
        assert!(!validate_page_token(&long_token, "分页令牌").is_valid());
    }

    #[test]
    fn test_validate_pagination_params() {
        // 有效参数
        assert!(validate_pagination_params(20, "", "页大小", "分页令牌").is_valid());
        assert!(validate_pagination_params(50, "abc123", "页大小", "分页令牌").is_valid());

        // 无效页大小
        assert!(!validate_pagination_params(0, "abc123", "页大小", "分页令牌").is_valid());
        assert!(!validate_pagination_params(200, "abc123", "页大小", "分页令牌").is_valid());

        // 无效分页令牌
        assert!(!validate_pagination_params(20, "abc@123", "页大小", "分页令牌").is_valid());
    }

    #[test]
    fn test_sanitize_tag() {
        // 正常标签
        assert_eq!(sanitize_tag("normal_tag"), "normal_tag");
        assert_eq!(sanitize_tag("Normal_Tag"), "normal_tag");

        // 包含特殊字符
        assert_eq!(
            sanitize_tag("tag@with#special$chars"),
            "tagwithspecialchars"
        );
        assert_eq!(sanitize_tag("  spaced tag  "), "spacedtag");

        // 包含下划线和连字符
        assert_eq!(sanitize_tag("tag_with-dash"), "tag_with-dash");
        assert_eq!(sanitize_tag("Tag_With-Dash123"), "tag_with-dash123");
    }

    #[test]
    fn test_pagination_limits() {
        assert!(pagination_limits::MIN_PAGE_SIZE > 0);
        assert!(pagination_limits::MAX_PAGE_SIZE > pagination_limits::MIN_PAGE_SIZE);
        assert!(pagination_limits::DEFAULT_PAGE_SIZE >= pagination_limits::MIN_PAGE_SIZE);
        assert!(pagination_limits::DEFAULT_PAGE_SIZE <= pagination_limits::MAX_PAGE_SIZE);
        assert!(pagination_limits::MAX_PAGE_TOKEN_LENGTH > 0);
        assert!(!pagination_limits::PAGE_TOKEN_REGEX.is_empty());
    }
}
