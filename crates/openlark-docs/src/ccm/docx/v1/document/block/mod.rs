/// block模块 - 文档块操作API
///
/// 按照bizTag/project/version/resource/name.rs模式组织
pub mod batch_update;
pub mod children;
pub mod descendant;
pub mod get;
pub mod list;
pub mod patch;

// 使用通配符导出所有子模块
pub use batch_update::*;
pub use children::*;
pub use descendant::*;
pub use get::*;
pub use list::*;
pub use patch::*;
