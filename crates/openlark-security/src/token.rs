//! 令牌管理模块

use crate::error::{SecurityError, SecurityResult};
use crate::models::*;
use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// 令牌管理服务特征
#[async_trait]
pub trait TokenService: Send + Sync {
    /// 创建访问令牌
    async fn create_access_token(
        &self,
        request: CreateTokenRequest,
    ) -> SecurityResult<TokenResponse>;

    /// 验证访问令牌
    async fn verify_access_token(
        &self,
        access_token: &str,
    ) -> SecurityResult<TokenVerificationResponse>;

    /// 刷新访问令牌
    async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> SecurityResult<TokenRefreshResponse>;

    /// 撤销访问令牌
    async fn revoke_access_token(&self, access_token: &str) -> SecurityResult<RevokeTokenResponse>;

    /// 获取令牌信息
    async fn get_token_info(&self, access_token: &str) -> SecurityResult<TokenInfo>;

    /// 列出用户的活跃令牌
    async fn list_user_tokens(
        &self,
        user_id: &str,
        filters: TokenFilters,
    ) -> SecurityResult<ListTokensResponse>;

    /// 撤销用户的所有令牌
    async fn revoke_user_tokens(
        &self,
        user_id: &str,
        reason: Option<String>,
    ) -> SecurityResult<RevokeUserTokensResponse>;
}

/// 默认令牌管理服务实现
#[derive(Debug, Clone)]
pub struct DefaultTokenService {
    config: TokenServiceConfig,
    // 这里可以添加Redis缓存、数据库连接等依赖
}

/// 令牌服务配置
#[derive(Debug, Clone)]
pub struct TokenServiceConfig {
    /// 访问令牌默认过期时间（秒）
    pub access_token_expires_in: u64,
    /// 刷新令牌默认过期时间（秒）
    pub refresh_token_expires_in: u64,
    /// 令牌签发密钥
    pub signing_key: String,
    /// 是否启用令牌黑名单
    pub enable_blacklist: bool,
    /// 令牌前缀
    pub token_prefix: String,
}

impl Default for TokenServiceConfig {
    fn default() -> Self {
        Self {
            access_token_expires_in: 7200,    // 2小时
            refresh_token_expires_in: 604800, // 7天
            signing_key: "default_signing_key".to_string(),
            enable_blacklist: true,
            token_prefix: "lark".to_string(),
        }
    }
}

impl DefaultTokenService {
    /// 创建新的令牌服务实例
    pub fn new(config: TokenServiceConfig) -> Self {
        Self { config }
    }

    /// 使用默认配置创建令牌服务
    pub fn with_default_config() -> Self {
        Self::new(TokenServiceConfig::default())
    }
}

