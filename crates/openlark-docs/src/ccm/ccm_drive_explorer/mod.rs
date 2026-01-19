/// 云盘浏览器模块
///
/// 提供云盘浏览器相关的API功能。
use openlark_core::config::Config;

// 重新导出所有子模块
pub use explorer::*;

// 子模块
pub mod explorer;
pub mod old;

/// 云盘浏览器服务
#[derive(Clone)]
pub struct CcmDriveExplorerService {
    config: Config,
}

impl CcmDriveExplorerService {
    /// 创建新的云盘浏览器服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取浏览器服务
    pub fn explorer(&self) -> explorer::ExplorerService {
        explorer::ExplorerService::new(self.config.clone())
    }
}

impl openlark_core::trait_system::service::Service for CcmDriveExplorerService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "ccm_drive_explorer"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::service::Service;

    fn create_test_service() -> CcmDriveExplorerService {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        CcmDriveExplorerService::new(config)
    }

    #[test]
    fn test_ccm_drive_explorer_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = CcmDriveExplorerService::new(config);

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[test]
    fn test_ccm_drive_explorer_service_clone() {
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
    fn test_explorer_service_access() {
        let service = create_test_service();
        let explorer_service = service.explorer();

        // 验证浏览器服务创建成功，配置信息一致
        assert_eq!(
            service.config().app_id(),
            explorer_service.config().app_id()
        );
        assert_eq!(
            service.config().app_secret(),
            explorer_service.config().app_secret()
        );
    }

    #[test]
    fn test_module_structure() {
        // 这个测试验证模块结构的完整性
        let service = create_test_service();

        // 验证可以访问所有服务
        let _explorer_service = service.explorer();
    }
}
