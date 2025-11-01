use open_lark_core::core::core::{config::Config, trait_system::Service};

pub mod department;
pub mod employee;
pub mod models;

pub use models::*;

/// 组织架构 v1 API
///
/// 提供完整的组织架构管理功能，包括员工和部门的创建、更新、删除、查询等操作。
/// 支持复杂的组织结构管理、权限控制和数据同步。
///
/// # 主要功能
///
/// ## 员工管理
/// - 👤 员工的创建、更新、删除
/// - 🔍 员工信息查询和搜索
/// - 📋 批量员工操作
/// - 🔄 员工状态管理（在职、离职、复职等）
/// - 🏷️ 员工分组和标签管理
///
/// ## 部门管理
/// - 🏢 部门的创建、更新、删除
/// - 🌳 部门层级结构管理
/// - 🔍 部门信息查询和搜索
/// - 📊 部门统计和分析
/// - 👥 部门成员管理
///
/// # 使用场景
///
/// - HR 管理系统
/// - 组织架构可视化
/// - 员工自助服务平台
/// - 权限管理系统
/// - 数据分析和报表
///
/// # 示例
///
/// ```rust
/// use open_lark::prelude::*;
///
/// let client = LarkClient::builder("app_id", "app_secret")
///     .build();
///
/// // 使用目录服务
/// let directory = &client.directory.v1;
///
/// // 创建员工
/// // let create_req = CreateEmployeeRequest::builder()
/// //     .name("张三")
/// //     .department_id("dept_123")
/// //     .build();
/// // let employee = directory.employee.create(create_req, None).await?;
///
/// // 查询部门
/// // let dept_req = GetDepartmentRequest::builder()
/// //     .department_id("dept_123")
/// //     .build();
/// // let department = directory.department.get(dept_req, None).await?;
/// ```
#[derive(Debug)]
pub struct V1 {
    /// 员工管理服务
    ///
    /// 提供员工的全生命周期管理，包括入职、调岗、离职等操作。
    /// 支持复杂的员工信息管理和批量操作。
    pub employee: employee::EmployeeService,

    /// 部门管理服务
    ///
    /// 提供部门的层级结构管理，支持无限层级的部门嵌套。
    /// 包含部门的创建、合并、拆分等高级功能。
    pub department: department::DepartmentService,


impl V1 {
    /// 创建新的组织架构服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的V1服务实例，包含员工和部门管理功能
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::core::config::Config;
    /// use open_lark::service::directory::v1::V1;
    ///
    /// let config = Config::builder()
    ///     .app_id("your_app_id")
    ///     .app_secret("your_app_secret")
    ///     .build();
    ///
    /// let directory_service = V1::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            employee: employee::EmployeeService::new(config.clone()),
            department: department::DepartmentService::new(config),
        }
    }

    /// 验证服务配置是否正确
    ///
    /// 检查所有子服务的配置是否一致且有效。
    ///
    /// # 返回值
    /// 如果配置有效返回 `true`，否则返回 `false`
    pub fn validate_config(&self) -> bool {
        // 检查员工服务配置
        if self.employee.config.app_id.is_empty() {
            return false;
        }

        // 检查部门服务配置
        if self.department.config.app_id.is_empty() {
            return false;
        }

        // 检查两个服务的 app_id 是否一致
        self.employee.config.app_id == self.department.config.app_id
    }

    /// 获取服务统计信息
    ///
    /// 返回当前服务实例的基本统计信息，用于监控和调试。
    ///
    /// # 返回值
    /// 包含服务名称和配置信息的字符串
    pub fn get_service_stats(&self) -> String {
        format!(
            "DirectoryV1{{ employee: {}, department: {}, app_id: {} }}",
            employee::EmployeeService::service_name(),
            department::DepartmentService::service_name(),
            self.employee.config.app_id
        )
    }


#[cfg(test)]
mod tests {
    use super::*;
    use open_lark_core::core::core::config::Config;
    use std::sync::Arc;

    // Helper function to create test config
    fn create_test_config() -> Config {
        Config::builder()
            .app_id("directory_test_app")
            .app_secret("directory_test_secret")
            .build()
    }

