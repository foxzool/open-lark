//! ServiceRegistry、ServiceFactory、DependencyResolver 和 FeatureFlags 测试
//!
//! 覆盖服务注册、工厂创建、依赖解析和功能标志管理的核心流程

use std::collections::{HashMap, HashSet};

use openlark_client::registry::feature_flags::FeatureFlagError;
use openlark_client::registry::{
    DefaultServiceRegistry, DependencyError, DependencyResolver, FeatureFlagManager, FlagMetadata,
    FlagValue, RegistryError, SegmentCondition, ServiceFactoryRegistry, ServiceInstanceManager,
    ServiceMetadata, ServiceRegistry, ServiceStatus, UserSegment,
};
use openlark_client::Config;

fn create_test_metadata(name: &str) -> ServiceMetadata {
    ServiceMetadata {
        name: name.to_string(),
        version: "1.0.0".to_string(),
        description: Some(format!("{name} 测试服务")),
        dependencies: vec![],
        provides: vec![format!("{name}-feature")],
        status: ServiceStatus::Uninitialized,
        priority: 1,
    }
}

fn create_test_metadata_with_deps(name: &str, deps: Vec<&str>) -> ServiceMetadata {
    ServiceMetadata {
        name: name.to_string(),
        version: "1.0.0".to_string(),
        description: None,
        dependencies: deps.iter().map(|s| s.to_string()).collect(),
        provides: vec![],
        status: ServiceStatus::Uninitialized,
        priority: 1,
    }
}

fn create_test_config() -> Config {
    Config {
        app_id: "test_app_id".to_string(),
        app_secret: "test_app_secret".to_string(),
        base_url: "https://test.feishu.cn".to_string(),
        ..Default::default()
    }
}

// ============================================================================
// ServiceRegistry Tests
// ============================================================================

mod service_registry_tests {
    use super::*;

    #[test]
    fn test_register_service_success() {
        let mut registry = DefaultServiceRegistry::new();
        let metadata = create_test_metadata("test-service");

        let result = registry.register_service(metadata);
        assert!(result.is_ok());
        assert!(registry.has_service("test-service"));
    }

    #[test]
    fn test_register_duplicate_service_fails() {
        let mut registry = DefaultServiceRegistry::new();
        let metadata = create_test_metadata("duplicate-service");

        registry.register_service(metadata.clone()).unwrap();
        let result = registry.register_service(metadata);

        assert!(matches!(
            result,
            Err(RegistryError::ServiceAlreadyExists { name }) if name == "duplicate-service"
        ));
    }

    #[test]
    fn test_unregister_service_success() {
        let mut registry = DefaultServiceRegistry::new();
        let metadata = create_test_metadata("to-remove");

        registry.register_service(metadata).unwrap();
        assert!(registry.has_service("to-remove"));

        let result = registry.unregister_service("to-remove");
        assert!(result.is_ok());
        assert!(!registry.has_service("to-remove"));
    }

    #[test]
    fn test_unregister_nonexistent_service_fails() {
        let mut registry = DefaultServiceRegistry::new();

        let result = registry.unregister_service("nonexistent");
        assert!(matches!(
            result,
            Err(RegistryError::ServiceNotFound { name }) if name == "nonexistent"
        ));
    }

    #[test]
    fn test_get_service_success() {
        let mut registry = DefaultServiceRegistry::new();
        let metadata = create_test_metadata("get-test");

        registry.register_service(metadata).unwrap();
        let result = registry.get_service("get-test");

        assert!(result.is_ok());
        let entry = result.unwrap();
        assert_eq!(entry.metadata.name, "get-test");
        assert_eq!(entry.metadata.version, "1.0.0");
    }

    #[test]
    fn test_get_service_not_found() {
        let registry = DefaultServiceRegistry::new();

        let result = registry.get_service("missing-service");
        assert!(matches!(
            result,
            Err(RegistryError::ServiceNotFound { name }) if name == "missing-service"
        ));
    }

    #[test]
    fn test_list_services_empty() {
        let registry = DefaultServiceRegistry::new();
        let services = registry.list_services();
        assert!(services.is_empty());
    }

    #[test]
    fn test_list_services_multiple() {
        let mut registry = DefaultServiceRegistry::new();

        registry
            .register_service(create_test_metadata("service-a"))
            .unwrap();
        registry
            .register_service(create_test_metadata("service-b"))
            .unwrap();
        registry
            .register_service(create_test_metadata("service-c"))
            .unwrap();

        let services = registry.list_services();
        assert_eq!(services.len(), 3);
    }

