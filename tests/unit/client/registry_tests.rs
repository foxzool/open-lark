//! openlark-client registry 模块单元测试
//!
//! 测试 ServiceRegistry、ServiceFactory、DependencyResolver 和 FeatureFlags

use openlark_client::registry::feature_flags::{FeatureFlagError, FlagMetadata, FlagValue};
use openlark_client::registry::*;

/// 创建测试用的服务元数据
fn create_test_metadata(name: &str, dependencies: Vec<&str>) -> ServiceMetadata {
    ServiceMetadata {
        name: name.to_string(),
        version: "1.0.0".to_string(),
        description: Some(format!("{} 服务", name)),
        dependencies: dependencies.into_iter().map(String::from).collect(),
        provides: vec![format!("{}-feature", name)],
        status: ServiceStatus::Uninitialized,
        priority: 1,
    }
}

#[cfg(test)]
mod service_registry_tests {
    use super::*;

    #[test]
    fn test_service_registration_success() {
        let mut registry = DefaultServiceRegistry::new();
        let metadata = create_test_metadata("test-service", vec![]);

        assert!(registry.register_service(metadata).is_ok());
        assert!(registry.has_service("test-service"));
    }

    #[test]
    fn test_service_registration_duplicate() {
        let mut registry = DefaultServiceRegistry::new();
        let metadata = create_test_metadata("test-service", vec![]);

        registry.register_service(metadata.clone()).unwrap();
        let result = registry.register_service(metadata);

        assert!(matches!(
            result,
            Err(RegistryError::ServiceAlreadyExists { name }) if name == "test-service"
        ));
    }

    #[test]
    fn test_service_unregistration_success() {
        let mut registry = DefaultServiceRegistry::new();
        let metadata = create_test_metadata("test-service", vec![]);

        registry.register_service(metadata).unwrap();
        assert!(registry.has_service("test-service"));

        let result = registry.unregister_service("test-service");
        assert!(result.is_ok());
        assert!(!registry.has_service("test-service"));
    }

    #[test]
    fn test_service_unregistration_not_found() {
        let mut registry = DefaultServiceRegistry::new();

        let result = registry.unregister_service("non-existent");
        assert!(matches!(
            result,
            Err(RegistryError::ServiceNotFound { name }) if name == "non-existent"
        ));
    }

    #[test]
    fn test_get_service_success() {
        let mut registry = DefaultServiceRegistry::new();
        let metadata = create_test_metadata("test-service", vec![]);

        registry.register_service(metadata).unwrap();

        let entry = registry.get_service("test-service");
        assert!(entry.is_ok());
        assert_eq!(entry.unwrap().metadata.name, "test-service");
    }

    #[test]
    fn test_get_service_not_found() {
        let registry = DefaultServiceRegistry::new();

        let result = registry.get_service("non-existent");
        assert!(matches!(
            result,
            Err(RegistryError::ServiceNotFound { name }) if name == "non-existent"
        ));
    }

    #[test]
    fn test_list_services() {
        let mut registry = DefaultServiceRegistry::new();

        registry
            .register_service(create_test_metadata("service-a", vec![]))
            .unwrap();
        registry
            .register_service(create_test_metadata("service-b", vec![]))
            .unwrap();
        registry
            .register_service(create_test_metadata("service-c", vec![]))
            .unwrap();

        let services = registry.list_services();
        assert_eq!(services.len(), 3);
    }

    #[test]
    fn test_update_service_status() {
        let mut registry = DefaultServiceRegistry::new();
        let metadata = create_test_metadata("test-service", vec![]);

        registry.register_service(metadata).unwrap();

        let result = registry.update_service_status("test-service", ServiceStatus::Ready);
        assert!(result.is_ok());

        let entry = registry.get_service("test-service").unwrap();
        assert_eq!(entry.metadata.status, ServiceStatus::Ready);
    }

    #[test]
    fn test_update_service_status_not_found() {
        let mut registry = DefaultServiceRegistry::new();

        let result = registry.update_service_status("non-existent", ServiceStatus::Ready);
        assert!(matches!(
            result,
            Err(RegistryError::ServiceNotFound { name }) if name == "non-existent"
        ));
    }

    #[test]
    fn test_get_dependency_graph() {
        let mut registry = DefaultServiceRegistry::new();

        registry
            .register_service(create_test_metadata("service-a", vec!["service-b"]))
            .unwrap();
        registry
            .register_service(create_test_metadata("service-b", vec![]))
            .unwrap();
        registry
            .register_service(create_test_metadata(
                "service-c",
                vec!["service-a", "service-b"],
            ))
            .unwrap();

        let graph = registry.get_dependency_graph();

        assert_eq!(graph.get("service-a").unwrap(), &vec!["service-b"]);
        assert!(graph.get("service-b").unwrap().is_empty());
        assert_eq!(graph.get("service-c").unwrap().len(), 2);
    }

