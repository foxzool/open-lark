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
    use std::collections::HashMap;
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
            (3599..=3600).contains(&expires_in),
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

    #[test]
    fn test_expiring_soon_exact_threshold_boundary() {
        let token = TokenInfo::new(
            "boundary_token".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(2),
            "test".to_string(),
        );

        assert!(token.is_expiring_soon(2));
    }

    #[test]
    fn test_expiring_soon_below_threshold_boundary() {
        let token = TokenInfo::new(
            "boundary_token".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(5),
            "test".to_string(),
        );

        assert!(!token.is_expiring_soon(2));
    }

    #[test]
    fn test_token_validation_needs_refresh_states() {
        let expiring_soon = TokenValidationResult::ExpiringSoon(Duration::from_secs(30));
        let expired = TokenValidationResult::Expired;
        let valid = TokenValidationResult::Valid;

        assert!(expiring_soon.needs_refresh());
        assert!(expired.needs_refresh());
        assert!(!valid.needs_refresh());
    }

    #[test]
    fn test_refresh_retry_interval_is_capped() {
        let config = TokenRefreshConfig {
            refresh_ahead_seconds: 120,
            max_retry_attempts: 5,
            retry_interval_base: 2,
            retry_interval_max: 10,
            auto_refresh: true,
        };

        let calc_interval = |attempt: u32| {
            let exponential = config.retry_interval_base.saturating_mul(1u64 << attempt);
            exponential.min(config.retry_interval_max)
        };

        assert_eq!(calc_interval(0), 2);
        assert_eq!(calc_interval(1), 4);
        assert_eq!(calc_interval(2), 8);
        assert_eq!(calc_interval(3), 10);
        assert_eq!(calc_interval(4), 10);
    }

    #[test]
    fn test_token_refresh_config_auto_refresh_toggle() {
        let mut config = TokenRefreshConfig::default();
        assert!(config.auto_refresh);

        config.auto_refresh = false;
        assert!(!config.auto_refresh);
    }

    #[test]
    fn test_token_cache_hit_miss_and_expiry_cleanup() {
        let mut cache: HashMap<String, TokenInfo> = HashMap::new();

        let valid_token = TokenInfo::new(
            "cli_valid".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(120),
            "test".to_string(),
        );
        let expired_token = TokenInfo::new(
            "cli_expired".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(0),
            "test".to_string(),
        );

        cache.insert("valid".to_string(), valid_token);
        cache.insert("expired".to_string(), expired_token);

        let valid_hit = cache
            .get("valid")
            .expect("valid token should exist in cache");
        assert!(!valid_hit.is_expired());

        let expired_hit = cache
            .get("expired")
            .expect("expired token should exist in cache");
        assert!(expired_hit.is_expired());

        cache.retain(|_, token| !token.is_expired());

        assert!(cache.contains_key("valid"));
        assert!(!cache.contains_key("expired"));
        assert!(!cache.contains_key("missing"));
    }

    #[test]
    fn test_token_cache_access_updates_lifecycle_metadata() {
        let mut cache: HashMap<String, TokenInfo> = HashMap::new();
        let token = TokenInfo::new(
            "cli_cached".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(300),
            "test".to_string(),
        );
        cache.insert("session".to_string(), token);

        {
            let cached = cache.get_mut("session").expect("cached token should exist");
            cached.update_access();
            cached.update_access();
        }

        let cached = cache
            .get("session")
            .expect("cached token should still exist");
        assert_eq!(cached.access_count, 2);
        assert!(cached.last_accessed_at >= cached.created_at);
    }

    // 新增测试：Token 类型完整测试
    #[test]
    fn test_all_token_types_properties() {
        let app_token = TokenInfo::new(
            "cli_test123".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(3600),
            "self_build".to_string(),
        );
        let tenant_token = TokenInfo::new(
            "t-test456".to_string(),
            TokenType::TenantAccessToken,
            Duration::from_secs(3600),
            "marketplace".to_string(),
        );
        let user_token = TokenInfo::new(
            "u-test789".to_string(),
            TokenType::UserAccessToken,
            Duration::from_secs(3600),
            "oauth".to_string(),
        );

        // 验证 TokenType 属性
        assert_eq!(TokenType::AppAccessToken.as_str(), "app_access_token");
        assert_eq!(TokenType::TenantAccessToken.as_str(), "tenant_access_token");
        assert_eq!(TokenType::UserAccessToken.as_str(), "user_access_token");

        assert_eq!(TokenType::AppAccessToken.prefix(), "cli_");
        assert_eq!(TokenType::TenantAccessToken.prefix(), "t-");
        assert_eq!(TokenType::UserAccessToken.prefix(), "u-");

        // 验证创建的 Token
        assert_eq!(app_token.token_type, TokenType::AppAccessToken);
        assert_eq!(tenant_token.token_type, TokenType::TenantAccessToken);
        assert_eq!(user_token.token_type, TokenType::UserAccessToken);
    }

    // 新增测试：Token 带有刷新令牌
    #[test]
    fn test_token_with_refresh_token() {
        let mut token = TokenInfo::new(
            "cli_access".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(3600),
            "oauth".to_string(),
        );

        assert!(token.refresh_token.is_none());

        token.refresh_token = Some("refresh_token_123".to_string());
        assert_eq!(token.refresh_token, Some("refresh_token_123".to_string()));
    }

    // 新增测试：Token 带有租户信息
    #[test]
    fn test_token_with_tenant_key() {
        let mut token = TokenInfo::new(
            "t-tenant_token".to_string(),
            TokenType::TenantAccessToken,
            Duration::from_secs(3600),
            "marketplace".to_string(),
        );

        assert!(token.tenant_key.is_none());

        token.tenant_key = Some("tenant_abc123".to_string());
        assert_eq!(token.tenant_key, Some("tenant_abc123".to_string()));
    }

    // 新增测试：Token 生命周期 - 多次访问
    #[test]
    fn test_token_multiple_accesses() {
        let mut token = TokenInfo::new(
            "cli_multi".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(3600),
            "test".to_string(),
        );

        let original_access_time = token.last_accessed_at;
        std::thread::sleep(Duration::from_millis(10));

        token.update_access();
        assert_eq!(token.access_count, 1);
        assert!(token.last_accessed_at > original_access_time);

        std::thread::sleep(Duration::from_millis(10));
        token.update_access();
        assert_eq!(token.access_count, 2);
    }

    // 新增测试：TokenValidationResult 完整状态测试
    #[test]
    fn test_token_validation_result_all_states() {
        let valid = TokenValidationResult::Valid;
        let expired = TokenValidationResult::Expired;
        let invalid = TokenValidationResult::Invalid("test error".to_string());
        let expiring_soon = TokenValidationResult::ExpiringSoon(Duration::from_secs(60));

        // is_valid
        assert!(valid.is_valid());
        assert!(!expired.is_valid());
        assert!(!invalid.is_valid());
        assert!(!expiring_soon.is_valid());

        // is_invalid
        assert!(!valid.is_invalid());
        assert!(!expired.is_invalid());
        assert!(invalid.is_invalid());
        assert!(!expiring_soon.is_invalid());

        // needs_refresh
        assert!(!valid.needs_refresh());
        assert!(expired.needs_refresh());
        assert!(!invalid.needs_refresh());
        assert!(expiring_soon.needs_refresh());

        // error_message
        assert_eq!(valid.error_message(), "令牌有效");
        assert_eq!(expired.error_message(), "令牌已过期");
        assert_eq!(invalid.error_message(), "test error");
        assert_eq!(expiring_soon.error_message(), "令牌将在60秒后过期");
    }

    // 新增测试：TokenRefreshConfig 自定义配置
    #[test]
    fn test_token_refresh_config_custom_values() {
        let config = TokenRefreshConfig {
            refresh_ahead_seconds: 600,
            max_retry_attempts: 5,
            retry_interval_base: 2,
            retry_interval_max: 60,
            auto_refresh: false,
        };

        assert_eq!(config.refresh_ahead_seconds, 600);
        assert_eq!(config.max_retry_attempts, 5);
        assert_eq!(config.retry_interval_base, 2);
        assert_eq!(config.retry_interval_max, 60);
        assert!(!config.auto_refresh);
    }

    // 新增测试：Token 过期边界 - 刚好过期
    #[test]
    fn test_token_expiry_boundary() {
        let expired_token = TokenInfo::new(
            "cli_expired".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(0),
            "test".to_string(),
        );

        // 由于创建后立即检查，可能仍然是有效的
        // 但在极短延迟后应该过期
        std::thread::sleep(Duration::from_millis(5));
        assert!(expired_token.is_expired());
        assert_eq!(expired_token.expires_in_seconds(), 0);
    }

    // 新增测试：不同 Token 类型的过期判断
    #[test]
    fn test_different_token_types_expiry() {
        let app_token = TokenInfo::new(
            "cli_app".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(0),
            "test".to_string(),
        );
        let tenant_token = TokenInfo::new(
            "t-tenant".to_string(),
            TokenType::TenantAccessToken,
            Duration::from_secs(0),
            "test".to_string(),
        );
        let user_token = TokenInfo::new(
            "u-user".to_string(),
            TokenType::UserAccessToken,
            Duration::from_secs(0),
            "test".to_string(),
        );

        std::thread::sleep(Duration::from_millis(5));

        assert!(app_token.is_expired());
        assert!(tenant_token.is_expired());
        assert!(user_token.is_expired());
    }

    // 新增测试：Token 刷新策略 - 配置克隆
    #[test]
    fn test_token_refresh_config_clone() {
        let config = TokenRefreshConfig::default();
        let cloned = config.clone();

        assert_eq!(config.refresh_ahead_seconds, cloned.refresh_ahead_seconds);
        assert_eq!(config.max_retry_attempts, cloned.max_retry_attempts);
        assert_eq!(config.retry_interval_base, cloned.retry_interval_base);
        assert_eq!(config.retry_interval_max, cloned.retry_interval_max);
        assert_eq!(config.auto_refresh, cloned.auto_refresh);
    }

    // 新增测试：Token 序列化与反序列化
    #[test]
    fn test_token_info_serialization() {
        let token = TokenInfo::new(
            "cli_serial".to_string(),
            TokenType::AppAccessToken,
            Duration::from_secs(3600),
            "self_build".to_string(),
        );

        let json = serde_json::to_string(&token).expect("should serialize");
        let deserialized: TokenInfo = serde_json::from_str(&json).expect("should deserialize");

        assert_eq!(token.access_token, deserialized.access_token);
        assert_eq!(token.token_type, deserialized.token_type);
        assert_eq!(token.app_type, deserialized.app_type);
    }

    // 新增测试：TokenType 序列化与反序列化
    #[test]
    fn test_token_type_serialization() {
        let app = TokenType::AppAccessToken;
        let tenant = TokenType::TenantAccessToken;
        let user = TokenType::UserAccessToken;

        let app_json = serde_json::to_string(&app).unwrap();
        let tenant_json = serde_json::to_string(&tenant).unwrap();
        let user_json = serde_json::to_string(&user).unwrap();

        let app_de: TokenType = serde_json::from_str(&app_json).unwrap();
        let tenant_de: TokenType = serde_json::from_str(&tenant_json).unwrap();
        let user_de: TokenType = serde_json::from_str(&user_json).unwrap();

        assert_eq!(app, app_de);
        assert_eq!(tenant, tenant_de);
        assert_eq!(user, user_de);
    }
}