    #[test]
    fn test_update_service_status() {
        let mut registry = DefaultServiceRegistry::new();
        let metadata = create_test_metadata("status-test");

        registry.register_service(metadata).unwrap();

        let result = registry.update_service_status("status-test", ServiceStatus::Initializing);
        assert!(result.is_ok());

        let entry = registry.get_service("status-test").unwrap();
        assert_eq!(entry.metadata.status, ServiceStatus::Initializing);
    }

    #[test]
    fn test_update_status_nonexistent_service() {
        let mut registry = DefaultServiceRegistry::new();

        let result = registry.update_service_status("ghost", ServiceStatus::Running);
        assert!(matches!(result, Err(RegistryError::ServiceNotFound { .. })));
    }

    #[test]
    fn test_get_dependency_graph() {
        let mut registry = DefaultServiceRegistry::new();

        registry
            .register_service(create_test_metadata_with_deps("api", vec!["auth", "db"]))
            .unwrap();
        registry
            .register_service(create_test_metadata_with_deps("auth", vec!["db"]))
            .unwrap();
        registry
            .register_service(create_test_metadata_with_deps("db", vec![]))
            .unwrap();

        let graph = registry.get_dependency_graph();

        assert_eq!(graph.get("api").unwrap().len(), 2);
        assert_eq!(graph.get("auth").unwrap().len(), 1);
        assert_eq!(graph.get("db").unwrap().len(), 0);
    }
}

// ============================================================================
// DependencyResolver Tests
// ============================================================================

mod dependency_resolver_tests {
    use super::*;

    #[test]
    fn test_resolve_simple_dependencies() {
        let resolver = DependencyResolver::new();
        let mut graph = HashMap::new();

        graph.insert("base".to_string(), vec![]);
        graph.insert("middle".to_string(), vec!["base".to_string()]);
        graph.insert("top".to_string(), vec!["middle".to_string()]);

        let result = resolver.resolve_dependencies(graph);
        assert!(result.is_ok());

        let sorted = result.unwrap();
        assert_eq!(sorted.first().unwrap(), "base");
        assert_eq!(sorted.get(1).unwrap(), "middle");
        assert_eq!(sorted.get(2).unwrap(), "top");
    }

    #[test]
    fn test_detect_circular_dependency() {
        let resolver = DependencyResolver::new();
        let mut graph = HashMap::new();

        graph.insert("a".to_string(), vec!["b".to_string()]);
        graph.insert("b".to_string(), vec!["c".to_string()]);
        graph.insert("c".to_string(), vec!["a".to_string()]);

        let result = resolver.resolve_dependencies(graph);
        assert!(matches!(
            result,
            Err(DependencyError::CircularDependency { .. })
        ));
    }

    #[test]
    fn test_missing_dependency() {
        let resolver = DependencyResolver::new();
        let mut graph = HashMap::new();

        graph.insert("service".to_string(), vec!["nonexistent".to_string()]);

        let result = resolver.resolve_dependencies(graph);
        assert!(matches!(
            result,
            Err(DependencyError::MissingDependencies { .. })
        ));
    }

    #[test]
    fn test_calculate_priorities() {
        let resolver = DependencyResolver::new();
        let mut graph = HashMap::new();

        graph.insert("db".to_string(), vec![]);
        graph.insert("cache".to_string(), vec!["db".to_string()]);
        graph.insert("api".to_string(), vec!["cache".to_string()]);

        let result = resolver.calculate_priorities(&graph);
        assert!(result.is_ok());

        let priorities = result.unwrap();
        assert!(priorities.get("db").unwrap() < priorities.get("cache").unwrap());
        assert!(priorities.get("cache").unwrap() < priorities.get("api").unwrap());
    }

