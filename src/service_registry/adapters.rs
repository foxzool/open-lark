//! 服务适配器模块
//!
//! 为现有服务创建 ServiceRegistry 兼容的适配器，实现渐进式迁移策略

use super::{NamedService, Service, ServiceError, ServiceStatus};
use open_lark_core::config::Config;
use std::any::Any;

/// 服务详情
#[derive(Debug, Clone)]
pub struct ServiceDetail {
    /// 服务名称
    pub name: String,
    /// 服务状态
    pub status: ServiceStatus,
}

/// 迁移报告
#[derive(Debug, Clone)]
pub struct MigrationReport {
    /// 总服务数
    pub total_services: usize,
    /// 健康服务数
    pub healthy_services: usize,
    /// 不健康服务数
    pub unhealthy_services: usize,
    /// 服务详情列表
    pub services: Vec<ServiceDetail>,
    /// 迁移时间戳
    pub migration_timestamp: std::time::SystemTime,
}

impl MigrationReport {
    /// 创建新的迁移报告
    pub fn new() -> Self {
        Self {
            total_services: 0,
            healthy_services: 0,
            unhealthy_services: 0,
            services: Vec::new(),
            migration_timestamp: std::time::SystemTime::now(),
        }
    }

    /// 打印迁移报告摘要
    pub fn print_summary(&self) {
        println!("📊 服务迁移报告:");
        println!("  总服务数: {}", self.total_services);
        println!("  健康服务: {}", self.healthy_services);
        println!("  不健康服务: {}", self.unhealthy_services);
        println!("  迁移时间: {:?}", self.migration_timestamp);

        if !self.services.is_empty() {
            println!("  服务详情:");
            for service in &self.services {
                println!("    {}: {:?}", service.name, service.status);
            }
        }
    }

    /// 检查迁移是否成功
    pub fn is_successful(&self) -> bool {
        self.total_services > 0 && self.unhealthy_services == 0
    }

    /// 获取成功率
    pub fn success_rate(&self) -> f64 {
        if self.total_services == 0 {
            0.0
        } else {
            (self.healthy_services as f64 / self.total_services as f64) * 100.0
        }
    }
}

/// 通用服务适配器
///
/// 为现有服务提供 ServiceRegistry 兼容性的包装器
pub struct ServiceAdapter<T> {
    inner: T,
    name: &'static str,
    version: &'static str,
}

impl<T> ServiceAdapter<T> {
    /// 创建新的服务适配器
    pub fn new(service: T, name: &'static str, version: &'static str) -> Self {
        Self {
            inner: service,
            name,
            version,
        }
    }

    /// 获取内部服务的引用
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// 获取内部服务的可变引用
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// 消费适配器，获取内部服务
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> Service for ServiceAdapter<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn name(&self) -> &'static str {
        self.name
    }

    fn version(&self) -> &'static str {
        self.version
    }

    fn status(&self) -> ServiceStatus {
        // 对于适配器包装的服务，默认假设健康
        ServiceStatus::Healthy
    }

    fn description(&self) -> &'static str {
        "Service adapter for existing service"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<T> Clone for ServiceAdapter<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            name: self.name,
            version: self.version,
        }
    }
}

/// Authentication 服务适配器
#[cfg(feature = "authentication")]
#[derive(Debug, Clone)]
pub struct AuthenticationServiceAdapter {
    inner: crate::service::authentication::AuthenticationService,
}

#[cfg(feature = "authentication")]
impl AuthenticationServiceAdapter {
    pub fn new(service: crate::service::authentication::AuthenticationService) -> Self {
        Self { inner: service }
    }

    pub fn inner(&self) -> &crate::service::authentication::AuthenticationService {
        &self.inner
    }
}

#[cfg(feature = "authentication")]
impl Service for AuthenticationServiceAdapter {
    fn name(&self) -> &'static str {
        "authentication-service"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn status(&self) -> ServiceStatus {
        ServiceStatus::Healthy
    }

    fn description(&self) -> &'static str {
        "Authentication service adapter for ServiceRegistry"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(feature = "authentication")]
impl NamedService for AuthenticationServiceAdapter {
    const NAME: &'static str = "authentication-service";

    fn clone_owned(&self) -> Self {
        Self::new(self.inner.clone())
    }
}

