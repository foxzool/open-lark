//! OAuth授权 old 版本 (Version: old)
//!
//! 提供向后兼容的OAuth授权API实现。

pub mod authorization;

// 重新导出服务
pub use authorization::AuthorizationService;
