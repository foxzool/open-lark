//! # 飞书词典服务
//!
//! 飞书词典 (Lingo) 服务提供完整的词典管理功能，支持以下核心能力：
//!
//! ## 功能特性
//!
//! - **草稿管理**：创建和更新词条草稿
//! - **词条管理**：免审词条的创建、更新、删除、查询、搜索和高亮
//! - **分类管理**：获取词典分类
//! - **词库管理**：获取词库列表
//! - **图片管理**：图片的上传和下载
//!
//! ## 服务模块
//!
//! 该服务包含以下功能模块：
//!
//! - [`models`] - 数据模型和类型定义
//! - [`draft`] - 草稿管理模块
//! - [`entity`] - 词条管理模块
//! - [`classification`] - 分类管理模块
//! - [`repo`] - 词库管理模块
//! - [`file`] - 图片管理模块
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::lingo::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder("app_id", "app_secret")
//!         .build();
//!
//!     // 获取词条列表
//!     let entities = client.lingo.entity.list_entities(
//!         entity::EntityListRequest::default(), None
//!     ).await?;
//!     
//!     // 获取词库列表
//!     let repos = client.lingo.repo.list_repos(
//!         repo::RepoListRequest::default(), None
//!     ).await?;
//!     
//!     // 搜索词条
//!     let results = client.lingo.entity.search_entities(
//!         entity::EntitySearchRequest {
//!             query: "搜索关键词".to_string(),
//!             ..Default::default()
//!         }, None
//!     ).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod classification;
pub mod draft;
pub mod entity;
pub mod file;
pub mod models;
pub mod repo;

use crate::{
    core::config::Config,
    service::lingo::{
        classification::ClassificationService, draft::DraftService, entity::EntityService,
        file::FileService, repo::RepoService,
    },
};

/// 飞书词典服务
///
/// 提供完整的词典管理功能，包括草稿、词条、分类、词库和图片管理
pub struct LingoService {
    /// 草稿管理服务
    pub draft: DraftService,
    /// 词条管理服务
    pub entity: EntityService,
    /// 分类管理服务
    pub classification: ClassificationService,
    /// 词库管理服务
    pub repo: RepoService,
    /// 图片管理服务
    pub file: FileService,
}

impl LingoService {
    /// 创建飞书词典服务实例
    pub fn new(config: Config) -> Self {
        Self {
            draft: DraftService::new(config.clone()),
            entity: EntityService::new(config.clone()),
            classification: ClassificationService::new(config.clone()),
            repo: RepoService::new(config.clone()),
            file: FileService::new(config),
        }
    }
}