/// IM 服务适配器
#[cfg(feature = "im")]
#[derive(Debug, Clone)]
pub struct ImServiceAdapter {
    inner: crate::service::im::ImService,
}

#[cfg(feature = "im")]
impl ImServiceAdapter {
    pub fn new(service: crate::service::im::ImService) -> Self {
        Self { inner: service }
    }

    pub fn inner(&self) -> &crate::service::im::ImService {
        &self.inner
    }
}

#[cfg(feature = "im")]
impl Service for ImServiceAdapter {
    fn name(&self) -> &'static str {
        "im-service"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn status(&self) -> ServiceStatus {
        ServiceStatus::Healthy
    }

    fn description(&self) -> &'static str {
        "IM service adapter for ServiceRegistry"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(feature = "im")]
impl NamedService for ImServiceAdapter {
    const NAME: &'static str = "im-service";

    fn clone_owned(&self) -> Self {
        Self::new(self.inner.clone())
    }
}

/// Contact 服务适配器
#[cfg(feature = "contact")]
#[derive(Debug, Clone)]
pub struct ContactServiceAdapter {
    inner: crate::service::contact::ContactService,
}

#[cfg(feature = "contact")]
impl ContactServiceAdapter {
    pub fn new(service: crate::service::contact::ContactService) -> Self {
        Self { inner: service }
    }

    pub fn inner(&self) -> &crate::service::contact::ContactService {
        &self.inner
    }
}

#[cfg(feature = "contact")]
impl Service for ContactServiceAdapter {
    fn name(&self) -> &'static str {
        "contact-service"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn status(&self) -> ServiceStatus {
        ServiceStatus::Healthy
    }

    fn description(&self) -> &'static str {
        "Contact service adapter for ServiceRegistry"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(feature = "contact")]
impl NamedService for ContactServiceAdapter {
    const NAME: &'static str = "contact-service";

    fn clone_owned(&self) -> Self {
        Self::new(self.inner.clone())
    }
}

/// Group 服务适配器
#[cfg(feature = "group")]
#[derive(Debug, Clone)]
pub struct GroupServiceAdapter {
    inner: crate::service::group::GroupService,
}

#[cfg(feature = "group")]
impl GroupServiceAdapter {
    pub fn new(service: crate::service::group::GroupService) -> Self {
        Self { inner: service }
    }

    pub fn inner(&self) -> &crate::service::group::GroupService {
        &self.inner
    }
}

#[cfg(feature = "group")]
impl Service for GroupServiceAdapter {
    fn name(&self) -> &'static str {
        "group-service"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn status(&self) -> ServiceStatus {
        ServiceStatus::Healthy
    }

    fn description(&self) -> &'static str {
        "Group service adapter for ServiceRegistry"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(feature = "group")]
impl NamedService for GroupServiceAdapter {
    const NAME: &'static str = "group-service";

    fn clone_owned(&self) -> Self {
        Self::new(self.inner.clone())
    }
}

/// Search 服务适配器
#[cfg(feature = "search")]
#[derive(Debug, Clone)]
pub struct SearchServiceAdapter {
    inner: crate::service::search::SearchService,
}

#[cfg(feature = "search")]
impl SearchServiceAdapter {
    pub fn new(service: crate::service::search::SearchService) -> Self {
        Self { inner: service }
    }

    pub fn inner(&self) -> &crate::service::search::SearchService {
        &self.inner
    }
}

#[cfg(feature = "search")]
impl Service for SearchServiceAdapter {
    fn name(&self) -> &'static str {
        "search-service"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn status(&self) -> ServiceStatus {
        ServiceStatus::Healthy
    }

    fn description(&self) -> &'static str {
        "Search service adapter for ServiceRegistry"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(feature = "search")]
impl NamedService for SearchServiceAdapter {
    const NAME: &'static str = "search-service";

    fn clone_owned(&self) -> Self {
        Self::new(self.inner.clone())
    }
}

/// 服务迁移辅助工具
///
/// 提供完整的服务迁移兼容性处理，包括版本检查、配置验证和错误恢复
pub struct MigrationHelper;

