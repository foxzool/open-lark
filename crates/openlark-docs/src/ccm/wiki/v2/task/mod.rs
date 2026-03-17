/// Wiki V2 任务模块
pub mod get;

// 使用通配符导出所有子模块
// get 模块显式导出
pub use get::{
    GetWikiTaskRequest,
    GetWikiTaskResponse,
};
