//! 文档块子内容模块

pub mod create;
pub mod batch_delete;
pub mod create_nested;
pub mod get;

// 导出所有API
pub use create::{CreateDocumentBlockChildrenRequest, CreateDocumentBlockChildrenParams, CreateDocumentBlockChildrenResponse};
pub use batch_delete::{BatchDeleteDocumentBlockChildrenRequest, BatchDeleteDocumentBlockChildrenParams, BatchDeleteDocumentBlockChildrenResponse};
pub use create_nested::{CreateNestedDocumentBlockChildrenRequest, CreateNestedDocumentBlockChildrenParams, CreateNestedDocumentBlockChildrenResponse};
pub use get::{GetDocumentBlockChildrenRequest, GetDocumentBlockChildrenParams, GetDocumentBlockChildrenResponse};