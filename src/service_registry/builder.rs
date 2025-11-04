//! 服务构建器特征和实现

use std::sync::Arc;

use super::{Service, ServiceError, ServiceResult};

/// 服务构建器特征
///
/// 定义了构建服务的统一接口，支持异步构建和配置驱动
pub trait ServiceBuilder: Send + Sync + 'static {
    /// 构建的服务类型
    type Output: Service + ?Sized;

    /// 构建服务实例
    fn build(&self) -> ServiceResult<Box<Self::Output>>;

    /// 获取构建器名称
    fn name(&self) -> &'static str;

    /// 获取构建器版本
    fn version(&self) -> &'static str {
        "1.0.0"
    }

    /// 验证构建配置
    fn validate(&self) -> ServiceResult<()> {
        Ok(())
    }

    /// 检查是否支持异步构建
    fn supports_async(&self) -> bool {
        false
    }
}

/// 基础服务构建器实现
pub struct BasicServiceBuilder<T> {
    name: &'static str,
    version: &'static str,
    factory: Box<dyn Fn() -> ServiceResult<T> + Send + Sync>,
}

impl<T> BasicServiceBuilder<T>
where
    T: Service,
{
    /// 创建新的基础构建器
    pub fn new<F>(name: &'static str, factory: F) -> Self
    where
        F: Fn() -> ServiceResult<T> + Send + Sync + 'static,
    {
        Self {
            name,
            version: "1.0.0",
            factory: Box::new(factory),
        }
    }

    /// 设置构建器版本
    pub fn with_version(mut self, version: &'static str) -> Self {
        self.version = version;
        self
    }
}

impl<T> ServiceBuilder for BasicServiceBuilder<T>
where
    T: Service,
{
    type Output = T;

    fn build(&self) -> ServiceResult<Box<Self::Output>> {
        self.validate()?;
        let service = (self.factory)();
        let service = service.map_err(|e| e)?;
        Ok(Box::new(service))
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn version(&self) -> &'static str {
        self.version
    }
}

/// 配置驱动的服务构建器
pub struct ConfigurableServiceBuilder<T, C> {
    name: &'static str,
    version: &'static str,
    config: C,
    factory: Box<dyn Fn(&C) -> ServiceResult<T> + Send + Sync>,
}

impl<T, C> ConfigurableServiceBuilder<T, C>
where
    T: Service,
    C: Send + Sync + Clone + 'static,
{
    /// 创建新的配置化构建器
    pub fn new<F>(name: &'static str, config: C, factory: F) -> Self
    where
        F: Fn(&C) -> ServiceResult<T> + Send + Sync + 'static,
    {
        Self {
            name,
            version: "1.0.0",
            config,
            factory: Box::new(factory),
        }
    }

    /// 设置构建器版本
    pub fn with_version(mut self, version: &'static str) -> Self {
        self.version = version;
        self
    }

    /// 更新配置
    pub fn update_config(&mut self, config: C) {
        self.config = config;
    }

    /// 获取配置引用
    pub fn config(&self) -> &C {
        &self.config
    }
}

impl<T, C> ServiceBuilder for ConfigurableServiceBuilder<T, C>
where
    T: Service,
    C: Send + Sync + Clone + 'static,
{
    type Output = T;

    fn build(&self) -> ServiceResult<Box<Self::Output>> {
        self.validate()?;
        let service = (self.factory)(&self.config);
        let service = service.map_err(|e| e)?;
        Ok(Box::new(service))
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn version(&self) -> &'static str {
        self.version
    }
}

/// 服务构建器工厂
pub struct ServiceBuilderFactory;

impl ServiceBuilderFactory {
    /// 创建基础构建器
    pub fn basic<T, F>(name: &'static str, factory: F) -> BasicServiceBuilder<T>
    where
        T: Service,
        F: Fn() -> ServiceResult<T> + Send + Sync + 'static,
    {
        BasicServiceBuilder::new(name, factory)
    }

    /// 创建配置化构建器
    pub fn configurable<T, C, F>(
        name: &'static str,
        config: C,
        factory: F,
    ) -> ConfigurableServiceBuilder<T, C>
    where
        T: Service,
        C: Send + Sync + Clone + 'static,
        F: Fn(&C) -> ServiceResult<T> + Send + Sync + 'static,
    {
        ConfigurableServiceBuilder::new(name, config, factory)
    }

    /// 创建类型擦除的基础构建器（用于ServiceRegistry）
    pub fn type_erased<T, F>(name: &'static str, factory: F) -> TypeErasedServiceBuilder
    where
        T: Service + 'static,
        F: Fn() -> ServiceResult<T> + Send + Sync + 'static,
    {
        TypeErasedServiceBuilder::new(name, factory)
    }

    // TODO: 重新实现单例构建器
    // pub fn singleton<T, F>(name: &'static str, factory: F) -> SingletonServiceBuilder<T>
    // where
    //     T: Service + Send + Sync + Clone + 'static,
    //     F: Fn() -> ServiceResult<T> + Send + Sync + 'static,
    // {
    //     SingletonServiceBuilder::new(name, factory)
    // }
}

/// 类型擦除的服务构建器包装器
pub struct TypeErasedServiceBuilder {
    name: &'static str,
    version: &'static str,
    build_fn: Box<dyn Fn() -> ServiceResult<Box<dyn Service>> + Send + Sync>,
    validate_fn: Box<dyn Fn() -> ServiceResult<()> + Send + Sync>,
    supports_async: bool,
}

impl TypeErasedServiceBuilder {
    /// 创建新的类型擦除构建器
    pub fn new<T, F>(name: &'static str, factory: F) -> Self
    where
        T: Service + 'static,
        F: Fn() -> ServiceResult<T> + Send + Sync + 'static,
    {
        Self {
            name,
            version: "1.0.0",
            build_fn: Box::new(move || {
                let service = factory()?;
                Ok(Box::new(service) as Box<dyn Service>)
            }),
            validate_fn: Box::new(|| Ok(())),
            supports_async: false,
        }
    }

    /// 创建带配置的类型擦除构建器
    pub fn with_config<T, C, F>(name: &'static str, _config: C, factory: F) -> Self
    where
        T: Service + 'static,
        C: Send + Sync + 'static,
        F: Fn(C) -> ServiceResult<T> + Send + Sync + 'static,
    {
        // 注意：这里简化处理，实际应用中可能需要存储配置
        Self {
            name,
            version: "1.0.0",
            build_fn: Box::new(move || {
                // 由于类型擦除，我们无法直接使用配置
                // 这是一个简化实现，实际使用中可能需要更复杂的策略
                Err(ServiceError::internal_error("Configured builders not fully supported with type erasure"))
            }),
            validate_fn: Box::new(|| Ok(())),
            supports_async: false,
        }
    }
}

impl ServiceBuilder for TypeErasedServiceBuilder {
    type Output = dyn Service;

    fn build(&self) -> ServiceResult<Box<Self::Output>> {
        (self.build_fn)()
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn version(&self) -> &'static str {
        self.version
    }

    fn validate(&self) -> ServiceResult<()> {
        (self.validate_fn)()
    }

    fn supports_async(&self) -> bool {
        self.supports_async
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service_registry::{NamedService, ServiceStatus};
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
        const NAME: &'static str = "test-service";

        fn clone_owned(&self) -> Self {
            Self::new()
        }
    }

    #[test]
    fn test_basic_service_builder() {
        let builder = ServiceBuilderFactory::basic("test-builder", || {
            Ok(TestService::new())
        });

        assert_eq!(builder.name(), "test-builder");
        assert_eq!(builder.version(), "1.0.0");

        let service = builder.build().unwrap();
        assert_eq!(service.name(), "test");
    }

    #[test]
    fn test_configurable_service_builder() {
        #[derive(Clone)]
        struct TestConfig {
            value: String,
        }

        let config = TestConfig {
            value: "configured".to_string(),
        };

        let builder = ServiceBuilderFactory::configurable("configurable-builder", config.clone(), |c| {
            Ok(TestService::new())
        });

        assert_eq!(builder.config().value, "configured");

        let service = builder.build().unwrap();
        assert_eq!(service.name(), "test");
    }

    #[test]
    fn test_builder_validation() {
        let builder = ServiceBuilderFactory::basic("test-builder", || {
            Ok(TestService::new())
        });

        // 默认验证应该成功
        assert!(builder.validate().is_ok());
    }
}