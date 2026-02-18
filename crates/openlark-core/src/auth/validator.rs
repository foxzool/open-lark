//! 令牌验证器

use std::time::{Duration, SystemTime};
use tracing::{debug, warn};

use super::token::{TokenInfo, TokenType, TokenValidationResult};

/// 令牌验证器
pub struct TokenValidator {
    /// 提前过期时间（秒）
    expiry_threshold_seconds: u64,
}

impl TokenValidator {
    /// 创建新的令牌验证器
    pub fn new(expiry_threshold_seconds: u64) -> Self {
        Self {
            expiry_threshold_seconds,
        }
    }

    /// 验证令牌
    pub fn validate(&self, token: &TokenInfo) -> TokenValidationResult {
        if token.access_token.is_empty() {
            warn!("Token validation failed: empty access token");
            return TokenValidationResult::Invalid("validation failed".to_string());
        }

        let now = SystemTime::now();

        // 检查是否已过期
        if now >= token.expires_at {
            debug!("Token validation result: expired");
            return TokenValidationResult::Expired;
        }

        // 检查是否即将过期
        let threshold = Duration::from_secs(self.expiry_threshold_seconds);
        if now + threshold >= token.expires_at {
            let time_until_expiry = token.expires_at.duration_since(now).unwrap_or_default();
            debug!("Token validation result: expiring soon");
            return TokenValidationResult::ExpiringSoon(time_until_expiry);
        }

        debug!("Token validation result: valid");
        TokenValidationResult::Valid
    }

    /// 验证令牌字符串格式
    pub fn validate_token_format(&self, token: &str) -> TokenValidationResult {
        if token.is_empty() {
            return TokenValidationResult::Invalid("validation failed".to_string());
        }

        // 检查令牌前缀
        if token.starts_with("cli_") {
            // 应用访问令牌格式检查
            if token.len() < 10 {
                return TokenValidationResult::Invalid("validation failed".to_string());
            }
        } else if token.starts_with("t-") {
            // 租户访问令牌格式检查
            if token.len() < 8 {
                return TokenValidationResult::Invalid("validation failed".to_string());
            }
        } else if token.starts_with("u-") {
            // 用户访问令牌格式检查
            if token.len() < 8 {
                return TokenValidationResult::Invalid("validation failed".to_string());
            }
        } else {
            // 未知令牌格式
            return TokenValidationResult::Invalid("validation failed".to_string());
        }

        TokenValidationResult::Valid
    }

    /// 验证令牌类型匹配
    pub fn validate_token_type(
        &self,
        token: &str,
        expected_type: TokenType,
    ) -> TokenValidationResult {
        let format_result = self.validate_token_format(token);
        if !format_result.is_valid() {
            return format_result;
        }

        match expected_type {
            TokenType::AppAccessToken => {
                if token.starts_with("cli_") {
                    TokenValidationResult::Valid
                } else {
                    TokenValidationResult::Invalid("invalid app access token format".to_string())
                }
            }
            TokenType::TenantAccessToken => {
                if token.starts_with("t-") {
                    TokenValidationResult::Valid
                } else {
                    TokenValidationResult::Invalid("invalid tenant access token format".to_string())
                }
            }
            TokenType::UserAccessToken => {
                if token.starts_with("u-") {
                    TokenValidationResult::Valid
                } else {
                    TokenValidationResult::Invalid("invalid user access token format".to_string())
                }
            }
        }
    }

    /// 检查令牌是否需要刷新
    pub fn should_refresh(&self, token: &TokenInfo) -> bool {
        !self.validate(token).is_valid()
    }

    /// 获取令牌剩余有效时间
    pub fn get_remaining_time(&self, token: &TokenInfo) -> Option<Duration> {
        if token.is_expired() {
            None
        } else {
            token.time_until_expiry()
        }
    }

    /// 批量验证令牌
    pub fn validate_batch(&self, tokens: &[TokenInfo]) -> Vec<TokenValidationResult> {
        tokens.iter().map(|token| self.validate(token)).collect()
    }

