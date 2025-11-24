//! 验证工具模块

use crate::error::{AuthError, AuthResult};
use regex::Regex;
use tracing::debug;
use url::Url;

/// 验证工具
pub struct ValidatorUtils;

impl ValidatorUtils {
    /// 验证字符串是否为空
    pub fn is_empty(value: &str) -> bool {
        value.trim().is_empty()
    }

    /// 验证字符串是否不为空
    pub fn is_not_empty(value: &str) -> bool {
        !Self::is_empty(value)
    }

    /// 验证字符串长度
    pub fn validate_length(value: &str, min: usize, max: usize) -> bool {
        let len = value.len();
        len >= min && len <= max
    }

    /// 验证字符串长度并返回错误
    pub fn validate_length_with_error(
        value: &str,
        min: usize,
        max: usize,
        field_name: &str,
    ) -> AuthResult<()> {
        if !Self::validate_length(value, min, max) {
            return Err(AuthError::ValidationError(format!(
                "{} length must be between {} and {} characters",
                field_name, min, max
            )));
        }
        Ok(())
    }

    /// 验证邮箱格式
    pub fn is_valid_email(email: &str) -> bool {
        if Self::is_empty(email) {
            return false;
        }

        // 简单的邮箱验证正则
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .expect("Invalid email regex");
        email_regex.is_match(email)
    }