    #[test]
    fn test_get_direct_dependencies() {
        let resolver = DependencyResolver::new();
        let mut graph = HashMap::new();

        graph.insert(
            "service".to_string(),
            vec!["dep1".to_string(), "dep2".to_string()],
        );
        graph.insert("dep1".to_string(), vec![]);
        graph.insert("dep2".to_string(), vec![]);

        let result = resolver.get_direct_dependencies("service", &graph);
        assert!(result.is_ok());

        let deps = result.unwrap();
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&"dep1".to_string()));
        assert!(deps.contains(&"dep2".to_string()));
    }

    #[test]
    fn test_get_all_dependencies_transitive() {
        let resolver = DependencyResolver::new();
        let mut graph = HashMap::new();

        graph.insert("a".to_string(), vec![]);
        graph.insert("b".to_string(), vec!["a".to_string()]);
        graph.insert("c".to_string(), vec!["b".to_string()]);

        let result = resolver.get_all_dependencies("c", &graph);
        assert!(result.is_ok());

        let all_deps = result.unwrap();
        assert!(all_deps.contains("a"));
        assert!(all_deps.contains("b"));
        assert!(!all_deps.contains("c"));
    }

    #[test]
    fn test_can_start_service() {
        let resolver = DependencyResolver::new();
        let mut graph = HashMap::new();

        graph.insert("base".to_string(), vec![]);
        graph.insert("dependent".to_string(), vec!["base".to_string()]);

        let running = HashSet::from(["base".to_string()]);

        let result = resolver.can_start("dependent", &graph, &running);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_cannot_start_service_missing_deps() {
        let resolver = DependencyResolver::new();
        let mut graph = HashMap::new();

        graph.insert("base".to_string(), vec![]);
        graph.insert("dependent".to_string(), vec!["base".to_string()]);

        let running = HashSet::new();

        let result = resolver.can_start("dependent", &graph, &running);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_generate_dependency_report() {
        let resolver = DependencyResolver::new();
        let mut graph = HashMap::new();

        graph.insert("core".to_string(), vec![]);
        graph.insert("service".to_string(), vec!["core".to_string()]);

        let result = resolver.generate_dependency_report(&graph);
        assert!(result.is_ok());

        let report = result.unwrap();
        assert_eq!(report.total_services, 2);
        assert!(!report.has_circular_dependencies);
        assert!(report.sorted_services.first().unwrap() == "core");
    }
}

// ============================================================================
// ServiceFactory Tests
// ============================================================================

mod service_factory_tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_service_factory_registry_creation() {
        let registry = ServiceFactoryRegistry::new();
        let factory = registry.get_factory("any-service");
        assert!(Arc::strong_count(&factory) >= 1);
    }

    #[test]
    fn test_service_factory_registry_default_factory() {
        let registry = ServiceFactoryRegistry::new();

        let factory1 = registry.get_factory("service-a");
        let factory2 = registry.get_factory("service-b");

        assert_eq!(Arc::as_ptr(&factory1), Arc::as_ptr(&factory2));
    }

    #[test]
    fn test_instance_manager_creation() {
        let config = create_test_config();
        let manager = ServiceInstanceManager::new(config);

        let stats = manager.get_service_stats();
        assert!(stats.is_empty());
    }

    #[test]
    fn test_instance_manager_register_factory() {
        let config = create_test_config();
        let manager = ServiceInstanceManager::new(config);

        manager.register_factory(
            "test-service",
            Arc::new(openlark_client::registry::DefaultServiceFactory),
        );

        let stats = manager.get_service_stats();
        assert!(stats.is_empty());
    }
}

// ============================================================================
// FeatureFlags Tests
// ============================================================================

mod feature_flags_tests {
    use super::*;

    fn create_test_flag(name: &str, enabled: bool, locked: bool) -> FlagMetadata {
        FlagMetadata {
            name: name.to_string(),
            description: Some(format!("{name} 功能标志")),
            default_value: FlagValue::Bool(false),
            current_value: FlagValue::Bool(enabled),
            locked,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            tags: vec!["test".to_string()],
            group: Some("test-group".to_string()),
        }
    }

    #[test]
    fn test_feature_flag_manager_creation() {
        let manager = FeatureFlagManager::new();
        let flags = manager.list_flags();
        assert!(flags.is_empty());
    }

