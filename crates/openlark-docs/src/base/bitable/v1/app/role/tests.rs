//! 自定义角色服务测试模块

#[cfg(test)]
mod tests {
    use crate::base::bitable::v1::app::role::AppRoleService;
    use openlark_core::config::Config;

    #[test]
    fn test_role_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = AppRoleService::new(config.clone());

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[test]
    fn test_create_role_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = AppRoleService::new(config);
        let _builder = service.create();
    }

    #[test]
    fn test_update_role_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = AppRoleService::new(config);
        let _builder = service.update();
    }

    #[test]
    fn test_delete_role_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = AppRoleService::new(config);
        let _builder = service.delete();
    }

    #[test]
    fn test_list_role_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = AppRoleService::new(config);
        let _builder = service.list();
    }

    #[test]
    fn test_role_service_all_methods() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = AppRoleService::new(config);

        let _create_builder = service.create();
        let _update_builder = service.update();
        let _delete_builder = service.delete();
        let _list_builder = service.list();
    }
}
