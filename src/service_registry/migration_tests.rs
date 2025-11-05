//! 服务迁移测试模块
//!
//! 验证现有服务到 ServiceRegistry 的迁移功能

#[cfg(test)]
mod tests {
    use crate::core::config::{Config, ConfigBuilder};
    use crate::service_registry::{
        MigrationHelper, Service, ServiceError, ServiceRegistry, ServiceStatus,
    };

    #[cfg(feature = "authentication")]
    use crate::service_registry::AuthenticationServiceAdapter;
    #[cfg(feature = "contact")]
    use crate::service_registry::ContactServiceAdapter;
    #[cfg(feature = "group")]
    use crate::service_registry::GroupServiceAdapter;
    #[cfg(feature = "im")]
    use crate::service_registry::ImServiceAdapter;
    #[cfg(feature = "search")]
    use crate::service_registry::SearchServiceAdapter;

    fn create_test_config() -> Config {
        ConfigBuilder::default()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    #[test]
    #[cfg(feature = "authentication")]
    fn test_authentication_service_migration() {
        let registry = ServiceRegistry::new();
        let config = create_test_config();

        // 注册认证服务
        let auth_service = crate::service::authentication::AuthenticationService::new(config);
        let auth_adapter = AuthenticationServiceAdapter::new(auth_service);

        assert!(registry.register(auth_adapter).is_ok());
        assert!(registry.has_service("authentication-service"));

        // 通过 ServiceRegistry 获取服务
        let retrieved: std::sync::Arc<AuthenticationServiceAdapter> = registry.get().unwrap();
        assert_eq!(retrieved.name(), "authentication-service");
        assert_eq!(retrieved.version(), "1.0.0");
        assert!(retrieved.is_available());
    }

    #[test]
    #[cfg(feature = "im")]
    fn test_im_service_migration() {
        let registry = ServiceRegistry::new();
        let config = create_test_config();

        // 注册IM服务
        let im_service = crate::service::im::ImService::new(config);
        let im_adapter = ImServiceAdapter::new(im_service);

        assert!(registry.register(im_adapter).is_ok());
        assert!(registry.has_service("im-service"));

        // 通过 ServiceRegistry 获取服务
        let retrieved: std::sync::Arc<ImServiceAdapter> = registry.get().unwrap();
        assert_eq!(retrieved.name(), "im-service");
        assert_eq!(retrieved.version(), "1.0.0");
        assert!(retrieved.is_available());
    }

    #[test]
    #[cfg(feature = "contact")]
    fn test_contact_service_migration() {
        let registry = ServiceRegistry::new();
        let config = create_test_config();

        // 注册联系人服务
        let contact_service = crate::service::contact::ContactService::new(config);
        let contact_adapter = ContactServiceAdapter::new(contact_service);

        assert!(registry.register(contact_adapter).is_ok());
        assert!(registry.has_service("contact-service"));

        // 通过 ServiceRegistry 获取服务
        let retrieved: std::sync::Arc<ContactServiceAdapter> = registry.get().unwrap();
        assert_eq!(retrieved.name(), "contact-service");
        assert_eq!(retrieved.version(), "1.0.0");
        assert!(retrieved.is_available());
    }

    #[test]
    #[cfg(feature = "search")]
    fn test_search_service_migration() {
        let registry = ServiceRegistry::new();
        let config = create_test_config();

        // 注册搜索服务
        let search_service = crate::service::search::SearchService::new(config);
        let search_adapter = SearchServiceAdapter::new(search_service);

        assert!(registry.register(search_adapter).is_ok());
        assert!(registry.has_service("search-service"));

        // 通过 ServiceRegistry 获取服务
        let retrieved: std::sync::Arc<SearchServiceAdapter> = registry.get().unwrap();
        assert_eq!(retrieved.name(), "search-service");
        assert_eq!(retrieved.version(), "1.0.0");
        assert!(retrieved.is_available());
    }

    #[test]
    #[cfg(feature = "group")]
    fn test_group_service_migration() {
        let registry = ServiceRegistry::new();
        let config = create_test_config();

        // 注册group服务
        let group_service = crate::service::group::GroupService::new(config);
        let group_adapter = GroupServiceAdapter::new(group_service);

        assert!(registry.register(group_adapter).is_ok());
        assert!(registry.has_service("group-service"));

        // 通过 ServiceRegistry 获取服务
        let retrieved: std::sync::Arc<GroupServiceAdapter> = registry.get().unwrap();
        assert_eq!(retrieved.name(), "group-service");
        assert_eq!(retrieved.version(), "1.0.0");
        assert!(retrieved.is_available());
    }

    #[test]
    #[cfg(all(feature = "authentication", feature = "im", feature = "contact"))]
    fn test_multiple_services_migration() {
        let registry = ServiceRegistry::new();
        let config = create_test_config();

        // 批量注册所有核心服务
        let result = MigrationHelper::register_services(&registry, &config);
        assert!(result.is_ok());

        // 验证所有服务都已注册
        assert!(registry.has_service("authentication-service"));
        assert!(registry.has_service("im-service"));
        assert!(registry.has_service("contact-service"));

        // 动态计算期望的服务数量
        let mut expected_count = 3; // authentication, im, contact
        if cfg!(feature = "group") {
            expected_count += 1;
            assert!(registry.has_service("group-service"));
        }
        if cfg!(feature = "search") {
            expected_count += 1;
            assert!(registry.has_service("search-service"));
        }

        // 验证服务数量
        assert_eq!(registry.service_count(), expected_count);

        // 验证健康服务数量
        assert_eq!(registry.healthy_service_count(), expected_count);

        // 获取服务信息
        let auth_info = registry.get_service_info("authentication-service").unwrap();
        assert_eq!(auth_info.name, "authentication-service");
        assert_eq!(auth_info.version, "1.0.0");
        assert_eq!(auth_info.status, ServiceStatus::Healthy);

        let im_info = registry.get_service_info("im-service").unwrap();
        assert_eq!(im_info.name, "im-service");
        assert_eq!(im_info.version, "1.0.0");
        assert_eq!(im_info.status, ServiceStatus::Healthy);

        let contact_info = registry.get_service_info("contact-service").unwrap();
        assert_eq!(contact_info.name, "contact-service");
        assert_eq!(contact_info.version, "1.0.0");
        assert_eq!(contact_info.status, ServiceStatus::Healthy);

        // 如果启用了group功能，验证group服务信息
        #[cfg(feature = "group")]
        {
            let group_info = registry.get_service_info("group-service").unwrap();
            assert_eq!(group_info.name, "group-service");
            assert_eq!(group_info.version, "1.0.0");
            assert_eq!(group_info.status, ServiceStatus::Healthy);
        }

        // 如果启用了search功能，验证search服务信息
        #[cfg(feature = "search")]
        {
            let search_info = registry.get_service_info("search-service").unwrap();
            assert_eq!(search_info.name, "search-service");
            assert_eq!(search_info.version, "1.0.0");
            assert_eq!(search_info.status, ServiceStatus::Healthy);
        }
    }

    #[test]
    #[cfg(all(
        feature = "authentication",
        feature = "im",
        feature = "contact",
        feature = "search"
    ))]
    fn test_four_services_migration() {
        let registry = ServiceRegistry::new();
        let config = create_test_config();

        // 批量注册所有核心服务
        let result = MigrationHelper::register_services(&registry, &config);
        assert!(result.is_ok());

        // 验证所有服务都已注册
        assert!(registry.has_service("authentication-service"));
        assert!(registry.has_service("im-service"));
        assert!(registry.has_service("contact-service"));
        assert!(registry.has_service("search-service"));

        // 动态计算期望的服务数量
        let mut expected_count = 4; // authentication, im, contact, search
        if cfg!(feature = "group") {
            expected_count += 1;
            assert!(registry.has_service("group-service"));
        }

        // 验证服务数量
        assert_eq!(registry.service_count(), expected_count);

        // 验证健康服务数量
        assert_eq!(registry.healthy_service_count(), expected_count);

        // 获取服务信息
        let auth_info = registry.get_service_info("authentication-service").unwrap();
        assert_eq!(auth_info.name, "authentication-service");
        assert_eq!(auth_info.version, "1.0.0");
        assert_eq!(auth_info.status, ServiceStatus::Healthy);

        let im_info = registry.get_service_info("im-service").unwrap();
        assert_eq!(im_info.name, "im-service");
        assert_eq!(im_info.version, "1.0.0");
        assert_eq!(im_info.status, ServiceStatus::Healthy);

        let contact_info = registry.get_service_info("contact-service").unwrap();
        assert_eq!(contact_info.name, "contact-service");
        assert_eq!(contact_info.version, "1.0.0");
        assert_eq!(contact_info.status, ServiceStatus::Healthy);

        let search_info = registry.get_service_info("search-service").unwrap();
        assert_eq!(search_info.name, "search-service");
        assert_eq!(search_info.version, "1.0.0");
        assert_eq!(search_info.status, ServiceStatus::Healthy);

        // 如果启用了group功能，验证group服务信息
        #[cfg(feature = "group")]
        {
            let group_info = registry.get_service_info("group-service").unwrap();
            assert_eq!(group_info.name, "group-service");
            assert_eq!(group_info.version, "1.0.0");
            assert_eq!(group_info.status, ServiceStatus::Healthy);
        }
    }

    #[test]
    #[cfg(all(feature = "authentication", feature = "im", feature = "contact"))]
    fn test_service_discovery() {
        let registry = ServiceRegistry::new();
        let config = create_test_config();

        // 注册服务
        MigrationHelper::register_services(&registry, &config).unwrap();

        // 发现所有服务
        let services = registry.discover_services();
        let mut expected_count = 3; // authentication, im, contact
        if cfg!(feature = "group") {
            expected_count += 1;
        }
        if cfg!(feature = "search") {
            expected_count += 1;
        }

        assert_eq!(services.len(), expected_count);
        assert!(services.contains(&"authentication-service"));
        assert!(services.contains(&"im-service"));
        assert!(services.contains(&"contact-service"));
        if cfg!(feature = "group") {
            assert!(services.contains(&"group-service"));
        }
        if cfg!(feature = "search") {
            assert!(services.contains(&"search-service"));
        }

        // 获取所有服务信息
        let all_services_info = registry.get_all_services_info();
        assert_eq!(all_services_info.len(), expected_count);

        // 验证服务统计
        let stats = registry.get_stats();
        assert_eq!(stats.total_services, expected_count);
        assert_eq!(stats.healthy_services, expected_count);
        assert_eq!(stats.unhealthy_services, 0);
    }

    #[test]
    #[cfg(feature = "im")]
    fn test_duplicate_service_registration() {
        let registry = ServiceRegistry::new();
        let config = create_test_config();

        // 第一次注册应该成功
        let im_service1 = crate::service::im::ImService::new(config.clone());
        let im_adapter1 = ImServiceAdapter::new(im_service1);
        assert!(registry.register(im_adapter1).is_ok());

        // 第二次注册相同服务应该失败
        let im_service2 = crate::service::im::ImService::new(config);
        let im_adapter2 = ImServiceAdapter::new(im_service2);
        let result = registry.register(im_adapter2);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ServiceError::ServiceAlreadyExists { .. })
        ));
    }

    #[test]
    #[cfg(feature = "im")]
    fn test_service_unregistration() {
        let registry = ServiceRegistry::new();
        let config = create_test_config();

        // 注册服务
        let im_service = crate::service::im::ImService::new(config);
        let im_adapter = ImServiceAdapter::new(im_service);
        registry.register(im_adapter).unwrap();
        assert!(registry.has_service("im-service"));

        // 注销服务
        assert!(registry.unregister("im-service").is_ok());
        assert!(!registry.has_service("im-service"));

        // 再次注销应该失败
        let result = registry.unregister("im-service");
        assert!(result.is_err());
        assert!(matches!(result, Err(ServiceError::ServiceNotFound { .. })));
    }

    #[test]
    #[cfg(all(feature = "authentication", feature = "im", feature = "contact"))]
    fn test_migration_validation() {
        let registry = ServiceRegistry::new();
        let config = create_test_config();

        // 批量注册服务
        MigrationHelper::register_services(&registry, &config).unwrap();

        // 验证迁移完整性
        let result = MigrationHelper::validate_migration(&registry);
        assert!(result.is_ok());
    }

    #[test]
    fn test_service_adapter_inner_access() {
        #[cfg(feature = "im")]
        {
            let config = create_test_config();
            let im_service = crate::service::im::ImService::new(config);
            let adapter = ImServiceAdapter::new(im_service);

            // 测试访问内部服务
            let inner_ref = adapter.inner();
            assert_eq!(inner_ref.service.config.app_id, "test_app_id");
        }

        #[cfg(feature = "contact")]
        {
            let config = create_test_config();
            let contact_service = crate::service::contact::ContactService::new(config);
            let adapter = ContactServiceAdapter::new(contact_service);

            // 测试访问内部服务
            let inner_ref = adapter.inner();
            assert_eq!(inner_ref.config.app_id, "test_app_id");
        }

        #[cfg(feature = "authentication")]
        {
            let config = create_test_config();
            let auth_service = crate::service::authentication::AuthenticationService::new(config);
            let adapter = AuthenticationServiceAdapter::new(auth_service);

            // 测试访问内部服务
            let inner_ref = adapter.inner();
            // authentication 服务可能没有直接的 config 字段，所以这里只验证引用有效
            let _ = inner_ref;
        }
    }

    #[tokio::test]
    #[cfg(all(feature = "authentication", feature = "im", feature = "contact"))]
    async fn test_concurrent_service_access() {
        use std::sync::Arc;
        use tokio::task::JoinSet;

        let registry = Arc::new(ServiceRegistry::new());
        let config = create_test_config();

        // 注册服务
        MigrationHelper::register_services(&registry.as_ref(), &config).unwrap();

        // 创建多个并发任务
        let mut set = JoinSet::new();

        for i in 0..10 {
            let registry_clone = registry.clone();
            set.spawn(async move {
                // 并发获取不同的服务
                match i % 3 {
                    0 => {
                        let _: std::sync::Arc<AuthenticationServiceAdapter> =
                            registry_clone.get().unwrap();
                        format!("auth-{}", i)
                    }
                    1 => {
                        let _: std::sync::Arc<ImServiceAdapter> = registry_clone.get().unwrap();
                        format!("im-{}", i)
                    }
                    _ => {
                        let _: std::sync::Arc<ContactServiceAdapter> =
                            registry_clone.get().unwrap();
                        format!("contact-{}", i)
                    }
                }
            });
        }

        // 等待所有任务完成
        let mut results = Vec::new();
        while let Some(result) = set.join_next().await {
            results.push(result.unwrap());
        }

        // 验证所有任务都成功完成
        assert_eq!(results.len(), 10);

        // 验证结果包含所有三种服务类型
        let auth_count = results.iter().filter(|r| r.starts_with("auth-")).count();
        let im_count = results.iter().filter(|r| r.starts_with("im-")).count();
        let contact_count = results.iter().filter(|r| r.starts_with("contact-")).count();

        assert!(auth_count > 0);
        assert!(im_count > 0);
        assert!(contact_count > 0);
        assert_eq!(auth_count + im_count + contact_count, 10);
    }

    #[test]
    fn test_service_registry_stats() {
        let registry = ServiceRegistry::new();
        let config = create_test_config();

        // 初始状态
        let stats = registry.get_stats();
        assert_eq!(stats.total_services, 0);
        assert_eq!(stats.healthy_services, 0);
        assert_eq!(stats.unhealthy_services, 0);

        // 注册一些服务
        #[cfg(feature = "im")]
        {
            let im_service = crate::service::im::ImService::new(config.clone());
            let im_adapter = ImServiceAdapter::new(im_service);
            registry.register(im_adapter).unwrap();
        }

        #[cfg(feature = "contact")]
        {
            let contact_service = crate::service::contact::ContactService::new(config.clone());
            let contact_adapter = ContactServiceAdapter::new(contact_service);
            registry.register(contact_adapter).unwrap();
        }

        // 检查更新后的统计
        let stats = registry.get_stats();
        let expected_count = if cfg!(feature = "im") && cfg!(feature = "contact") {
            2
        } else if cfg!(feature = "im") || cfg!(feature = "contact") {
            1
        } else {
            0
        };

        assert_eq!(stats.total_services, expected_count);
        assert_eq!(stats.healthy_services, expected_count);
        assert_eq!(stats.unhealthy_services, 0);
    }
}