#[async_trait]
impl TokenService for DefaultTokenService {
    async fn create_access_token(
        &self,
        request: CreateTokenRequest,
    ) -> SecurityResult<TokenResponse> {
        tracing::info!(
            "创建访问令牌: 用户={}, 权限范围={:?}",
            request.user_id,
            request.scopes
        );

        // 验证请求参数
        if request.user_id.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: "user_id".to_string(),
                reason: "用户ID不能为空".to_string(),
            });
        }

        // 生成令牌
        let access_token = format!(
            "{}_access_{}",
            self.config.token_prefix,
            uuid::Uuid::new_v4()
        );
        let refresh_token = format!(
            "{}_refresh_{}",
            self.config.token_prefix,
            uuid::Uuid::new_v4()
        );

        let now = Utc::now();
        let expires_in = request
            .expires_in
            .unwrap_or(self.config.access_token_expires_in);
        let expires_at = now + chrono::Duration::seconds(expires_in as i64);

        // 生成令牌信息
        let token_info = TokenInfo {
            token_type: TokenType::AccessToken,
            access_token: access_token.clone(),
            refresh_token: Some(refresh_token.clone()),
            scopes: request.scopes.clone(),
            expires_at,
            issued_at: now,
            user_id: Some(request.user_id.clone()),
            app_id: request.app_id.clone(),
            permissions: request.permissions.unwrap_or_default(),
        };

        let response = TokenResponse {
            access_token: access_token.clone(),
            token_type: TokenType::AccessToken,
            expires_in,
            refresh_token: Some(refresh_token.clone()),
            scopes: request.scopes,
            token_info,
        };

        tracing::info!(
            "访问令牌创建成功: 用户={}, 令牌前缀={}",
            request.user_id,
            &access_token[..std::cmp::min(20, access_token.len())]
        );
        Ok(response)
    }

    async fn verify_access_token(
        &self,
        access_token: &str,
    ) -> SecurityResult<TokenVerificationResponse> {
        tracing::info!(
            "验证访问令牌: {}",
            &access_token[..std::cmp::min(20, access_token.len())]
        );

        // 验证令牌格式
        if !access_token.starts_with(&format!("{}_access_", self.config.token_prefix)) {
            return Ok(TokenVerificationResponse {
                valid: false,
                token_info: None,
                error: Some("令牌格式无效".to_string()),
                error_code: Some("INVALID_TOKEN_FORMAT".to_string()),
            });
        }

        // 模拟令牌验证逻辑
        let token_info = TokenInfo {
            token_type: TokenType::AccessToken,
            access_token: access_token.to_string(),
            refresh_token: None,
            scopes: vec!["user.info".to_string(), "message.send".to_string()],
            expires_at: Utc::now() + chrono::Duration::hours(1),
            issued_at: Utc::now() - chrono::Duration::minutes(30),
            user_id: Some("user_mock".to_string()),
            app_id: Some("app_mock".to_string()),
            permissions: vec!["read".to_string(), "write".to_string()],
        };

        let now = Utc::now();
        let is_expired = now > token_info.expires_at;

        if is_expired {
            Ok(TokenVerificationResponse {
                valid: false,
                token_info: Some(token_info),
                error: Some("令牌已过期".to_string()),
                error_code: Some("TOKEN_EXPIRED".to_string()),
            })
        } else {
            Ok(TokenVerificationResponse {
                valid: true,
                token_info: Some(token_info),
                error: None,
                error_code: None,
            })
        }
    }

    async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> SecurityResult<TokenRefreshResponse> {
        tracing::info!(
            "刷新访问令牌: {}",
            &refresh_token[..std::cmp::min(20, refresh_token.len())]
        );

        // 验证刷新令牌格式
        if !refresh_token.starts_with(&format!("{}_refresh_", self.config.token_prefix)) {
            return Err(SecurityError::TokenError {
                message: "刷新令牌格式无效".to_string(),
                token_type: Some(TokenType::RefreshToken),
                expires_at: None,
            });
        }

        // 生成新的访问令牌
        let new_access_token = format!(
            "{}_access_{}",
            self.config.token_prefix,
            uuid::Uuid::new_v4()
        );
        let new_refresh_token = format!(
            "{}_refresh_{}",
            self.config.token_prefix,
            uuid::Uuid::new_v4()
        );

        let now = Utc::now();
        let expires_at =
            now + chrono::Duration::seconds(self.config.access_token_expires_in as i64);

        let token_info = TokenInfo {
            token_type: TokenType::AccessToken,
            access_token: new_access_token.clone(),
            refresh_token: Some(new_refresh_token.clone()),
            scopes: vec!["user.info".to_string(), "message.send".to_string()],
            expires_at,
            issued_at: now,
            user_id: Some("user_mock".to_string()),
            app_id: Some("app_mock".to_string()),
            permissions: vec!["read".to_string(), "write".to_string()],
        };

        let response = TokenRefreshResponse {
            access_token: new_access_token.clone(),
            refresh_token: new_refresh_token.clone(),
            token_type: TokenType::AccessToken,
            expires_in: self.config.access_token_expires_in,
            scopes: token_info.scopes.clone(),
            token_info,
        };

        tracing::info!(
            "访问令牌刷新成功: {}",
            &new_access_token[..std::cmp::min(20, new_access_token.len())]
        );
        Ok(response)
    }

    async fn revoke_access_token(&self, access_token: &str) -> SecurityResult<RevokeTokenResponse> {
        tracing::info!(
            "撤销访问令牌: {}",
            &access_token[..std::cmp::min(20, access_token.len())]
        );

        // 模拟令牌撤销逻辑
        // 实际实现中会将令牌加入黑名单或从存储中删除

        let response = RevokeTokenResponse {
            success: true,
            revoked_at: Utc::now(),
            reason: Some("用户主动撤销".to_string()),
        };

        tracing::info!("访问令牌撤销成功");
        Ok(response)
    }

    async fn get_token_info(&self, access_token: &str) -> SecurityResult<TokenInfo> {
        tracing::info!(
            "获取令牌信息: {}",
            &access_token[..std::cmp::min(20, access_token.len())]
        );

        // 验证令牌并返回信息
        let verification = self.verify_access_token(access_token).await?;

        if let Some(token_info) = verification.token_info {
            Ok(token_info)
        } else {
            Err(SecurityError::TokenError {
                message: verification
                    .error
                    .unwrap_or_else(|| "令牌验证失败".to_string()),
                token_type: Some(TokenType::AccessToken),
                expires_at: None,
            })
        }
    }

    async fn list_user_tokens(
        &self,
        user_id: &str,
        filters: TokenFilters,
    ) -> SecurityResult<ListTokensResponse> {
        tracing::info!("列出用户令牌: 用户={}, 筛选条件={:?}", user_id, filters);

        // 模拟用户令牌列表
        let mock_tokens = vec![
            ActiveToken {
                token_id: format!("token_{}", uuid::Uuid::new_v4()),
                access_token: format!(
                    "{}_access_{}",
                    self.config.token_prefix,
                    uuid::Uuid::new_v4()
                ),
                token_type: TokenType::AccessToken,
                scopes: vec!["user.info".to_string(), "message.send".to_string()],
                created_at: Utc::now() - chrono::Duration::hours(2),
                expires_at: Utc::now() + chrono::Duration::hours(6),
                last_used_at: Some(Utc::now() - chrono::Duration::minutes(30)),
                device_info: Some(DeviceInfo {
                    device_id: "device_001".to_string(),
                    device_type: DeviceType::Desktop,
                    device_name: "Windows PC".to_string(),
                    os: "Windows 10".to_string(),
                    browser: Some("Chrome 120.0".to_string()),
                    fingerprint: None,
                }),
                ip_address: "192.168.1.100".to_string(),
                is_active: true,
            },
            ActiveToken {
                token_id: format!("token_{}", uuid::Uuid::new_v4()),
                access_token: format!(
                    "{}_access_{}",
                    self.config.token_prefix,
                    uuid::Uuid::new_v4()
                ),
                token_type: TokenType::AccessToken,
                scopes: vec!["user.info".to_string()],
                created_at: Utc::now() - chrono::Duration::days(1),
                expires_at: Utc::now() + chrono::Duration::days(6),
                last_used_at: Some(Utc::now() - chrono::Duration::hours(12)),
                device_info: Some(DeviceInfo {
                    device_id: "device_002".to_string(),
                    device_type: DeviceType::Mobile,
                    device_name: "iPhone 15".to_string(),
                    os: "iOS 17.0".to_string(),
                    browser: None,
                    fingerprint: None,
                }),
                ip_address: "192.168.1.101".to_string(),
                is_active: true,
            },
        ];

        // 应用筛选条件
        let filtered_tokens: Vec<ActiveToken> = mock_tokens
            .into_iter()
            .filter(|token| {
                let mut matches = true;

                if let Some(filter_device_type) = &filters.device_type {
                    matches = matches
                        && token
                            .device_info
                            .as_ref()
                            .map(|info| &info.device_type == filter_device_type)
                            .unwrap_or(false);
                }

                if let Some(filter_active) = filters.is_active {
                    matches = matches && (token.is_active == filter_active);
                }

                matches
            })
            .collect();

        let total = filtered_tokens.len() as u32;
        let response = ListTokensResponse {
            tokens: filtered_tokens,
            total,
        };

        tracing::info!(
            "用户令牌列表获取完成: 用户={}, 令牌数量={}",
            user_id,
            response.total
        );
        Ok(response)
    }

    async fn revoke_user_tokens(
        &self,
        user_id: &str,
        reason: Option<String>,
    ) -> SecurityResult<RevokeUserTokensResponse> {
        tracing::info!("撤销用户所有令牌: 用户={}, 理由={:?}", user_id, reason);

        // 模拟撤销用户所有令牌
        let revoked_count = 5; // 模拟撤销了5个令牌

        let response = RevokeUserTokensResponse {
            success: true,
            revoked_count,
            revoked_at: Utc::now(),
            reason: reason.unwrap_or_else(|| "管理员撤销".to_string()),
        };

        tracing::info!(
            "用户令牌撤销完成: 用户={}, 撤销数量={}",
            user_id,
            revoked_count
        );
        Ok(response)
    }
}

