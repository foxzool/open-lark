/// descendant模块 - 文档块子孙块操作API
///
/// 按照bizTag/project/version/resource/name.rs模式组织
pub mod create;

// 使用通配符导出所有子模块
// create 模块显式导出
pub use create::{
    BlockIdRelation, CreateDocumentBlockDescendantParams, CreateDocumentBlockDescendantRequest,
    CreateDocumentBlockDescendantResponse,
};