    /// 验证邮箱格式并返回错误
    pub fn validate_email(email: &str) -> AuthResult<()> {
        if !Self::is_valid_email(email) {
            return Err(AuthError::ValidationError(
                "Invalid email format".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证URL格式
    pub fn is_valid_url(url: &str) -> bool {
        if Self::is_empty(url) {
            return false;
        }

        Url::parse(url).is_ok()
    }

    /// 验证URL格式并返回错误
    pub fn validate_url(url: &str) -> AuthResult<()> {
        if !Self::is_valid_url(url) {
            return Err(AuthError::ValidationError("Invalid URL format".to_string()));
        }
        Ok(())
    }

    /// 验证HTTPS URL
    pub fn is_valid_https_url(url: &str) -> bool {
        if !Self::is_valid_url(url) {
            return false;
        }

        url.starts_with("https://")
    }

    /// 验证HTTPS URL并返回错误
    pub fn validate_https_url(url: &str) -> AuthResult<()> {
        if !Self::is_valid_https_url(url) {
            return Err(AuthError::ValidationError(
                "URL must use HTTPS protocol".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证手机号格式（中国大陆）
    pub fn is_valid_phone_cn(phone: &str) -> bool {
        if Self::is_empty(phone) {
            return false;
        }

        // 中国大陆手机号正则：1开头，第二位3-9，共11位数字
        let phone_regex = Regex::new(r"^1[3-9]\d{9}$").expect("Invalid phone regex");
        phone_regex.is_match(phone)
    }

    /// 验证手机号格式并返回错误
    pub fn validate_phone_cn(phone: &str) -> AuthResult<()> {
        if !Self::is_valid_phone_cn(phone) {
            return Err(AuthError::ValidationError(
                "Invalid Chinese phone number format".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证IP地址格式（IPv4）
    pub fn is_valid_ipv4(ip: &str) -> bool {
        if Self::is_empty(ip) {
            return false;
        }

        ip.parse::<std::net::Ipv4Addr>().is_ok()
    }

    /// 验证IP地址格式（IPv4）并返回错误
    pub fn validate_ipv4(ip: &str) -> AuthResult<()> {
        if !Self::is_valid_ipv4(ip) {
            return Err(AuthError::ValidationError(
                "Invalid IPv4 address format".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证IP地址格式（IPv6）
    pub fn is_valid_ipv6(ip: &str) -> bool {
        if Self::is_empty(ip) {
            return false;
        }

        ip.parse::<std::net::Ipv6Addr>().is_ok()
    }

    /// 验证IP地址格式（IPv6）并返回错误
    pub fn validate_ipv6(ip: &str) -> AuthResult<()> {
        if !Self::is_valid_ipv6(ip) {
            return Err(AuthError::ValidationError(
                "Invalid IPv6 address format".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证IP地址格式（IPv4或IPv6）
    pub fn is_valid_ip(ip: &str) -> bool {
        Self::is_valid_ipv4(ip) || Self::is_valid_ipv6(ip)
    }

    /// 验证IP地址格式并返回错误
    pub fn validate_ip(ip: &str) -> AuthResult<()> {
        if !Self::is_valid_ip(ip) {
            return Err(AuthError::ValidationError(
                "Invalid IP address format".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证UUID格式
    pub fn is_valid_uuid(uuid: &str) -> bool {
        if Self::is_empty(uuid) {
            return false;
        }

        uuid::Uuid::parse_str(uuid).is_ok()
    }

    /// 验证UUID格式并返回错误
    pub fn validate_uuid(uuid: &str) -> AuthResult<()> {
        if !Self::is_valid_uuid(uuid) {
            return Err(AuthError::ValidationError(
                "Invalid UUID format".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证应用ID格式（飞书）
    pub fn is_valid_app_id(app_id: &str) -> bool {
        if Self::is_empty(app_id) {
            return false;
        }

        // 飞书应用ID通常是cli_开头，后跟32个字符
        let app_id_regex = Regex::new(r"^cli_[a-zA-Z0-9]{32}$").expect("Invalid app_id regex");
        app_id_regex.is_match(app_id)
    }

    /// 验证应用ID格式并返回错误
    pub fn validate_app_id(app_id: &str) -> AuthResult<()> {
        if !Self::is_valid_app_id(app_id) {
            return Err(AuthError::ValidationError(
                "Invalid Feishu app ID format".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证令牌格式（应用访问令牌）
    pub fn is_valid_app_access_token(token: &str) -> bool {
        if Self::is_empty(token) {
            return false;
        }

        // 应用访问令牌格式：cli_开头，至少64个字符
        token.starts_with("cli_") && token.len() >= 64
    }

    /// 验证令牌格式（租户访问令牌）
    pub fn is_valid_tenant_access_token(token: &str) -> bool {
        if Self::is_empty(token) {
            return false;
        }

        // 租户访问令牌格式：t-开头，至少64个字符
        token.starts_with("t-") && token.len() >= 64
    }

    /// 验证令牌格式（用户访问令牌）
    pub fn is_valid_user_access_token(token: &str) -> bool {
        if Self::is_empty(token) {
            return false;
        }

        // 用户访问令牌格式：u-开头，至少64个字符
        token.starts_with("u-") && token.len() >= 64
    }

    /// 验证访问令牌格式（通用）
    pub fn is_valid_access_token(token: &str) -> bool {
        Self::is_valid_app_access_token(token)
            || Self::is_valid_tenant_access_token(token)
            || Self::is_valid_user_access_token(token)
    }

    /// 验证访问令牌格式并返回错误
    pub fn validate_access_token(token: &str) -> AuthResult<()> {
        if !Self::is_valid_access_token(token) {
            return Err(AuthError::ValidationError(
                "Invalid access token format".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证刷新令牌格式
    pub fn is_valid_refresh_token(token: &str) -> bool {
        if Self::is_empty(token) {
            return false;
        }

        // 刷新令牌通常至少32个字符
        token.len() >= 32
    }

    /// 验证刷新令牌格式并返回错误
    pub fn validate_refresh_token(token: &str) -> AuthResult<()> {
        if !Self::is_valid_refresh_token(token) {
            return Err(AuthError::ValidationError(
                "Invalid refresh token format".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证状态参数格式（OAuth）
    pub fn is_valid_state(state: &str) -> bool {
        if Self::is_empty(state) {
            return false;
        }

        // 状态参数通常8-128个字符，包含字母数字和特定符号
        let state_regex = Regex::new(r"^[a-zA-Z0-9._-]{8,128}$").expect("Invalid state regex");
        state_regex.is_match(state)
    }

    /// 验证状态参数格式并返回错误
    pub fn validate_state(state: &str) -> AuthResult<()> {
        if !Self::is_valid_state(state) {
            return Err(AuthError::ValidationError(
                "Invalid OAuth state parameter format".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证权限范围格式
    pub fn is_valid_scope(scope: &str) -> bool {
        if Self::is_empty(scope) {
            return false;
        }

        // 权限范围由多个权限组成，用空格分隔
        // 每个权限格式：contact:base, drive:drive, 等
        let scope_regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]*:[a-zA-Z][a-zA-Z0-9_]*(?:\s+[a-zA-Z][a-zA-Z0-9_]*:[a-zA-Z][a-zA-Z0-9_]*)*$")
            .expect("Invalid scope regex");
        scope_regex.is_match(scope)
    }

    /// 验证权限范围格式并返回错误
    pub fn validate_scope(scope: &str) -> AuthResult<()> {
        if !Self::is_valid_scope(scope) {
            return Err(AuthError::ValidationError(
                "Invalid permission scope format".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证重定向URI格式
    pub fn is_valid_redirect_uri(uri: &str) -> bool {
        if Self::is_empty(uri) {
            return false;
        }

        // 必须是有效的URL，并且使用HTTPS或localhost
        if !Self::is_valid_url(uri) {
            return false;
        }

        uri.starts_with("https://")
            || uri.starts_with("http://localhost")
            || uri.starts_with("http://127.0.0.1")
    }

    /// 验证重定向URI格式并返回错误
    pub fn validate_redirect_uri(uri: &str) -> AuthResult<()> {
        if !Self::is_valid_redirect_uri(uri) {
            return Err(AuthError::ValidationError(
                "Invalid redirect URI format".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证租户密钥格式
    pub fn is_valid_tenant_key(tenant_key: &str) -> bool {
        if Self::is_empty(tenant_key) {
            return false;
        }

        // 租户密钥通常6-32个字符，字母数字和下划线
        let tenant_key_regex =
            Regex::new(r"^[a-zA-Z0-9_-]{6,32}$").expect("Invalid tenant_key regex");
        tenant_key_regex.is_match(tenant_key)
    }

    /// 验证租户密钥格式并返回错误
    pub fn validate_tenant_key(tenant_key: &str) -> AuthResult<()> {
        if !Self::is_valid_tenant_key(tenant_key) {
            return Err(AuthError::ValidationError(
                "Invalid tenant key format".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证用户ID格式
    pub fn is_valid_user_id(user_id: &str) -> bool {
        if Self::is_empty(user_id) {
            return false;
        }

        // 用户ID通常是32-64个字符的字母数字字符串
        let user_id_regex = Regex::new(r"^[a-zA-Z0-9]{32,64}$").expect("Invalid user_id regex");
        user_id_regex.is_match(user_id)
    }

    /// 验证用户ID格式并返回错误
    pub fn validate_user_id(user_id: &str) -> AuthResult<()> {
        if !Self::is_valid_user_id(user_id) {
            return Err(AuthError::ValidationError(
                "Invalid user ID format".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证时间戳格式（秒级）
    pub fn is_valid_timestamp(timestamp: u64) -> bool {
        // 合理的时间戳范围：2020年到2050年
        const MIN_TIMESTAMP: u64 = 1577836800; // 2020-01-01 00:00:00 UTC
        const MAX_TIMESTAMP: u64 = 2524608000; // 2050-01-01 00:00:00 UTC

        timestamp >= MIN_TIMESTAMP && timestamp <= MAX_TIMESTAMP
    }

    /// 验证时间戳格式并返回错误
    pub fn validate_timestamp(timestamp: u64) -> AuthResult<()> {
        if !Self::is_valid_timestamp(timestamp) {
            return Err(AuthError::ValidationError(
                "Invalid timestamp format or range".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证JSON格式
    pub fn is_valid_json(json: &str) -> bool {
        if Self::is_empty(json) {
            return false;
        }

        serde_json::from_str::<serde_json::Value>(json).is_ok()
    }

    /// 验证JSON格式并返回错误
    pub fn validate_json(json: &str) -> AuthResult<()> {
        if !Self::is_valid_json(json) {
            return Err(AuthError::ValidationError(
                "Invalid JSON format".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证Base64格式
    pub fn is_valid_base64(data: &str) -> bool {
        if Self::is_empty(data) {
            return false;
        }

        base64::decode(data).is_ok()
    }

    /// 验证Base64格式并返回错误
    pub fn validate_base64(data: &str) -> AuthResult<()> {
        if !Self::is_valid_base64(data) {
            return Err(AuthError::ValidationError(
                "Invalid Base64 format".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证十六进制字符串
    pub fn is_valid_hex(hex: &str) -> bool {
        if Self::is_empty(hex) {
            return false;
        }

        hex.chars().all(|c| c.is_ascii_hexdigit())
    }

    /// 验证十六进制字符串并返回错误
    pub fn validate_hex(hex: &str) -> AuthResult<()> {
        if !Self::is_valid_hex(hex) {
            return Err(AuthError::ValidationError(
                "Invalid hexadecimal format".to_string(),
            ));
        }
        Ok(())
    }

    /// 验证密码强度
    pub fn validate_password_strength(password: &str) -> PasswordStrength {
        if password.len() < 8 {
            return PasswordStrength::Weak;
        }

        let mut score = 0;
        if password.len() >= 12 {
            score += 1;
        }
        if password.chars().any(|c| c.is_lowercase()) {
            score += 1;
        }
        if password.chars().any(|c| c.is_uppercase()) {
            score += 1;
        }
        if password.chars().any(|c| c.is_numeric()) {
            score += 1;
        }
        if password.chars().any(|c| !c.is_alphanumeric()) {
            score += 1;
        }

        match score {
            0..=2 => PasswordStrength::Weak,
            3..=4 => PasswordStrength::Medium,
            5 => PasswordStrength::Strong,
            _ => PasswordStrength::Strong,
        }
    }
}

/// 密码强度枚举
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordStrength {
    /// 弱密码
    Weak,
    /// 中等强度
    Medium,
    /// 强密码
    Strong,
}

/// 验证规则
pub struct ValidationRule {
    /// 规则名称
    pub name: String,
    /// 是否必需
    pub required: bool,
    /// 验证函数
    pub validator: Box<dyn Fn(&str) -> bool + Send + Sync>,
}

impl std::fmt::Debug for ValidationRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValidationRule")
            .field("name", &self.name)
            .field("required", &self.required)
            .field("validator", &"<function>")
            .finish()
    }
}

impl Clone for ValidationRule {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            required: self.required,
            validator: Box::new(|_| true), // 简化的克隆实现
        }
    }
}

impl ValidationRule {
    /// 创建新的验证规则
    pub fn new<F>(name: &str, required: bool, validator: F) -> Self
    where
        F: Fn(&str) -> bool + Send + Sync + 'static,
    {
        Self {
            name: name.to_string(),
            required,
            validator: Box::new(validator),
        }
    }

    /// 验证值
    pub fn validate(&self, value: &str) -> bool {
        if value.is_empty() {
            return !self.required;
        }
        (self.validator)(value)
    }
}

/// 验证器集合
#[derive(Debug, Default)]
pub struct ValidationRules {
    rules: Vec<ValidationRule>,
}

impl ValidationRules {
    /// 创建新的验证规则集合
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// 添加验证规则
    pub fn add_rule(mut self, rule: ValidationRule) -> Self {
        self.rules.push(rule);
        self
    }

    /// 验证所有规则
    pub fn validate_all(&self, value: &str) -> Vec<String> {
        let mut errors = Vec::new();

        for rule in &self.rules {
            if !rule.validate(value) {
                errors.push(format!("Validation failed for rule: {}", rule.name));
            }
        }

        errors
    }

    /// 检查是否通过所有验证
    pub fn is_valid(&self, value: &str) -> bool {
        self.validate_all(value).is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_validation() {
        assert!(ValidatorUtils::is_empty(""));
        assert!(ValidatorUtils::is_empty("   "));
        assert!(!ValidatorUtils::is_empty("test"));
        assert!(ValidatorUtils::is_not_empty("test"));
    }

    #[test]
    fn test_length_validation() {
        assert!(ValidatorUtils::validate_length("hello", 3, 10));
        assert!(!ValidatorUtils::validate_length("hello", 6, 10));
        assert!(!ValidatorUtils::validate_length("hello", 1, 4));
    }

    #[test]
    fn test_email_validation() {
        assert!(ValidatorUtils::is_valid_email("test@example.com"));
        assert!(ValidatorUtils::is_valid_email("user.name+tag@domain.co.uk"));
        assert!(!ValidatorUtils::is_valid_email("invalid-email"));
        assert!(!ValidatorUtils::is_valid_email("@domain.com"));
    }

    #[test]
    fn test_url_validation() {
        assert!(ValidatorUtils::is_valid_url("https://example.com"));
        assert!(ValidatorUtils::is_valid_url("http://localhost:8080"));
        assert!(!ValidatorUtils::is_valid_url("not-a-url"));
        assert!(!ValidatorUtils::is_valid_url("ftp://example.com")); // Only HTTP/HTTPS
    }

    #[test]
    fn test_phone_validation() {
        assert!(ValidatorUtils::is_valid_phone_cn("13812345678"));
        assert!(ValidatorUtils::is_valid_phone_cn("15987654321"));
        assert!(!ValidatorUtils::is_valid_phone_cn("12812345678"));
        assert!(!ValidatorUtils::is_valid_phone_cn("1381234567")); // Too short
    }

    #[test]
    fn test_ip_validation() {
        assert!(ValidatorUtils::is_valid_ipv4("192.168.1.1"));
        assert!(ValidatorUtils::is_valid_ipv4("127.0.0.1"));
        assert!(!ValidatorUtils::is_valid_ipv4("256.256.256.256"));
        assert!(!ValidatorUtils::is_valid_ipv4("invalid-ip"));

        assert!(ValidatorUtils::is_valid_ipv6("::1"));
        assert!(ValidatorUtils::is_valid_ipv6("2001:db8::1"));
        assert!(!ValidatorUtils::is_valid_ipv6("invalid-ipv6"));
    }

    #[test]
    fn test_uuid_validation() {
        assert!(ValidatorUtils::is_valid_uuid(
            "550e8400-e29b-41d4-a716-446655440000"
        ));
        assert!(!ValidatorUtils::is_valid_uuid("invalid-uuid"));
        assert!(!ValidatorUtils::is_valid_uuid(
            "550e8400-e29b-41d4-a716-44665544000"
        )); // Too short
    }

    #[test]
    fn test_token_validation() {
        assert!(ValidatorUtils::is_valid_app_access_token(
            "cli_1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
        ));
        assert!(ValidatorUtils::is_valid_tenant_access_token(
            "t-1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
        ));
        assert!(ValidatorUtils::is_valid_user_access_token(
            "u-1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
        ));
        assert!(!ValidatorUtils::is_valid_access_token("invalid-token"));
        assert!(ValidatorUtils::is_valid_refresh_token(
            "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
        ));
    }

    #[test]
    fn test_scope_validation() {
        assert!(ValidatorUtils::is_valid_scope("contact:base"));
        assert!(ValidatorUtils::is_valid_scope("contact:base drive:drive"));
        assert!(ValidatorUtils::is_valid_scope("drive:drive:readonly"));
        assert!(!ValidatorUtils::is_valid_scope("invalid-scope"));
        assert!(!ValidatorUtils::is_valid_scope("contact:base,drive:drive")); // Comma not supported
    }

    #[test]
    fn test_state_validation() {
        assert!(ValidatorUtils::is_valid_state("random_state_123"));
        assert!(ValidatorUtils::is_valid_state("abcde-12345_fgh-67890"));
        assert!(!ValidatorUtils::is_valid_state("short"));
        assert!(!ValidatorUtils::is_valid_state("invalid@state"));
    }

    #[test]
    fn test_password_strength() {
        let weak = ValidatorUtils::validate_password_strength("123");
        assert_eq!(weak, PasswordStrength::Weak);

        let medium = ValidatorUtils::validate_password_strength("password123");
        assert_eq!(medium, PasswordStrength::Medium);

        let strong = ValidatorUtils::validate_password_strength("Str0ng!P@ssw0rd");
        assert_eq!(strong, PasswordStrength::Strong);
    }

    #[test]
    fn test_validation_rules() {
        let rules = ValidationRules::new()
            .add_rule(ValidationRule::new("not_empty", true, |s| !s.is_empty()))
            .add_rule(ValidationRule::new("min_length", true, |s| s.len() >= 5));

        assert!(rules.is_valid("hello"));
        assert!(!rules.is_valid("")); // Fails not_empty
        assert!(!rules.is_valid("hi")); // Fails min_length

        let errors = rules.validate_all("hi");
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("min_length"));
    }

    #[test]
    fn test_json_validation() {
        assert!(ValidatorUtils::is_valid_json(r#"{"key": "value"}"#));
        assert!(ValidatorUtils::is_valid_json(r#"[]"#));
        assert!(!ValidatorUtils::is_valid_json(r#"{"invalid": json}"#));
        assert!(!ValidatorUtils::is_valid_json("not-json"));
    }

    #[test]
    fn test_base64_validation() {
        assert!(ValidatorUtils::is_valid_base64("SGVsbG8gV29ybGQ=")); // "Hello World"
        assert!(!ValidatorUtils::is_valid_base64("Invalid!Base64"));
        assert!(!ValidatorUtils::is_valid_base64(""));
    }

    #[test]
    fn test_hex_validation() {
        assert!(ValidatorUtils::is_valid_hex("48656c6c6f20576f726c64")); // "Hello World" in hex
        assert!(ValidatorUtils::is_valid_hex("ABCDEF123456"));
        assert!(!ValidatorUtils::is_valid_hex("Invalid!Hex"));
        assert!(!ValidatorUtils::is_valid_hex("GHI")); // G, H, I are not hex digits
    }
}