    #[test]
    fn test_register_and_get_flag() {
        let manager = FeatureFlagManager::new();
        let flag = create_test_flag("test-flag", false, false);

        manager.register_flag(flag).unwrap();

        let result = manager.get_flag("test-flag");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), FlagValue::Bool(false));
    }

    #[test]
    fn test_set_bool_flag() {
        let manager = FeatureFlagManager::new();
        let flag = create_test_flag("bool-flag", false, false);

        manager.register_flag(flag).unwrap();
        manager.set_bool_flag("bool-flag", true).unwrap();

        assert!(manager.is_enabled("bool-flag"));
    }

    #[test]
    fn test_set_string_flag() {
        let manager = FeatureFlagManager::new();
        let flag = FlagMetadata {
            name: "string-flag".to_string(),
            description: None,
            default_value: FlagValue::String(String::new()),
            current_value: FlagValue::String(String::new()),
            locked: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            tags: vec![],
            group: None,
        };

        manager.register_flag(flag).unwrap();
        manager
            .set_string_flag("string-flag", "test-value")
            .unwrap();

        let result = manager.get_flag("string-flag").unwrap();
        assert_eq!(result, FlagValue::String("test-value".to_string()));
    }

    #[test]
    fn test_set_integer_flag() {
        let manager = FeatureFlagManager::new();
        let flag = FlagMetadata {
            name: "int-flag".to_string(),
            description: None,
            default_value: FlagValue::Integer(0),
            current_value: FlagValue::Integer(0),
            locked: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            tags: vec![],
            group: None,
        };

        manager.register_flag(flag).unwrap();
        manager.set_integer_flag("int-flag", 42).unwrap();

        let result = manager.get_flag("int-flag").unwrap();
        assert_eq!(result, FlagValue::Integer(42));
    }

    #[test]
    fn test_set_percentage_flag_valid() {
        let manager = FeatureFlagManager::new();
        let flag = FlagMetadata {
            name: "pct-flag".to_string(),
            description: None,
            default_value: FlagValue::Percentage(0.0),
            current_value: FlagValue::Percentage(0.0),
            locked: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            tags: vec![],
            group: None,
        };

        manager.register_flag(flag).unwrap();
        let result = manager.set_percentage_flag("pct-flag", 0.5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_percentage_flag_invalid() {
        let manager = FeatureFlagManager::new();
        let flag = FlagMetadata {
            name: "pct-flag-invalid".to_string(),
            description: None,
            default_value: FlagValue::Percentage(0.0),
            current_value: FlagValue::Percentage(0.0),
            locked: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            tags: vec![],
            group: None,
        };

        manager.register_flag(flag).unwrap();
        let result = manager.set_percentage_flag("pct-flag-invalid", 1.5);
        assert!(matches!(result, Err(FeatureFlagError::InvalidValue { .. })));
    }

    #[test]
    fn test_locked_flag_cannot_be_modified() {
        let manager = FeatureFlagManager::new();
        let flag = create_test_flag("locked-flag", false, true);

        manager.register_flag(flag).unwrap();
        let result = manager.set_bool_flag("locked-flag", true);

        assert!(matches!(
            result,
            Err(FeatureFlagError::FlagLocked { name }) if name == "locked-flag"
        ));
    }

    #[test]
    fn test_lock_and_unlock_flag() {
        let manager = FeatureFlagManager::new();
        let flag = create_test_flag("lockable-flag", false, false);

        manager.register_flag(flag).unwrap();

        manager.lock_flag("lockable-flag").unwrap();
        let locked_result = manager.set_bool_flag("lockable-flag", true);
        assert!(matches!(
            locked_result,
            Err(FeatureFlagError::FlagLocked { .. })
        ));

        manager.unlock_flag("lockable-flag").unwrap();
        let unlocked_result = manager.set_bool_flag("lockable-flag", true);
        assert!(unlocked_result.is_ok());
    }

    #[test]
    fn test_get_nonexistent_flag() {
        let manager = FeatureFlagManager::new();

        let result = manager.get_flag("nonexistent");
        assert!(matches!(
            result,
            Err(FeatureFlagError::FlagNotFound { name }) if name == "nonexistent"
        ));
    }

    #[test]
    fn test_list_flags_by_group() {
        let manager = FeatureFlagManager::new();

        manager
            .register_flag(create_test_flag("flag-a", false, false))
            .unwrap();
        manager
            .register_flag(FlagMetadata {
                name: "flag-b".to_string(),
                description: None,
                default_value: FlagValue::Bool(false),
                current_value: FlagValue::Bool(false),
                locked: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                tags: vec![],
                group: Some("other-group".to_string()),
            })
            .unwrap();

        let test_group_flags = manager.list_flags_by_group("test-group");
        assert_eq!(test_group_flags.len(), 1);
        assert_eq!(test_group_flags[0].name, "flag-a");
    }

    #[test]
    fn test_flag_value_as_bool() {
        assert!(FlagValue::Bool(true).as_bool());
        assert!(!FlagValue::Bool(false).as_bool());
        assert!(FlagValue::String("text".to_string()).as_bool());
        assert!(!FlagValue::String(String::new()).as_bool());
        assert!(FlagValue::Integer(1).as_bool());
        assert!(!FlagValue::Integer(0).as_bool());
        assert!(FlagValue::Float(1.0).as_bool());
        assert!(!FlagValue::Float(0.0).as_bool());
        assert!(FlagValue::Percentage(0.5).as_bool());
        assert!(!FlagValue::Percentage(0.0).as_bool());
    }

    #[test]
    fn test_flag_value_as_string() {
        assert_eq!(FlagValue::Bool(true).as_string(), "true");
        assert_eq!(FlagValue::String("test".to_string()).as_string(), "test");
        assert_eq!(FlagValue::Integer(42).as_string(), "42");
        assert_eq!(FlagValue::Percentage(0.5).as_string(), "50%");
    }

    #[test]
    fn test_flag_value_as_integer() {
        assert_eq!(FlagValue::Bool(true).as_integer(), 1);
        assert_eq!(FlagValue::Bool(false).as_integer(), 0);
        assert_eq!(FlagValue::Integer(100).as_integer(), 100);
        assert_eq!(FlagValue::Float(3.7).as_integer(), 3);
        assert_eq!(FlagValue::Percentage(0.5).as_integer(), 50);
    }

    #[test]
    fn test_flag_value_as_float() {
        assert_eq!(FlagValue::Bool(true).as_float(), 1.0);
        assert_eq!(FlagValue::Integer(42).as_float(), 42.0);
        assert_eq!(FlagValue::Float(2.5).as_float(), 2.5);
        assert_eq!(FlagValue::Percentage(0.75).as_float(), 0.75);
    }

    #[test]
    fn test_add_user_segment() {
        let manager = FeatureFlagManager::new();
        let segment = UserSegment {
            name: "beta-users".to_string(),
            description: Some("Beta 测试用户".to_string()),
            conditions: vec![SegmentCondition::UserIdPrefix("beta-".to_string())],
            weight: 1.0,
        };

        manager.add_user_segment(segment);
    }

    #[test]
    fn test_init_default_flags() {
        let manager = FeatureFlagManager::new();
        manager.init_default_flags();

        assert!(manager.is_enabled("auth"));
        assert!(!manager.is_enabled("docs"));
        assert!(!manager.is_enabled("hr"));
    }
}

// ============================================================================
// ServiceEntry and ServiceMetadata Tests
// ============================================================================

mod service_entry_tests {
    use super::*;

    #[test]
    fn test_service_metadata_serialization() {
        let metadata = ServiceMetadata {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            description: Some("测试".to_string()),
            dependencies: vec!["dep".to_string()],
            provides: vec!["feature".to_string()],
            status: ServiceStatus::Ready,
            priority: 5,
        };

        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: ServiceMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "test");
        assert_eq!(deserialized.version, "1.0.0");
        assert_eq!(deserialized.priority, 5);
    }

    #[test]
    fn test_service_status_equality() {
        assert_eq!(ServiceStatus::Ready, ServiceStatus::Ready);
        assert_ne!(ServiceStatus::Ready, ServiceStatus::Running);
        assert_eq!(
            ServiceStatus::Error("msg".to_string()),
            ServiceStatus::Error("msg".to_string())
        );
    }

    #[test]
    fn test_service_status_variants() {
        let statuses = vec![
            ServiceStatus::Uninitialized,
            ServiceStatus::Initializing,
            ServiceStatus::Ready,
            ServiceStatus::Running,
            ServiceStatus::Stopped,
            ServiceStatus::Error("error".to_string()),
        ];

        for status in statuses {
            let cloned = status.clone();
            assert_eq!(status, cloned);
        }
    }
}

