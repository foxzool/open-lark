//! OpenLark Auth API 端点定义
//!
//! 提供统一的 API 端点管理和 URL 生成功能

/// Auth V3 API 端点枚举
#[derive(Debug, Clone)]
pub enum AuthApiV3 {
    /// 自建应用获取 app_access_token
    AppAccessTokenInternal,
    /// 自建应用获取 tenant_access_token
    TenantAccessTokenInternal,
    /// 重新获取 app_ticket
    AppTicketResend,
    /// 商店应用获取 app_access_token
    AppAccessToken,
    /// 商店应用获取 tenant_access_token
    TenantAccessToken,
}

impl AuthApiV3 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            AuthApiV3::AppAccessTokenInternal => {
                "/open-apis/auth/v3/app_access_token/internal".to_string()
            }
            AuthApiV3::TenantAccessTokenInternal => {
                "/open-apis/auth/v3/tenant_access_token/internal".to_string()
            }
            AuthApiV3::AppTicketResend => "/open-apis/auth/v3/app_ticket/resend".to_string(),
            AuthApiV3::AppAccessToken => "/open-apis/auth/v3/app_access_token".to_string(),
            AuthApiV3::TenantAccessToken => "/open-apis/auth/v3/tenant_access_token".to_string(),
        }
    }
}

/// Authen V1 API 端点枚举
#[derive(Debug, Clone)]
pub enum AuthenApiV1 {
    /// 获取用户信息
    UserInfo,
    /// 获取 user_access_token（v1 版本）
    AccessToken,
    /// 刷新 user_access_token（v1 版本）
    RefreshAccessToken,
    /// 获取 user_access_token (OIDC)
    OidcAccessToken,
    /// 刷新 user_access_token (OIDC)
    OidcRefreshAccessToken,
}

impl AuthenApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            AuthenApiV1::UserInfo => "/open-apis/authen/v1/user_info".to_string(),
            AuthenApiV1::AccessToken => "/open-apis/authen/v1/access_token".to_string(),
            AuthenApiV1::RefreshAccessToken => {
                "/open-apis/authen/v1/refresh_access_token".to_string()
            }
            AuthenApiV1::OidcAccessToken => "/open-apis/authen/v1/oidc/access_token".to_string(),
            AuthenApiV1::OidcRefreshAccessToken => {
                "/open-apis/authen/v1/oidc/refresh_access_token".to_string()
            }
        }
    }
}

/// OAuth Old API 端点枚举
#[derive(Debug, Clone)]
pub enum OAuthApiOld {
    /// 获取登录预授权码
    Index,
}

impl OAuthApiOld {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            OAuthApiOld::Index => "/open-apis/authen/v1/index".to_string(),
        }
    }
}

/// 模块导出
pub mod prelude {
    pub use super::{AuthApiV3, AuthenApiV1, OAuthApiOld};
}
