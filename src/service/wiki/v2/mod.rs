//! Wiki知识库服务 v2
//!
//! 提供飞书知识库v2版本的完整管理功能，包括：
//! - 知识空间管理
//! - 空间成员权限管理
//! - 知识节点内容管理
//! - 搜索和任务管理

pub mod space_member;
pub mod space_node;
pub mod task;

// 重新导出所有服务类型
pub use space_member::*;
pub use space_node::*;
pub use task::*;

use open_lark_core::config::Config;

/// Wiki知识库服务 v2版本
///
/// 提供飞书知识库v2版本的统一入口，支持现代化的知识管理。
/// 包括空间管理、成员协作、内容组织等企业级功能。
#[derive(Debug, Clone)]
pub struct WikiServiceV2 {
    config: Config,
    /// 空间成员管理服务
    pub space_member: SpaceMemberService,
    /// 空间节点管理服务
    pub space_node: SpaceNodeService,
    /// 任务管理服务
    pub task: TaskService,
}

impl WikiServiceV2 {
    /// 创建Wiki v2服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::wiki::v2::WikiServiceV2;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = WikiServiceV2::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            space_member: SpaceMemberService::new(config.clone()),
            space_node: SpaceNodeService::new(config.clone()),
            task: TaskService::new(config),
        }
    }
}

impl crate::core::trait_system::Service for WikiServiceV2 {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "WikiServiceV2"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_lark_core::trait_system::Service;

    #[test]
    fn test_wiki_v2_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = WikiServiceV2::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_v2_service_trait_implementation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = WikiServiceV2::new(config);

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id, "test_app_id");
    }

    #[test]
    fn test_space_member_service_available() {
        let config = Config::default();
        let service = WikiServiceV2::new(config);

        // 验证space_member服务可用
        let space_member_service_str = format!("{:?}", service.space_member);
        assert!(!space_member_service_str.is_empty());
    }

    #[test]
    fn test_space_node_service_available() {
        let config = Config::default();
        let service = WikiServiceV2::new(config);

        // 验证space_node服务可用
        let space_node_service_str = format!("{:?}", service.space_node);
        assert!(!space_node_service_str.is_empty());
    }

    #[test]
    fn test_task_service_available() {
        let config = Config::default();
        let service = WikiServiceV2::new(config);

        // 验证task服务可用
        let task_service_str = format!("{:?}", service.task);
        assert!(!task_service_str.is_empty());
    }
}
