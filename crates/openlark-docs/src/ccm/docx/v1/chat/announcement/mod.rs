//! announcement模块 - 群公告相关API
//!
//! 按照bizTag/project/version/resource/name.rs模式组织

pub mod block;
pub mod get;

pub use block::*;
pub use get::*;
