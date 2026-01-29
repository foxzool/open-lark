/// Wiki V2 API 模块
pub mod space;
pub mod task;

// 导出数据模型
pub mod models;

// 导出数据模型
pub use models::{
    WikiSearchResult, WikiSpace, WikiSpaceMember, WikiSpaceNode, WikiSpaceSetting, WikiTask,
};

// 导出API服务 - 使用glob导入避免复杂的路径指定
// space 模块显式导出
#[allow(deprecated)]
pub use space::*;
