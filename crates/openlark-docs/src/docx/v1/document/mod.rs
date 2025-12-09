//! 文档模块

pub mod create;
pub mod get;
pub mod raw_content;
pub mod block;
pub mod convert;

// 导出所有API
pub use create::{CreateDocumentRequest, CreateDocumentParams, CreateDocumentResponse};
pub use get::{GetDocumentRequest, GetDocumentParams, GetDocumentResponse};
pub use raw_content::{GetDocumentRawContentRequest, GetDocumentRawContentParams, GetDocumentRawContentResponse};
pub use block::*;
pub use convert::{ConvertContentToBlocksRequest, ConvertContentToBlocksParams, ConvertContentToBlocksResponse};