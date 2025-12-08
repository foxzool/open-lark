//! Wiki V2 API 模块

pub mod space;
pub mod task;

// 导出数据模型
pub mod models;
pub mod service;

// 导出数据模型
pub use models::{
    WikiSpace, WikiSpaceNode, WikiSpaceMember, WikiSpaceSetting,
    WikiTask, WikiSearchResult
};

// 导出API服务 - 使用glob导入避免复杂的路径指定
pub use space::*;

// 重新导出主要服务
pub use service::WikiService;