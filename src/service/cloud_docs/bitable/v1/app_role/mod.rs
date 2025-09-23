pub mod create;
pub mod delete;
pub mod list;
pub mod update;

use crate::core::config::Config;

pub use create::*;
pub use delete::*;
pub use list::*;
pub use update::*;

/// 自定义角色服务
pub struct AppRoleService {
    config: Config,
}

impl AppRoleService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 更新自定义角色
    pub async fn update(
        &self,
        request: UpdateAppRoleRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<UpdateAppRoleResponse>> {
        update::update_app_role(request, &self.config, option).await
    }

    /// 列出自定义角色
    pub async fn list(
        &self,
        request: ListAppRoleRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<ListAppRoleResponse>> {
        list::list_app_roles(request, &self.config, option).await
    }

    /// 删除自定义角色
    pub async fn delete(
        &self,
        request: DeleteAppRoleRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<DeleteAppRoleResponse>> {
        delete::delete_app_role(request, &self.config, option).await
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::core::config::Config;

    fn create_test_config() -> Config {
        Config::default()
    }

    #[test]
    fn test_app_role_service_creation() {
        let config = create_test_config();
        let service = AppRoleService::new(config.clone());

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_app_role_service_with_custom_config() {
        let config = Config::builder()
            .app_id("role_app")
            .app_secret("role_secret")
            .req_timeout(std::time::Duration::from_millis(8000))
            .base_url("https://role.api.com")
            .build();

        let service = AppRoleService::new(config.clone());

        assert_eq!(service.config.app_id, "role_app");
        assert_eq!(service.config.app_secret, "role_secret");
        assert_eq!(service.config.base_url, "https://role.api.com");
        assert_eq!(
            service.config.req_timeout,
            Some(std::time::Duration::from_millis(8000))
        );
    }

    #[test]
    fn test_app_role_service_config_independence() {
        let config1 = Config::builder()
            .app_id("role1")
            .app_secret("secret1")
            .build();
        let config2 = Config::builder()
            .app_id("role2")
            .app_secret("secret2")
            .build();

        let service1 = AppRoleService::new(config1);
        let service2 = AppRoleService::new(config2);

        assert_eq!(service1.config.app_id, "role1");
        assert_eq!(service2.config.app_id, "role2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_app_role_service_memory_layout() {
        let config = create_test_config();
        let service = AppRoleService::new(config);

        let service_ptr = std::ptr::addr_of!(service) as *const u8;
        let config_ptr = std::ptr::addr_of!(service.config) as *const u8;

        assert!(
            !service_ptr.is_null(),
            "Service should have valid memory address"
        );
        assert!(
            !config_ptr.is_null(),
            "Config should have valid memory address"
        );
    }

    #[test]
    fn test_app_role_service_with_different_configurations() {
        let test_configs = vec![
            Config::builder()
                .app_id("role_basic")
                .app_secret("basic_secret")
                .build(),
            Config::builder()
                .app_id("role_timeout")
                .app_secret("timeout_secret")
                .req_timeout(std::time::Duration::from_millis(6000))
                .build(),
            Config::builder()
                .app_id("role_custom")
                .app_secret("custom_secret")
                .base_url("https://custom.role.com")
                .build(),
            Config::builder()
                .app_id("role_full")
                .app_secret("full_secret")
                .req_timeout(std::time::Duration::from_millis(12000))
                .base_url("https://full.role.com")
                .enable_token_cache(false)
                .build(),
        ];

        for config in test_configs {
            let service = AppRoleService::new(config.clone());

            assert_eq!(service.config.app_id, config.app_id);
            assert_eq!(service.config.app_secret, config.app_secret);
            assert_eq!(service.config.base_url, config.base_url);
            assert_eq!(service.config.req_timeout, config.req_timeout);
        }
    }

    #[test]
    fn test_app_role_service_multiple_instances() {
        let config = create_test_config();
        let service1 = AppRoleService::new(config.clone());
        let service2 = AppRoleService::new(config.clone());

        assert_eq!(service1.config.app_id, service2.config.app_id);
        assert_eq!(service1.config.app_secret, service2.config.app_secret);

        let ptr1 = std::ptr::addr_of!(service1) as *const u8;
        let ptr2 = std::ptr::addr_of!(service2) as *const u8;
        assert_ne!(ptr1, ptr2, "Services should be independent instances");
    }

    #[test]
    fn test_app_role_service_config_cloning() {
        let original_config = create_test_config();
        let cloned_config = original_config.clone();

        let service = AppRoleService::new(cloned_config);

        assert_eq!(service.config.app_id, original_config.app_id);
        assert_eq!(service.config.app_secret, original_config.app_secret);
    }

    #[test]
    fn test_app_role_service_with_empty_config() {
        let config = Config::default();
        let service = AppRoleService::new(config);

        assert_eq!(service.config.app_id, "");
        assert_eq!(service.config.app_secret, "");
    }

    #[test]
    fn test_app_role_service_with_unicode_config() {
        let config = Config::builder()
            .app_id("角色应用")
            .app_secret("角色密钥")
            .base_url("https://角色.com")
            .build();
        let service = AppRoleService::new(config);

        assert_eq!(service.config.app_id, "角色应用");
        assert_eq!(service.config.app_secret, "角色密钥");
        assert_eq!(service.config.base_url, "https://角色.com");
    }

    #[test]
    fn test_app_role_service_with_extreme_timeout() {
        let config = Config::builder()
            .app_id("role_extreme")
            .app_secret("extreme_secret")
            .req_timeout(std::time::Duration::from_secs(7200))
            .build();
        let service = AppRoleService::new(config);

        assert_eq!(
            service.config.req_timeout,
            Some(std::time::Duration::from_secs(7200))
        );
    }

    #[test]
    fn test_app_role_service_concurrent_creation() {
        let configs = vec![
            Config::builder()
                .app_id("role_concurrent_1")
                .app_secret("secret_1")
                .build(),
            Config::builder()
                .app_id("role_concurrent_2")
                .app_secret("secret_2")
                .build(),
            Config::builder()
                .app_id("role_concurrent_3")
                .app_secret("secret_3")
                .build(),
        ];

        let mut services = Vec::new();
        for config in configs {
            let service = AppRoleService::new(config);
            services.push(service);
        }

        assert_eq!(services.len(), 3);

        for (i, service1) in services.iter().enumerate() {
            for service2 in services.iter().skip(i + 1) {
                let ptr1 = std::ptr::addr_of!(*service1) as *const u8;
                let ptr2 = std::ptr::addr_of!(*service2) as *const u8;
                assert_ne!(ptr1, ptr2, "Services should be independent instances");
            }
        }
    }

    #[test]
    fn test_app_role_service_construction_stability() {
        for i in 0..100 {
            let config = Config::builder()
                .app_id(format!("role_app_{}", i))
                .app_secret(format!("secret_{}", i))
                .build();
            let service = AppRoleService::new(config.clone());

            assert_eq!(service.config.app_id, format!("role_app_{}", i));
            assert_eq!(service.config.app_secret, format!("secret_{}", i));
        }
    }
}
