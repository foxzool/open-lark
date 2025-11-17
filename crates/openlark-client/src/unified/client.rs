//! OpenLark 统一客户端核心实现

use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;

use openlark_core::{SDKResult, config::Config as CoreConfig};

use super::{
    traits::{UnifiedService, ServiceLifecycle, APICall, ServiceDescriptor},
    config::UnifiedConfig,
    error::{UnifiedError, UnifiedResult},
    middleware::MiddlewareChain,
    registry::ServiceRegistry,
    transport::TransportLayer,
    api::UnifiedAPIClient,
    service_adapter::{APIDispatcher, ServiceAdapterFactory},
};

/// OpenLark 统一客户端
///
/// 提供所有飞书服务的统一访问接口，支持：
/// - 动态服务注册和发现
/// - 统一的配置管理
/// - 中间件支持
/// - 类型安全的API调用
/// - 异步并发控制
#[derive(Debug, Clone)]
pub struct UnifiedClient {
    /// 客户端配置
    config: Arc<UnifiedConfig>,
    /// 服务注册表
    registry: Arc<ServiceRegistry>,
    /// 传输层
    transport: Arc<TransportLayer>,
    /// 中间件链
    middleware: Arc<MiddlewareChain>,
    /// 客户端元数据
    metadata: HashMap<String, String>,
    /// API分发器
    dispatcher: Arc<APIDispatcher>,
}

impl UnifiedClient {
    /// 创建新的统一客户端
    pub async fn new(config: UnifiedConfig) -> UnifiedResult<Self> {
        // 创建服务注册表
        let registry = Arc::new(ServiceRegistry::new());

        // 创建传输层
        let transport = Arc::new(TransportLayer::from_config(&config.core)?);

        // 创建中间件链
        let middleware = Arc::new(MiddlewareChain::new());

        // 创建API分发器
        let dispatcher = Arc::new(ServiceAdapterFactory::create_dispatcher());

        let client = Self {
            config: Arc::new(config),
            registry,
            transport,
            middleware,
            metadata: HashMap::new(),
            dispatcher,
        };

        // 初始化客户端
        client.init().await?;

        Ok(client)
    }

    /// 初始化客户端
    async fn init(&self) -> UnifiedResult<()> {
        // 启动所有已注册的服务
        let services = self.registry.list_services();
        for service_name in services {
            if let Some(mut service) = self.registry.get_service_mut(&service_name) {
                service.start().await?;
            }
        }
        Ok(())
    }

    /// 获取客户端配置
    pub fn config(&self) -> &UnifiedConfig {
        &self.config
    }

    /// 获取服务注册表
    pub fn registry(&self) -> &ServiceRegistry {
        &self.registry
    }

    /// 获取传输层
    pub fn transport(&self) -> &TransportLayer {
        &self.transport
    }

    /// 获取中间件链
    pub fn middleware(&self) -> &MiddlewareChain {
        &self.middleware
    }

    /// 获取API分发器
    pub fn dispatcher(&self) -> &APIDispatcher {
        &self.dispatcher
    }

    /// 获取服务
    ///
    /// # 类型参数
    /// - `T`: 服务类型
    ///
    /// # 返回值
    /// 返回服务实例，如果服务不存在或不可用则返回错误
    pub fn service<T>(&self) -> UnifiedResult<&T>
    where
        T: UnifiedService + 'static,
    {
        let service_name = std::any::type_name::<T>();
        self.registry
            .get_service::<T>(service_name)
            .ok_or_else(|| UnifiedError::ServiceNotFound(service_name.to_string()))
    }

    /// 获取服务并执行API调用
    ///
    /// # 类型参数
    /// - `T`: 服务类型
    /// - `R`: API响应类型
    ///
    /// # 参数
    /// - `api_call`: 要执行的API调用
    ///
    /// # 返回值
    /// 返回API调用结果
    pub async fn execute<T, R, F>(&self, api_call: F) -> UnifiedResult<R>
    where
        T: UnifiedService + 'static,
        F: APICall<T::Config, R> + Send + Sync,
        R: Send + 'static,
    {
        // 获取服务
        let service = self.service::<T>()?;

        // 检查服务状态
        if service.status() != super::traits::ServiceStatus::Running {
            return Err(UnifiedError::ServiceNotAvailable(
                std::any::type_name::<T>().to_string(),
            ));
        }

        // 使用API分发器执行调用
        // 这里需要根据具体的API调用类型来实现
        // 暂时返回一个未实现错误，等待后续完善
        Err(UnifiedError::NotImplemented(
            "API调用执行逻辑正在开发中".to_string(),
        ))
    }

    /// 注册服务
    ///
    /// # 参数
    /// - `service`: 要注册的服务
    pub fn register_service<T>(&self, service: T) -> UnifiedResult<()>
    where
        T: UnifiedService + Send + Sync + 'static,
    {
        let descriptor = service.descriptor();
        self.registry.register(service.name(), Box::new(service))?;
        Ok(())
    }

    /// 注销服务
    ///
    /// # 参数
    /// - `service_name`: 服务名称
    pub async fn unregister_service(&self, service_name: &str) -> UnifiedResult<()> {
        if let Some(mut service) = self.registry.get_service_mut(service_name) {
            service.stop().await?;
        }
        self.registry.unregister(service_name)?;
        Ok(())
    }

    /// 列出所有可用服务
    pub fn list_services(&self) -> Vec<&str> {
        self.registry.list_services()
    }