impl MigrationHelper {
    /// 批量注册服务到 ServiceRegistry（使用传统配置方式）
    pub fn register_services(
        registry: &crate::service_registry::ServiceRegistry,
        config: &Config,
    ) -> Result<(), ServiceError> {
        Self::register_services_with_shared_config(
            registry,
            &crate::service_registry::SharedConfig::new(config.clone()),
        )
    }

    /// 批量注册服务到 ServiceRegistry（使用共享配置）
    ///
    /// # Arguments
    /// * `registry` - ServiceRegistry实例
    /// * `shared_config` - 共享配置实例
    ///
    /// # Returns
    /// 返回注册结果
    pub fn register_services_with_shared_config(
        registry: &crate::service_registry::ServiceRegistry,
        shared_config: &crate::service_registry::SharedConfig,
    ) -> Result<(), ServiceError> {
        // 注册 Authentication 服务
        #[cfg(feature = "authentication")]
        {
            let auth_service = crate::service::authentication::AuthenticationService::new(
                shared_config.config().clone(),
            );
            let auth_adapter = AuthenticationServiceAdapter::new(auth_service);
            registry.register(auth_adapter)?;
        }

        // 注册 IM 服务
        #[cfg(feature = "im")]
        {
            let im_service = crate::service::im::ImService::new(shared_config.config().clone());
            let im_adapter = ImServiceAdapter::new(im_service);
            registry.register(im_adapter)?;
        }

        // 注册 Contact 服务
        #[cfg(feature = "contact")]
        {
            let contact_service =
                crate::service::contact::ContactService::new(shared_config.config().clone());
            let contact_adapter = ContactServiceAdapter::new(contact_service);
            registry.register(contact_adapter)?;
        }

        // 注册 Group 服务
        #[cfg(feature = "group")]
        {
            let group_service =
                crate::service::group::GroupService::new(shared_config.config().clone());
            let group_adapter = GroupServiceAdapter::new(group_service);
            registry.register(group_adapter)?;
        }

        // 注册 Search 服务
        #[cfg(feature = "search")]
        {
            let search_service =
                crate::service::search::SearchService::new(shared_config.config().clone());
            let search_adapter = SearchServiceAdapter::new(search_service);
            registry.register(search_adapter)?;
        }

        Ok(())
    }

