//! Wiki知识库服务
//!
//! 提供飞书知识库的完整管理功能，包括：
//! - 知识空间创建和管理
//! - 空间成员权限管理
//! - 知识节点内容管理
//! - 文档导入导出功能
//! - 搜索和任务管理
//!
//! # 服务架构
//!
//! ## v2版本
//! - [`v2::space_member`] - 空间成员管理服务，提供成员增删改查功能
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
//! // 删除知识空间成员
//! let result = client.wiki.v2.space_member
//!     .delete_space_member_builder()
//!     .space_id("space_id")
//!     .member_id("member_id")
//!     .execute(&client.wiki.v2.space_member)
//!     .await?;
//! ```

// 核心服务模块 - 使用条件编译
#[cfg(feature = "collaboration")]
pub mod v2;

// 重新导出所有服务类型
#[cfg(feature = "collaboration")]
pub use v2::*;

use crate::core::config::Config;

/// Wiki知识库服务
///
/// 提供飞书知识库的统一入口，支持知识库的全生命周期管理。
/// 包括空间管理、成员权限、内容组织、搜索等企业级功能。
#[derive(Debug, Clone)]
pub struct WikiService {
    config: Config,
    /// v2版本服务
    #[cfg(feature = "collaboration")]
    pub v2: v2::WikiServiceV2,
}

impl WikiService {
    /// 创建Wiki服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::wiki::WikiService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = WikiService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            #[cfg(feature = "collaboration")]
            v2: v2::WikiServiceV2::new(config),
        }
    }
}

impl crate::core::trait_system::Service for WikiService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "WikiService"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::trait_system::Service;

    #[test]
    fn test_wiki_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = WikiService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_service_trait_implementation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = WikiService::new(config);

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id, "test_app_id");
    }

    #[test]
    fn test_v2_service_available() {
        let config = Config::default();
        let service = WikiService::new(config);

        // 验证v2服务可用
        #[cfg(feature = "collaboration")]
        {
            let v2_service_str = format!("{:?}", service.v2);
            assert!(!v2_service_str.is_empty());
        }
    }
}
