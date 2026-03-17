/// Baike v1 API 模块
pub mod classification;
pub mod draft;
pub mod entity;
pub mod file;
pub mod models;
pub mod service;

// 重新导出分类模块
pub use classification::{ListClassificationRequest, ListClassificationResponse};

// 重新导出草稿模块
pub use draft::{
    CreateDraftReq, CreateDraftRequest, CreateDraftResp, Draft, UpdateDraftReq, UpdateDraftRequest,
    UpdateDraftResp,
};

// 重新导出实体模块
pub use entity::{
    ClassificationFilter, CreateEntityReq, CreateEntityRequest, CreateEntityResp,
    ExtractEntityReqBody, ExtractEntityRequest, ExtractEntityResponse, ExtractedWord,
    GetEntityRequest, GetEntityResp, HighlightEntityReqBody, HighlightEntityRequest,
    HighlightEntityResponse, ListEntityRequest, ListEntityResp, MatchEntityReq, MatchEntityRequest,
    MatchEntityResp, MatchEntityResult, Phrase, SearchEntityReqBody, SearchEntityRequest,
    SearchEntityResponse, Span, UpdateEntityReq, UpdateEntityRequest, UpdateEntityResp,
};

// 重新导出文件模块
pub use file::{DownloadFileRequest, UploadFileRequest, UploadFileResponse};

// 重新导出模型模块
pub use models::*;