    /// 检查服务是否可用
    pub fn is_service_available(&self, service_name: &str) -> bool {
        self.registry
            .get_service_info(service_name)
            .map(|info| info.status == super::traits::ServiceStatus::Running)
            .unwrap_or(false)
    }

    /// 添加元数据
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// 获取元数据
    pub fn metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// 获取统一API客户端
    pub fn api(&self) -> UnifiedAPIClient {
        UnifiedAPIClient::new(Arc::new(self.clone()))
    }

    /// 获取通信服务API
    pub fn communication(&self) -> UnifiedResult<super::api::CommunicationAPI> {
        if !self.is_service_available("communication") {
            return Err(UnifiedError::ServiceNotAvailable("communication".to_string()));
        }
        Ok(super::api::CommunicationAPI::new(self.api()))
    }

    /// 获取HR服务API
    pub fn hr(&self) -> UnifiedResult<super::api::HRAPI> {
        if !self.is_service_available("hr") {
            return Err(UnifiedError::ServiceNotAvailable("hr".to_string()));
        }
        Ok(super::api::HRAPI::new(self.api()))
    }

    /// 获取文档服务API
    pub fn docs(&self) -> UnifiedResult<super::api::DocsAPI> {
        if !self.is_service_available("docs") {
            return Err(UnifiedError::ServiceNotAvailable("docs".to_string()));
        }
        Ok(super::api::DocsAPI::new(self.api()))
    }

    /// 获取AI服务API
    pub fn ai(&self) -> UnifiedResult<super::api::AIAPI> {
        if !self.is_service_available("ai") {
            return Err(UnifiedError::ServiceNotAvailable("ai".to_string()));
        }
        Ok(super::api::AIAPI::new(self.api()))
    }

    /// 获取认证服务API
    pub fn auth(&self) -> UnifiedResult<super::api::AuthAPI> {
        if !self.is_service_available("auth") {
            return Err(UnifiedError::ServiceNotAvailable("auth".to_string()));
        }
        Ok(super::api::AuthAPI::new(self.api()))
    }

    /// 执行健康检查
    pub async fn health_check(&self) -> UnifiedResult<HashMap<String, bool>> {
        let mut results = HashMap::new();

        let services = self.registry.list_services();
        for service_name in services {
            if let Some(service) = self.registry.get_service(&service_name) {
                let is_healthy = service.health_check().await.unwrap_or(false);
                results.insert(service_name, is_healthy);
            }
        }

        Ok(results)
    }

    /// 关闭客户端
    ///
    /// 停止所有服务并清理资源
    pub async fn shutdown(&self) -> UnifiedResult<()> {
        // 停止所有服务
        let services = self.registry.list_services();
        for service_name in services {
            if let Some(mut service) = self.registry.get_service_mut(&service_name) {
                if let Err(e) = service.stop().await {
                    tracing::warn!("停止服务 {} 失败: {}", service_name, e);
                }
            }
        }

        Ok(())
    }
}

#[async_trait]
impl ServiceLifecycle for UnifiedClient {
    async fn start(&mut self) -> SDKResult<()> {
        self.init().await?;
        Ok(())
    }

    async fn stop(&mut self) -> SDKResult<()> {
        self.shutdown().await?;
        Ok(())
    }

    async fn health_check(&self) -> SDKResult<bool> {
        let results = self.health_check().await?;
        Ok(results.values().all(|&healthy| healthy))
    }
}

/// 统一客户端构建器
///
/// 提供流畅的API来配置和构建UnifiedClient实例。
#[derive(Debug, Clone)]
pub struct UnifiedClientBuilder {
    config: Option<UnifiedConfig>,
    services: Vec<Box<dyn std::any::Any + Send + Sync>>,
    metadata: HashMap<String, String>,
}

impl UnifiedClientBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            config: None,
            services: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// 设置客户端配置
    pub fn config(mut self, config: UnifiedConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// 从核心配置构建
    pub fn core_config(mut self, core_config: CoreConfig) -> Self {
        let config = UnifiedConfig::from_core(core_config);
        self.config = Some(config);
        self
    }

    /// 添加服务
    pub fn service<T>(mut self, service: T) -> Self
    where
        T: UnifiedService + Send + Sync + 'static,
    {
        self.services.push(Box::new(service));
        self
    }

    /// 添加元数据
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// 从环境变量构建配置
    pub fn from_env() -> UnifiedResult<Self> {
        let config = UnifiedConfig::from_env()?;
        Ok(Self::new().config(config))
    }

    /// 构建客户端
    pub async fn build(self) -> UnifiedResult<UnifiedClient> {
        let config = self.config.ok_or_else(|| {
            UnifiedError::ConfigurationError("客户端配置未设置".to_string())
        })?;

        // 创建客户端
        let mut client = UnifiedClient::new(config).await?;

        // 添加元数据
        for (key, value) in self.metadata {
            client = client.with_metadata(key, value);
        }

        // 注册服务
        for service_any in self.services {
            // 这里需要更复杂的服务注册逻辑
            // 暂时跳过，在后续步骤中完善
        }

        Ok(client)
    }
}

impl Default for UnifiedClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_unified_client_builder() {
        let builder = UnifiedClientBuilder::new()
            .metadata("test_key", "test_value");

        assert_eq!(builder.metadata.get("test_key"), Some(&"test_value".to_string()));
    }

    #[tokio::test]
    async fn test_client_creation() {
        // 这里需要从环境变量或测试配置创建配置
        // 暂时跳过实际测试
    }
}