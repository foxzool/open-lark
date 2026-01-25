/// Wiki V1 节点模块
pub mod search;

// 导出所有子模块内容，避免命名冲突
// search 模块显式导出
pub use search::{
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
