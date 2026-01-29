/// Baike v1 API 模块
pub mod classification;
pub mod draft;
pub mod entity;
pub mod file;
pub mod models;
pub mod service;

pub use classification::ListClassificationRequest;

pub use draft::{CreateDraftReq, CreateDraftRequest, CreateDraftResp, Draft, UpdateDraftReq, UpdateDraftRequest, UpdateDraftResp};

pub use entity::{
    CreateEntityReq, CreateEntityRequest, CreateEntityResp,
    ExtractEntityRequest, ExtractEntityResponse,
    GetEntityRequest, GetEntityResp,
    HighlightEntityRequest, HighlightEntityResponse,
    ListEntityRequest, ListEntityResp,
    MatchEntityRequest, MatchEntityResp,
    SearchEntityRequest, SearchEntityResponse,
    UpdateEntityReq, UpdateEntityRequest, UpdateEntityResp,
};

pub use file::{DownloadFileRequest, UploadFileRequest, UploadFileResponse};

pub use models::{
    BaikeImage, ClassificationItem, DisplayStatus, Entity, EntityStatistics,
    OuterInfo, RelatedMeta, Referer, Term, UserIdType,
};

pub use service::BaikeV1Service;
