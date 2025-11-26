//! 服务工厂
//!
//! 负责创建和管理服务实例，支持延迟初始化和单例模式

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;

use super::{ServiceMetadata, ServiceStatus}; // 移除未使用的 RegistryResult
use crate::Config;

/// 服务工厂错误
#[derive(Error, Debug)]
pub enum FactoryError {
    /// 服务创建失败错误
    ///
    /// 当服务实例创建过程中出现错误时触发
    #[error("服务 '{name}' 创建失败: {reason}")]
    CreationFailed {
        /// 创建失败的服务名称
        name: String,
        /// 失败原因
        reason: String,
    },

    /// 不支持的服务错误
    ///
    /// 当尝试创建不支持的服务时触发
    #[error("服务 '{name}' 不支持")]
    UnsupportedService {
        /// 不支持的服务名称
        name: String,
    },

    /// 无效配置错误
    ///
    /// 当服务配置无效时触发
    #[error("配置无效: {reason}")]
    InvalidConfig {
        /// 配置无效的原因
        reason: String,
    },

    /// 依赖服务未就绪错误
    ///
    /// 当服务依赖的其他服务尚未就绪时触发
    #[error("依赖服务未就绪: {dependencies:?}")]
    DependenciesNotReady {
        /// 未就绪的依赖服务列表
        dependencies: Vec<String>,
    },
}

/// 服务工厂结果类型
pub type FactoryResult<T> = Result<T, FactoryError>;

/// 服务工厂特征
#[async_trait]
pub trait ServiceFactory: Send + Sync {
    /// 创建服务实例
    async fn create_service(
        &self,
        metadata: &ServiceMetadata,
        config: &Config,
        dependencies: &HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
    ) -> FactoryResult<Box<dyn std::any::Any + Send + Sync>>;

    /// 验证服务配置
    fn validate_config(&self, metadata: &ServiceMetadata, config: &Config) -> FactoryResult<()>;

    /// 检查依赖是否满足
    fn check_dependencies(
        &self,
        metadata: &ServiceMetadata,
        available_services: &HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
    ) -> FactoryResult<()>;
}

/// 服务实例管理器
pub struct ServiceInstanceManager {
    /// 服务实例存储
    instances: RwLock<HashMap<String, ServiceInstance>>,
    /// 服务工厂注册表
    factories: RwLock<HashMap<String, Arc<dyn ServiceFactory>>>,
    /// 配置
    config: Config,
}

impl std::fmt::Debug for ServiceInstanceManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServiceInstanceManager")
            .field("instances_count", &self.instances.read().unwrap().len())
            .field("factories_count", &self.factories.read().unwrap().len())
            .field("config", &self.config)
            .finish()
    }
}

/// 服务实例
#[derive(Debug)]
pub struct ServiceInstance {
    /// 服务名称
    pub name: String,
    /// 服务实例
    pub instance: Box<dyn std::any::Any + Send + Sync>,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 最后访问时间
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    /// 访问计数
    pub access_count: u64,
    /// 是否为单例
    pub singleton: bool,
}

impl ServiceInstanceManager {
    /// 创建新的服务实例管理器
    pub fn new(config: Config) -> Self {
        Self {
            instances: RwLock::new(HashMap::new()),
            factories: RwLock::new(HashMap::new()),
            config,
        }
    }

    /// 注册服务工厂
    pub fn register_factory(&self, service_name: &str, factory: Arc<dyn ServiceFactory>) {
        let mut factories = self.factories.write().unwrap();
        factories.insert(service_name.to_string(), factory);
    }

    /// 获取或创建服务实例
    pub async fn get_or_create_service(
        &self,
        metadata: &ServiceMetadata,
        dependencies: &HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
    ) -> FactoryResult<Box<dyn std::any::Any + Send + Sync>> {
        // 检查是否已有实例（单例模式）
        {
            let instances = self.instances.read().unwrap();
            if metadata.status == ServiceStatus::Running {
                if let Some(instance) = instances.get(&metadata.name) {
                    return Ok(self.clone_instance(instance));
                }
            }
        }

        // 创建新实例
        self.create_service_instance(metadata, dependencies).await
    }

    /// 创建服务实例
    async fn create_service_instance(
        &self,
        metadata: &ServiceMetadata,
        dependencies: &HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
    ) -> FactoryResult<Box<dyn std::any::Any + Send + Sync>> {
        // 在 async 调用前获取工厂，避免 await_holding_lock
        let factory = {
            let factories = self.factories.read().unwrap();
            factories
                .get(&metadata.name)
                .ok_or_else(|| FactoryError::UnsupportedService {
                    name: metadata.name.clone(),
                })?
                .clone()
        };

        // 验证配置
        factory.validate_config(metadata, &self.config)?;

        // 检查依赖
        factory.check_dependencies(metadata, dependencies)?;

        // 创建实例
        let instance = factory
            .create_service(metadata, &self.config, dependencies)
            .await?;

        // 存储实例（如果需要）
        if metadata.status == ServiceStatus::Running {
            let service_instance = ServiceInstance {
                name: metadata.name.clone(),
                instance: self.clone_any(&instance),
                created_at: chrono::Utc::now(),
                last_accessed: chrono::Utc::now(),
                access_count: 1,
                singleton: true,
            };

            let mut instances = self.instances.write().unwrap();
            instances.insert(metadata.name.clone(), service_instance);
        }

        Ok(instance)
    }