    #[test]
    fn test_directory_v1_service_creation() {
        let config = create_test_config();
        let directory_service = V1::new(config.clone());

        // Verify both sub-services are created
        assert_eq!(
            directory_service.employee.config.app_id,
            "directory_test_app"
        );
        assert_eq!(
            directory_service.department.config.app_id,
            "directory_test_app"
        );
        assert_eq!(
            directory_service.employee.config.app_secret,
            "directory_test_secret"
        );
        assert_eq!(
            directory_service.department.config.app_secret,
            "directory_test_secret"
        );
    }

    #[test]
    fn test_directory_v1_service_config_independence() {
        let config = create_test_config();
        let directory_service = V1::new(config);

        // Create different config for comparison
        let different_config = Config::builder()
            .app_id("different_app")
            .app_secret("different_secret")
            .build();

        let different_service = V1::new(different_config);

        // Services should have different configs
        assert_ne!(
            directory_service.employee.config.app_id,
            different_service.employee.config.app_id
        );
        assert_ne!(
            directory_service.department.config.app_id,
            different_service.department.config.app_id
        );
    }

    #[test]
    fn test_directory_v1_validate_config() {
        let valid_config = create_test_config();
        let valid_service = V1::new(valid_config);
        assert!(valid_service.validate_config());

        // Test with empty app_id
        let invalid_config = Config::builder()
            .app_id("") // Empty app_id
            .app_secret("test_secret")
            .build();
        let invalid_service = V1::new(invalid_config);
        assert!(!invalid_service.validate_config());
    }

    #[test]
    fn test_directory_v1_get_service_stats() {
        let config = create_test_config();
        let directory_service = V1::new(config);

        let stats = directory_service.get_service_stats();

        assert!(stats.contains("DirectoryV1"));
        assert!(stats.contains("employee"));
        assert!(stats.contains("department"));
        assert!(stats.contains("directory_test_app"));
    }

    #[test]
    fn test_directory_v1_service_consistency() {
        let config = create_test_config();
        let directory_service = V1::new(config);

        // Both services should have the same config
        assert_eq!(
            directory_service.employee.config.app_id,
            directory_service.department.config.app_id
        );
        assert_eq!(
            directory_service.employee.config.app_secret,
            directory_service.department.config.app_secret
        );
    }

    #[test]
    fn test_directory_v1_unicode_support() {
        let unicode_config = Config::builder()
            .app_id("目录服务_测试_123")
            .app_secret("密钥_🔐_特殊字符")
            .build();

        let directory_service = V1::new(unicode_config);

        assert_eq!(
            directory_service.employee.config.app_id,
            "目录服务_测试_123"
        );
        assert_eq!(
            directory_service.employee.config.app_secret,
            "密钥_🔐_特殊字符"
        );
        assert_eq!(
            directory_service.department.config.app_id,
            "目录服务_测试_123"
        );
        assert_eq!(
            directory_service.department.config.app_secret,
            "密钥_🔐_特殊字符"
        );
    }

    #[test]
    fn test_directory_v1_large_values() {
        let large_app_id = "a".repeat(500);
        let large_secret = "s".repeat(1000);

        let large_config = Config::builder()
            .app_id(large_app_id.clone())
            .app_secret(large_secret.clone())
            .build();

        let directory_service = V1::new(large_config);

        assert_eq!(directory_service.employee.config.app_id, large_app_id);
        assert_eq!(directory_service.employee.config.app_secret, large_secret);
        assert_eq!(directory_service.department.config.app_id, large_app_id);
        assert_eq!(directory_service.department.config.app_secret, large_secret);
    }

    #[test]
    fn test_directory_v1_multiple_instances() {
        let config1 = Config::builder()
            .app_id("instance_1")
            .app_secret("secret_1")
            .build();

        let config2 = Config::builder()
            .app_id("instance_2")
            .app_secret("secret_2")
            .build();

        let service1 = V1::new(config1);
        let service2 = V1::new(config2);

        // Verify instances are independent
        assert_ne!(
            service1.employee.config.app_id,
            service2.employee.config.app_id
        );
        assert_ne!(
            service1.department.config.app_id,
            service2.department.config.app_id
        );
        assert_eq!(service1.employee.config.app_id, "instance_1");
        assert_eq!(service2.employee.config.app_id, "instance_2");
    }

