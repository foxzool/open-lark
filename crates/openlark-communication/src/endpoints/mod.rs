//! API 端点常量定义
//!
//! 按服务分类组织的端点常量

pub mod aily;
pub mod contact;
pub mod event;
pub mod im;
pub mod mail;
pub mod moments;
pub mod vc;

// Re-export commonly used endpoints
pub use aily::*;
pub use contact::*;
pub use event::*;
pub use im::*;
pub use mail::*;
pub use moments::*;
pub use vc::*;
