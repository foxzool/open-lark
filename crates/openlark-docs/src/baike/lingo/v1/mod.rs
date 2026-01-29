/// Lingo v1 API 模块
pub mod classification;
pub mod draft;
pub mod entity;
pub mod file;
pub mod models;
pub mod repo;

pub use classification::{ListClassificationRequest, ListClassificationResp};
pub use draft::{CreateDraftRequest, CreateDraftResp, UpdateDraftRequest, UpdateDraftResp};
pub use entity::{
    ClassificationFilter, CreateEntityRequest, CreateEntityResp, DeleteEntityRequest,
    DeleteEntityResp, GetEntityRequest, GetEntityResp, HighlightEntityBody, HighlightEntityRequest,
    HighlightEntityResp, ListEntityRequest, ListEntityResp, MatchEntityBody, MatchEntityRequest,
    MatchEntityResp, MatchInfo, Phrase, SearchEntityBody, SearchEntityRequest, SearchEntityResp,
    Span, TermType, UpdateEntityRequest, UpdateEntityResp,
};
pub use file::{DownloadFileRequest, UploadFileRequest, UploadFileResp};
pub use models::{
    Abbreviation, BaikeImage, Classification, ClassificationRef, DisplayStatus, Draft,
    DraftEntityInput, DraftUpdateEntityInput, Entity, EntityInput, I18nClsName, I18nEntryDesc,
    Language, LingoDraft, LingoEntity, OuterInfo, RelatedChat, RelatedDoc, RelatedLink,
    RelatedMeta, RelatedOncall, RelatedUser, Repo, Statistics, Term, UserIdType,
};
pub use repo::{ListRepoRequest, ListRepoResp};
