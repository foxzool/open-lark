//! 企业应用认证服务 (auth项目)
//!
//! 提供 v3 版本的企业应用认证API服务。

pub mod v3;

// 重新导出 v3 服务
pub use v3::AuthServiceV3;
