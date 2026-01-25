/// descendant模块 - 文档块子孙块操作API
///
/// 按照bizTag/project/version/resource/name.rs模式组织
pub mod create;

// create 模块显式导出
pub use create::{
    BlockIdRelation,
    CreateDocumentBlockDescendantParams,
    CreateDocumentBlockDescendantRequest,
    CreateDocumentBlockDescendantResponse,
    execute,
    execute_with_options,
    new,
};
