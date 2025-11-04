//! ServiceRegistry核心实现

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, PoisonError, RwLock};

use super::{
    builder::{ServiceBuilder, TypeErasedServiceBuilder},
    error::ServiceError,
    metadata::ServiceMetadata,
    service::{NamedService, Service, ServiceInfo, ServiceStatus},
    RegistryConfig,
};

/// 线程安全的服务注册表
pub struct ServiceRegistry {
    /// 服务存储
    services: Arc<RwLock<HashMap<&'static str, Arc<dyn Service>>>>,
    /// 服务构建器存储
    builders: Arc<
        RwLock<HashMap<&'static str, Box<dyn ServiceBuilder<Output = dyn Service> + Send + Sync>>>,
    >,
    /// 服务元数据
    metadata: Arc<RwLock<ServiceMetadata>>,
    /// 配置
    config: RegistryConfig,
}

impl ServiceRegistry {
    /// 创建新的ServiceRegistry实例
    pub fn new() -> Self {
        Self::with_config(RegistryConfig::default())
    }

    /// 使用自定义配置创建ServiceRegistry实例
    pub fn with_config(config: RegistryConfig) -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            builders: Arc::new(RwLock::new(HashMap::new())),
            metadata: Arc::new(RwLock::new(ServiceMetadata::new())),
            config,
        }
    }

    /// 注册服务实例
    pub fn register<S>(&self, service: S) -> Result<(), ServiceError>
    where
        S: Service,
    {
        self.validate_capacity()?;

        let name = service.name();
        let service_arc = Arc::new(service);

        // 检查服务是否已存在
        {
            let services = self
                .services
                .read()
                .map_err(|_| ServiceError::internal_error("Poisoned lock"))?;
            if services.contains_key(name) {
                return Err(ServiceError::service_already_exists(name));
            }
        }

        // 注册服务
        {
            let mut services = self
                .services
                .write()
                .map_err(|_| ServiceError::internal_error("Poisoned lock"))?;
            services.insert(name, service_arc);
        }

        // 更新元数据
        {
            let services = self
                .services
                .read()
                .map_err(|_| ServiceError::internal_error("Poisoned lock"))?;
            if let Some(service) = services.get(name) {
                let info = ServiceInfo::new(service.as_ref());
                let mut metadata = self
                    .metadata
                    .write()
                    .map_err(|_| ServiceError::internal_error("Poisoned lock"))?;
                metadata.register_service(info);
            }
        }

        Ok(())
    }

    /// 注册服务构建器（接受类型擦除的构建器）
    pub fn register_builder(&self, builder: TypeErasedServiceBuilder) -> Result<(), ServiceError> {
        let name = builder.name();

        // 检查构建器是否已存在
        {
            let builders = self
                .builders
                .read()
                .map_err(|_| ServiceError::internal_error("Poisoned lock"))?;
            if builders.contains_key(name) {
                return Err(ServiceError::service_already_exists(name));
            }
        }

        // 注册构建器
        {
            let mut builders = self
                .builders
                .write()
                .map_err(|_| ServiceError::internal_error("Poisoned lock"))?;
            builders.insert(
                name,
                Box::new(builder) as Box<dyn ServiceBuilder<Output = dyn Service> + Send + Sync>,
            );
        }

        Ok(())
    }

    /// 通过构建器创建并注册服务
    pub fn build_and_register_service(&self, builder_name: &str) -> Result<(), ServiceError> {
        // 构建服务（需要从锁中取出构建器来构建）
        let service_box = {
            let builders = self
                .builders
                .read()
                .map_err(|_| ServiceError::internal_error("Poisoned lock"))?;
            let builder = builders.get(builder_name).ok_or_else(|| {
                ServiceError::service_not_found(&format!("Builder '{}' not found", builder_name))
            })?;

            // 验证构建器
            builder.validate()?;

            // 构建服务
            builder.build()?
        };

        // 将Box<dyn Service>转换为Arc<dyn Service>
        let service: Arc<dyn Service> = Arc::from(service_box);
        let service_name = service.name();

        // 注册服务
        self.validate_capacity()?;

        // 检查服务是否已存在
        {
            let services = self
                .services
                .read()
                .map_err(|_| ServiceError::internal_error("Poisoned lock"))?;
            if services.contains_key(service_name) {
                return Err(ServiceError::service_already_exists(service_name));
            }
        }

        // 注册服务
        {
            let mut services = self
                .services
                .write()
                .map_err(|_| ServiceError::internal_error("Poisoned lock"))?;
            services.insert(service_name, service.clone());
        }

        // 更新元数据
        {
            let info = ServiceInfo::new(service.as_ref());
            let mut metadata = self
                .metadata
                .write()
                .map_err(|_| ServiceError::internal_error("Poisoned lock"))?;
            metadata.register_service(info);
        }

        Ok(())
    }

    /// 获取服务实例（类型安全）
    pub fn get<S>(&self) -> Result<Arc<S>, ServiceError>
    where
        S: Service + NamedService,
    {
        let services = self
            .services
            .read()
            .map_err(|_| ServiceError::internal_error("Poisoned lock"))?;

        let service_name = S::name_static().ok_or_else(|| {
            ServiceError::invalid_configuration("service_name", "missing static name")
        })?;

        let entry = services
            .get(service_name)
            .ok_or_else(|| ServiceError::service_not_found(service_name))?;

        let concrete = entry.as_any().downcast_ref::<S>().ok_or_else(|| {
            ServiceError::type_mismatch(std::any::type_name::<S>(), std::any::type_name::<()>())
        })?;

        // 检查服务是否可用
        if !concrete.is_available() {
            return Err(ServiceError::service_unavailable(
                service_name,
                "Service is not available",
            ));
        }

        Ok(Arc::new(concrete.clone_owned()))
    }

    /// 通过名称获取服务（动态）
    pub fn get_by_name(&self, name: &str) -> Result<Arc<dyn Service>, ServiceError> {
        let services = self
            .services
            .read()
            .map_err(|_| ServiceError::internal_error("Poisoned lock"))?;

        // 首先尝试静态名称
        if let Some(service) = services.get(name) {
            if service.is_available() {
                return Ok(Arc::clone(service));
            } else {
                return Err(ServiceError::service_unavailable(
                    name,
                    "Service is not available",
                ));
            }
        }

        // 如果没有找到，返回错误
        Err(ServiceError::service_not_found(name))
    }

    /// 发现所有已注册的服务
    pub fn discover_services(&self) -> Vec<&'static str> {
        let services = self.services.read().unwrap_or_else(|_| {
            // 如果锁被污染，返回空列表
            std::process::abort();
        });
        services.keys().cloned().collect()
    }

    /// 获取服务信息
    pub fn get_service_info(&self, name: &str) -> Option<ServiceInfo> {
        let metadata = self.metadata.read().ok()?;
        metadata.get_service(name).cloned()
    }

    /// 获取所有服务信息
    pub fn get_all_services_info(&self) -> Vec<ServiceInfo> {
        let metadata = self.metadata.read().unwrap_or_else(|_| {
            std::process::abort();
        });
        metadata.get_all_services().values().cloned().collect()
    }

    /// 注销服务
    pub fn unregister(&self, name: &str) -> Result<(), ServiceError> {
        // 从服务存储中移除
        {
            let mut services = self
                .services
                .write()
                .map_err(|_| ServiceError::internal_error("Poisoned lock"))?;
            services
                .remove(name)
                .ok_or_else(|| ServiceError::service_not_found(name))?;
        }

        // 从元数据中移除
        {
            let mut metadata = self
                .metadata
                .write()
                .map_err(|_| ServiceError::internal_error("Poisoned lock"))?;
            metadata.remove_service(name);
        }

        Ok(())
    }

    /// 检查服务是否存在
    pub fn has_service(&self, name: &str) -> bool {
        let services = self.services.read().unwrap_or_else(|_| {
            std::process::abort();
        });
        services.contains_key(name)
    }

    /// 获取注册的服务数量
    pub fn service_count(&self) -> usize {
        let services = self.services.read().unwrap_or_else(|_| {
            std::process::abort();
        });
        services.len()
    }

    /// 获取健康服务数量
    pub fn healthy_service_count(&self) -> usize {
        let metadata = self.metadata.read().unwrap_or_else(|_| {
            std::process::abort();
        });
        metadata.get_healthy_count()
    }

    /// 清理过期服务
    pub fn cleanup_expired_services(&self) -> usize {
        let mut metadata = self.metadata.write().unwrap_or_else(|_| {
            std::process::abort();
        });
        let expired_services = metadata.cleanup_expired_services();

        // 从服务存储中移除过期服务
        if expired_services > 0 {
            let services_to_remove: Vec<String> =
                metadata.get_all_services().keys().cloned().collect();

            let mut services = self.services.write().unwrap_or_else(|_| {
                std::process::abort();
            });
            for service_name in services_to_remove {
                if !metadata.has_service(&service_name) {
                    services.remove(service_name.as_str());
                }
            }
        }

        expired_services
    }

    /// 执行健康检查
    pub async fn health_check_all(&self) -> HashMap<String, bool> {
        let services = self.services.read().unwrap_or_else(|_| {
            std::process::abort();
        });

        let mut results = HashMap::new();
        let mut handles = Vec::new();

        // 并发执行健康检查
        for (name, service) in services.iter() {
            let name = *name;
            let service = Arc::clone(service);

            let handle = tokio::spawn(async move {
                let is_healthy = service.is_available();
                (name, is_healthy)
            });
            handles.push(handle);
        }

        // 等待所有检查完成
        for handle in handles {
            if let Ok((name, is_healthy)) = handle.await {
                results.insert(name.to_string(), is_healthy);
            }
        }

        // 更新元数据中的状态
        self.update_health_status(&results);

        results
    }

    /// 更新健康状态
    fn update_health_status(&self, results: &HashMap<String, bool>) {
        let mut metadata = self.metadata.write().unwrap_or_else(|_| {
            std::process::abort();
        });

        for (name, is_healthy) in results {
            if let Some(mut info) = metadata.get_service(name).cloned() {
                info.status = if *is_healthy {
                    ServiceStatus::Healthy
                } else {
                    ServiceStatus::Unhealthy
                };
                let _ = metadata.update_service(info);
            }
        }
    }

    /// 验证容量限制
    fn validate_capacity(&self) -> Result<(), ServiceError> {
        let services = self
            .services
            .read()
            .map_err(|_| ServiceError::internal_error("Poisoned lock"))?;
        if services.len() >= self.config.max_services {
            return Err(ServiceError::registry_full(
                services.len(),
                self.config.max_services,
            ));
        }
        Ok(())
    }

    /// 获取配置
    pub fn config(&self) -> &RegistryConfig {
        &self.config
    }

    /// 获取统计信息
    pub fn get_stats(&self) -> RegistryStats {
        let metadata = self.metadata.read().unwrap_or_else(|_| {
            std::process::abort();
        });

        let metadata_stats = metadata.get_stats();
        let services = self.services.read().unwrap_or_else(|_| {
            std::process::abort();
        });

        RegistryStats {
            total_services: services.len(),
            healthy_services: metadata.get_healthy_count(),
            unhealthy_services: metadata.get_total_count() - metadata.get_healthy_count(),
            registered_builders: 0, // TODO: 实现构建器计数
            metadata_stats,
        }
    }
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// 注册表统计信息
#[derive(Debug, Clone)]
pub struct RegistryStats {
    /// 总服务数量
    pub total_services: usize,
    /// 健康服务数量
    pub healthy_services: usize,
    /// 不健康服务数量
    pub unhealthy_services: usize,
    /// 已注册构建器数量
    pub registered_builders: usize,
    /// 元数据统计
    pub metadata_stats: super::metadata::MetadataStats,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service_registry::builder::ServiceBuilderFactory;
    use std::any::Any;

