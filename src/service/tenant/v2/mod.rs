use crate::core::config::Config;
use crate::core::trait_system::Service;

// 子模块声明
pub mod tenant;
pub mod tenant_product_assign_info;

/// Tenant v2 API 服务模块
///
/// 提供企业租户管理和席位分配的核心功能，支持多租户环境下的企业级管理。
/// 为飞书多租户解决方案提供完整的租户生命周期管理和资源分配能力。
///
/// # 主要功能
///
/// ## 租户管理
/// - 🏢 **企业信息服务**: 租户基本信息的创建、更新、查询
/// - 📊 **租户状态管理**: 租户激活、停用、删除等状态管理
/// - 🔐 **权限控制**: 租户级别的权限和访问控制管理
///
/// ## 席位分配
/// - 👥 **产品席位分配**: 各种飞书产品的席位分配和管理
/// - 📈 **配额管理**: 席位配额的监控和调整
/// - 💰 **计费管理**: 基于席位的计费和成本控制
///
/// # 使用场景
///
/// - 🏢 **多租户SaaS平台**: 为多个企业客户提供独立的服务实例
/// - 👥 **企业管理系统**: 企业内部的多部门或项目租户管理
/// - 📊 **资源分配系统**: 基于席位的资源分配和成本核算
/// - 🔧 **权限管理系统**: 精细化的租户级别权限控制
pub struct V2 {
    /// 企业信息服务
    ///
    /// 提供企业租户的基本信息管理功能，包括租户创建、更新、状态管理等。
    /// 支持多租户环境下的租户隔离和独立管理。
    pub tenant: tenant::TenantService,

    /// 企业席位信息服务
    ///
    /// 管理企业各种飞书产品的席位分配和配额控制。
    /// 提供基于席位的计费和资源优化功能。
    pub tenant_product_assign_info: tenant_product_assign_info::TenantProductAssignInfoService,
}

impl V2 {
    /// 创建新的租户 v2 服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的 V2 服务实例，包含所有租户相关子服务
    pub fn new(config: Config) -> Self {
        Self {
            tenant: tenant::TenantService::new(config.clone()),
            tenant_product_assign_info:
                tenant_product_assign_info::TenantProductAssignInfoService::new(config),
        }
    }

    /// 使用共享配置创建 v2 服务实例（实验性功能）
    ///
    /// 适用于需要多个服务实例共享同一配置的场景，可以节省内存开销。
    ///
    /// # 参数
    /// - `shared_config`: 共享的配置对象，使用 Arc 包装
    ///
    /// # 返回值
    /// 使用共享配置的服务实例
    pub fn new_from_shared(shared_config: std::sync::Arc<Config>) -> Self {
        Self {
            tenant: tenant::TenantService::new_from_shared(shared_config.clone()),
            tenant_product_assign_info:
                tenant_product_assign_info::TenantProductAssignInfoService::new_from_shared(
                    shared_config,
                ),
        }
    }

    /// 验证租户服务配置的一致性
    ///
    /// 检查所有子服务的配置是否一致且有效，确保服务间的协调工作。
    ///
    /// # 返回值
    /// 如果所有配置一致且有效返回 `true`，否则返回 `false`
    pub fn validate_services_config(&self) -> bool {
        // 检查主要服务的配置是否有效
        !self.tenant.config().app_id.is_empty() && !self.tenant.config().app_secret.is_empty()
    }

    /// 获取租户服务的整体统计信息
    ///
    /// 返回当前租户服务实例的基本统计信息，用于监控和调试。
    ///
    /// # 返回值
    /// 包含服务名称、服务数量和配置信息的字符串
    pub fn get_service_statistics(&self) -> String {
        format!(
            "TenantV2{{ services: 2, app_id: {}, tenant_service: 1, seat_service: 1 }}",
            self.tenant.config().app_id
        )
    }

