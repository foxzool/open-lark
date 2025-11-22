//! 令牌类型和管理定义
//!
//! 此模块提供了 OpenLark SDK 中令牌管理的核心数据结构和功能，包括：
//!
//! - **令牌类型定义**: 不同类型的访问令牌（应用、租户、用户）
//! - **令牌信息**: 完整的令牌元数据和管理功能
//! - **令牌验证**: 令牌有效性检查和验证结果
//! - **刷新配置**: 令牌刷新的配置参数
//! - **管理接口**: 令牌管理的抽象接口
//!
//! # 快速开始
//!
//! ```rust
//! use openlark_core::auth::token::*;
//! use std::time::Duration;
//!
//! // 创建一个新的应用令牌
//! let app_token = TokenInfo::new(
//!     "cli_xxxxxxxx".to_string(),
//!     TokenType::AppAccessToken,
//!     Duration::from_secs(3600),
//!     "self_build".to_string()
//! );
//!
//! // 检查令牌是否过期
//! if !app_token.is_expired() {
//!     println!("令牌有效，剩余时间: {} 秒", app_token.expires_in_seconds());
//! }
//!
//! // 检查令牌是否即将过期
//! if app_token.is_expiring_soon(600) {
//!     println!("令牌即将过期，建议刷新");
//! }
//! ```
//!
//! # 令牌类型
//!
//! OpenLark 支持三种主要的令牌类型：
//!
//! - **AppAccessToken** (`cli_`): 应用访问令牌，用于应用级 API 调用
//! - **TenantAccessToken** (`t-`): 租户访问令牌，用于租户级 API 调用
//! - **UserAccessToken** (`u-`): 用户访问令牌，用于用户级 API 调用
//!
//! # 令牌生命周期管理
//!
//! ## 创建和配置
//!
//! ```rust
//! // 使用默认配置
//! let refresh_config = TokenRefreshConfig::default();
//!
//! // 自定义配置
//! let custom_config = TokenRefreshConfig {
//!     refresh_ahead_seconds: 600,  // 提前10分钟刷新
//!     max_retry_attempts: 5,        // 最大重试次数
//!     retry_interval_base: 2,       // 基础重试间隔（秒）
//!     retry_interval_max: 60,       // 最大重试间隔（秒）
//!     auto_refresh: true,           // 自动刷新
//! };
//! ```
//!
//! ## 令牌验证
//!
//! ```rust
//! let validation_result = TokenValidationResult::Valid;
//! if validation_result.is_valid() {
//!     println!("令牌验证通过");
//! } else {
//!     println!("令牌验证失败: {:?}", validation_result.error_message());
//! }
//! ```
//!
//! # 最佳实践
//!
//! 1. **提前刷新**: 建议在令牌过期前 5-10 分钟进行刷新
//! 2. **错误处理**: 总是检查令牌验证结果，处理无效令牌
//! 3. **缓存策略**: 使用内存缓存减少网络请求
//! 4. **监控统计**: 跟踪令牌访问次数和刷新频率
//!
//! # 错误处理
//!
//! 令牌相关错误通过 `TokenValidationResult` 和 `SDKResult` 处理：
//!
//! ```rust
//! match validate_token(token) {
//!     TokenValidationResult::Valid => {
//!         println!("令牌有效");
//!     }
//!     TokenValidationResult::Invalid(msg) => {
//!         eprintln!("令牌无效: {}", msg);
//!     }
//! }
//! ```

use crate::error::SDKResult;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// 令牌类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    /// 应用访问令牌
    AppAccessToken,
    /// 租户访问令牌
    TenantAccessToken,
    /// 用户访问令牌
    UserAccessToken,
}

impl TokenType {
    /// 获取令牌类型字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenType::AppAccessToken => "app_access_token",
            TokenType::TenantAccessToken => "tenant_access_token",
            TokenType::UserAccessToken => "user_access_token",
        }
    }

    /// 获取令牌前缀
    pub fn prefix(&self) -> &'static str {
        match self {
            TokenType::AppAccessToken => "cli_",
            TokenType::TenantAccessToken => "t-",
            TokenType::UserAccessToken => "u-",
        }
    }
}