// 以下是请求和响应结构体定义

/// 创建令牌请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTokenRequest {
    /// 用户ID
    pub user_id: String,
    /// 应用ID
    pub app_id: Option<String>,
    /// 权限范围
    pub scopes: Vec<String>,
    /// 权限列表
    pub permissions: Option<Vec<String>>,
    /// 过期时间（秒）
    pub expires_in: Option<u64>,
    /// 客户端信息
    pub client_info: Option<ClientInfo>,
}

/// 令牌响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: TokenType,
    /// 过期时间（秒）
    pub expires_in: u64,
    /// 刷新令牌
    pub refresh_token: Option<String>,
    /// 权限范围
    pub scopes: Vec<String>,
    /// 令牌详细信息
    pub token_info: TokenInfo,
}

/// 令牌验证响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenVerificationResponse {
    /// 是否有效
    pub valid: bool,
    /// 令牌信息
    pub token_info: Option<TokenInfo>,
    /// 错误信息
    pub error: Option<String>,
    /// 错误代码
    pub error_code: Option<String>,
}

/// 令牌刷新响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenRefreshResponse {
    /// 新的访问令牌
    pub access_token: String,
    /// 新的刷新令牌
    pub refresh_token: String,
    /// 令牌类型
    pub token_type: TokenType,
    /// 过期时间（秒）
    pub expires_in: u64,
    /// 权限范围
    pub scopes: Vec<String>,
    /// 令牌详细信息
    pub token_info: TokenInfo,
}

