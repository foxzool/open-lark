//! 令牌验证工具

/// 令牌验证器
pub struct TokenValidator;

impl TokenValidator {
    /// 验证令牌格式
    pub fn validate_token_format(token: &str) -> bool {
        !token.is_empty() && token.len() >= 10
    }

    /// 验证应用ID格式
    pub fn validate_app_id(app_id: &str) -> bool {
        !app_id.is_empty() && app_id.len() >= 1
    }

    /// 验证应用密钥格式
    pub fn validate_app_secret(app_secret: &str) -> bool {
        !app_secret.is_empty() && app_secret.len() >= 8
    }
}
