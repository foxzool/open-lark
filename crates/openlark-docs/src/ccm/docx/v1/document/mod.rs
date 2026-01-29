/// 文档模块
pub mod block;
pub mod convert;
pub mod create;
pub mod get;
pub mod raw_content;

pub use block::{
    BatchUpdateDocumentBlocksParams, BatchUpdateDocumentBlocksRequest,
    BatchUpdateDocumentBlocksResponse, BatchUpdateRequest,
    CreateDocumentBlockChildrenParams, CreateDocumentBlockChildrenRequest,
    CreateDocumentBlockChildrenResponse, CreateDocumentBlockDescendantParams,
    CreateDocumentBlockDescendantRequest, CreateDocumentBlockDescendantResponse, BlockIdRelation,
    GetDocumentBlockChildrenParams, GetDocumentBlockChildrenRequest,
    GetDocumentBlockChildrenResponse, GetDocumentBlockParams, GetDocumentBlockRequest,
    GetDocumentBlockResponse, GetDocumentBlocksParams, GetDocumentBlocksRequest,
    GetDocumentBlocksResponse, UpdateDocumentBlockParams, UpdateDocumentBlockRequest,
    UpdateDocumentBlockResponse,
};

pub use convert::{
    ConvertContentToBlocksParams, ConvertContentToBlocksRequest, ConvertContentToBlocksResponse,
    ContentType,
};

pub use create::{
    CreateDocumentParams, CreateDocumentRequest, CreateDocumentResponse, CreatedDocument,
};

pub use get::{
    Document, DocumentCover, DocumentDisplaySetting, GetDocumentRequest, GetDocumentResponse,
};

pub use raw_content::{
    GetDocumentRawContentParams, GetDocumentRawContentRequest, GetDocumentRawContentResponse,
};
