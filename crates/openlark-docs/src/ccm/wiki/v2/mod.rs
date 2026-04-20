/// Wiki v2 模型模块。
pub mod models;
/// Wiki v2 空间模块。
pub mod space;
/// Wiki v2 任务模块。
pub mod task;

/// 重新导出 Wiki v2 模型。
pub use models::{
    WikiSearchResult, WikiSpace, WikiSpaceMember, WikiSpaceNode, WikiSpaceSetting, WikiTask,
};

/// 重新导出 Wiki v2 空间接口。
#[allow(deprecated)]
pub use space::*;