    #[test]
    fn test_service_status_variants() {
        let statuses = vec![
            ServiceStatus::Uninitialized,
            ServiceStatus::Initializing,
            ServiceStatus::Ready,
            ServiceStatus::Running,
            ServiceStatus::Stopped,
            ServiceStatus::Error("test error".to_string()),
        ];

        // 测试所有状态可以正确创建
        for status in statuses {
            let mut registry = DefaultServiceRegistry::new();
            let mut metadata = create_test_metadata("test-service", vec![]);
            metadata.status = status.clone();

            registry.register_service(metadata).unwrap();
            let entry = registry.get_service("test-service").unwrap();
            assert_eq!(entry.metadata.status, status);
        }
    }
}

#[cfg(test)]
mod dependency_resolver_tests {
    use super::*;

    #[test]
    fn test_resolve_dependencies_linear() {
        let resolver = DependencyResolver::new();
        let mut graph = std::collections::HashMap::new();
        graph.insert("service-a".to_string(), vec!["service-b".to_string()]);
        graph.insert("service-b".to_string(), vec!["service-c".to_string()]);
        graph.insert("service-c".to_string(), vec![]);

        let result = resolver.resolve_dependencies(graph);
        assert!(result.is_ok());

        let order = result.unwrap();
        // service-c 应该在 service-b 之前，service-b 应该在 service-a 之前
        let pos_c = order.iter().position(|s| s == "service-c").unwrap();
        let pos_b = order.iter().position(|s| s == "service-b").unwrap();
        let pos_a = order.iter().position(|s| s == "service-a").unwrap();

        assert!(pos_c < pos_b);
        assert!(pos_b < pos_a);
    }

    #[test]
    fn test_resolve_dependencies_multiple_deps() {
        let resolver = DependencyResolver::new();
        let mut graph = std::collections::HashMap::new();
        graph.insert(
            "service-main".to_string(),
            vec!["service-db".to_string(), "service-cache".to_string()],
        );
        graph.insert("service-db".to_string(), vec![]);
        graph.insert("service-cache".to_string(), vec![]);

        let result = resolver.resolve_dependencies(graph);
        assert!(result.is_ok());

        let order = result.unwrap();
        let pos_main = order.iter().position(|s| s == "service-main").unwrap();
        let pos_db = order.iter().position(|s| s == "service-db").unwrap();
        let pos_cache = order.iter().position(|s| s == "service-cache").unwrap();

        // service-main 应该在所有依赖之后
        assert!(pos_main > pos_db);
        assert!(pos_main > pos_cache);
    }

