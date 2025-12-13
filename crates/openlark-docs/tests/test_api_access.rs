/// 测试API访问
//!
/// 这个文件用于验证base和bitable模块的API是否可以正确访问

#[cfg(test)]
mod tests {
    use openlark_core::config::Config;
    use openlark_docs::{BaseService, BitableService};

    #[test]
    fn test_base_api_access() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let base_service = BaseService::new(config);
        let v2_service = base_service.v2();

        // 测试角色服务访问
        let role_service = v2_service.role_service();
        assert!(!role_service.config().app_id.is_empty());
    }

    #[test]
    fn test_bitable_api_access() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let bitable_service = BitableService::new(config);
        let v1_service = bitable_service.v1();

        // 测试应用服务访问
        let app_service = v1_service.app();
        assert!(!app_service.config().app_id.is_empty());

        // 测试表格服务访问
        let table_service = app_service.table_service();
        assert!(!table_service.config().app_id.is_empty());
    }

    #[test]
    fn test_specific_api_existence() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let base_service = BaseService::new(config.clone());
        let bitable_service = BitableService::new(config);

        // 测试Base API: 新增自定义角色
        let _create_role = base_service
            .v2()
            .role_service()
            .create_role_builder("app_token", "test_role");

        // 测试Bitable API: 创建多维表格
        let _create_app = bitable_service.v1().app().create_builder("test_app");
    }
}
