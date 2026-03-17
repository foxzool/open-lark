//! Auth API实现模块 (meta.project=auth)
//!
//! 包含企业认证相关的API实现：
//! - app_access_token: 商店应用获取app_access_token
//! - app_access_token_internal: 自建应用获取app_access_token
//! - tenant_access_token: 商店应用获取tenant_access_token
//! - tenant_access_token_internal: 自建应用获取tenant_access_token
//! - app_ticket: 重新获取app_ticket

#![allow(clippy::module_inception)]

// v3 auth 模块显式导出
pub use self::auth::v3::auth::{
    AppAccessTokenBuilder, AppAccessTokenInternalBuilder, AppTicketResendBuilder, AuthServiceV3,
    TenantAccessTokenBuilder, TenantAccessTokenInternalRequestBuilder,
};

pub mod auth;
pub mod authen;
pub mod oauth;