    #[test]
    fn test_detect_circular_dependencies() {
        let resolver = DependencyResolver::new();
        let mut graph = std::collections::HashMap::new();
        graph.insert("service-a".to_string(), vec!["service-b".to_string()]);
        graph.insert("service-b".to_string(), vec!["service-c".to_string()]);
        graph.insert("service-c".to_string(), vec!["service-a".to_string()]);

        let result = resolver.resolve_dependencies(graph);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_empty_dependencies() {
        let resolver = DependencyResolver::new();
        let graph = std::collections::HashMap::new();

        let result = resolver.resolve_dependencies(graph);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_resolve_self_dependency() {
        let resolver = DependencyResolver::new();
        let mut graph = std::collections::HashMap::new();
        graph.insert("service-a".to_string(), vec!["service-a".to_string()]);

        let result = resolver.resolve_dependencies(graph);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod feature_flags_tests {
    use super::*;

    #[test]
    fn test_flag_value_conversions() {
        // Bool
        let bool_val = FlagValue::Bool(true);
        assert!(bool_val.as_bool());
        assert_eq!(bool_val.as_integer(), 1);
        assert_eq!(bool_val.as_string(), "true");

        // String
        let str_val = FlagValue::String("test".to_string());
        assert!(str_val.as_bool());
        assert_eq!(str_val.as_string(), "test");

        // Integer
        let int_val = FlagValue::Integer(42);
        assert!(int_val.as_bool());
        assert_eq!(int_val.as_integer(), 42);

        // Float
        let float_val = FlagValue::Float(3.14);
        assert!(float_val.as_bool());
        assert!((float_val.as_float() - 3.14).abs() < 0.001);

        // Percentage
        let pct_val = FlagValue::Percentage(0.75);
        assert!(pct_val.as_bool());
        assert_eq!(pct_val.as_integer(), 75);
    }

    #[test]
    fn test_flag_manager_set_and_get() {
        let manager = FeatureFlagManager::new();

        // 先注册标志
        let metadata_a = FlagMetadata {
            name: "feature-a".to_string(),
            description: None,
            default_value: FlagValue::Bool(false),
            current_value: FlagValue::Bool(false),
            locked: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            tags: vec![],
            group: None,
        };
        manager.register_flag(metadata_a).unwrap();

        let metadata_b = FlagMetadata {
            name: "feature-b".to_string(),
            description: None,
            default_value: FlagValue::Bool(false),
            current_value: FlagValue::Bool(false),
            locked: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            tags: vec![],
            group: None,
        };
        manager.register_flag(metadata_b).unwrap();

        // 设置布尔标志
        manager
            .set_flag("feature-a", FlagValue::Bool(true))
            .unwrap();

        // 设置字符串标志
        manager
            .set_flag("feature-b", FlagValue::String("value".to_string()))
            .unwrap();

        // 获取标志
        let flag_a = manager.get_flag("feature-a").unwrap();
        assert!(flag_a.as_bool());

        let flag_b = manager.get_flag("feature-b").unwrap();
        assert_eq!(flag_b.as_string(), "value");
    }

    #[test]
    fn test_flag_manager_get_not_found() {
        let manager = FeatureFlagManager::new();

        let result = manager.get_flag("non-existent");
        assert!(matches!(
            result,
            Err(FeatureFlagError::FlagNotFound { name }) if name == "non-existent"
        ));
    }

    #[test]
    fn test_flag_manager_is_enabled() {
        let manager = FeatureFlagManager::new();

        // 注册并设置标志
        for (name, value) in [
            ("enabled-feature", FlagValue::Bool(true)),
            ("disabled-feature", FlagValue::Bool(false)),
            ("percentage-feature", FlagValue::Percentage(0.5)),
            ("zero-percentage", FlagValue::Percentage(0.0)),
        ] {
            let metadata = FlagMetadata {
                name: name.to_string(),
                description: None,
                default_value: value.clone(),
                current_value: value.clone(),
                locked: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                tags: vec![],
                group: None,
            };
            manager.register_flag(metadata).unwrap();
        }

        assert!(manager.is_enabled("enabled-feature"));
        assert!(!manager.is_enabled("disabled-feature"));
        assert!(manager.is_enabled("percentage-feature"));
        assert!(!manager.is_enabled("zero-percentage"));
    }

    #[test]
    fn test_flag_metadata() {
        let metadata = FlagMetadata {
            name: "test-flag".to_string(),
            description: Some("测试标志".to_string()),
            default_value: FlagValue::Bool(false),
            current_value: FlagValue::Bool(true),
            locked: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            tags: vec![],
            group: None,
        };

        assert_eq!(metadata.name, "test-flag");
        assert_eq!(metadata.description.as_ref().unwrap(), "测试标志");
        assert!(!metadata.default_value.as_bool());
    }
}

#[cfg(test)]
mod registry_error_tests {
    use super::*;

    #[test]
    fn test_service_already_exists_error_display() {
        let err = RegistryError::ServiceAlreadyExists {
            name: "test-service".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("test-service"));
        assert!(msg.contains("已存在"));
    }

    #[test]
    fn test_service_not_found_error_display() {
        let err = RegistryError::ServiceNotFound {
            name: "missing-service".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("missing-service"));
        assert!(msg.contains("不存在"));
    }

    #[test]
    fn test_circular_dependency_error_display() {
        let err = RegistryError::CircularDependency {
            dependency_chain: "A -> B -> C -> A".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("循环依赖"));
    }

    #[test]
    fn test_missing_dependencies_error_display() {
        let err = RegistryError::MissingDependencies {
            missing_dependencies: vec!["dep-a".to_string(), "dep-b".to_string()],
        };
        let msg = err.to_string();
        assert!(msg.contains("dep-a"));
        assert!(msg.contains("dep-b"));
    }

    #[test]
    fn test_feature_flag_error_display() {
        let err = RegistryError::InvalidFeatureFlag {
            flag: "invalid-flag".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("invalid-flag"));
    }
}

#[cfg(test)]
mod registry_config_tests {
    use super::*;

    #[test]
    fn test_default_registry_config() {
        let config = RegistryConfig::default();

        assert!(config.feature_flags.is_empty());
        assert!(config.service_discovery.auto_discover);
        assert!(config.service_discovery.scan_paths.is_empty());
        assert_eq!(config.service_discovery.include_patterns, vec!["*"]);
        assert!(config.service_discovery.exclude_patterns.is_empty());
    }

    #[test]
    fn test_registry_from_config() {
        let mut config = RegistryConfig::default();
        config
            .feature_flags
            .insert("test-feature".to_string(), true);

        let registry = DefaultServiceRegistry::from_config(config);

        // 应该创建成功，功能标志应该被设置
        assert!(registry.has_service("test-feature") == false); // feature_flags 不等于 services
    }
}
