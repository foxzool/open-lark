/// 文档处理 v1 API 模块
///
/// 提供文档处理和聊天公告相关的API功能，包括文档创建、内容管理、区块操作等。
pub mod chat;
/// document 子模块。
pub mod document;

/// 重新导出相关类型。
pub use chat::{
    GetChatAnnouncementBlockChildrenParams, GetChatAnnouncementBlockChildrenRequest,
    GetChatAnnouncementBlockChildrenResponse, GetChatAnnouncementBlockParams,
    GetChatAnnouncementBlockRequest, GetChatAnnouncementBlockResponse,
    GetChatAnnouncementBlocksParams, GetChatAnnouncementBlocksRequest,
    GetChatAnnouncementBlocksResponse, GetChatAnnouncementRequest, GetChatAnnouncementResponse,
};

/// 重新导出相关类型。
pub use document::{
    BatchUpdateDocumentBlocksParams, BatchUpdateDocumentBlocksRequest,
    BatchUpdateDocumentBlocksResponse, BatchUpdateRequest, BlockIdRelation, ContentType,
    ConvertContentToBlocksParams, ConvertContentToBlocksRequest, ConvertContentToBlocksResponse,
    CreateDocumentBlockChildrenParams, CreateDocumentBlockChildrenRequest,
    CreateDocumentBlockChildrenResponse, CreateDocumentBlockDescendantParams,
    CreateDocumentBlockDescendantRequest, CreateDocumentBlockDescendantResponse,
    CreateDocumentRequest, CreateDocumentResponse, CreatedDocument, Document, DocumentCover,
    DocumentDisplaySetting, GetDocumentBlockChildrenParams, GetDocumentBlockChildrenRequest,
    GetDocumentBlockChildrenResponse, GetDocumentBlockParams, GetDocumentBlockRequest,
    GetDocumentBlockResponse, GetDocumentBlocksParams, GetDocumentBlocksRequest,
    GetDocumentBlocksResponse, GetDocumentRawContentParams, GetDocumentRawContentRequest,
    GetDocumentRawContentResponse, GetDocumentRequest, GetDocumentResponse,
    UpdateDocumentBlockParams, UpdateDocumentBlockRequest, UpdateDocumentBlockResponse,
};