    #[derive(Debug, Clone)]
    struct TestService {
        name: &'static str,
    }

    impl TestService {
        fn new() -> Self {
            Self { name: "test" }
        }
    }

    impl Service for TestService {
        fn name(&self) -> &'static str {
            self.name
        }

        fn version(&self) -> &'static str {
            "1.0.0"
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    impl NamedService for TestService {
        const NAME: &'static str = "test";

        fn clone_owned(&self) -> Self {
            Self::new()
        }
    }

    #[test]
    fn test_service_registry_basic() {
        let registry = ServiceRegistry::new();
        let service = TestService::new();

        // 注册服务
        assert!(registry.register(service).is_ok());
        assert_eq!(registry.service_count(), 1);
        assert!(registry.has_service("test"));

        // 获取服务
        let retrieved: Arc<TestService> = registry.get().unwrap();
        assert_eq!(retrieved.name(), "test");

        // 发现服务
        let services = registry.discover_services();
        assert_eq!(services.len(), 1);
        assert!(services.contains(&"test"));
    }

    #[test]
    fn test_service_registry_builder() {
        let registry = ServiceRegistry::new();

        // 注册构建器
        let builder = ServiceBuilderFactory::type_erased("test-builder", || Ok(TestService::new()));
        assert!(registry.register_builder(builder).is_ok());

        // 通过构建器创建服务
        assert!(registry.build_and_register_service("test-builder").is_ok());
        assert_eq!(registry.service_count(), 1);

        // 获取服务
        let service: Arc<TestService> = registry.get().unwrap();
        assert_eq!(service.name(), "test");
    }

