//! 认证相关类型定义
//!
//! 包含应用认证、用户认证相关的请求和响应数据结构。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 应用访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间（秒）
    pub expires_in: u64,
    /// 获取令牌的时间戳
    pub created_at: DateTime<Utc>,
}

impl AccessToken {
    /// 创建新的访问令牌
    pub fn new(access_token: String, token_type: String, expires_in: u64) -> Self {
        Self {
            access_token,
            token_type,
            expires_in,
            created_at: Utc::now(),
        }
    }

    /// 检查令牌是否过期
    pub fn is_expired(&self) -> bool {
        let elapsed = Utc::now().signed_duration_since(self.created_at);
        elapsed.num_seconds() >= (self.expires_in as i64 - 60) // 提前1分钟过期
    }

    /// 获取剩余有效时间（秒）
    pub fn remaining_seconds(&self) -> i64 {
        let elapsed = Utc::now().signed_duration_since(self.created_at);
        (self.expires_in as i64) - elapsed.num_seconds()
    }
}

/// 用户访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccessToken {
    /// 访问令牌
    pub access_token: String,
    /// 刷新令牌
    pub refresh_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 访问令牌过期时间（秒）
    pub expires_in: u64,
    /// 刷新令牌过期时间（秒）
    pub refresh_expires_in: u64,
    /// 令牌作用域
    pub scope: String,
    /// 获取令牌的时间戳
    pub created_at: DateTime<Utc>,
}

impl UserAccessToken {
    /// 创建新的用户访问令牌
    pub fn new(
        access_token: String,
        refresh_token: String,
        token_type: String,
        expires_in: u64,
        refresh_expires_in: u64,
        scope: String,
    ) -> Self {
        Self {
            access_token,
            refresh_token,
            token_type,
            expires_in,
            refresh_expires_in,
            scope,
            created_at: Utc::now(),
        }
    }

    /// 检查访问令牌是否过期
    pub fn is_access_token_expired(&self) -> bool {
        let elapsed = Utc::now().signed_duration_since(self.created_at);
        elapsed.num_seconds() >= (self.expires_in as i64 - 60) // 提前1分钟过期
    }

    /// 检查刷新令牌是否过期
    pub fn is_refresh_token_expired(&self) -> bool {
        let elapsed = Utc::now().signed_duration_since(self.created_at);
        elapsed.num_seconds() >= (self.refresh_expires_in as i64 - 60)
    }

    /// 获取访问令牌剩余有效时间（秒）
    pub fn access_token_remaining_seconds(&self) -> i64 {
        let elapsed = Utc::now().signed_duration_since(self.created_at);
        (self.expires_in as i64) - elapsed.num_seconds()
    }

    /// 获取刷新令牌剩余有效时间（秒）
    pub fn refresh_token_remaining_seconds(&self) -> i64 {
        let elapsed = Utc::now().signed_duration_since(self.created_at);
        (self.refresh_expires_in as i64) - elapsed.num_seconds()
    }
}

/// 令牌信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    /// 令牌内容
    pub content: serde_json::Value,
    /// 验证时间戳
    pub verified_at: DateTime<Utc>,
}

impl TokenInfo {
    /// 创建新的令牌信息
    pub fn new(content: serde_json::Value) -> Self {
        Self {
            content,
            verified_at: Utc::now(),
        }
    }
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub mobile: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
    /// 部门信息
    pub department_ids: Option<Vec<String>>,
    /// 获取信息的时间戳
    pub fetched_at: DateTime<Utc>,
}

