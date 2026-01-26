/// block模块 - 群公告块相关API
///
/// 按照bizTag/project/version/resource/name.rs模式组织
pub mod batch_update;
pub mod children;
pub mod get;
pub mod list;

// 使用通配符导出所有子模块
pub use batch_update::*;
pub use children::*;
pub use get::*;
pub use list::*;
