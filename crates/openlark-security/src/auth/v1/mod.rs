//! 认证API v1版本
//!
//! 实现飞书开放平台认证相关的v1版本API接口。

pub mod login;
pub mod logout;
pub mod token;
pub mod user;

// 重新导出所有v1 API函数
pub use login::*;
pub use logout::*;
pub use token::*;
pub use user::*;
