//! 认证相关的类型定义

use super::token::{TokenInfo, TokenType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// OAuth 2.0 授权类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OAuthGrantType {
    /// 授权码模式
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
    /// 客户端凭证模式
    #[serde(rename = "client_credentials")]
    ClientCredentials,
    /// 刷新令牌模式
    #[serde(rename = "refresh_token")]
    RefreshToken,
}

/// OAuth 2.0 请求参数
#[derive(Debug, Clone, Serialize)]
pub struct OAuthRequest {
    /// 授权类型
    pub grant_type: OAuthGrantType,
    /// 客户端ID
    pub client_id: String,
    /// 客户端密钥
    pub client_secret: String,
    /// 授权码（授权码模式使用）
    pub code: Option<String>,
    /// 重定向URI（授权码模式使用）
    pub redirect_uri: Option<String>,
    /// 刷新令牌（刷新令牌模式使用）
    pub refresh_token: Option<String>,
    /// 授权范围
    pub scope: Option<String>,
    /// 状态参数（防止CSRF攻击）
    pub state: Option<String>,
}

/// OAuth 2.0 响应
#[derive(Debug, Clone, Deserialize)]
pub struct OAuthResponse {
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间（秒）
    pub expires_in: u64,
    /// 刷新令牌
    pub refresh_token: Option<String>,
    /// 授权范围
    pub scope: Option<String>,
    /// 状态参数
    pub state: Option<String>,
}

/// 预授权码响应
#[derive(Debug, Clone, Deserialize)]
pub struct PreAuthCodeResponse {
    /// 预授权码
    #[serde(rename = "pre_auth_code")]
    pub pre_auth_code: String,
    /// 过期时间（秒）
    #[serde(rename = "expires_in")]
    pub expires_in: u64,
}

/// 用户信息
#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 邮箱
    pub email: Option<String>,
    /// 头像URL
    pub avatar_url: Option<String>,
    /// 租户信息
    pub tenant: Option<TenantInfo>,
}

/// 租户信息
#[derive(Debug, Clone, Deserialize)]
pub struct TenantInfo {
    /// 租户ID
    pub tenant_id: String,
    /// 租户名称
    pub tenant_name: String,
    /// 租户类型
    pub tenant_type: Option<String>,
}

/// 权限范围定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionScope {
    /// 权限ID
    pub permission_id: String,
    /// 权限名称
    pub permission_name: String,
    /// 权限描述
    pub description: Option<String>,
}

/// 应用权限信息
#[derive(Debug, Clone, Deserialize)]
pub struct AppPermissions {
    /// 应用权限列表
    pub permissions: Vec<PermissionScope>,
    /// 获取时间戳
    pub ts: u64,
}

/// 令牌交换请求
#[derive(Debug, Clone, Serialize)]
pub struct TokenExchangeRequest {
    /// 交换的令牌类型
    pub grant_type: String,
    /// 要交换的令牌
    pub token: String,
    /// 目标令牌类型
    pub requested_token_type: String,
    /// 权限范围
    pub scope: Option<String>,
    /// 目标租户（可选）
    pub tenant_key: Option<String>,
}

/// 令牌交换响应
#[derive(Debug, Clone, Deserialize)]
pub struct TokenExchangeResponse {
    /// 交换后的令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间（秒）
    pub expires_in: u64,
    /// 权限范围
    pub scope: Option<String>,
    /// 租户信息
    pub tenant_key: Option<String>,
}

/// 认证上下文
#[derive(Debug, Clone)]
pub struct AuthContext {
    /// 请求ID
    pub request_id: String,
    /// 应用ID
    pub app_id: String,
    /// 用户ID（可选）
    pub user_id: Option<String>,
    /// 租户ID（可选）
    pub tenant_id: Option<String>,
    /// 请求时间
    pub timestamp: std::time::SystemTime,
    /// 额外的元数据
    pub metadata: HashMap<String, String>,
}

impl AuthContext {
    /// 创建新的认证上下文
    pub fn new(app_id: String) -> Self {
        Self {
            request_id: uuid::Uuid::new_v4().to_string(),
            app_id,
            user_id: None,
            tenant_id: None,
            timestamp: std::time::SystemTime::now(),
            metadata: HashMap::new(),
        }
    }

    /// 设置用户ID
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// 设置租户ID
    pub fn with_tenant_id(mut self, tenant_id: String) -> Self {
        self.tenant_id = Some(tenant_id);
        self
    }