impl UserInfo {
    /// 创建新的用户信息
    pub fn new(
        user_id: String,
        name: String,
        email: Option<String>,
        mobile: Option<String>,
        avatar: Option<String>,
        department_ids: Option<Vec<String>>,
    ) -> Self {
        Self {
            user_id,
            name,
            email,
            mobile,
            avatar,
            department_ids,
            fetched_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_token_creation() {
        let token = AccessToken::new(
            "test_access_token".to_string(),
            "Bearer".to_string(),
            7200, // 2小时
        );

        assert_eq!(token.access_token, "test_access_token");
        assert_eq!(token.token_type, "Bearer");
        assert_eq!(token.expires_in, 7200);
        assert!(!token.is_expired()); // 新创建的令牌不应该过期
        assert!(token.remaining_seconds() > 7000); // 应该接近7200秒
    }

    #[test]
    fn test_access_token_expiry() {
        // 创建一个已过期的令牌
        let token = AccessToken::new(
            "expired_token".to_string(),
            "Bearer".to_string(),
            1, // 1秒后过期
        );

        // 等待2秒确保令牌过期
        std::thread::sleep(std::time::Duration::from_secs(2));

        assert!(token.is_expired());
        assert!(token.remaining_seconds() <= 0);
    }

    #[test]
    fn test_access_token_remaining_seconds() {
        let token = AccessToken::new(
            "test_token".to_string(),
            "Bearer".to_string(),
            3600, // 1小时
        );

        let remaining = token.remaining_seconds();
        assert!(remaining > 3500); // 应该接近3600秒
        assert!(remaining <= 3600); // 不应该超过3600秒
    }

    #[test]
    fn test_user_access_token_creation() {
        let user_token = UserAccessToken::new(
            "user_access_token".to_string(),
            "refresh_token".to_string(),
            "Bearer".to_string(),
            7200,    // 访问令牌2小时
            2592000, // 刷新令牌30天
            "scope1 scope2".to_string(),
        );

        assert_eq!(user_token.access_token, "user_access_token");
        assert_eq!(user_token.refresh_token, "refresh_token");
        assert_eq!(user_token.token_type, "Bearer");
        assert_eq!(user_token.expires_in, 7200);
        assert_eq!(user_token.refresh_expires_in, 2592000);
        assert_eq!(user_token.scope, "scope1 scope2");
        assert!(!user_token.is_access_token_expired());
        assert!(!user_token.is_refresh_token_expired());
    }

    #[test]
    fn test_user_access_token_expiry() {
        let user_token = UserAccessToken::new(
            "expired_user_token".to_string(),
            "refresh_token".to_string(),
            "Bearer".to_string(),
            1, // 访问令牌1秒后过期
            2, // 刷新令牌2秒后过期
            "test_scope".to_string(),
        );

        // 等待3秒确保两个令牌都过期
        std::thread::sleep(std::time::Duration::from_secs(3));

        assert!(user_token.is_access_token_expired());
        assert!(user_token.is_refresh_token_expired());
    }

    #[test]
    fn test_user_access_token_remaining_seconds() {
        let user_token = UserAccessToken::new(
            "test_user_token".to_string(),
            "refresh_token".to_string(),
            "Bearer".to_string(),
            3600, // 访问令牌1小时
            7200, // 刷新令牌2小时
            "scope".to_string(),
        );

        let access_remaining = user_token.access_token_remaining_seconds();
        let refresh_remaining = user_token.refresh_token_remaining_seconds();

        assert!(access_remaining > 3500 && access_remaining <= 3600);
        assert!(refresh_remaining > 7000 && refresh_remaining <= 7200);
    }

    #[test]
    fn test_token_info_creation() {
        let content = serde_json::json!({
            "user_id": "test_user",
            "tenant_key": "test_tenant"
        });

        let token_info = TokenInfo::new(content.clone());

        assert_eq!(token_info.content, content);
        // 验证时间戳是最近的（在1秒内）
        let now = Utc::now();
        let time_diff = (now - token_info.verified_at).num_seconds().abs();
        assert!(time_diff <= 1);
    }

    #[test]
    fn test_user_info_creation() {
        let departments = vec!["dept1".to_string(), "dept2".to_string()];
        let user_info = UserInfo::new(
            "user_123".to_string(),
            "张三".to_string(),
            Some("zhangsan@example.com".to_string()),
            Some("+86 138 0013 8000".to_string()),
            Some("https://example.com/avatar.jpg".to_string()),
            Some(departments.clone()),
        );

        assert_eq!(user_info.user_id, "user_123");
        assert_eq!(user_info.name, "张三");
        assert_eq!(user_info.email, Some("zhangsan@example.com".to_string()));
        assert_eq!(user_info.mobile, Some("+86 138 0013 8000".to_string()));
        assert_eq!(
            user_info.avatar,
            Some("https://example.com/avatar.jpg".to_string())
        );
        assert_eq!(user_info.department_ids, Some(departments));

        // 验证时间戳是最近的
        let now = Utc::now();
        let time_diff = (now - user_info.fetched_at).num_seconds().abs();
        assert!(time_diff <= 1);
    }

    #[test]
    fn test_user_info_optional_fields() {
        let user_info = UserInfo::new(
            "user_456".to_string(),
            "李四".to_string(),
            None,
            None,
            None,
            None,
        );

        assert_eq!(user_info.user_id, "user_456");
        assert_eq!(user_info.name, "李四");
        assert!(user_info.email.is_none());
        assert!(user_info.mobile.is_none());
        assert!(user_info.avatar.is_none());
        assert!(user_info.department_ids.is_none());
    }

    #[test]
    fn test_token_serialization() {
        let token = AccessToken::new("test_token".to_string(), "Bearer".to_string(), 3600);

        // 测试序列化
        let json_str = serde_json::to_string(&token).unwrap();
        let parsed: AccessToken = serde_json::from_str(&json_str).unwrap();

        assert_eq!(parsed.access_token, token.access_token);
        assert_eq!(parsed.token_type, token.token_type);
        assert_eq!(parsed.expires_in, token.expires_in);
    }

    #[test]
    fn test_user_token_serialization() {
        let user_token = UserAccessToken::new(
            "access_token".to_string(),
            "refresh_token".to_string(),
            "Bearer".to_string(),
            3600,
            7200,
            "read write".to_string(),
        );

        // 测试序列化
        let json_str = serde_json::to_string(&user_token).unwrap();
        let parsed: UserAccessToken = serde_json::from_str(&json_str).unwrap();

        assert_eq!(parsed.access_token, user_token.access_token);
        assert_eq!(parsed.refresh_token, user_token.refresh_token);
        assert_eq!(parsed.token_type, user_token.token_type);
        assert_eq!(parsed.scope, user_token.scope);
    }

    #[test]
    fn test_edge_case_expiry_times() {
        // 测试边界情况：正好在过期边界
        let token = AccessToken::new(
            "boundary_token".to_string(),
            "Bearer".to_string(),
            0, // 立即过期
        );

        // 由于我们有1分钟的缓冲期，0过期的令牌应该立即被认为是过期的
        assert!(token.is_expired());

        // 测试负数过期时间（不应该发生，但要确保代码能处理）
        let negative_token =
            AccessToken::new("negative_token".to_string(), "Bearer".to_string(), 0);
        assert!(negative_token.is_expired());
    }

    #[test]
    fn test_debug_formatting() {
        let token = AccessToken::new("debug_token".to_string(), "Bearer".to_string(), 3600);

        let debug_str = format!("{:?}", token);
        assert!(debug_str.contains("AccessToken"));
        assert!(debug_str.contains("debug_token"));
        // 注意：由于安全原因，我们不应该在debug输出中显示完整的令牌
        // 但这是测试数据，所以可以接受
    }

    #[test]
    fn test_clone_functionality() {
        let original_token = AccessToken::new(
            "clone_test_token".to_string(),
            "Bearer".to_string(),
            1800, // 30分钟
        );

        let cloned_token = original_token.clone();
        assert_eq!(original_token.access_token, cloned_token.access_token);
        assert_eq!(original_token.token_type, cloned_token.token_type);
        assert_eq!(original_token.expires_in, cloned_token.expires_in);
        // created_at 应该相同，因为它是复制而不是重新创建
        assert_eq!(original_token.created_at, cloned_token.created_at);

        // 同样测试 UserAccessToken
        let original_user_token = UserAccessToken::new(
            "user_access_token".to_string(),
            "refresh_token".to_string(),
            "Bearer".to_string(),
            3600,
            7200,
            "read".to_string(),
        );

        let cloned_user_token = original_user_token.clone();
        assert_eq!(
            original_user_token.access_token,
            cloned_user_token.access_token
        );
        assert_eq!(
            original_user_token.refresh_token,
            cloned_user_token.refresh_token
        );
    }
}