    /// 克隆实例（简化实现）
    fn clone_instance(&self, instance: &ServiceInstance) -> Box<dyn std::any::Any + Send + Sync> {
        // 这里应该实现实际的克隆逻辑
        // 现在只是返回一个引用的副本
        self.clone_any(&instance.instance)
    }

    /// 简化的Any克隆
    fn clone_any(
        &self,
        _any: &Box<dyn std::any::Any + Send + Sync>,
    ) -> Box<dyn std::any::Any + Send + Sync> {
        // 这是一个简化实现，实际应该根据具体类型进行克隆
        // 或者使用 Arc 包装来实现共享
        Box::new(())
    }

    /// 获取服务统计信息
    pub fn get_service_stats(&self) -> HashMap<String, ServiceStats> {
        let instances = self.instances.read().unwrap();
        instances
            .iter()
            .map(|(name, instance)| {
                let stats = ServiceStats {
                    name: name.clone(),
                    created_at: instance.created_at,
                    last_accessed: instance.last_accessed,
                    access_count: instance.access_count,
                    is_singleton: instance.singleton,
                };
                (name.clone(), stats)
            })
            .collect()
    }

    /// 清理未使用的实例
    pub fn cleanup_unused_instances(&self, timeout_seconds: u64) {
        let mut instances = self.instances.write().unwrap();
        let cutoff_time = chrono::Utc::now() - chrono::Duration::seconds(timeout_seconds as i64);

        instances.retain(|_, instance| instance.last_accessed > cutoff_time || instance.singleton);
    }
}

/// 服务统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStats {
    /// 服务名称
    pub name: String,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 最后访问时间
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    /// 访问计数
    pub access_count: u64,
    /// 是否为单例
    pub is_singleton: bool,
}

/// 默认服务工厂实现
#[derive(Debug, Clone, Copy)]
pub struct DefaultServiceFactory;

#[async_trait]
impl ServiceFactory for DefaultServiceFactory {
    async fn create_service(
        &self,
        metadata: &ServiceMetadata,
        _config: &Config,
        _dependencies: &HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
    ) -> FactoryResult<Box<dyn std::any::Any + Send + Sync>> {
        // 这里应该根据服务类型创建实际的服务实例
        // 现在只是返回一个占位符
        match metadata.name.as_str() {
            "communication" => {
                // 创建通讯服务实例
                let service = PlaceholderService::new(&metadata.name);
                Ok(Box::new(service))
            }
            "docs" => {
                // 创建文档服务实例
                let service = PlaceholderService::new(&metadata.name);
                Ok(Box::new(service))
            }
            "auth" => {
                // 创建认证服务实例
                let service = PlaceholderService::new(&metadata.name);
                Ok(Box::new(service))
            }
            "hr" => {
                // 创建人力资源服务实例
                let service = PlaceholderService::new(&metadata.name);
                Ok(Box::new(service))
            }
            "ai" => {
                // 创建AI服务实例
                let service = PlaceholderService::new(&metadata.name);
                Ok(Box::new(service))
            }
            "calendar" => {
                // 创建日历服务实例
                let service = PlaceholderService::new(&metadata.name);
                Ok(Box::new(service))
            }
            "admin" => {
                // 创建管理服务实例
                let service = PlaceholderService::new(&metadata.name);
                Ok(Box::new(service))
            }
            "approval" => {
                // 创建审批服务实例
                let service = PlaceholderService::new(&metadata.name);
                Ok(Box::new(service))
            }
            "helpdesk" => {
                // 创建帮助台服务实例
                let service = PlaceholderService::new(&metadata.name);
                Ok(Box::new(service))
            }
            _ => Err(FactoryError::UnsupportedService {
                name: metadata.name.clone(),
            }),
        }
    }

    fn validate_config(&self, metadata: &ServiceMetadata, config: &Config) -> FactoryResult<()> {
        // 基本配置验证
        if config.app_id.is_empty() {
            return Err(FactoryError::InvalidConfig {
                reason: "app_id 不能为空".to_string(),
            });
        }

        if config.app_secret.is_empty() {
            return Err(FactoryError::InvalidConfig {
                reason: "app_secret 不能为空".to_string(),
            });
        }

        // 服务特定的配置验证
        match metadata.name.as_str() {
            "communication" | "ai" => {
                // 这些服务可能需要网络访问
                if config.base_url.is_empty() {
                    return Err(FactoryError::InvalidConfig {
                        reason: "base_url 不能为空".to_string(),
                    });
                }
            }
            _ => {
                // 其他服务的特定验证
            }
        }

        Ok(())
    }