    #[test]
    fn test_service_registry_duplicate() {
        let registry = ServiceRegistry::new();
        let service1 = TestService::new();
        let service2 = TestService::new();

        // 注册第一个服务
        assert!(registry.register(service1).is_ok());

        // 尝试注册重复服务
        let result = registry.register(service2);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ServiceError::ServiceAlreadyExists { .. })
        ));
    }

    #[test]
    fn test_service_registry_not_found() {
        let registry = ServiceRegistry::new();

        // 尝试获取不存在的服务
        let result: Result<Arc<TestService>, _> = registry.get();
        assert!(result.is_err());
        assert!(matches!(result, Err(ServiceError::ServiceNotFound { .. })));
    }

    #[test]
    fn test_service_registry_capacity() {
        let config = RegistryConfig {
            max_services: 1,
            enable_health_check: true,
            health_check_interval: std::time::Duration::from_secs(30),
            enable_cache: true,
            cache_ttl: std::time::Duration::from_secs(300),
        };

        let registry = ServiceRegistry::with_config(config);
        let service1 = TestService::new();
        let service2 = TestService::new();

        // 注册第一个服务（应该成功）
        assert!(registry.register(service1).is_ok());

        // 尝试注册第二个服务（应该失败）
        let result = registry.register(service2);
        assert!(result.is_err());
        assert!(matches!(result, Err(ServiceError::RegistryFull { .. })));
    }

    #[tokio::test]
    async fn test_health_check() {
        let registry = ServiceRegistry::new();
        let service = TestService::new();

        registry.register(service).unwrap();

        let results = registry.health_check_all().await;
        assert_eq!(results.len(), 1);
        assert!(results.get("test").unwrap_or(&false));
    }
}