    #[test]
    fn test_directory_v1_config_modification_independence() {
        let original_config = create_test_config();
        let original_service = V1::new(original_config);

        // Create modified config
        let modified_config = Config::builder()
            .app_id("modified_app")
            .app_secret("modified_secret")
            .build();

        let modified_service = V1::new(modified_config);

        // Original service should not be affected
        assert_eq!(
            original_service.employee.config.app_id,
            "directory_test_app"
        );
        assert_eq!(
            original_service.department.config.app_id,
            "directory_test_app"
        );

        // Modified service should have new values
        assert_eq!(modified_service.employee.config.app_id, "modified_app");
        assert_eq!(modified_service.department.config.app_id, "modified_app");
    }

    #[test]
    fn test_directory_v1_memory_efficiency() {
        let config = create_test_config();

        // Create multiple service instances
        let services: Vec<V1> = (0..50).map(|_| V1::new(config.clone())).collect();

        assert_eq!(services.len(), 50);

        // All services should have the same app_id
        for service in &services {
            assert_eq!(service.employee.config.app_id, "directory_test_app");
            assert_eq!(service.department.config.app_id, "directory_test_app");
        }
    }

    #[test]
    fn test_directory_v1_arc_sharing() {
        let shared_config = Arc::new(create_test_config());

        // Create services using shared config
        let config1 = (*shared_config).clone();
        let config2 = (*shared_config).clone();
        let service1 = V1::new(config1);
        let service2 = V1::new(config2);

        // Both services should have the same values
        assert_eq!(service1.employee.config.app_id, "directory_test_app");
        assert_eq!(service2.employee.config.app_id, "directory_test_app");
        assert_eq!(
            service1.department.config.app_secret,
            "directory_test_secret"
        );
        assert_eq!(
            service2.department.config.app_secret,
            "directory_test_secret"
        );
    }

    #[test]
    fn test_directory_v1_config_properties() {
        let config = Config::builder()
            .app_id("props_test")
            .app_secret("props_secret")
            .enable_token_cache(false)
            .build();

        let directory_service = V1::new(config);

        // Test config properties
        assert_eq!(directory_service.employee.config.app_id, "props_test");
        assert_eq!(directory_service.employee.config.app_secret, "props_secret");
        assert!(!directory_service.employee.config.enable_token_cache);

        // Department service should have same properties
        assert_eq!(directory_service.department.config.app_id, "props_test");
        assert!(!directory_service.department.config.enable_token_cache);
    }

    #[test]
    fn test_directory_v1_service_names() {
        let config = create_test_config();
        let _directory_service = V1::new(config);

        // Test service names and versions
        assert_eq!(employee::EmployeeService::service_name(), "employee");
        assert_eq!(employee::EmployeeService::service_version(), "v1");
        assert_eq!(department::DepartmentService::service_name(), "department");
        assert_eq!(department::DepartmentService::service_version(), "v1");
    }

    #[test]
    fn test_directory_v1_thread_safety() {
        use std::thread;

        let config = create_test_config();
        let directory_service = Arc::new(V1::new(config));

        let handles: Vec<_> = (0..10)
            .map(|i| {
                let service_clone = Arc::clone(&directory_service);
                thread::spawn(move || {
                    format!(
                        "thread_{}_employee: {}",
                        i, service_clone.employee.config.app_id
                    )
                })
            })
            .collect();

        // All threads should be able to access the service safely
        for handle in handles {
            let result = handle.join().unwrap();
            assert!(result.contains("directory_test_app"));
        }
    }

    #[test]
    fn test_directory_v1_config_field_access() {
        let config = create_test_config();
        let directory_service = V1::new(config);

        // Test that we can access all config fields
        assert!(!directory_service.employee.config.base_url.is_empty());
        assert!(directory_service.employee.config.enable_token_cache);
        assert!(directory_service.employee.config.header.is_empty()); // Default empty header map
        assert!(!directory_service.department.config.base_url.is_empty());
        assert!(directory_service.department.config.enable_token_cache);
        assert!(directory_service.department.config.header.is_empty());
    }

