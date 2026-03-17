/// Wiki V2 API 模块
pub mod space;
pub mod task;

// 导出数据模型
pub mod models;

// 导出数据模型
pub use models::{
    WikiSearchResult, WikiSpace, WikiSpaceMember, WikiSpaceNode, WikiSpaceSetting, WikiTask,
};

// 导出API服务 - 使用通配符导出space模块的所有内容
#[allow(deprecated)]
pub use space::*;
