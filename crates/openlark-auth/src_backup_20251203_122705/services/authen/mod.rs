//! 用户身份认证服务 (authen项目)
//!
//! 提供 v1 版本的用户身份认证API服务。

pub mod v1;

// 重新导出 v1 服务
pub use v1::AuthenServiceV1;
