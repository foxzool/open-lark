#![allow(clippy::module_inception)]

//! 知识库模块入口，聚合 baike、lingo 与共享模型能力。

/// Baike 模块
pub mod baike;
/// Lingo 模块。
pub mod lingo;
/// Baike/Lingo 共享数据模型。
pub mod models;

// 显式导出 models 中的公共数据类型（避免与 baike/lingo 子模块名冲突）
pub use models::{
    ApiResponse, Classification, Draft, DraftStatus, Entity, EntityCover, EntityExtractResult,
    EntityMatchResult, EntitySearchResult, EntityType, FileUploadRequest, FileUploadResult,
    HighlightInfo, PageResponse, Repository,
};
