/// 敏感信息脱敏处理。
///
/// 对非空字符串统一返回 `***`，避免 token / secret / PII 在日志与 Debug 中泄露。
pub fn mask_sensitive(value: &str) -> String {
    if value.is_empty() {
        String::new()
    } else {
        "***".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::mask_sensitive;

    #[test]
    fn test_mask_sensitive_non_empty() {
        assert_eq!(mask_sensitive("token_abc"), "***");
    }

    #[test]
    fn test_mask_sensitive_empty() {
        assert_eq!(mask_sensitive(""), "");
    }
}
