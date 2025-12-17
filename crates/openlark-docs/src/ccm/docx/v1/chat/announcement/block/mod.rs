/// block模块 - 群公告块相关API
///
/// 按照bizTag/project/version/resource/name.rs模式组织
mod batch_update;
mod children;
mod get;
mod list;

pub use batch_update::*;
pub use children::*;
pub use get::*;
pub use list::*;