    #[test]
    fn test_directory_v1_debug_representation() {
        let config = create_test_config();
        let directory_service = V1::new(config);

        let debug_str = format!("{:?}", directory_service);

        // Note: This test might fail if the struct doesn't derive Debug
        // It's included for completeness
        println!("Directory service debug: {}", debug_str);
    }

    #[test]
    fn test_directory_v1_service_clone_behavior() {
        let config = create_test_config();
        let original_service = V1::new(config);
        let cloned_config = original_service.employee.config.clone();

        // Create a new service with cloned config
        let cloned_service = V1::new(cloned_config);

        // Both services should have the same values
        assert_eq!(
            original_service.employee.config.app_id,
            cloned_service.employee.config.app_id
        );
        assert_eq!(
            original_service.department.config.app_id,
            cloned_service.department.config.app_id
        );
    }

    #[test]
    fn test_directory_v1_config_comparison() {
        let config1 = create_test_config();
        let config2 = Config::builder()
            .app_id("directory_test_app")
            .app_secret("directory_test_secret")
            .build();

        let service1 = V1::new(config1);
        let service2 = V1::new(config2);

        // Services with equivalent configs should have same values
        assert_eq!(
            service1.employee.config.app_id,
            service2.employee.config.app_id
        );
        assert_eq!(
            service1.department.config.app_id,
            service2.department.config.app_id
        );
        assert_eq!(
            service1.employee.config.app_secret,
            service2.employee.config.app_secret
        );
        assert_eq!(
            service1.department.config.app_secret,
            service2.department.config.app_secret
        );
    }

    #[test]
    fn test_directory_v1_edge_case_configs() {
        // Test with minimal config
        let minimal_config = Config::builder().build();
        let minimal_service = V1::new(minimal_config);
        assert!(!minimal_service.validate_config()); // Empty app_id should fail validation

        // Test with only app_id
        let partial_config = Config::builder().app_id("partial_app").build();
        let partial_service = V1::new(partial_config);
        assert!(partial_service.validate_config()); // Has app_id, should pass validation
    }

    #[test]
    fn test_directory_v1_service_serialization_compatibility() {
        let config = create_test_config();
        let directory_service = V1::new(config);

        // Test that config values can be serialized to strings if needed
        let app_id_str = directory_service.employee.config.app_id.clone();
        let secret_str = directory_service.department.config.app_secret.clone();
        let service_name = employee::EmployeeService::service_name().to_string();

        assert_eq!(app_id_str, "directory_test_app");
        assert_eq!(secret_str, "directory_test_secret");
        assert_eq!(service_name, "employee");
        assert!(!app_id_str.is_empty());
        assert!(!secret_str.is_empty());
        assert!(!service_name.is_empty());
    }

    #[test]
    fn test_directory_v1_config_modular_design() {
        // Test that employee and department services can be used independently
        let config = create_test_config();
        let employee_service = employee::EmployeeService::new(config.clone());
        let department_service = department::DepartmentService::new(config);

        // Both should have the same config values
        assert_eq!(
            employee_service.config.app_id,
            department_service.config.app_id
        );
        assert_eq!(
            employee_service.config.app_secret,
            department_service.config.app_secret
        );

        // But they should be separate instances
        let employee_ptr = std::ptr::addr_of!(employee_service.config);
        let department_ptr = std::ptr::addr_of!(department_service.config);
        assert_ne!(employee_ptr, department_ptr);
    }

    #[test]
    fn test_directory_v1_error_handling_simulation() {
        // This test simulates error handling scenarios
        let config = create_test_config();
        let directory_service = V1::new(config);

        // Test validation with various scenarios
        assert!(directory_service.validate_config()); // Should pass with valid config

        // Test stats generation doesn't panic
        let _stats = directory_service.get_service_stats();

        // Test service access doesn't panic
        let _employee_name = employee::EmployeeService::service_name();
        let _department_name = department::DepartmentService::service_name();
        let _employee_version = employee::EmployeeService::service_version();
        let _department_version = department::DepartmentService::service_version();
    }