    /// 过滤有效令牌
    pub fn filter_valid_tokens<'a>(&self, tokens: &'a [TokenInfo]) -> Vec<&'a TokenInfo> {
        tokens
            .iter()
            .filter(|token| self.validate(token).is_valid())
            .collect()
    }

    /// 过滤需要刷新的令牌
    pub fn filter_tokens_needing_refresh<'a>(&self, tokens: &'a [TokenInfo]) -> Vec<&'a TokenInfo> {
        tokens
            .iter()
            .filter(|token| self.should_refresh(token))
            .collect()
    }
}

impl Default for TokenValidator {
    fn default() -> Self {
        Self::new(300) // 默认5分钟提前过期时间
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_token(expires_in_secs: u64) -> TokenInfo {
        TokenInfo::new(
            "cli_test_token".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(expires_in_secs),
            "test_app".to_string(),
        )
    }

    #[test]
    fn test_valid_token() {
        let validator = TokenValidator::default();
        let token = create_test_token(3600); // 1小时后过期

        let result = validator.validate(&token);
        assert!(result.is_valid());
    }

    #[test]
    fn test_expired_token() {
        let validator = TokenValidator::default();
        let token = create_test_token(0); // 立即过期

        let result = validator.validate(&token);
        assert_eq!(result, TokenValidationResult::Expired);
    }

    #[test]
    fn test_expiring_soon_token() {
        let validator = TokenValidator::new(3600); // 1小时提前过期时间
        let token = create_test_token(1800); // 30分钟后过期

        let result = validator.validate(&token);
        assert!(matches!(result, TokenValidationResult::ExpiringSoon(_)));
    }

    #[test]
    fn test_token_format_validation() {
        let validator = TokenValidator::default();

        // 有效格式
        assert!(validator.validate_token_format("cli_test123").is_valid());
        assert!(validator.validate_token_format("t-test123").is_valid());
        assert!(validator.validate_token_format("u-test123").is_valid());

        // 无效格式
        assert!(validator.validate_token_format("").is_invalid());
        assert!(validator.validate_token_format("short").is_invalid());
        assert!(validator
            .validate_token_format("unknown_prefix")
            .is_invalid());
    }

    #[test]
    fn test_token_type_validation() {
        let validator = TokenValidator::default();

        // 应用访问令牌
        assert!(validator
            .validate_token_type("cli_test123", TokenType::AppAccessToken)
            .is_valid());
        assert!(validator
            .validate_token_type("t-test123", TokenType::AppAccessToken)
            .is_invalid());

        // 租户访问令牌
        assert!(validator
            .validate_token_type("t-test123", TokenType::TenantAccessToken)
            .is_valid());
        assert!(validator
            .validate_token_type("cli_test123", TokenType::TenantAccessToken)
            .is_invalid());

        // 用户访问令牌
        assert!(validator
            .validate_token_type("u-test123", TokenType::UserAccessToken)
            .is_valid());
        assert!(validator
            .validate_token_type("cli_test123", TokenType::UserAccessToken)
            .is_invalid());
    }

    #[test]
    fn test_batch_validation() {
        let validator = TokenValidator::default();
        let tokens = vec![
            create_test_token(3600), // 有效
            create_test_token(0),    // 过期
            create_test_token(1800), // 有效
        ];

        let results = validator.validate_batch(&tokens);
        assert_eq!(results[0], TokenValidationResult::Valid);
        assert_eq!(results[1], TokenValidationResult::Expired);
        assert_eq!(results[2], TokenValidationResult::Valid);
    }

    #[test]
    fn test_filter_tokens() {
        let validator = TokenValidator::default();
        let tokens = vec![
            create_test_token(3600), // 有效
            create_test_token(0),    // 过期
            create_test_token(1800), // 有效
        ];

        let valid_tokens = validator.filter_valid_tokens(&tokens);
        assert_eq!(valid_tokens.len(), 2);

        let refresh_needed = validator.filter_tokens_needing_refresh(&tokens);
        assert_eq!(refresh_needed.len(), 1); // 只有过期的令牌需要刷新
    }

    #[test]
    fn test_validate_rejects_empty_access_token() {
        let validator = TokenValidator::default();
        let mut token = create_test_token(3600);
        token.access_token.clear();

        let result = validator.validate(&token);
        assert!(matches!(result, TokenValidationResult::Invalid(_)));
    }

    #[test]
    fn test_validate_token_format_length_boundaries() {
        let validator = TokenValidator::default();

        assert!(validator.validate_token_format("cli_123456").is_valid());
        assert!(validator.validate_token_format("cli_12345").is_invalid());

        assert!(validator.validate_token_format("t-123456").is_valid());
        assert!(validator.validate_token_format("t-12345").is_invalid());

        assert!(validator.validate_token_format("u-123456").is_valid());
        assert!(validator.validate_token_format("u-12345").is_invalid());
    }

    #[test]
    fn test_validate_token_type_mismatch_returns_specific_message() {
        let validator = TokenValidator::default();

        let result = validator.validate_token_type("u-test123", TokenType::AppAccessToken);
        match result {
            TokenValidationResult::Invalid(msg) => {
                assert_eq!(msg, "invalid app access token format");
            }
            _ => panic!("expected invalid token type result"),
        }
    }

    #[test]
    fn test_should_refresh_for_expiring_soon_and_expired() {
        let validator = TokenValidator::new(600);

        let expiring_soon = create_test_token(300);
        let expired = create_test_token(0);
        let long_lived = create_test_token(3600);

        assert!(validator.should_refresh(&expiring_soon));
        assert!(validator.should_refresh(&expired));
        assert!(!validator.should_refresh(&long_lived));
    }

    #[test]
    fn test_get_remaining_time_for_valid_and_expired_tokens() {
        let validator = TokenValidator::default();
        let valid = create_test_token(1200);
        let expired = create_test_token(0);

        let remaining = validator
            .get_remaining_time(&valid)
            .expect("valid token should have remaining time");
        assert!(remaining.as_secs() <= 1200);
        assert!(remaining.as_secs() > 0);

        assert!(validator.get_remaining_time(&expired).is_none());
    }

    #[test]
    fn test_filter_tokens_needing_refresh_includes_expiring_soon() {
        let validator = TokenValidator::new(3600);
        let tokens = vec![
            create_test_token(3601), // 不需要刷新
            create_test_token(3500), // 即将过期
            create_test_token(0),    // 已过期
        ];

        let refresh_needed = validator.filter_tokens_needing_refresh(&tokens);
        assert_eq!(refresh_needed.len(), 2);
    }

    // 新增测试：空令牌列表的批量验证
    #[test]
    fn test_validate_batch_empty_list() {
        let validator = TokenValidator::default();
        let tokens: Vec<TokenInfo> = vec![];

        let results = validator.validate_batch(&tokens);
        assert!(results.is_empty());
    }

    // 新增测试：空令牌列表的过滤
    #[test]
    fn test_filter_valid_tokens_empty_list() {
        let validator = TokenValidator::default();
        let tokens: Vec<TokenInfo> = vec![];

        let valid = validator.filter_valid_tokens(&tokens);
        assert!(valid.is_empty());

        let need_refresh = validator.filter_tokens_needing_refresh(&tokens);
        assert!(need_refresh.is_empty());
    }

    // 新增测试：验证器默认配置
    #[test]
    fn test_validator_default_configuration() {
        let validator = TokenValidator::default();
        let token = create_test_token(240); // 4分钟后过期（小于默认5分钟阈值）

        let result = validator.validate(&token);
        assert!(matches!(result, TokenValidationResult::ExpiringSoon(_)));
    }

    // 新增测试：验证器自定义阈值
    #[test]
    fn test_validator_custom_threshold() {
        let validator = TokenValidator::new(60); // 1分钟阈值
        let token = create_test_token(120); // 2分钟后过期

        let result = validator.validate(&token);
        assert!(result.is_valid()); // 2分钟大于1分钟阈值，应该是有效的

        let token_near = create_test_token(30); // 30秒后过期
        let result_near = validator.validate(&token_near);
        assert!(matches!(
            result_near,
            TokenValidationResult::ExpiringSoon(_)
        ));
    }

    // 新增测试：零阈值验证器
    #[test]
    fn test_validator_zero_threshold() {
        let validator = TokenValidator::new(0);
        let token = create_test_token(1); // 1秒后过期

        let result = validator.validate(&token);
        // 0阈值意味着只有实际过期才算过期
        assert!(result.is_valid() || matches!(result, TokenValidationResult::ExpiringSoon(_)));
    }

    // 新增测试：所有令牌类型的格式验证
    #[test]
    fn test_validate_token_format_all_types() {
        let validator = TokenValidator::default();

        // App Access Token
        assert!(validator.validate_token_format("cli_1234567890").is_valid());
        assert!(validator.validate_token_format("cli_123").is_invalid()); // 太短

        // Tenant Access Token
        assert!(validator.validate_token_format("t-12345678").is_valid());
        assert!(validator.validate_token_format("t-123").is_invalid()); // 太短

        // User Access Token
        assert!(validator.validate_token_format("u-12345678").is_valid());
        assert!(validator.validate_token_format("u-123").is_invalid()); // 太短
    }

    // 新增测试：不同租户类型的令牌验证
    #[test]
    fn test_validate_token_with_different_app_types() {
        let validator = TokenValidator::default();

        let self_build = TokenInfo::new(
            "cli_self_build".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(3600),
            "self_build".to_string(),
        );
        let marketplace = TokenInfo::new(
            "cli_marketplace".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(3600),
            "marketplace".to_string(),
        );

        assert!(validator.validate(&self_build).is_valid());
        assert!(validator.validate(&marketplace).is_valid());
    }

    // 新增测试：包含刷新令牌的验证
    #[test]
    fn test_validate_token_with_refresh_token() {
        let validator = TokenValidator::default();
        let mut token = create_test_token(3600);
        token.refresh_token = Some("refresh_123".to_string());

        let result = validator.validate(&token);
        assert!(result.is_valid());
    }

    // 新增测试：超大阈值验证
    #[test]
    fn test_validator_large_threshold() {
        let validator = TokenValidator::new(86400); // 24小时阈值
        let token = create_test_token(3600); // 1小时后过期

        let result = validator.validate(&token);
        // 1小时小于24小时阈值，应该标记为即将过期
        assert!(matches!(result, TokenValidationResult::ExpiringSoon(_)));
    }

    // 新增测试：精确边界测试
    #[test]
    fn test_validator_exact_boundary() {
        let threshold = 300; // 5分钟
        let validator = TokenValidator::new(threshold);
        let token = create_test_token(threshold); // 正好在阈值边界

        let result = validator.validate(&token);
        // 正好在边界上应该被视为即将过期
        assert!(matches!(result, TokenValidationResult::ExpiringSoon(_)));
    }

    // 新增测试：全部令牌过期的批量验证
    #[test]
    fn test_validate_batch_all_expired() {
        let validator = TokenValidator::default();
        let tokens = vec![
            create_test_token(0),
            create_test_token(0),
            create_test_token(0),
        ];

        let results = validator.validate_batch(&tokens);
        assert_eq!(results.len(), 3);
        assert!(results
            .iter()
            .all(|r| matches!(r, TokenValidationResult::Expired)));
    }

    // 新增测试：全部令牌有效的批量验证
    #[test]
    fn test_validate_batch_all_valid() {
        let validator = TokenValidator::default();
        let tokens = vec![
            create_test_token(3600),
            create_test_token(7200),
            create_test_token(600), // 10分钟，大于默认5分钟阈值
        ];

        let results = validator.validate_batch(&tokens);
        assert_eq!(results.len(), 3);
        // 所有令牌都大于默认5分钟阈值，应该是有效的
        assert_eq!(results[0], TokenValidationResult::Valid);
        assert_eq!(results[1], TokenValidationResult::Valid);
        assert_eq!(results[2], TokenValidationResult::Valid);
    }

    // 新增测试：过滤器返回正确引用
    #[test]
    fn test_filter_valid_tokens_returns_correct_references() {
        let validator = TokenValidator::default();
        let tokens = vec![
            TokenInfo::new(
                "cli_valid1".to_string(),
                TokenType::AppAccessToken,
                Duration::from_secs(3600),
                "test".to_string(),
            ),
            TokenInfo::new(
                "cli_expired".to_string(),
                TokenType::AppAccessToken,
                Duration::from_secs(0),
                "test".to_string(),
            ),
            TokenInfo::new(
                "cli_valid2".to_string(),
                TokenType::AppAccessToken,
                Duration::from_secs(7200),
                "test".to_string(),
            ),
        ];

        std::thread::sleep(Duration::from_millis(5));

        let valid_refs = validator.filter_valid_tokens(&tokens);
        assert_eq!(valid_refs.len(), 2);
        assert_eq!(valid_refs[0].access_token, "cli_valid1");
        assert_eq!(valid_refs[1].access_token, "cli_valid2");
    }
}
