//! 客户端层模块
//!
//! 提供高级客户端接口，简化认证操作的复杂性。

pub mod auth_client;
pub mod config;
pub mod token_client;

// 重新导出主要类型
pub use auth_client::*;
pub use config::*;
pub use token_client::*;
