/// Wiki V2 任务模块
pub mod get;

// 导出所有子模块内容，避免命名冲突
// get 模块显式导出
pub use get::{
    GetWikiTaskRequest,
    GetWikiTaskResponse,
    execute,
    execute_with_options,
    new,
    task_id,
    task_type,
};
