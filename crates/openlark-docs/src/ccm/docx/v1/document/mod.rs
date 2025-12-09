//! document模块 - 文档基础操作API
//!
//! 按照bizTag/project/version/resource/name.rs模式组织

pub mod create;
pub mod get;
pub mod raw_content;
pub mod convert;
pub mod block;

pub use create::*;
pub use get::*;
pub use raw_content::*;
pub use convert::*;
pub use block::*;