/// 令牌信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    /// 访问令牌
    pub access_token: String,
    /// 刷新令牌（可选）
    pub refresh_token: Option<String>,
    /// 过期时间
    pub expires_at: SystemTime,
    /// 令牌类型
    pub token_type: TokenType,
    /// 应用类型
    pub app_type: String,
    /// 租户ID（可选）
    pub tenant_key: Option<String>,
    /// 创建时间
    pub created_at: SystemTime,
    /// 最后访问时间
    pub last_accessed_at: SystemTime,
    /// 访问次数
    pub access_count: u64,
}

impl TokenInfo {
    /// 创建新的令牌信息
    pub fn new(
        access_token: String,
        token_type: TokenType,
        expires_in: Duration,
        app_type: String,
    ) -> Self {
        let now = SystemTime::now();
        let expires_at = now + expires_in;

        Self {
            access_token,
            refresh_token: None,
            expires_at,
            token_type,
            app_type,
            tenant_key: None,
            created_at: now,
            last_accessed_at: now,
            access_count: 0,
        }
    }

    /// 检查令牌是否已过期
    pub fn is_expired(&self) -> bool {
        SystemTime::now() >= self.expires_at
    }

    /// 检查令牌是否即将过期（指定秒数内）
    pub fn is_expiring_soon(&self, threshold_seconds: u64) -> bool {
        let threshold = Duration::from_secs(threshold_seconds);
        SystemTime::now() + threshold >= self.expires_at
    }

    /// 获取剩余有效时间
    pub fn time_until_expiry(&self) -> Option<Duration> {
        if self.is_expired() {
            None
        } else {
            self.expires_at.duration_since(SystemTime::now()).ok()
        }
    }

    /// 更新访问信息
    pub fn update_access(&mut self) {
        self.last_accessed_at = SystemTime::now();
        self.access_count += 1;
    }

    /// 获取令牌的剩余有效时间（秒）
    pub fn expires_in_seconds(&self) -> u64 {
        if self.is_expired() {
            0
        } else {
            self.expires_at
                .duration_since(SystemTime::now())
                .unwrap_or_default()
                .as_secs()
        }
    }
}

impl Default for TokenInfo {
    fn default() -> Self {
        let now = SystemTime::now();
        Self {
            access_token: String::new(),
            refresh_token: None,
            expires_at: now + Duration::from_secs(3600),
            token_type: TokenType::AppAccessToken,
            app_type: String::new(),
            tenant_key: None,
            created_at: now,
            last_accessed_at: now,
            access_count: 0,
        }
    }
}

/// 令牌验证结果
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenValidationResult {
    /// 令牌有效
    Valid,
    /// 令牌已过期
    Expired,
    /// 令牌无效
    Invalid(String),
    /// 令牌即将过期
    ExpiringSoon(Duration),
}

impl TokenValidationResult {
    /// 检查验证结果是否有效
    pub fn is_valid(&self) -> bool {
        matches!(self, TokenValidationResult::Valid)
    }

    /// 检查验证结果是否无效
    pub fn is_invalid(&self) -> bool {
        matches!(self, TokenValidationResult::Invalid(_))
    }

    /// 检查是否需要刷新
    pub fn needs_refresh(&self) -> bool {
        matches!(
            self,
            TokenValidationResult::Expired | TokenValidationResult::ExpiringSoon(_)
        )
    }

    /// 获取错误消息
    pub fn error_message(&self) -> String {
        match self {
            TokenValidationResult::Invalid(msg) => msg.clone(),
            TokenValidationResult::Expired => "令牌已过期".to_string(),
            TokenValidationResult::ExpiringSoon(duration) => {
                format!("令牌将在{}秒后过期", duration.as_secs())
            }
            TokenValidationResult::Valid => "令牌有效".to_string(),
        }
    }
}

/// 令牌刷新配置
#[derive(Debug, Clone)]
pub struct TokenRefreshConfig {
    /// 提前刷新时间（秒）
    pub refresh_ahead_seconds: u64,
    /// 最大重试次数
    pub max_retry_attempts: u32,
    /// 重试间隔基数（秒）
    pub retry_interval_base: u64,
    /// 重试间隔最大值（秒）
    pub retry_interval_max: u64,
    /// 是否启用自动刷新
    pub auto_refresh: bool,
}

impl Default for TokenRefreshConfig {
    fn default() -> Self {
        Self {
            refresh_ahead_seconds: 300, // 5分钟
            max_retry_attempts: 3,
            retry_interval_base: 1, // 1秒
            retry_interval_max: 30, // 30秒
            auto_refresh: true,
        }
    }
}

