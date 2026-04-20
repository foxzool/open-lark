/// children模块 - 文档块子块操作API
///
/// 按照bizTag/project/version/resource/name.rs模式组织
pub mod batch_delete;
/// create 子模块。
pub mod create;
/// get 子模块。
pub mod get;

// 使用通配符导出所有子模块
// batch_delete 模块显式导出
/// 重新导出相关类型。
pub use batch_delete::{
    BatchDeleteDocumentBlockChildrenParams, BatchDeleteDocumentBlockChildrenRequest,
    BatchDeleteDocumentBlockChildrenResponse,
};
// create 模块显式导出
/// 重新导出相关类型。
pub use create::{
    CreateDocumentBlockChildrenParams, CreateDocumentBlockChildrenRequest,
    CreateDocumentBlockChildrenResponse,
};
// get 模块显式导出
/// 重新导出相关类型。
pub use get::{
    GetDocumentBlockChildrenParams, GetDocumentBlockChildrenRequest,
    GetDocumentBlockChildrenResponse,
};