    /// 添加元数据
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// 认证验证请求
#[derive(Debug, Clone)]
pub struct AuthValidationRequest {
    /// 令牌
    pub token: String,
    /// 预期令牌类型
    pub expected_type: Option<TokenType>,
    /// 权限范围（可选）
    pub required_scope: Option<Vec<String>>,
    /// 验证上下文
    pub context: AuthContext,
}

/// 认证验证结果详情
#[derive(Debug, Clone)]
pub struct AuthValidationDetails {
    /// 是否有效
    pub valid: bool,
    /// 令牌信息
    pub token_info: Option<TokenInfo>,
    /// 权限验证结果
    pub scope_valid: Option<bool>,
    /// 验证错误（如果无效）
    pub error: Option<String>,
    /// 验证耗时
    pub validation_duration: std::time::Duration,
    /// 验证时间戳
    pub timestamp: std::time::SystemTime,
}

/// 令牌刷新策略
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RefreshStrategy {
    /// 在过期前刷新
    #[serde(rename = "proactive")]
    Proactive,
    /// 在过期时刷新
    #[serde(rename = "reactive")]
    Reactive,
    /// 混合策略
    #[serde(rename = "hybrid")]
    Hybrid,
}

impl Default for RefreshStrategy {
    fn default() -> Self {
        Self::Proactive
    }
}

/// 缓存策略
#[derive(Debug, Clone, Copy)]
pub enum CacheStrategy {
    /// 仅内存缓存
    Memory,
    /// 仅外部缓存（Redis）
    External,
    /// 多级缓存
    MultiLevel,
}

impl Default for CacheStrategy {
    fn default() -> Self {
        Self::Memory
    }
}

/// 令牌存储位置
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenStorageLocation {
    /// 内存中
    Memory,
    /// 磁盘上
    Disk,
    /// 外部存储（Redis等）
    External,
    /// 加密存储
    Encrypted,
}

/// 令牌安全配置
#[derive(Debug, Clone)]
pub struct TokenSecurityConfig {
    /// 是否加密存储令牌
    pub encrypt_storage: bool,
    /// 存储位置
    pub storage_location: TokenStorageLocation,
    /// 是否启用令牌轮换
    pub enable_rotation: bool,
    /// 轮换间隔（小时）
    pub rotation_interval_hours: u64,
    /// 最大令牌生命周期（小时）
    pub max_lifetime_hours: u64,
}

impl Default for TokenSecurityConfig {
    fn default() -> Self {
        Self {
            encrypt_storage: false,
            storage_location: TokenStorageLocation::Memory,
            enable_rotation: false,
            rotation_interval_hours: 24,
            max_lifetime_hours: 720, // 30天
        }
    }
}

/// OAuth 配置
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    /// 应用ID
    pub client_id: String,
    /// 应用密钥
    pub client_secret: String,
    /// 重定向URI
    pub redirect_uri: String,
    /// 授权范围
    pub scope: String,
    /// 响应类型
    pub response_type: String,
    /// 是否启用PKCE
    pub enable_pkce: bool,
    /// 额外参数
    pub extra_params: HashMap<String, String>,
}

impl OAuthConfig {
    /// 创建新的OAuth配置
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        scope: String,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
            scope,
            response_type: "code".to_string(),
            enable_pkce: false,
            extra_params: HashMap::new(),
        }
    }

    /// 添加额外参数
    pub fn with_extra_param(mut self, key: String, value: String) -> Self {
        self.extra_params.insert(key, value);
        self
    }

    /// 启用PKCE
    pub fn enable_pkce(mut self) -> Self {
        self.enable_pkce = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth_request_serialization() {
        let request = OAuthRequest {
            grant_type: OAuthGrantType::AuthorizationCode,
            client_id: "test_client".to_string(),
            client_secret: "test_secret".to_string(),
            code: Some("auth_code".to_string()),
            redirect_uri: Some("https://example.com/callback".to_string()),
            refresh_token: None,
            scope: Some("read write".to_string()),
            state: Some("random_state".to_string()),
        };

        // 测试序列化
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("authorization_code"));
        assert!(json.contains("test_client"));
    }

    #[test]
    fn test_auth_context_creation() {
        let context = AuthContext::new("test_app".to_string())
            .with_user_id("user_123".to_string())
            .with_tenant_id("tenant_456".to_string())
            .with_metadata("key".to_string(), "value".to_string());

        assert_eq!(context.app_id, "test_app");
        assert_eq!(context.user_id, Some("user_123".to_string()));
        assert_eq!(context.tenant_id, Some("tenant_456".to_string()));
        assert_eq!(context.metadata.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_token_security_config() {
        let config = TokenSecurityConfig::default();
        assert!(!config.encrypt_storage);
        assert_eq!(config.storage_location, TokenStorageLocation::Memory);
        assert!(!config.enable_rotation);
        assert_eq!(config.rotation_interval_hours, 24);
    }

    #[test]
    fn test_oauth_config() {
        let config = OAuthConfig::new(
            "client_id".to_string(),
            "client_secret".to_string(),
            "https://example.com/callback".to_string(),
            "read write".to_string(),
        )
        .enable_pkce()
        .with_extra_param("test".to_string(), "value".to_string());

        assert_eq!(config.client_id, "client_id");
        assert!(config.enable_pkce);
        assert_eq!(config.extra_params.get("test"), Some(&"value".to_string()));
    }
}
