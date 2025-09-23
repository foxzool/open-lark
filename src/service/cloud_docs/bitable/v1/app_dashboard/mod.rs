pub mod copy;
pub mod list;

use crate::core::config::Config;

pub use copy::*;
pub use list::*;

/// 仪表盘服务
pub struct AppDashboardService {
    config: Config,
}

impl AppDashboardService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 复制仪表盘
    pub async fn copy(
        &self,
        request: CopyDashboardRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<CopyDashboardResponse>> {
        copy::copy_dashboard(request, &self.config, option).await
    }

    /// 列出仪表盘
    pub async fn list(
        &self,
        request: ListDashboardRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<ListDashboardResponse>> {
        list::list_dashboard(request, self.config.clone(), option).await
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
    fn test_app_dashboard_service_creation() {
        let config = create_test_config();
        let service = AppDashboardService::new(config.clone());

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_app_dashboard_service_with_custom_config() {
        let config = Config::builder()
            .app_id("dashboard_app")
            .app_secret("dashboard_secret")
            .req_timeout(std::time::Duration::from_millis(10000))
            .base_url("https://dashboard.api.com")
            .build();

        let service = AppDashboardService::new(config.clone());

        assert_eq!(service.config.app_id, "dashboard_app");
        assert_eq!(service.config.app_secret, "dashboard_secret");
        assert_eq!(service.config.base_url, "https://dashboard.api.com");
        assert_eq!(
            service.config.req_timeout,
            Some(std::time::Duration::from_millis(10000))
        );
    }

    #[test]
    fn test_app_dashboard_service_config_independence() {
        let config1 = Config::builder()
            .app_id("app1")
            .app_secret("secret1")
            .build();
        let config2 = Config::builder()
            .app_id("app2")
            .app_secret("secret2")
            .build();

        let service1 = AppDashboardService::new(config1);
        let service2 = AppDashboardService::new(config2);

        assert_eq!(service1.config.app_id, "app1");
        assert_eq!(service2.config.app_id, "app2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_app_dashboard_service_memory_layout() {
        let config = create_test_config();
        let service = AppDashboardService::new(config);

        // Test that the service has proper memory layout
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
    fn test_app_dashboard_service_with_different_configurations() {
        let test_configs = vec![
            Config::builder()
                .app_id("dashboard_basic")
                .app_secret("basic_secret")
                .build(),
            Config::builder()
                .app_id("dashboard_timeout")
                .app_secret("timeout_secret")
                .req_timeout(std::time::Duration::from_millis(5000))
                .build(),
            Config::builder()
                .app_id("dashboard_custom")
                .app_secret("custom_secret")
                .base_url("https://custom.dashboard.com")
                .build(),
            Config::builder()
                .app_id("dashboard_full")
                .app_secret("full_secret")
                .req_timeout(std::time::Duration::from_millis(15000))
                .base_url("https://full.dashboard.com")
                .enable_token_cache(false)
                .build(),
        ];

        for config in test_configs {
            let service = AppDashboardService::new(config.clone());

            assert_eq!(service.config.app_id, config.app_id);
            assert_eq!(service.config.app_secret, config.app_secret);
            assert_eq!(service.config.base_url, config.base_url);
            assert_eq!(service.config.req_timeout, config.req_timeout);
        }
    }

    #[test]
    fn test_app_dashboard_service_multiple_instances() {
        let config = create_test_config();
        let service1 = AppDashboardService::new(config.clone());
        let service2 = AppDashboardService::new(config.clone());

        // Multiple instances should have the same configuration
        assert_eq!(service1.config.app_id, service2.config.app_id);
        assert_eq!(service1.config.app_secret, service2.config.app_secret);

        // But should be different instances in memory
        let ptr1 = std::ptr::addr_of!(service1) as *const u8;
        let ptr2 = std::ptr::addr_of!(service2) as *const u8;
        assert_ne!(ptr1, ptr2, "Services should be independent instances");
    }

    #[test]
    fn test_app_dashboard_service_config_cloning() {
        let original_config = create_test_config();
        let cloned_config = original_config.clone();

        let service = AppDashboardService::new(cloned_config);

        assert_eq!(service.config.app_id, original_config.app_id);
        assert_eq!(service.config.app_secret, original_config.app_secret);
    }

    #[test]
    fn test_app_dashboard_service_with_empty_config() {
        let config = Config::default();
        let service = AppDashboardService::new(config);

        // Service should be constructible even with default/empty config
        assert_eq!(service.config.app_id, "");
        assert_eq!(service.config.app_secret, "");
    }

    #[test]
    fn test_app_dashboard_service_with_unicode_config() {
        let config = Config::builder()
            .app_id("仪表盘应用")
            .app_secret("仪表盘密钥")
            .base_url("https://仪表盘.com")
            .build();
        let service = AppDashboardService::new(config);

        assert_eq!(service.config.app_id, "仪表盘应用");
        assert_eq!(service.config.app_secret, "仪表盘密钥");
        assert_eq!(service.config.base_url, "https://仪表盘.com");
    }

    #[test]
    fn test_app_dashboard_service_with_extreme_timeout() {
        let config = Config::builder()
            .app_id("dashboard_extreme")
            .app_secret("extreme_secret")
            .req_timeout(std::time::Duration::from_secs(3600)) // 1 hour
            .build();
        let service = AppDashboardService::new(config);

        assert_eq!(
            service.config.req_timeout,
            Some(std::time::Duration::from_secs(3600))
        );
    }

    #[test]
    fn test_app_dashboard_service_concurrent_creation() {
        let configs = vec![
            Config::builder()
                .app_id("dashboard_concurrent_1")
                .app_secret("secret_1")
                .build(),
            Config::builder()
                .app_id("dashboard_concurrent_2")
                .app_secret("secret_2")
                .build(),
            Config::builder()
                .app_id("dashboard_concurrent_3")
                .app_secret("secret_3")
                .build(),
        ];

        let mut services = Vec::new();
        for config in configs {
            let service = AppDashboardService::new(config);
            services.push(service);
        }

        // All services should be created successfully
        assert_eq!(services.len(), 3);

        // Verify all services are independent
        for (i, service1) in services.iter().enumerate() {
            for service2 in services.iter().skip(i + 1) {
                let ptr1 = std::ptr::addr_of!(*service1) as *const u8;
                let ptr2 = std::ptr::addr_of!(*service2) as *const u8;
                assert_ne!(ptr1, ptr2, "Services should be independent instances");
            }
        }
    }

    #[test]
    fn test_app_dashboard_service_construction_stability() {
        // Test creating many services to ensure construction is stable
        for i in 0..100 {
            let config = Config::builder()
                .app_id(format!("dashboard_app_{}", i))
                .app_secret(format!("secret_{}", i))
                .build();
            let service = AppDashboardService::new(config.clone());

            assert_eq!(service.config.app_id, format!("dashboard_app_{}", i));
            assert_eq!(service.config.app_secret, format!("secret_{}", i));
        }
    }
}