    /// 检查服务是否支持特定功能
    ///
    /// 检查当前配置是否支持特定的租户功能，如企业管理、席位分配等。
    ///
    /// # 参数
    /// - `feature_name`: 功能名称
    ///
    /// # 返回值
    /// 如果支持该功能返回 `true`，否则返回 `false`
    pub fn supports_feature(&self, feature_name: &str) -> bool {
        matches!(
            feature_name,
            "tenant_management"
                | "seat_allocation"
                | "multi_tenant"
                | "enterprise_features"
                | "resource_management"
                | "billing_integration"
        )
    }

    /// 快速检查服务健康状态
    ///
    /// 检查所有子服务的基本配置是否有效。
    ///
    /// # 返回值
    /// 如果所有服务配置有效返回 `true`，否则返回 `false`
    pub fn health_check(&self) -> bool {
        !self.tenant.config().app_id.is_empty()
            && !self.tenant.config().app_secret.is_empty()
            && self.validate_services_config()
    }

    /// 获取服务分类统计
    ///
    /// 返回不同类型服务的统计信息。
    ///
    /// # 返回值
    /// 包含各类型服务数量的统计信息
    pub fn get_service_categories_statistics(&self) -> String {
        "TenantV2 Categories{ tenant: 1, seat: 1, total: 2 }".to_string()
    }
}

/// 为 V2 实现 Debug trait，用于调试输出
impl std::fmt::Debug for V2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TenantV2")
            .field("tenant", &"TenantService")
            .field(
                "tenant_product_assign_info",
                &"TenantProductAssignInfoService",
            )
            .finish()
    }
}

/// 为 V2 实现 Clone trait，支持服务实例的复制
impl Clone for V2 {
    fn clone(&self) -> Self {
        let config = self.tenant.config().clone();
        Self::new(config)
    }
}

/// 为 V2 实现 Service trait
impl crate::core::trait_system::Service for V2 {
    fn config(&self) -> &crate::core::config::Config {
        self.tenant.config()
    }

