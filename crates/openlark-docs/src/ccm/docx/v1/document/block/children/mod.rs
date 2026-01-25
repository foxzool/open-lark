/// children模块 - 文档块子块操作API
///
/// 按照bizTag/project/version/resource/name.rs模式组织
pub mod batch_delete;
pub mod create;
pub mod get;

// batch_delete 模块显式导出
pub use batch_delete::{
    BatchDeleteDocumentBlockChildrenParams,
    BatchDeleteDocumentBlockChildrenRequest,
    BatchDeleteDocumentBlockChildrenResponse,
    CreateDocumentBlockChildrenParams,
    CreateDocumentBlockChildrenRequest,
    CreateDocumentBlockChildrenResponse,
    GetDocumentBlockChildrenParams,
    GetDocumentBlockChildrenRequest,
    GetDocumentBlockChildrenResponse,
    execute,
    execute_with_options,
    new,
};
// create 模块显式导出
pub use create::{
    BatchDeleteDocumentBlockChildrenParams,
    BatchDeleteDocumentBlockChildrenRequest,
    BatchDeleteDocumentBlockChildrenResponse,
    CreateDocumentBlockChildrenParams,
    CreateDocumentBlockChildrenRequest,
    CreateDocumentBlockChildrenResponse,
    GetDocumentBlockChildrenParams,
    GetDocumentBlockChildrenRequest,
    GetDocumentBlockChildrenResponse,
    execute,
    execute_with_options,
    new,
};
// get 模块显式导出
pub use get::{
    BatchDeleteDocumentBlockChildrenParams,
    BatchDeleteDocumentBlockChildrenRequest,
    BatchDeleteDocumentBlockChildrenResponse,
    CreateDocumentBlockChildrenParams,
    CreateDocumentBlockChildrenRequest,
    CreateDocumentBlockChildrenResponse,
    GetDocumentBlockChildrenParams,
    GetDocumentBlockChildrenRequest,
    GetDocumentBlockChildrenResponse,
    execute,
    execute_with_options,
    new,
};
