/// Wiki V1 API 模块
pub mod node;

// 导出所有子模块内容，避免命名冲突
// node 模块显式导出
pub use node::{
    SearchWikiParams,
    SearchWikiRequest,
    SearchWikiResponse,
    execute,
    execute_with_options,
    new,
    node_id,
    page_size,
    page_token,
    query,
    space_id,
};
