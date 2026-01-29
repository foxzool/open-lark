/// 文档处理 v1 API 模块
///
/// 提供文档处理和聊天公告相关的API功能，包括文档创建、内容管理、区块操作等。
pub mod chat;
pub mod document;

pub use chat::{
    GetChatAnnouncementBlockChildrenParams, GetChatAnnouncementBlockChildrenRequest,
    GetChatAnnouncementBlockChildrenResponse, GetChatAnnouncementBlockParams,
    GetChatAnnouncementBlockRequest, GetChatAnnouncementBlockResponse,
    GetChatAnnouncementBlocksParams, GetChatAnnouncementBlocksRequest,
    GetChatAnnouncementBlocksResponse, GetChatAnnouncementRequest, GetChatAnnouncementResponse,
};

pub use document::{
    BatchUpdateDocumentBlocksParams, BatchUpdateDocumentBlocksRequest,
    BatchUpdateDocumentBlocksResponse, BatchUpdateRequest,
    ConvertContentToBlocksParams, ConvertContentToBlocksRequest, ConvertContentToBlocksResponse,
    ContentType,
    CreateDocumentBlockChildrenParams, CreateDocumentBlockChildrenRequest,
    CreateDocumentBlockChildrenResponse, CreateDocumentBlockDescendantParams,
    CreateDocumentBlockDescendantRequest, CreateDocumentBlockDescendantResponse, BlockIdRelation,
    CreateDocumentParams, CreateDocumentRequest, CreateDocumentResponse, CreatedDocument,
    Document, DocumentCover, DocumentDisplaySetting,
    GetDocumentBlockChildrenParams, GetDocumentBlockChildrenRequest,
    GetDocumentBlockChildrenResponse, GetDocumentBlockParams, GetDocumentBlockRequest,
    GetDocumentBlockResponse, GetDocumentBlocksParams, GetDocumentBlocksRequest,
    GetDocumentBlocksResponse, GetDocumentRawContentParams, GetDocumentRawContentRequest,
    GetDocumentRawContentResponse, GetDocumentRequest, GetDocumentResponse,
    UpdateDocumentBlockParams, UpdateDocumentBlockRequest, UpdateDocumentBlockResponse,
};
