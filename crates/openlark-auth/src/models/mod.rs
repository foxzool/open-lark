//! 数据模型定义
//!
//! 本模块包含认证相关的所有数据结构定义，按照业务领域组织。

pub mod auth;
pub mod authen;
pub mod oauth;

// 重新导出所有公共类型
pub use auth::*;
pub use authen::*;
pub use oauth::*;