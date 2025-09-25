//! auth 服务端点常量定义
//!
//! 认证相关 API 端点常量，包括：
//! - 应用访问令牌管理
//! - 用户访问令牌管理
//! - 令牌刷新

/// 获取应用访问令牌
pub const AUTH_ACCESS_TOKEN: &str = "/open-apis/auth/v3/app_access_token";

/// 获取用户访问令牌
pub const AUTH_USER_ACCESS_TOKEN: &str = "/open-apis/auth/v3/user_access_token";

/// 刷新用户访问令牌
pub const AUTH_REFRESH_USER_TOKEN: &str = "/open-apis/auth/v3/refresh_user_access_token";

/// 获取用户信息
pub const AUTHEN_V1_USER_INFO: &str = "/open-apis/authen/v1/user_info";
