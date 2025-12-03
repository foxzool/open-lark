//! 令牌相关数据模型

use serde::{Deserialize, Serialize};

/// 租户访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantAccessTokenResponse {
    /// 租户访问令牌
    pub tenant_access_token: String,
    /// 令牌有效期（秒）
    pub expire: i32,
}

/// 应用访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppAccessTokenResponse {
    /// 应用访问令牌
    pub app_access_token: String,
    /// 令牌有效期（秒）
    pub expire: i32,
}

/// 应用票据请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppTicketRequest {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
}

/// 应用票据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppTicketResponse {
    /// 应用票据
    pub app_ticket: String,
}

/// 用户访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccessTokenResponse {
    /// 用户访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 令牌有效期（秒）
    pub expires_in: i32,
    /// 刷新令牌
    pub refresh_token: Option<String>,
    /// 授权范围
    pub scope: Option<String>,
}

/// 刷新用户访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshUserAccessTokenResponse {
    /// 用户访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 令牌有效期（秒）
    pub expires_in: i32,
    /// 刷新令牌
    pub refresh_token: String,
    /// 授权范围
    pub scope: Option<String>,
}

/// OIDC访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcAccessTokenResponse {
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 令牌有效期（秒）
    pub expires_in: i32,
    /// ID令牌（JWT格式）
    pub id_token: String,
    /// 刷新令牌
    pub refresh_token: Option<String>,
    /// 授权范围
    pub scope: Option<String>,
}

/// 刷新OIDC访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcRefreshAccessTokenResponse {
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 令牌有效期（秒）
    pub expires_in: i32,
    /// ID令牌（JWT格式）
    pub id_token: Option<String>,
    /// 刷新令牌
    pub refresh_token: String,
    /// 授权范围
    pub scope: Option<String>,
}

/// 登录预授权码响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationIndexResponse {
    /// 登录预授权码
    pub code: String,
    /// 应用状态
    pub app_status: String,
    /// 授权URL
    pub auth_url: String,
    /// 过期时间
    pub expire_in: i32,
}
