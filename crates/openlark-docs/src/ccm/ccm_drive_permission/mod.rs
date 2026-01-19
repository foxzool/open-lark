#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

/// CCM Drive Permission v1 API 模块
///
/// 提供云盘权限管理相关的API功能，包括权限查询、拥有者转移等。
use openlark_core::config::Config;

// 重新导出所有子模块
pub use permission::*;

// 子模块
pub mod old;
pub mod permission;

/// CCM Drive Permission 服务
///
/// 提供云盘权限的完整管理功能，包括权限查询、拥有者转移等。
/// 支持多种权限类型的统一管理。
#[derive(Clone)]
pub struct CcmDrivePermissionService {
    config: Config,
}

impl CcmDrivePermissionService {
    /// 创建新的CCM Drive Permission服务实例
    ///
    /// # 参数
    /// * `config` - SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取权限API服务
    ///
    /// 访问权限相关的API，包括权限查询、拥有者转移等。
    ///
    /// # 返回
    /// 返回权限API服务实例
    pub fn permission(&self) -> crate::ccm::ccm_drive_permission::permission::PermissionService {
        crate::ccm::ccm_drive_permission::permission::PermissionService::new(self.config.clone())
    }
}

impl openlark_core::trait_system::service::Service for CcmDrivePermissionService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "ccmdrivepermission"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::service::Service;

    fn create_test_service() -> CcmDrivePermissionService {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        CcmDrivePermissionService::new(config)
    }

    #[test]
    fn test_ccm_drive_permission_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = CcmDrivePermissionService::new(config);

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[test]
    fn test_ccm_drive_permission_service_clone() {
        let service = create_test_service();
        let cloned_service = service.clone();

        assert_eq!(service.config().app_id(), cloned_service.config().app_id());
        assert_eq!(
            service.config().app_secret(),
            cloned_service.config().app_secret()
        );
    }

    #[test]
    fn test_service_trait_implementation() {
        let service = create_test_service();

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id(), "test_app_id");
    }

    #[test]
    fn test_permission_service_access() {
        let service = create_test_service();
        let permission_service = service.permission();

        // 验证权限服务创建成功，配置信息一致
        assert_eq!(
            service.config().app_id(),
            permission_service.config().app_id()
        );
        assert_eq!(
            service.config().app_secret(),
            permission_service.config().app_secret()
        );
    }

    #[test]
    fn test_module_structure() {
        // 这个测试验证模块结构的完整性
        let service = create_test_service();

        // 验证可以访问所有服务
        let _permission_service = service.permission();
    }
}
