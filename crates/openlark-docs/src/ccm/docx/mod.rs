/// ccm/docx模块 - 文档块内容管理
///
/// 按照bizTag/project/version/resource/name.rs模式组织
/// 包含chat公告和document操作的相关API
/// 数据模型定义（模型，不算 API）
pub mod documents;
pub mod models;
pub mod v1;

// 使用通配符导出所有子模块
// models 模块显式导出
pub use models::{
    DocumentBasicInfo, DocumentItem, DocumentMetaItem, DocumentPermissions, DocumentStatistics,
    FolderInfo, GetDocMetaRequest, GetDocMetaResponse, SearchDocsRequest, SearchDocsResponse,
    UserInfo,
};
// models::common_types 子模块导出
pub use models::common_types::{
    BatchOperationResult, BlockContent, BlockItem, BlockUpdate, DocxBlock, DocxBlockPage,
    FailedItem, PageData, Paragraph, RichText, TextElement, TextRun, TextStyle,
};
// models::models_docx 子模块导出
pub use models::models_docx::{
    BatchDeleteBlocksRequest, BatchDeleteBlocksResponse, BatchUpdateBlocksRequest,
    BatchUpdateBlocksResponse, BlockInfo, CreateBlockRequest, CreateBlockResponse,
    CreateDocumentRequest, CreateDocumentResponse, DeleteBlockRequest, DeleteBlockResponse,
    DocumentInfo, GetBlockRequest, GetBlockResponse, GetDocumentRequest, GetDocumentResponse,
    GetRawContentRequest, GetRawContentResponse, ListBlocksRequest, ListBlocksResponse,
    UpdateBlockRequest, UpdateBlockResponse,
};
// v1 模块显式导出
pub use v1::*;
