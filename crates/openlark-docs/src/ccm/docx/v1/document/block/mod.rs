//! block模块 - 文档块操作API
//!
//! 按照bizTag/project/version/resource/name.rs模式组织

pub mod list;
pub mod get;
pub mod patch;
pub mod batch_update;
pub mod children;
pub mod descendant;

pub use list::*;
pub use get::*;
pub use patch::*;
pub use batch_update::*;
pub use children::*;
pub use descendant::*;