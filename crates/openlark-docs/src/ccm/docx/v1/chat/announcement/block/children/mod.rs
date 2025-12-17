/// children模块 - 群公告块子块相关API
///
/// 按照bizTag/project/version/resource/name.rs模式组织
pub mod batch_delete;
pub mod create;
pub mod get;

pub use batch_delete::*;
pub use create::*;
pub use get::*;
