//! 核心认证服务
//!
//! 本模块提供飞书开放平台的认证服务实现，包括企业应用认证、用户认证和OAuth授权。

/// 企业认证服务
pub mod auth_service;

/// 用户认证服务
pub mod authen_service;

/// OAuth授权服务
pub mod oauth_service;

// 重新导出服务类型
pub use auth_service::AuthService;
pub use authen_service::AuthenService;
pub use oauth_service::OAuthService;