// ============================================================================
// Registry Config Tests
// ============================================================================

mod registry_config_tests {
    use openlark_client::registry::{RegistryConfig, ServiceDiscoveryConfig};

    #[test]
    fn test_registry_config_default() {
        let config = RegistryConfig::default();

        assert!(config.feature_flags.is_empty());
        assert!(config.service_discovery.auto_discover);
        assert!(config.service_discovery.scan_paths.is_empty());
    }

    #[test]
    fn test_service_discovery_config() {
        let config = ServiceDiscoveryConfig {
            auto_discover: false,
            scan_paths: vec!["/custom/path".to_string()],
            include_patterns: vec!["service-*".to_string()],
            exclude_patterns: vec!["*-internal".to_string()],
        };

        assert!(!config.auto_discover);
        assert_eq!(config.scan_paths.len(), 1);
    }
}

// ============================================================================
// DependencyReport Tests
// ============================================================================

mod dependency_report_tests {
    use super::*;

    #[test]
    fn test_dependency_report_to_text() {
        let resolver = DependencyResolver::new();
        let mut graph = HashMap::new();

        graph.insert("db".to_string(), vec![]);
        graph.insert("api".to_string(), vec!["db".to_string()]);

        let report = resolver.generate_dependency_report(&graph).unwrap();
        let text = report.to_text();

        assert!(text.contains("依赖关系分析报告"));
        assert!(text.contains("db"));
        assert!(text.contains("api"));
    }
}
