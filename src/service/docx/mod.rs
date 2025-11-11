//! Docx文档服务
//!
//! 提供飞书协作文档的完整管理功能，包括：
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
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 创建协作文档
//! let document = client.docx.v1.document
//!     .create_document_builder()
//!     .title("项目计划文档")
//!     .folder_token("folder_token")
//!     .execute(&client.docx.v1.document)
//!     .await?;
//! ```

pub mod v1;

// 重新导出所有服务类型
pub use v1::*;

use open_lark_core::config::Config;

/// Docx文档服务
///
/// 提供飞书协作文档的统一入口，支持文档的全生命周期管理。
/// 包括创建、编辑、分享、协作等企业级功能。
#[derive(Debug, Clone)]
pub struct DocxService {
    config: Config,
    /// v1版本服务
    #[cfg(feature = "collaboration")]
    pub v1: v1::DocxServiceV1,
}

impl DocxService {
    /// 创建Docx服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::docx::DocxService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = DocxService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            #[cfg(feature = "collaboration")]
            v1: v1::DocxServiceV1::new(config),
        }
    }
}

impl crate::core::trait_system::Service for DocxService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "DocxService"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_lark_core::trait_system::Service;

    #[test]
    fn test_docx_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = DocxService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_service_trait_implementation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = DocxService::new(config);

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id, "test_app_id");
    }
}
