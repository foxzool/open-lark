//! 用户身份认证 v1 版本 (Version: v1)
//!
//! 提供用户身份认证的v1版本API实现。

pub mod access_token;
pub mod oidc;
pub mod user_info;

// 重新导出服务
pub use access_token::AccessTokenService;
pub use oidc::OidcService;
pub use user_info::UserInfoService;
