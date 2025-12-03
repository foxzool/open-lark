//! OAuth授权服务 (oauth项目)
//!
//! 提供 old 版本的OAuth授权API服务。

pub mod old;

// 重新导出 old 服务
pub use old::OAuthServiceOld;
