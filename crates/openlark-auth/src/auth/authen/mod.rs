//! Authen API实现模块 (meta.project=authen)
//!
//! 包含用户认证相关的API实现：
//! - user_info.get: 获取用户信息
//! - access_token.create: 获取user_access_token (v1版本)
//! - refresh_access_token.create: 刷新user_access_token (v1版本)
//! - oidc.access_token.create: 获取user_access_token (OIDC)
//! - oidc.refresh_access_token.create: 刷新user_access_token (OIDC)

// 重新导出具体API实现
pub use self::v1::*;

pub mod v1;
