//! 令牌相关数据模型

use serde::{Deserialize, Serialize};
use openlark_core::api::ApiResponseTrait;

/// 租户访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantAccessTokenResponse {
    /// 错误码，0表示成功
    pub code: i32,
    /// 租户访问令牌
    pub tenant_access_token: String,
    /// 令牌有效期（秒）
    pub expire: i32,
}

/// 应用访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppAccessTokenResponse {
    /// 错误码，0表示成功
    pub code: i32,
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
    /// 错误码，0表示成功
    pub code: i32,
    /// 应用票据
    pub app_ticket: String,
}

/// 用户访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccessTokenResponse {
    /// 错误码，0表示成功
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: UserAccessTokenData,
}

/// 用户访问令牌数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccessTokenData {
    /// 用户访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 令牌有效期（秒）
    pub expires_in: i32,
    /// 用户姓名
    pub name: Option<String>,
    /// 用户英文名
    pub en_name: Option<String>,
    /// 用户头像URL
    pub avatar_url: Option<String>,
    /// 用户头像缩略图
    pub avatar_thumb: Option<String>,
    /// 用户头像中等尺寸
    pub avatar_middle: Option<String>,
    /// 用户头像大图
    pub avatar_big: Option<String>,
    /// 用户开放ID
    pub open_id: String,
    /// 用户联合ID
    pub union_id: String,
    /// 用户邮箱
    pub email: Option<String>,
    /// 企业邮箱
    pub enterprise_email: Option<String>,
    /// 用户ID
    pub user_id: String,
    /// 用户手机号
    pub mobile: Option<String>,
    /// 租户密钥
    pub tenant_key: Option<String>,
    /// 刷新令牌过期时间（秒）
    pub refresh_expires_in: Option<i32>,
    /// 刷新令牌
    pub refresh_token: String,
    /// 会话ID
    pub sid: String,
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

// 为所有响应类型实现 ApiResponseTrait
impl ApiResponseTrait for TenantAccessTokenResponse {}
impl ApiResponseTrait for AppAccessTokenResponse {}
impl ApiResponseTrait for AppTicketResponse {}
impl ApiResponseTrait for UserAccessTokenResponse {}
impl ApiResponseTrait for RefreshUserAccessTokenResponse {}
impl ApiResponseTrait for OidcAccessTokenResponse {}
impl ApiResponseTrait for OidcRefreshAccessTokenResponse {}
impl ApiResponseTrait for AuthorizationIndexResponse {}
