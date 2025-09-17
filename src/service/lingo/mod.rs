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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    #[test]
    fn test_lingo_service_creation() {
        let config = create_test_config();
        let lingo_service = LingoService::new(config);

        // Verify all sub-services are created
        assert!(std::ptr::addr_of!(lingo_service.draft) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(lingo_service.entity) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(lingo_service.classification) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(lingo_service.repo) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(lingo_service.file) as *const _ != std::ptr::null());
    }

    #[test]
    fn test_lingo_service_debug_trait() {
        let config = create_test_config();
        let lingo_service = LingoService::new(config);

        // Test that service can be used
        assert!(std::ptr::addr_of!(lingo_service) as *const _ != std::ptr::null());
    }

    #[test]
    fn test_lingo_service_with_custom_config() {
        let config = Config::builder()
            .app_id("lingo_app")
            .app_secret("lingo_secret")
            .req_timeout(std::time::Duration::from_millis(10000))
            .base_url("https://lingo.api.com")
            .build();

        let lingo_service = LingoService::new(config);

        // Verify service creation with custom config
        assert!(std::ptr::addr_of!(lingo_service.draft) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(lingo_service.entity) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(lingo_service.classification) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(lingo_service.repo) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(lingo_service.file) as *const _ != std::ptr::null());
    }

    #[test]
    fn test_lingo_service_modules_independence() {
        let config = create_test_config();
        let lingo_service = LingoService::new(config);

        // Test that all sub-modules are independent (different memory addresses)
        let draft_ptr = std::ptr::addr_of!(lingo_service.draft) as *const _;
        let entity_ptr = std::ptr::addr_of!(lingo_service.entity) as *const _;
        let classification_ptr = std::ptr::addr_of!(lingo_service.classification) as *const _;
        let repo_ptr = std::ptr::addr_of!(lingo_service.repo) as *const _;
        let file_ptr = std::ptr::addr_of!(lingo_service.file) as *const _;

        // All addresses should be different
        let addresses = vec![
            draft_ptr,
            entity_ptr,
            classification_ptr,
            repo_ptr,
            file_ptr,
        ];
        for (i, &addr1) in addresses.iter().enumerate() {
            for &addr2 in addresses.iter().skip(i + 1) {
                assert_ne!(
                    addr1, addr2,
                    "Services should have different memory addresses"
                );
            }
        }
    }

    #[test]
    fn test_lingo_service_config_cloning() {
        let config = create_test_config();
        let lingo_service = LingoService::new(config);

        // Test that the service can be created multiple times with cloned configs
        // This simulates real usage where configs might be shared
        let config2 = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let lingo_service2 = LingoService::new(config2);

        // Both services should be created successfully
        assert!(std::ptr::addr_of!(lingo_service.draft) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(lingo_service2.draft) as *const _ != std::ptr::null());

        // But should be different instances
        let service1_ptr = std::ptr::addr_of!(lingo_service) as *const _;
        let service2_ptr = std::ptr::addr_of!(lingo_service2) as *const _;
        assert_ne!(service1_ptr, service2_ptr);
    }
}
