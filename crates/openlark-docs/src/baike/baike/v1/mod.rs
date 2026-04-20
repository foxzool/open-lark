/// Baike v1 分类模块。
pub mod classification;
/// Baike v1 草稿模块。
pub mod draft;
/// Baike v1 词条模块。
pub mod entity;
/// Baike v1 文件模块。
pub mod file;
/// Baike v1 数据模型模块。
pub mod models;
/// Baike v1 服务模块。
pub mod service;

/// 重新导出分类相关类型。
pub use classification::{ListClassificationRequest, ListClassificationResponse};

/// 重新导出草稿相关类型。
pub use draft::{
    CreateDraftReq, CreateDraftRequest, CreateDraftResp, Draft, UpdateDraftReq, UpdateDraftRequest,
    UpdateDraftResp,
};

/// 重新导出词条相关类型。
pub use entity::{
    ClassificationFilter, CreateEntityReq, CreateEntityRequest, CreateEntityResp,
    ExtractEntityReqBody, ExtractEntityRequest, ExtractEntityResponse, ExtractedWord,
    GetEntityRequest, GetEntityResp, HighlightEntityReqBody, HighlightEntityRequest,
    HighlightEntityResponse, ListEntityRequest, ListEntityResp, MatchEntityReq, MatchEntityRequest,
    MatchEntityResp, MatchEntityResult, Phrase, SearchEntityReqBody, SearchEntityRequest,
    SearchEntityResponse, Span, UpdateEntityReq, UpdateEntityRequest, UpdateEntityResp,
};

/// 重新导出文件相关类型。
pub use file::{DownloadFileRequest, UploadFileRequest, UploadFileResponse};

/// 重新导出通用模型。
pub use models::*;
