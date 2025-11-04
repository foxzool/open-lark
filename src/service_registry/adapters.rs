//! 服务适配器模块
//!
//! 为现有服务创建 ServiceRegistry 兼容的适配器，实现渐进式迁移策略

use std::any::Any;
use crate::core::config::Config;
use super::{Service, NamedService, ServiceStatus, ServiceError};

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

/// 服务迁移辅助工具
pub struct MigrationHelper;

impl MigrationHelper {
    /// 批量注册服务到 ServiceRegistry
    pub fn register_services(
        registry: &crate::service_registry::ServiceRegistry,
        config: &Config,
    ) -> Result<(), ServiceError> {
        // 注册 Authentication 服务
        #[cfg(feature = "authentication")]
        {
            let auth_service = crate::service::authentication::AuthenticationService::new(config.clone());
            let auth_adapter = AuthenticationServiceAdapter::new(auth_service);
            registry.register(auth_adapter)?;
        }

        // 注册 IM 服务
        #[cfg(feature = "im")]
        {
            let im_service = crate::service::im::ImService::new(config.clone());
            let im_adapter = ImServiceAdapter::new(im_service);
            registry.register(im_adapter)?;
        }

        // 注册 Contact 服务
        #[cfg(feature = "contact")]
        {
            let contact_service = crate::service::contact::ContactService::new(config.clone());
            let contact_adapter = ContactServiceAdapter::new(contact_service);
            registry.register(contact_adapter)?;
        }

        Ok(())
    }

    /// 验证服务迁移的完整性
    pub fn validate_migration(registry: &crate::service_registry::ServiceRegistry) -> Result<(), ServiceError> {
        let expected_services = vec![
            ("authentication-service", "authentication"),
            ("im-service", "im"),
            ("contact-service", "contact"),
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
                _ => {}
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::{Config, ConfigBuilder};
    use crate::service_registry::{ServiceRegistry, Service};

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
            let auth_service = crate::service::authentication::AuthenticationService::new(config.clone());
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
}