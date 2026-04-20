/// 文档模块
pub mod block;
/// convert 子模块。
pub mod convert;
/// create 子模块。
pub mod create;
/// get 子模块。
pub mod get;
/// raw_content 子模块。
pub mod raw_content;

/// 重新导出相关类型。
pub use block::{
    BatchUpdateDocumentBlocksParams, BatchUpdateDocumentBlocksRequest,
    BatchUpdateDocumentBlocksResponse, BatchUpdateRequest, BlockIdRelation,
    CreateDocumentBlockChildrenParams, CreateDocumentBlockChildrenRequest,
    CreateDocumentBlockChildrenResponse, CreateDocumentBlockDescendantParams,
    CreateDocumentBlockDescendantRequest, CreateDocumentBlockDescendantResponse,
    GetDocumentBlockChildrenParams, GetDocumentBlockChildrenRequest,
    GetDocumentBlockChildrenResponse, GetDocumentBlockParams, GetDocumentBlockRequest,
    GetDocumentBlockResponse, GetDocumentBlocksParams, GetDocumentBlocksRequest,
    GetDocumentBlocksResponse, UpdateDocumentBlockParams, UpdateDocumentBlockRequest,
    UpdateDocumentBlockResponse,
};

/// 重新导出相关类型。
pub use convert::{
    ContentType, ConvertContentToBlocksParams, ConvertContentToBlocksRequest,
    ConvertContentToBlocksResponse,
};

/// 重新导出相关类型。
pub use create::{CreateDocumentRequest, CreateDocumentResponse, CreatedDocument};

/// 重新导出相关类型。
pub use get::{
    Document, DocumentCover, DocumentDisplaySetting, GetDocumentRequest, GetDocumentResponse,
};

/// 重新导出相关类型。
pub use raw_content::{
    GetDocumentRawContentParams, GetDocumentRawContentRequest, GetDocumentRawContentResponse,
};
