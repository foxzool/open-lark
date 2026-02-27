#![allow(clippy::module_inception)]

/// Baike 模块
pub mod baike;
pub mod lingo;
pub mod models;

// 显式导出 models 中的公共数据类型（避免与 baike/lingo 子模块名冲突）
pub use models::{
    ApiResponse, Classification, Draft, DraftStatus, Entity, EntityCover, EntityExtractResult,
    EntityMatchResult, EntitySearchResult, EntityType, FileUploadRequest, FileUploadResult,
    HighlightInfo, PageResponse, Repository,
};