/// 令牌管理器特征
#[allow(async_fn_in_trait)]
pub trait TokenManager: Send + Sync {
    /// 获取访问令牌
    async fn get_access_token(&self, token_type: TokenType) -> SDKResult<TokenInfo>;

    /// 刷新访问令牌
    async fn refresh_token(&self, refresh_token: &str) -> SDKResult<TokenInfo>;

    /// 验证令牌
    fn validate_token(&self, token: &str) -> TokenValidationResult;

    /// 清除无效令牌
    async fn invalidate_token(&self, token: &str) -> SDKResult<()>;

    /// 获取所有有效令牌
    async fn list_valid_tokens(&self) -> SDKResult<Vec<TokenInfo>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_token_type_display() {
        assert_eq!(TokenType::AppAccessToken.as_str(), "app_access_token");
        assert_eq!(TokenType::TenantAccessToken.as_str(), "tenant_access_token");
        assert_eq!(TokenType::UserAccessToken.as_str(), "user_access_token");
    }

    #[test]
    fn test_token_type_prefix() {
        assert_eq!(TokenType::AppAccessToken.prefix(), "cli_");
        assert_eq!(TokenType::TenantAccessToken.prefix(), "t-");
        assert_eq!(TokenType::UserAccessToken.prefix(), "u-");
    }

    #[test]
    fn test_token_info_creation() {
        let token = TokenInfo::new(
            "test_token".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(3600),
            "test_app".to_string(),
        );

        assert_eq!(token.access_token, "test_token");
        assert_eq!(token.token_type, TokenType::AppAccessToken);
        assert_eq!(token.app_type, "test_app");
        assert!(!token.is_expired());
        assert_eq!(token.access_count, 0);
        let expires_in = token.expires_in_seconds();
        assert!(
            expires_in >= 3599 && expires_in <= 3600,
            "expires_in_seconds should be ~3600, got {}",
            expires_in
        );
    }

    #[test]
    fn test_token_info_expiry() {
        let expired_token = TokenInfo::new(
            "expired".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(0),
            "test".to_string(),
        );

        assert!(expired_token.is_expired());
        assert_eq!(expired_token.expires_in_seconds(), 0);
    }

    #[test]
    fn test_token_info_update_access() {
        let mut token = TokenInfo::new(
            "test".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(3600),
            "test".to_string(),
        );

        let original_count = token.access_count;
        token.update_access();

        assert_eq!(token.access_count, original_count + 1);
    }

    #[test]
    fn test_token_info_is_expiring_soon() {
        let token = TokenInfo::new(
            "test".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(300), // 5 minutes
            "test".to_string(),
        );

        // 应该在300秒内过期
        assert!(token.is_expiring_soon(600));
        assert!(token.is_expiring_soon(300));
        assert!(!token.is_expiring_soon(60));
    }

    #[test]
    fn test_token_validation_result() {
        let valid_result = TokenValidationResult::Valid;
        assert!(valid_result.is_valid());
        assert!(!valid_result.is_invalid());

        let invalid_result = TokenValidationResult::Invalid("test error".to_string());
        assert!(!invalid_result.is_valid());
        assert!(invalid_result.is_invalid());
        assert_eq!(invalid_result.error_message(), "test error");
    }

    #[test]
    fn test_token_refresh_config_default() {
        let config = TokenRefreshConfig::default();
        assert_eq!(config.refresh_ahead_seconds, 300);
        assert_eq!(config.max_retry_attempts, 3);
        assert_eq!(config.retry_interval_base, 1);
        assert_eq!(config.retry_interval_max, 30);
    }

    #[test]
    fn test_token_info_default() {
        let default_token = TokenInfo::default();
        assert_eq!(default_token.access_token, "");
        assert_eq!(default_token.token_type, TokenType::AppAccessToken);
        assert!(!default_token.is_expired()); // 默认1小时有效期
        assert_eq!(default_token.access_count, 0);
    }

    #[test]
    fn test_time_until_expiry() {
        let token = TokenInfo::new(
            "test".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(3600),
            "test".to_string(),
        );

        let time_until = token.time_until_expiry();
        assert!(time_until.is_some());
        assert!(time_until.unwrap().as_secs() > 3500); // 允许一些时间差
        assert!(time_until.unwrap().as_secs() <= 3600);
    }

    #[test]
    fn test_expired_token_time_until_expiry() {
        let expired_token = TokenInfo::new(
            "expired".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(0),
            "test".to_string(),
        );

        assert!(expired_token.time_until_expiry().is_none());
    }
}
