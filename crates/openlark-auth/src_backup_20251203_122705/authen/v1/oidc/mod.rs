//! OIDC相关API模块
//!
//! 包含OIDC（OpenID Connect）相关的认证API：
//! - access_token: 获取OIDC访问令牌
//! - refresh_access_token: 刷新OIDC访问令牌

pub mod access_token;
pub mod refresh_access_token;

// 重新导出所有服务
pub use access_token::*;
pub use refresh_access_token::*;