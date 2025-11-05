//! ServiceRegistry - 飞书SDK服务注册与发现系统
//!
//! 提供类型安全的服务注册、发现和管理功能，支持动态服务加载和生命周期管理。

pub mod adapters;
pub mod advanced_compatibility;
pub mod benchmark;
pub mod builder;
pub mod compatibility;
pub mod dependency_analyzer;
pub mod error;
pub mod metadata;
pub mod migration;
pub mod migration_tests;
pub mod registry;
pub mod service;
pub mod shared_config;
pub mod shared_config_tests;
pub mod tests;

// 重新导出核心公共接口
#[cfg(feature = "authentication")]
pub use adapters::AuthenticationServiceAdapter;
#[cfg(feature = "contact")]
pub use adapters::ContactServiceAdapter;
#[cfg(feature = "group")]
pub use adapters::GroupServiceAdapter;
#[cfg(feature = "im")]
pub use adapters::ImServiceAdapter;
pub use adapters::MigrationHelper;
#[cfg(feature = "search")]
pub use adapters::SearchServiceAdapter;
pub use advanced_compatibility::{
    AdvancedCompatibilityAnalyzer, CompatibilityAnalysisReport, RecommendedStrategy,
    ServiceCompatibilityAnalysis, ServiceRisk, ServiceRiskType,
};
pub use builder::{ServiceBuilder, TypeErasedServiceBuilder};
pub use compatibility::{
    CompatibilityChecker, CompatibilityConfig, CompatibilityHandler, CompatibilityReport,
    CompatibilityResult, ServiceVersion,
};
pub use dependency_analyzer::{
    CircularDependencySeverity, CriticalPathType, DependencyAnalysisReport, DependencyAnalyzer,
    DependencyGraphData, MigrationImpactAnalysis, RecommendationCategory, RecommendationPriority,
    RiskLevel,
};
pub use error::{ServiceError, ServiceResult};
pub use metadata::ServiceMetadata;
pub use migration::{
    AdvancedMigrationHelper, MigrationPlan, MigrationResult, MigrationStatus, MigrationStrategy,
    MigrationTask, ServiceMigrationReport,
};
pub use registry::ServiceRegistry;
pub use service::{NamedService, Service, ServiceStatus};
pub use shared_config::{ConfigUsageStats, SharedConfig, SharedConfigFactory};

/// ServiceRegistry预配置，提供常用的默认配置
pub struct ServiceRegistryBuilder {
    config: RegistryConfig,
}

impl ServiceRegistryBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self {
            config: RegistryConfig::default(),
        }
    }

    /// 设置最大服务数量限制
    pub fn with_max_services(mut self, max: usize) -> Self {
        self.config.max_services = max;
        self
    }

    /// 启用服务健康检查
    pub fn with_health_check(mut self, enabled: bool) -> Self {
        self.config.enable_health_check = enabled;
        self
    }

    /// 设置健康检查间隔
    pub fn with_health_check_interval(mut self, interval: std::time::Duration) -> Self {
        self.config.health_check_interval = interval;
        self
    }

    /// 构建ServiceRegistry实例
    pub fn build(self) -> ServiceRegistry {
        ServiceRegistry::with_config(self.config)
    }
}

impl Default for ServiceRegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// ServiceRegistry配置
#[derive(Debug, Clone)]
pub struct RegistryConfig {
    /// 最大允许注册的服务数量
    pub max_services: usize,
    /// 是否启用健康检查
    pub enable_health_check: bool,
    /// 健康检查间隔
    pub health_check_interval: std::time::Duration,
    /// 是否启用服务缓存
    pub enable_cache: bool,
    /// 缓存过期时间
    pub cache_ttl: std::time::Duration,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            max_services: 1000,
            enable_health_check: true,
            health_check_interval: std::time::Duration::from_secs(30),
            enable_cache: true,
            cache_ttl: std::time::Duration::from_secs(300),
        }
    }
}
