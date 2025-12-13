#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

/// Wiki v2 API 模块
///
/// 提供知识库（Wiki）相关的API功能，包括知识空间、节点、任务、权限管理等。

use openlark_core::config::Config;

// 重新导出所有子模块
pub use space::*;
pub use space_member::*;
pub use space_setting::*;
pub use space_node::*;
pub use task::*;

// 子模块
pub mod space;
pub mod space_nodes;
pub mod space_member;
pub mod space_setting;
pub mod space_node;
pub mod task;

/// Wiki v2 服务
///
/// 提供知识库的完整管理功能，包括知识空间、文档节点、权限控制、任务管理等。
/// 支持个人和团队知识库的创建、编辑、分享和协作。
#[derive(Clone)]
pub struct WikiService {
    config: Config,
}

impl WikiService {
    /// 创建新的 Wiki 服务实例
    ///
    /// # 参数
    /// * `config` - SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取知识空间服务
    ///
    /// 访问知识空间相关的API，包括创建、查询、列表管理等。
    ///
    /// # 返回
    /// 返回知识空间服务实例
    pub fn space(&self) -> crate::ccm::wiki::v2::space::SpaceService {
        crate::ccm::wiki::v2::space::SpaceService::new(self.config.clone())
    }

    /// 获取知识空间成员服务
    ///
    /// 访问知识空间成员相关的API，包括成员添加、删除、列表查询等。
    ///
    /// # 返回
    /// 返回知识空间成员服务实例
    pub fn space_member(&self) -> crate::ccm::wiki::v2::space_member::SpaceMemberService {
        crate::ccm::wiki::v2::space_member::SpaceMemberService::new(self.config.clone())
    }

    /// 获取知识空间设置服务
    ///
    /// 访问知识空间设置相关的API，包括设置查询、权限管理等。
    ///
    /// # 返回
    /// 返回知识空间设置服务实例
    pub fn space_setting(&self) -> crate::ccm::wiki::v2::space_setting::SpaceSettingService {
        crate::ccm::wiki::v2::space_setting::SpaceSettingService::new(self.config.clone())
    }

    /// 获取知识空间节点服务
    ///
    /// 访问知识空间节点相关的API，包括节点创建、列表查询、移动等。
    ///
    /// # 返回
    /// 返回知识空间节点服务实例
    pub fn space_node(&self) -> crate::ccm::wiki::v2::space_node::SpaceNodeService {
        crate::ccm::wiki::v2::space_node::SpaceNodeService::new(self.config.clone())
    }

    /// 获取任务服务
    ///
    /// 访问任务相关的API，包括异步任务创建、状态查询等。
    ///
    /// # 返回
    /// 返回任务服务实例
    pub fn task(&self) -> crate::ccm::wiki::v2::task::TaskService {
        crate::ccm::wiki::v2::task::TaskService::new(self.config.clone())
}

impl openlark_core::trait_system::service::Service for WikiService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "wiki"
}

    fn transport(&self) -> &dyn openlark_core::http::Transport {
        panic!("WikiService does not have a transport instance")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_service() -> WikiService {
        let config = openlark_core::config::Config::new("test_app_id", "test_app_secret");
        WikiService::new(config)
    }

    #[test]
    fn test_wiki_service_creation() {
        let config = openlark_core::config::Config::new("test_app_id", "test_app_secret");
        let service = WikiService::new(config);

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[test]
    fn test_wiki_service_clone() {
        let service = create_test_service();
        let cloned_service = service.clone();

        assert_eq!(service.config().app_id(), cloned_service.config().app_id());
        assert_eq!(service.config().app_secret(), cloned_service.config().app_secret());
    }

    #[test]
    fn test_service_trait_implementation() {
        let service = create_test_service();

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id(), "test_app_id");
    }

    #[test]
    fn test_space_service_access() {
        let service = create_test_service();
        let space_service = service.space();

        // 验证空间服务创建成功，配置信息一致
        assert_eq!(service.config().app_id(), space_service.config().app_id());
        assert_eq!(service.config().app_secret(), space_service.config().app_secret());
    }

    #[test]
    fn test_space_member_service_access() {
        let service = create_test_service();
        let space_member_service = service.space_member();

        // 验证空间成员服务创建成功，配置信息一致
        assert_eq!(service.config().app_id(), space_member_service.config().app_id());
        assert_eq!(service.config().app_secret(), space_member_service.config().app_secret());
    }

    #[test]
    fn test_space_setting_service_access() {
        let service = create_test_service();
        let space_setting_service = service.space_setting();

        // 验证空间设置服务创建成功，配置信息一致
        assert_eq!(service.config().app_id(), space_setting_service.config().app_id());
        assert_eq!(service.config().app_secret(), space_setting_service.config().app_secret());
    }

    #[test]
    fn test_space_node_service_access() {
        let service = create_test_service();
        let space_node_service = service.space_node();

        // 验证空间节点服务创建成功，配置信息一致
        assert_eq!(service.config().app_id(), space_node_service.config().app_id());
        assert_eq!(service.config().app_secret(), space_node_service.config().app_secret());
    }

    #[test]
    fn test_task_service_access() {
        let service = create_test_service();
        let task_service = service.task();

        // 验证任务服务创建成功，配置信息一致
        assert_eq!(service.config().app_id(), task_service.config().app_id());
        assert_eq!(service.config().app_secret(), task_service.config().app_secret());
    }

    #[test]
    fn test_multiple_wiki_services() {
        let config1 = openlark_core::config::Config::new("app_id_1", "app_secret_1");
        let config2 = openlark_core::config::Config::new("app_id_2", "app_secret_2");

        let service1 = WikiService::new(config1);
        let service2 = WikiService::new(config2);

        assert_eq!(service1.config().app_id(), "app_id_1");
        assert_eq!(service2.config().app_id(), "app_id_2");
        // 验证两个服务实例是独立的
        assert_ne!(service1.config().app_id(), service2.config().app_id());
    }

    #[test]
    fn test_module_structure() {
        // 这个测试验证模块结构的完整性
        let service = create_test_service();

        // 验证可以访问所有服务
        let _space_service = service.space();
        let _space_member_service = service.space_member();
        let _space_setting_service = service.space_setting();
        let _space_node_service = service.space_node();
        let _task_service = service.task();

        // 如果编译通过，说明模块结构正确
        assert!(true);
}
}
