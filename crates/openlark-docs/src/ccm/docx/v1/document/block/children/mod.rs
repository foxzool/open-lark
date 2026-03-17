/// children模块 - 文档块子块操作API
///
/// 按照bizTag/project/version/resource/name.rs模式组织
pub mod batch_delete;
pub mod create;
pub mod get;

// 使用通配符导出所有子模块
// batch_delete 模块显式导出
pub use batch_delete::{
    BatchDeleteDocumentBlockChildrenParams, BatchDeleteDocumentBlockChildrenRequest,
    BatchDeleteDocumentBlockChildrenResponse,
};
// create 模块显式导出
pub use create::{
    CreateDocumentBlockChildrenParams, CreateDocumentBlockChildrenRequest,
    CreateDocumentBlockChildrenResponse,
};
// get 模块显式导出
pub use get::{
    GetDocumentBlockChildrenParams, GetDocumentBlockChildrenRequest,
    GetDocumentBlockChildrenResponse,
};
