//! Docs文档服务
//!
//! 提供飞书文档的完整管理功能，包括：
//! - 创建和删除文档
//! - 读取和更新文档内容
//! - 文档块操作和内容管理
//! - 文档版本控制和历史管理
//! - 协作编辑和权限管理
//!
//! # 服务架构
//!
//! ## v1版本
//! - [`v1::document`] - 文档基础管理服务，提供创建、查询等核心功能
//!
//! ## v2版本（旧版文档）
//! - [`v2::create`] - 旧版文档创建服务
//! - [`v2::meta`] - 旧版文档元信息服务
//! - [`v2::content`] - 旧版文档内容服务（纯文本和富文本）
//! - [`v2::sheet_meta`] - 旧版电子表格元数据服务
//! - [`v2::batch_update`] - 旧版文档批量编辑服务
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 创建协作文档（v1版本）
//! let document = client.docs.v1.document
//!     .create_document_builder()
//!     .title("项目计划文档")
//!     .folder_token("folder_token")
//!     .execute(&client.docs.v1.document)
//!     .await?;
//!
//! // 创建旧版文档（v2版本）
//! let old_doc = client.docs.v2.create
//!     .create_doc_builder()
//!     .title("旧版文档")
//!     .execute(&client.docs.v2.create)
//!     .await?;
//! ```

// pub mod v1;  // 暂时禁用，待修复
pub mod v2;

// 重新导出所有服务类型，避免名称冲突
// pub use v1::{DocsServiceV1, DocumentService};

pub use v2::{CreateDocBuilder, CreateDocService, DocV2Service};

// 为向后兼容性提供DocxService别名
// #[cfg(feature = "ccm-doc")]
// pub use v1::DocsServiceV1 as DocxService;

use openlark_core::config::Config;

/// Docs文档服务
///
/// 提供飞书文档的统一入口，支持文档的全生命周期管理。
/// 包括创建、编辑、分享、协作等企业级功能。
#[derive(Clone, Debug)]
pub struct DocsService {
    config: Config,
    /// v2版本服务（旧版文档）
    #[cfg(feature = "ccm-doc")]
    pub v2: v2::DocV2Service,
    // v1版本服务暂时禁用，待修复
    // #[cfg(feature = "ccm-doc")]
    // pub v1: v1::DocsServiceV1,
}

impl DocsService {
    /// 创建Docs服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::docs::DocsService;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = DocsService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            #[cfg(feature = "ccm-doc")]
            v2: v2::DocV2Service::new(config),
            // v1: v1::DocsServiceV1::new(config),  // 暂时禁用
        }
    }
}

impl openlark_core::trait_system::Service for DocsService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "DocsService"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::Service;

    #[test]
    fn test_docs_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = DocsService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_service_trait_implementation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = DocsService::new(config);

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id, "test_app_id");
    }
}
