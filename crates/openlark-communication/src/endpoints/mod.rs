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

// aily 模块显式导出
pub use aily::*;
// contact 模块显式导出
pub use contact::*;
// event 模块显式导出
pub use event::*;
// im 模块显式导出
pub use im::*;
// mail 模块显式导出
pub use mail::*;
// moments 模块显式导出
pub use moments::*;
// vc 模块显式导出
pub use vc::*;
