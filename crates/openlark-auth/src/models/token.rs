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