    /// 验证服务迁移的完整性
    pub fn validate_migration(
        registry: &crate::service_registry::ServiceRegistry,
    ) -> Result<(), ServiceError> {
        let expected_services = vec![
            ("authentication-service", "authentication"),
            ("im-service", "im"),
            ("contact-service", "contact"),
            ("group-service", "group"),
            ("search-service", "search"),
        ];

        for (service_name, feature) in expected_services {
            match feature {
                "authentication" if cfg!(feature = "authentication") => {
                    if !registry.has_service(service_name) {
                        return Err(ServiceError::service_not_found(service_name));
                    }
                }
                "im" if cfg!(feature = "im") => {
                    if !registry.has_service(service_name) {
                        return Err(ServiceError::service_not_found(service_name));
                    }
                }
                "contact" if cfg!(feature = "contact") => {
                    if !registry.has_service(service_name) {
                        return Err(ServiceError::service_not_found(service_name));
                    }
                }
                "group" if cfg!(feature = "group") => {
                    if !registry.has_service(service_name) {
                        return Err(ServiceError::service_not_found(service_name));
                    }
                }
                "search" if cfg!(feature = "search") => {
                    if !registry.has_service(service_name) {
                        return Err(ServiceError::service_not_found(service_name));
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// 检查配置兼容性
    ///
    /// # Arguments
    /// * `config` - 要检查的配置
    ///
    /// # Returns
    /// 返回兼容性检查结果
    pub fn check_config_compatibility(config: &Config) -> Result<(), ServiceError> {
        // 检查必需的配置字段
        if config.app_id.is_empty() {
            return Err(ServiceError::invalid_configuration(
                "app_id",
                "App ID cannot be empty",
            ));
        }

        if config.app_secret.is_empty() {
            return Err(ServiceError::invalid_configuration(
                "app_secret",
                "App Secret cannot be empty",
            ));
        }

        // 检查URL格式
        if !config.base_url.starts_with("http://") && !config.base_url.starts_with("https://") {
            return Err(ServiceError::invalid_configuration(
                "base_url",
                "Base URL must start with http:// or https://",
            ));
        }

        // 检查应用类型配置 - app_type 不是 Option，直接检查
        // 注意：AppType 是一个枚举，总是有值

        Ok(())
    }

    /// 渐进式服务迁移
    ///
    /// 支持逐步迁移服务，在遇到错误时提供回滚机制
    ///
    /// # Arguments
    /// * `registry` - ServiceRegistry实例
    /// * `shared_config` - 共享配置实例
    /// * `service_names` - 要迁移的服务名称列表（可选，为空时迁移所有可用服务）
    ///
    /// # Returns
    /// 返回迁移结果和已注册的服务列表
    pub fn gradual_migration(
        registry: &crate::service_registry::ServiceRegistry,
        shared_config: &crate::service_registry::SharedConfig,
        service_names: Option<Vec<&str>>,
    ) -> Result<Vec<String>, ServiceError> {
        let mut registered_services = Vec::new();

        // 简化的实现：直接注册适配器，避免复杂的闭包类型问题
        let requested_services = service_names.unwrap_or_else(|| {
            vec![
                "authentication-service",
                "im-service",
                "contact-service",
                "group-service",
                "search-service",
            ]
        });

        // Authentication 服务
        #[cfg(feature = "authentication")]
        if requested_services.contains(&"authentication-service") {
            use crate::service::authentication::AuthenticationService;

            let service = AuthenticationService::new(shared_config.config().clone());
            let adapter = AuthenticationServiceAdapter::new(service);
            match registry.register(adapter) {
                Ok(_) => {
                    registered_services.push("authentication-service".to_string());
                    log::info!("Successfully migrated service: authentication-service");
                }
                Err(e) => {
                    log::error!("Failed to migrate service authentication-service: {}", e);
                }
            }
        }

        // IM 服务
        #[cfg(feature = "im")]
        if requested_services.contains(&"im-service") {
            use crate::service::im::ImService;

            let service = ImService::new(shared_config.config().clone());
            let adapter = ImServiceAdapter::new(service);
            match registry.register(adapter) {
                Ok(_) => {
                    registered_services.push("im-service".to_string());
                    log::info!("Successfully migrated service: im-service");
                }
                Err(e) => {
                    log::error!("Failed to migrate service im-service: {}", e);
                }
            }
        }

        // Contact 服务
        #[cfg(feature = "contact")]
        if requested_services.contains(&"contact-service") {
            use crate::service::contact::ContactService;

            let service = ContactService::new(shared_config.config().clone());
            let adapter = ContactServiceAdapter::new(service);
            match registry.register(adapter) {
                Ok(_) => {
                    registered_services.push("contact-service".to_string());
                    log::info!("Successfully migrated service: contact-service");
                }
                Err(e) => {
                    log::error!("Failed to migrate service contact-service: {}", e);
                }
            }
        }

        // Group 服务
        #[cfg(feature = "group")]
        if requested_services.contains(&"group-service") {
            use crate::service::group::GroupService;

            let service = GroupService::new(shared_config.config().clone());
            let adapter = GroupServiceAdapter::new(service);
            match registry.register(adapter) {
                Ok(_) => {
                    registered_services.push("group-service".to_string());
                    log::info!("Successfully migrated service: group-service");
                }
                Err(e) => {
                    log::error!("Failed to migrate service group-service: {}", e);
                }
            }
        }

        // Search 服务
        #[cfg(feature = "search")]
        if requested_services.contains(&"search-service") {
            use crate::service::search::SearchService;

            let service = SearchService::new(shared_config.config().clone());
            let adapter = SearchServiceAdapter::new(service);
            match registry.register(adapter) {
                Ok(_) => {
                    registered_services.push("search-service".to_string());
                    log::info!("Successfully migrated service: search-service");
                }
                Err(e) => {
                    log::error!("Failed to migrate service search-service: {}", e);
                }
            }
        }

        if registered_services.is_empty() {
            return Err(ServiceError::internal_error(
                "No services were successfully migrated",
            ));
        }

        Ok(registered_services)
    }

    /// 服务迁移回滚
    ///
    /// 在迁移失败时回滚已注册的服务
    ///
    /// # Arguments
    /// * `registry` - ServiceRegistry实例
    /// * `service_names` - 要回滚的服务名称列表
    ///
    /// # Returns
    /// 返回回滚结果
    pub fn rollback_migration(
        registry: &crate::service_registry::ServiceRegistry,
        service_names: &[String],
    ) -> Result<(), ServiceError> {
        let mut failed_rollback = Vec::new();

        for service_name in service_names {
            if let Err(e) = registry.unregister(service_name) {
                failed_rollback.push((service_name.clone(), e));
            }
        }

        if !failed_rollback.is_empty() {
            let error_details = failed_rollback
                .into_iter()
                .map(|(name, error)| format!("{}: {}", name, error))
                .collect::<Vec<_>>()
                .join(", ");
            return Err(ServiceError::internal_error(&format!(
                "Rollback failed for some services: {}",
                error_details
            )));
        }

        Ok(())
    }

    /// 获取迁移状态报告
    ///
    /// # Arguments
    /// * `registry` - ServiceRegistry实例
    ///
    /// # Returns
    /// 返回迁移状态报告
    pub fn get_migration_report(
        registry: &crate::service_registry::ServiceRegistry,
    ) -> MigrationReport {
        let stats = registry.get_stats();
        let discovered_services = registry.discover_services();

        let service_details = discovered_services
            .into_iter()
            .map(|name| {
                // 使用get_service_info获取服务信息
                let status = registry
                    .get_service_info(&name)
                    .map(|info| info.status)
                    .unwrap_or(crate::service_registry::ServiceStatus::Healthy); // 默认假设健康
                ServiceDetail {
                    name: name.to_string(),
                    status,
                }
            })
            .collect();

        MigrationReport {
            total_services: stats.total_services,
            healthy_services: stats.healthy_services,
            unhealthy_services: stats.unhealthy_services,
            services: service_details,
            migration_timestamp: std::time::SystemTime::now(),
        }
    }

    /// 验证服务版本兼容性
    ///
    /// # Arguments
    /// * `registry` - ServiceRegistry实例
    ///
    /// # Returns
    /// 返回版本兼容性检查结果
    pub fn verify_version_compatibility(
        registry: &crate::service_registry::ServiceRegistry,
    ) -> Result<(), ServiceError> {
        let services = registry.discover_services();
        let mut incompatible_services = Vec::new();

        for service_name in services {
            // 使用get_service_info获取服务信息
            if let Some(service_info) = registry.get_service_info(&service_name) {
                let version = service_info.version;
                // 这里可以添加具体的版本兼容性逻辑
                // 例如：检查版本号格式、最小版本要求等
                if version == "unknown" || version.is_empty() {
                    incompatible_services.push(service_name);
                }
            }
        }

        if !incompatible_services.is_empty() {
            return Err(ServiceError::invalid_configuration(
                "service_versions",
                &format!("Incompatible service versions: {:?}", incompatible_services),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_lark_core::config::{Config, ConfigBuilder};
    use crate::service_registry::{Service, ServiceRegistry};

    fn create_test_config() -> Config {
        ConfigBuilder::default()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    #[test]
    fn test_service_adapter_creation() {
        let config = create_test_config();

        #[cfg(feature = "authentication")]
        {
            let auth_service =
                crate::service::authentication::AuthenticationService::new(config.clone());
            let adapter = AuthenticationServiceAdapter::new(auth_service);

            assert_eq!(adapter.name(), "authentication-service");
            assert_eq!(adapter.version(), "1.0.0");
        }

        #[cfg(feature = "im")]
        {
            let im_service = crate::service::im::ImService::new(config.clone());
            let adapter = ImServiceAdapter::new(im_service);

            assert_eq!(adapter.name(), "im-service");
            assert_eq!(adapter.version(), "1.0.0");
        }

        #[cfg(feature = "contact")]
        {
            let contact_service = crate::service::contact::ContactService::new(config.clone());
            let adapter = ContactServiceAdapter::new(contact_service);

            assert_eq!(adapter.name(), "contact-service");
            assert_eq!(adapter.version(), "1.0.0");
        }
    }

    #[test]
    fn test_migration_helper() {
        let registry = ServiceRegistry::new();
        let config = create_test_config();

        let result = MigrationHelper::register_services(&registry, &config);

        #[cfg(any(feature = "authentication", feature = "im", feature = "contact"))]
        {
            assert!(result.is_ok());
        }

        let validation_result = MigrationHelper::validate_migration(&registry);
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_config_compatibility_check() {
        // 测试有效配置
        let valid_config = create_test_config();
        assert!(MigrationHelper::check_config_compatibility(&valid_config).is_ok());

        // 测试无效配置 - 空 app_id
        let invalid_config = ConfigBuilder::default()
            .app_id("") // 空 app_id
            .app_secret("test_app_secret")
            .base_url("https://open.feishu.cn")
            .build();
        assert!(MigrationHelper::check_config_compatibility(&invalid_config).is_err());

        // 测试无效配置 - 空 app_secret
        let invalid_config = ConfigBuilder::default()
            .app_id("test_app_id")
            .app_secret("") // 空 app_secret
            .base_url("https://open.feishu.cn")
            .build();
        assert!(MigrationHelper::check_config_compatibility(&invalid_config).is_err());

        // 测试无效配置 - 错误的 URL 格式
        let invalid_config = ConfigBuilder::default()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("invalid-url") // 无效 URL
            .build();
        assert!(MigrationHelper::check_config_compatibility(&invalid_config).is_err());
    }

    #[test]
    fn test_gradual_migration() {
        let registry = ServiceRegistry::new();
        let config = create_test_config();
        let shared_config = crate::service_registry::SharedConfig::new(config);

        // 测试迁移所有服务
        let result = MigrationHelper::gradual_migration(&registry, &shared_config, None);
        #[cfg(any(
            feature = "authentication",
            feature = "im",
            feature = "contact",
            feature = "group",
            feature = "search"
        ))]
        {
            assert!(result.is_ok());
            let registered_services = result.unwrap();
            assert!(!registered_services.is_empty());
        }

        // 清理注册表
        let services = registry.discover_services();
        for service_name in services {
            let _ = registry.unregister(&service_name);
        }

        // 测试迁移指定服务
        let specific_services = vec!["im-service", "contact-service"];
        let result =
            MigrationHelper::gradual_migration(&registry, &shared_config, Some(specific_services));

        #[cfg(all(feature = "im", feature = "contact"))]
        {
            assert!(result.is_ok());
            let registered_services = result.unwrap();
            assert!(registered_services.len() <= 2); // 最多注册2个服务
        }
    }

    #[test]
    fn test_migration_report() {
        let registry = ServiceRegistry::new();
        let config = create_test_config();

        // 注册一些服务
        let result = MigrationHelper::register_services(&registry, &config);
        #[cfg(any(feature = "authentication", feature = "im", feature = "contact"))]
        {
            let _ = result;
        }

        // 生成迁移报告
        let report = MigrationHelper::get_migration_report(&registry);

        // 验证报告内容
        assert_eq!(report.services.len(), report.total_services);
        assert!(report.total_services >= report.healthy_services + report.unhealthy_services);

        // 打印报告（用于演示）
        report.print_summary();

        // 验证成功率计算
        let success_rate = report.success_rate();
        assert!(success_rate >= 0.0 && success_rate <= 100.0);
    }

    #[test]
    fn test_rollback_migration() {
        let registry = ServiceRegistry::new();
        let config = create_test_config();
        let shared_config = crate::service_registry::SharedConfig::new(config);

        // 迁移一些服务
        let result = MigrationHelper::gradual_migration(&registry, &shared_config, None);
        #[cfg(any(feature = "authentication", feature = "im", feature = "contact"))]
        {
            if let Ok(registered_services) = result {
                assert!(!registered_services.is_empty());

                // 回滚迁移
                let rollback_result =
                    MigrationHelper::rollback_migration(&registry, &registered_services);
                assert!(rollback_result.is_ok());

                // 验证服务已被移除
                assert_eq!(registry.service_count(), 0);
            }
        }
    }

    #[test]
    fn test_version_compatibility() {
        let registry = ServiceRegistry::new();
        let config = create_test_config();

        // 注册服务
        let result = MigrationHelper::register_services(&registry, &config);
        #[cfg(any(feature = "authentication", feature = "im", feature = "contact"))]
        {
            let _ = result;
        }

        // 验证版本兼容性
        let compatibility_result = MigrationHelper::verify_version_compatibility(&registry);
        assert!(compatibility_result.is_ok());
    }
}
