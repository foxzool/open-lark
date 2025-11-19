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
}
