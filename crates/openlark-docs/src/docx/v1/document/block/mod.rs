//! 文档块模块

pub mod list;
pub mod children;
pub mod batch_update;
pub mod get;
pub mod patch;

// 导出所有API
pub use list::{ListDocumentBlocksRequest, ListDocumentBlocksParams, ListDocumentBlocksResponse};
pub use children::*;
pub use batch_update::{BatchUpdateDocumentBlocksRequest, BatchUpdateDocumentBlocksParams, BatchUpdateDocumentBlocksResponse};
pub use get::{GetDocumentBlockRequest, GetDocumentBlockParams, GetDocumentBlockResponse};
pub use patch::{PatchDocumentBlockRequest, PatchDocumentBlockParams, PatchDocumentBlockResponse};