/// 撤销令牌响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RevokeTokenResponse {
    /// 是否成功
    pub success: bool,
    /// 撤销时间
    pub revoked_at: chrono::DateTime<chrono::Utc>,
    /// 撤销原因
    pub reason: Option<String>,
}

/// 令牌筛选条件
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenFilters {
    /// 设备类型
    pub device_type: Option<DeviceType>,
    /// 是否活跃
    pub is_active: Option<bool>,
    /// 创建时间范围（开始）
    pub created_after: Option<chrono::DateTime<chrono::Utc>>,
    /// 创建时间范围（结束）
    pub created_before: Option<chrono::DateTime<chrono::Utc>>,
    /// 分页参数
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// 列出令牌响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListTokensResponse {
    /// 令牌列表
    pub tokens: Vec<ActiveToken>,
    /// 总数量
    pub total: u32,
}

/// 活跃令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveToken {
    /// 令牌ID
    pub token_id: String,
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: TokenType,
    /// 权限范围
    pub scopes: Vec<String>,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 过期时间
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// 最后使用时间
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 设备信息
    pub device_info: Option<DeviceInfo>,
    /// IP地址
    pub ip_address: String,
    /// 是否活跃
    pub is_active: bool,
}

/// 撤销用户令牌响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RevokeUserTokensResponse {
    /// 是否成功
    pub success: bool,
    /// 撤销的令牌数量
    pub revoked_count: u32,
    /// 撤销时间
    pub revoked_at: chrono::DateTime<chrono::Utc>,
    /// 撤销原因
    pub reason: String,
}

/// 客户端信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientInfo {
    /// 客户端ID
    pub client_id: String,
    /// 客户端名称
    pub client_name: String,
    /// 客户端版本
    pub client_version: String,
    /// 平台
    pub platform: String,
}