    fn check_dependencies(
        &self,
        metadata: &ServiceMetadata,
        available_services: &HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
    ) -> FactoryResult<()> {
        for dep in &metadata.dependencies {
            if !available_services.contains_key(dep) {
                return Err(FactoryError::DependenciesNotReady {
                    dependencies: vec![dep.clone()],
                });
            }
        }
        Ok(())
    }
}

/// 占位符服务（用于演示）
#[derive(Debug)]
pub struct PlaceholderService {
    name: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl PlaceholderService {
    /// 创建新的占位符服务实例
    ///
    /// # 参数
    ///
    /// * `name` - 服务名称
    ///
    /// # 返回值
    ///
    /// 返回新的PlaceholderService实例
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            created_at: chrono::Utc::now(),
        }
    }

    /// 获取服务名称
    ///
    /// # 返回值
    ///
    /// 返回服务名称的字符串引用
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 获取服务创建时间
    ///
    /// # 返回值
    ///
    /// 返回服务创建时的UTC时间戳
    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    /// 执行示例操作
    ///
    /// 这是一个演示方法，用于展示服务的基本功能
    ///
    /// # 返回值
    ///
    /// 返回操作结果的字符串
    pub async fn do_something(&self) -> FactoryResult<String> {
        Ok(format!("服务 {} 正在执行操作", self.name))
    }
}

/// 服务工厂注册器
pub struct ServiceFactoryRegistry {
    /// 工厂存储
    factories: HashMap<String, Arc<dyn ServiceFactory>>,
    /// 默认工厂
    default_factory: Arc<dyn ServiceFactory>,
}

impl std::fmt::Debug for ServiceFactoryRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServiceFactoryRegistry")
            .field("factories_count", &self.factories.len())
            .field("has_default_factory", &true)
            .finish()
    }
}

impl ServiceFactoryRegistry {
    /// 创建新的工厂注册器
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
            default_factory: Arc::new(DefaultServiceFactory),
        }
    }

    /// 注册服务工厂
    pub fn register_factory(&mut self, service_name: &str, factory: Arc<dyn ServiceFactory>) {
        self.factories.insert(service_name.to_string(), factory);
    }

    /// 获取服务工厂
    pub fn get_factory(&self, service_name: &str) -> Arc<dyn ServiceFactory> {
        self.factories
            .get(service_name)
            .cloned()
            .unwrap_or_else(|| self.default_factory.clone())
    }

    /// 注册默认服务工厂
    pub fn register_default_factories(&mut self) {
        // 这里可以注册各种特定服务的工厂
        // 现在只使用默认工厂
    }
}

impl Default for ServiceFactoryRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        registry.register_default_factories();
        registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_factory_registry() {
        let registry = ServiceFactoryRegistry::new();

        // 测试获取默认工厂
        let _factory = registry.get_factory("test-service");
        // 这里可以进一步测试工厂的功能
    }

    #[tokio::test]
    async fn test_service_creation() {
        let factory = DefaultServiceFactory;
        let metadata = ServiceMetadata {
            name: "test-service".to_string(),
            version: "1.0.0".to_string(),
            description: None,
            dependencies: vec![],
            provides: vec![],
            status: ServiceStatus::Ready,
            priority: 1,
        };

        let config = Config {
            app_id: "test-app".to_string(),
            app_secret: "test-secret".to_string(),
            base_url: "https://test.com".to_string(),
            ..Default::default()
        };

        let dependencies = HashMap::new();

        let result = factory
            .create_service(&metadata, &config, &dependencies)
            .await;
        // 目前会返回错误，因为 test-service 不在支持的列表中
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation() {
        let factory = DefaultServiceFactory;
        let metadata = ServiceMetadata {
            name: "auth".to_string(),
            version: "1.0.0".to_string(),
            description: None,
            dependencies: vec![],
            provides: vec![],
            status: ServiceStatus::Ready,
            priority: 1,
        };

        // 测试有效配置
        let valid_config = Config {
            app_id: "test-app".to_string(),
            app_secret: "test-secret".to_string(),
            ..Default::default()
        };

        assert!(factory.validate_config(&metadata, &valid_config).is_ok());

        // 测试无效配置
        let invalid_config = Config {
            app_id: "".to_string(), // 空的app_id
            app_secret: "test-secret".to_string(),
            ..Default::default()
        };

        assert!(factory.validate_config(&metadata, &invalid_config).is_err());
    }
}