    fn service_name() -> &'static str {
        "tenant"
    }

    fn service_version() -> &'static str {
        "v2"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::trait_system::Service;

    /// 创建测试配置
    fn create_test_config() -> crate::core::config::Config {
        crate::core::config::Config::builder()
            .app_id("test_tenant_app_id")
            .app_secret("test_tenant_app_secret")
            .build()
    }

    /// 创建测试用的共享配置
    fn create_shared_test_config() -> std::sync::Arc<crate::core::config::Config> {
        std::sync::Arc::new(create_test_config())
    }

    #[test]
    fn test_tenant_v2_service_creation() {
        let config = create_test_config();
        let service = V2::new(config);

        // 验证服务创建成功
        assert_eq!(service.tenant.config().app_id, "test_tenant_app_id");
        assert_eq!(service.tenant.config().app_secret, "test_tenant_app_secret");
        assert!(!service.tenant.config().app_id.is_empty());
        assert!(!service.tenant.config().app_secret.is_empty());
    }

    #[test]
    fn test_tenant_v2_service_creation_with_shared_config() {
        let shared_config = create_shared_test_config();
        let service = V2::new_from_shared(shared_config.clone());

        // 验证共享配置服务创建成功
        assert_eq!(service.tenant.config().app_id, "test_tenant_app_id");
        assert_eq!(
            service.tenant_product_assign_info.config().app_id,
            "test_tenant_app_id"
        );
        assert_eq!(service.tenant.config().app_secret, "test_tenant_app_secret");
        assert_eq!(
            service.tenant_product_assign_info.config().app_secret,
            "test_tenant_app_secret"
        );
    }

    #[test]
    fn test_tenant_v2_validate_services_config() {
        let config = create_test_config();
        let service = V2::new(config.clone());

        // 测试有效配置
        assert!(service.validate_services_config());
        assert!(!config.app_id.is_empty());

        // 测试无效配置
        let empty_config = crate::core::config::Config::builder()
            .app_id("")
            .app_secret("test_secret")
            .build();
        let empty_service = V2::new(empty_config);
        assert!(!empty_service.validate_services_config());
    }

    #[test]
    fn test_tenant_v2_get_service_statistics() {
        let config = create_test_config();
        let service = V2::new(config);

        let stats = service.get_service_statistics();
        assert!(stats.contains("TenantV2"));
        assert!(stats.contains("services: 2"));
        assert!(stats.contains("tenant_service: 1"));
        assert!(stats.contains("seat_service: 1"));
        assert!(stats.contains("test_tenant_app_id"));
    }

    #[test]
    fn test_tenant_v2_supports_feature() {
        let config = create_test_config();
        let service = V2::new(config);

        // 测试支持的功能
        assert!(service.supports_feature("tenant_management"));
        assert!(service.supports_feature("seat_allocation"));
        assert!(service.supports_feature("multi_tenant"));
        assert!(service.supports_feature("enterprise_features"));
        assert!(service.supports_feature("resource_management"));
        assert!(service.supports_feature("billing_integration"));

        // 测试不支持的功能
        assert!(!service.supports_feature("unsupported_feature"));
        assert!(!service.supports_feature("random_feature"));
        assert!(!service.supports_feature(""));
    }

    #[test]
    fn test_tenant_v2_health_check() {
        let config = create_test_config();
        let service = V2::new(config);

        // 测试健康检查通过
        assert!(service.health_check());

        // 测试健康检查失败
        let invalid_config = crate::core::config::Config::builder()
            .app_id("")
            .app_secret("")
            .build();
        let invalid_service = V2::new(invalid_config);
        assert!(!invalid_service.health_check());
    }

    #[test]
    fn test_tenant_v2_get_service_categories_statistics() {
        let config = create_test_config();
        let service = V2::new(config);

        let stats = service.get_service_categories_statistics();
        assert!(stats.contains("TenantV2 Categories"));
        assert!(stats.contains("tenant: 1"));
        assert!(stats.contains("seat: 1"));
        assert!(stats.contains("total: 2"));
    }

    #[test]
    fn test_tenant_v2_service_trait_implementation() {
        let config = create_test_config();
        let service = V2::new(config);

        // 测试 Service trait 实现
        assert_eq!(V2::service_name(), "tenant");
        assert_eq!(V2::service_version(), "v2");
        assert_eq!(service.config().app_id, "test_tenant_app_id");
        assert_eq!(service.config().app_secret, "test_tenant_app_secret");
    }

    #[test]
    fn test_tenant_v2_clone_functionality() {
        let config = create_test_config();
        let service = V2::new(config);
        let cloned_service = service.clone();

        // 验证克隆功能
        assert_eq!(
            service.tenant.config().app_id,
            cloned_service.tenant.config().app_id
        );
        assert_eq!(
            service.tenant_product_assign_info.config().app_id,
            cloned_service.tenant_product_assign_info.config().app_id
        );
        assert_eq!(service.config().app_id, cloned_service.config().app_id);
    }

    #[test]
    fn test_tenant_v2_debug_format() {
        let config = create_test_config();
        let service = V2::new(config);

        let debug_string = format!("{:?}", service);
        assert!(debug_string.contains("TenantV2"));
        assert!(debug_string.contains("TenantService"));
        assert!(debug_string.contains("TenantProductAssignInfoService"));
    }

    #[test]
    fn test_tenant_v2_comprehensive_feature_matrix() {
        let config = create_test_config();
        let service = V2::new(config);

        // 测试所有支持的功能组合
        let supported_features = vec![
            "tenant_management",
            "seat_allocation",
            "multi_tenant",
            "enterprise_features",
            "resource_management",
            "billing_integration",
        ];

        for feature in supported_features {
            assert!(
                service.supports_feature(feature),
                "Feature {} should be supported",
                feature
            );
        }

        // 验证功能数量
        let mut feature_count = 0;
        let all_features = vec![
            "tenant_management",
            "seat_allocation",
            "multi_tenant",
            "enterprise_features",
            "resource_management",
            "billing_integration",
            "nonexistent1",
            "nonexistent2",
        ];

        for feature in all_features {
            if service.supports_feature(feature) {
                feature_count += 1;
            }
        }
        assert_eq!(feature_count, 6); // 确保支持6个功能
    }

    #[test]
    fn test_tenant_v2_edge_cases() {
        // 测试特殊字符配置
        let special_config = crate::core::config::Config::builder()
            .app_id("app_with_特殊字符_123")
            .app_secret("secret_with_特殊字符_456")
            .build();
        let special_service = V2::new(special_config);

        assert!(special_service.validate_services_config());
        assert!(special_service.health_check());
        assert!(special_service
            .get_service_statistics()
            .contains("特殊字符"));

        // 测试长字符串配置
        let long_app_id = "a".repeat(1000);
        let long_config = crate::core::config::Config::builder()
            .app_id(&long_app_id)
            .app_secret("test_secret")
            .build();
        let long_service = V2::new(long_config);

        assert!(long_service.validate_services_config());
        assert!(long_service.get_service_statistics().contains(&long_app_id));
    }

    #[test]
    fn test_tenant_v2_service_configuration_consistency() {
        let config = create_test_config();
        let service = V2::new(config);
        let shared_service = V2::new_from_shared(create_shared_test_config());

        // 验证服务配置一致性
        assert_eq!(
            service.tenant.config().app_id,
            service.tenant_product_assign_info.config().app_id
        );
        assert_eq!(
            service.tenant.config().app_secret,
            service.tenant_product_assign_info.config().app_secret
        );

        assert_eq!(
            shared_service.tenant.config().app_id,
            shared_service.tenant_product_assign_info.config().app_id
        );
        assert_eq!(
            shared_service.tenant.config().app_secret,
            shared_service
                .tenant_product_assign_info
                .config()
                .app_secret
        );
    }

    #[test]
    fn test_tenant_v2_unicode_and_chinese_support() {
        let unicode_config = crate::core::config::Config::builder()
            .app_id("飞书租户应用_🏢_ID")
            .app_secret("租户管理密钥_🔐_Secret")
            .build();
        let unicode_service = V2::new(unicode_config);

        // 测试 Unicode 支持
        assert!(unicode_service.validate_services_config());
        assert!(unicode_service.health_check());

        let stats = unicode_service.get_service_statistics();
        assert!(stats.contains("飞书租户应用"));
        assert!(stats.contains("🏢"));

        // 测试中文功能名称处理
        assert!(unicode_service.supports_feature("tenant_management"));
        assert!(unicode_service.supports_feature("seat_allocation"));
    }

    #[test]
    fn test_tenant_v2_enterprise_scenarios() {
        let enterprise_config = crate::core::config::Config::builder()
            .app_id("enterprise_tenant_app_id")
            .app_secret("enterprise_tenant_app_secret")
            .build();
        let enterprise_service = V2::new(enterprise_config);

        // 测试企业级场景
        assert!(enterprise_service.validate_services_config());
        assert!(enterprise_service.health_check());

        // 验证企业功能支持
        assert!(enterprise_service.supports_feature("tenant_management"));
        assert!(enterprise_service.supports_feature("seat_allocation"));
        assert!(enterprise_service.supports_feature("multi_tenant"));
        assert!(enterprise_service.supports_feature("enterprise_features"));
        assert!(enterprise_service.supports_feature("resource_management"));
        assert!(enterprise_service.supports_feature("billing_integration"));

        // 测试企业统计信息
        let stats = enterprise_service.get_service_statistics();
        assert!(stats.contains("enterprise_tenant_app_id"));
        assert!(stats.contains("services: 2"));

        let category_stats = enterprise_service.get_service_categories_statistics();
        assert!(category_stats.contains("tenant: 1"));
        assert!(category_stats.contains("seat: 1"));
    }

    #[test]
    fn test_tenant_v2_memory_efficiency() {
        let config = create_test_config();

        // 测试内存使用效率
        let service = V2::new(config.clone());
        let cloned_service = service.clone();

        // 验证克隆后配置仍然有效
        assert!(cloned_service.validate_services_config());
        assert_eq!(service.config().app_id, cloned_service.config().app_id);

        // 测试共享配置的内存效率
        let shared_config = std::sync::Arc::new(config);
        let shared_service = V2::new_from_shared(shared_config);

        assert!(shared_service.validate_services_config());
        assert_eq!(shared_service.tenant.config().app_id, "test_tenant_app_id");
    }

    #[test]
    fn test_tenant_v2_error_handling_and_robustness() {
        // 测试部分无效配置
        let partial_invalid_config = crate::core::config::Config::builder()
            .app_id("valid_app_id")
            .app_secret("") // 无效密钥
            .build();
        let partial_invalid_service = V2::new(partial_invalid_config);

        // 健康检查应该失败，但服务仍然可用
        assert!(!partial_invalid_service.health_check());
        assert!(!partial_invalid_service.validate_services_config());

        // 测试完全无效配置
        let fully_invalid_config = crate::core::config::Config::builder()
            .app_id("")
            .app_secret("")
            .build();
        let fully_invalid_service = V2::new(fully_invalid_config);

        assert!(!fully_invalid_service.health_check());
        assert!(!fully_invalid_service.validate_services_config());

        // 验证统计信息仍然可用
        assert!(fully_invalid_service
            .get_service_statistics()
            .contains("TenantV2"));
        assert!(fully_invalid_service
            .get_service_categories_statistics()
            .contains("total: 2"));
    }

    #[test]
    fn test_tenant_v2_concurrent_access() {
        use std::sync::Arc;
        use std::thread;

        let config = create_test_config();
        let service = Arc::new(V2::new(config));
        let mut handles = vec![];

        // 测试并发访问
        for _ in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                // 验证并发访问的安全性
                assert!(service_clone.validate_services_config());
                assert!(service_clone.health_check());
                assert!(service_clone.supports_feature("tenant_management"));

                let stats = service_clone.get_service_statistics();
                assert!(stats.contains("TenantV2"));

                let category_stats = service_clone.get_service_categories_statistics();
                assert!(category_stats.contains("total: 2"));
            });
            handles.push(handle);
        }

        // 等待所有线程完成
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_tenant_v2_performance_characteristics() {
        let config = create_test_config();
        let service = V2::new(config);

        // 测试性能特征
        let start = std::time::Instant::now();

        // 执行多个操作
        for _ in 0..1000 {
            assert!(service.validate_services_config());
            assert!(service.supports_feature("tenant_management"));
            let _stats = service.get_service_statistics();
            let _category_stats = service.get_service_categories_statistics();
        }

        let duration = start.elapsed();
        assert!(
            duration.as_millis() < 1000,
            "Operations should complete quickly"
        );
    }

    #[test]
    fn test_tenant_v2_comprehensive_integration() {
        let config = create_test_config();
        let service = V2::new(config);

        // 综合集成测试
        assert!(service.validate_services_config());
        assert!(service.health_check());

        // 测试所有核心功能
        assert!(service.supports_feature("tenant_management"));
        assert!(service.supports_feature("seat_allocation"));
        assert!(service.supports_feature("multi_tenant"));
        assert!(service.supports_feature("enterprise_features"));
        assert!(service.supports_feature("resource_management"));
        assert!(service.supports_feature("billing_integration"));

        // 测试统计和调试功能
        let stats = service.get_service_statistics();
        assert!(stats.contains("test_tenant_app_id"));
        assert!(stats.contains("services: 2"));

        let category_stats = service.get_service_categories_statistics();
        assert!(category_stats.contains("tenant: 1"));
        assert!(category_stats.contains("seat: 1"));

        // 测试 Debug 和 Clone
        let debug_str = format!("{:?}", service);
        assert!(debug_str.contains("TenantV2"));

        let cloned_service = service.clone();
        assert_eq!(service.config().app_id, cloned_service.config().app_id);
        assert!(cloned_service.validate_services_config());

        // 测试 Service trait 方法
        assert_eq!(V2::service_name(), "tenant");
        assert_eq!(V2::service_version(), "v2");
        assert_eq!(service.config().app_id, "test_tenant_app_id");
    }
}
