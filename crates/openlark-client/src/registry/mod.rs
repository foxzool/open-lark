//! 服务注册和发现机制
//!
//! 提供动态服务管理、功能标志控制和依赖解析功能

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;

pub mod dependency_resolver;
pub mod feature_flags;
pub mod service_factory;

pub use dependency_resolver::*;
pub use feature_flags::{
    FeatureFlagManager, FlagMetadata, FlagValue, SegmentCondition, UserSegment,
};
pub use service_factory::*;

/// 服务注册和发现错误
#[derive(Error, Debug, Clone)]
pub enum RegistryError {
    /// 服务已存在错误
    ///
    /// 当尝试注册的服务名称已经存在时触发
    #[error("服务 '{name}' 已存在")]
    ServiceAlreadyExists {
        /// 已存在的服务名称
        name: String,
    },

    /// 服务不存在错误
    ///
    /// 当尝试访问不存在的服务时触发
    #[error("服务 '{name}' 不存在")]
    ServiceNotFound {
        /// 不存在的服务名称
        name: String,
    },

    /// 循环依赖错误
    ///
    /// 当检测到服务间存在循环依赖关系时触发
    #[error("循环依赖检测: {dependency_chain}")]
    CircularDependency {
        /// 循环依赖链
        dependency_chain: String,
    },

    /// 缺少依赖服务错误
    ///
    /// 当服务依赖的其他服务不存在时触发
    #[error("缺少依赖服务: {missing_dependencies:?}")]
    MissingDependencies {
        /// 缺失的依赖服务列表
        missing_dependencies: Vec<String>,
    },

    /// 无效功能标志错误
    ///
    /// 当使用了未定义的功能标志时触发
    #[error("功能标志 '{flag}' 无效")]
    InvalidFeatureFlag {
        /// 无效的功能标志名称
        flag: String,
    },

    /// 依赖解析错误
    ///
    /// 由依赖解析过程中产生的错误
    #[error("依赖解析错误: {0}")]
    DependencyError(#[from] dependency_resolver::DependencyError),

    /// 功能标志错误
    ///
    /// 由功能标志管理过程中产生的错误
    #[error("功能标志错误: {0}")]
    FeatureFlagError(#[from] feature_flags::FeatureFlagError),
}

/// 服务注册表结果类型
pub type RegistryResult<T> = Result<T, RegistryError>;

/// 服务元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetadata {
    /// 服务名称
    pub name: String,
    /// 服务版本
    pub version: String,
    /// 服务描述
    pub description: Option<String>,
    /// 依赖的服务列表
    pub dependencies: Vec<String>,
    /// 提供的功能
    pub provides: Vec<String>,
    /// 服务状态
    pub status: ServiceStatus,
    /// 服务优先级（数值越小优先级越高）
    pub priority: u32,
}

/// 服务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceStatus {
    /// 未初始化
    Uninitialized,
    /// 初始化中
    Initializing,
    /// 已就绪
    Ready,
    /// 运行中
    Running,
    /// 已停止
    Stopped,
    /// 错误状态
    Error(String),
}

/// 服务条目
#[derive(Debug)]
pub struct ServiceEntry {
    /// 服务元数据
    pub metadata: ServiceMetadata,
    /// 服务实例
    pub instance: Option<Box<dyn std::any::Any + Send + Sync>>,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 最后更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 服务注册表特征
pub trait ServiceRegistry: Send + Sync {
    /// 注册服务
    fn register_service(&mut self, metadata: ServiceMetadata) -> RegistryResult<()>;

    /// 注销服务
    fn unregister_service(&mut self, name: &str) -> RegistryResult<()>;

    /// 获取服务
    fn get_service(&self, name: &str) -> RegistryResult<&ServiceEntry>;

    /// 获取服务实例（类型安全）
    fn get_service_typed<T>(&self, name: &str) -> RegistryResult<&T>
    where
        T: 'static;

    /// 列出所有服务
    fn list_services(&self) -> Vec<&ServiceEntry>;

    /// 检查服务是否存在
    fn has_service(&self, name: &str) -> bool;

    /// 更新服务状态
    fn update_service_status(&mut self, name: &str, status: ServiceStatus) -> RegistryResult<()>;

    /// 获取依赖关系图
    fn get_dependency_graph(&self) -> HashMap<String, Vec<String>>;
}

/// 默认服务注册表实现
#[derive(Debug)]
pub struct DefaultServiceRegistry {
    /// 服务存储
    services: HashMap<String, ServiceEntry>,
    /// 功能标志管理器
    feature_flags: Arc<RwLock<FeatureFlagManager>>,
    /// 依赖解析器
    dependency_resolver: Arc<DependencyResolver>,
}

impl DefaultServiceRegistry {}

impl Default for DefaultServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultServiceRegistry {
    /// 创建新的服务注册表
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
            feature_flags: Arc::new(RwLock::new(FeatureFlagManager::new())),
            dependency_resolver: Arc::new(DependencyResolver::new()),
        }
    }

    /// 从配置创建服务注册表
    pub fn from_config(config: RegistryConfig) -> Self {
        let registry = Self::new();

        // 设置功能标志
        {
            let flags = registry.feature_flags.write().unwrap();
            for (flag, enabled) in config.feature_flags {
                let _ = flags.set_flag(&flag, FlagValue::Bool(enabled));
            }
        }

        registry
    }

    /// 初始化所有服务
    pub fn initialize_services(&mut self) -> RegistryResult<()> {
        // 按依赖顺序排序服务
        let sorted_services = self
            .dependency_resolver
            .resolve_dependencies(self.get_dependency_graph())?;

        // 按顺序初始化服务
        for service_name in sorted_services {
            if let Some(_entry) = self.services.get_mut(&service_name) {
                self.update_service_status(&service_name, ServiceStatus::Initializing)?;

                // 这里应该调用服务的初始化逻辑
                // 现在只是模拟初始化
                std::thread::sleep(std::time::Duration::from_millis(10));

                self.update_service_status(&service_name, ServiceStatus::Ready)?;
            }
        }

        Ok(())
    }

    /// 启动所有就绪的服务
    pub fn start_services(&mut self) -> RegistryResult<()> {
        let ready_services: Vec<String> = self
            .services
            .iter()
            .filter(|(_, entry)| entry.metadata.status == ServiceStatus::Ready)
            .map(|(name, _)| name.clone())
            .collect();

        for service_name in ready_services {
            self.update_service_status(&service_name, ServiceStatus::Running)?;
        }

        Ok(())
    }
}

impl ServiceRegistry for DefaultServiceRegistry {
    fn register_service(&mut self, metadata: ServiceMetadata) -> RegistryResult<()> {
        // 检查服务是否已存在
        if self.services.contains_key(&metadata.name) {
            return Err(RegistryError::ServiceAlreadyExists {
                name: metadata.name,
            });
        }

        // 创建服务条目
        let entry = ServiceEntry {
            metadata: metadata.clone(),
            instance: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // 注册服务
        self.services.insert(metadata.name.clone(), entry);

        Ok(())
    }

    fn unregister_service(&mut self, name: &str) -> RegistryResult<()> {
        if !self.services.contains_key(name) {
            return Err(RegistryError::ServiceNotFound {
                name: name.to_string(),
            });
        }

        self.services.remove(name);
        Ok(())
    }

    fn get_service(&self, name: &str) -> RegistryResult<&ServiceEntry> {
        self.services
            .get(name)
            .ok_or_else(|| RegistryError::ServiceNotFound {
                name: name.to_string(),
            })
    }

    fn get_service_typed<T>(&self, name: &str) -> RegistryResult<&T>
    where
        T: 'static,
    {
        let entry = self.get_service(name)?;

        entry
            .instance
            .as_ref()
            .and_then(|instance| instance.downcast_ref::<T>())
            .ok_or_else(|| RegistryError::ServiceNotFound {
                name: format!("类型转换失败: {}", name),
            })
    }

    fn list_services(&self) -> Vec<&ServiceEntry> {
        self.services.values().collect()
    }

    fn has_service(&self, name: &str) -> bool {
        self.services.contains_key(name)
    }

    fn update_service_status(&mut self, name: &str, status: ServiceStatus) -> RegistryResult<()> {
        let entry = self
            .services
            .get_mut(name)
            .ok_or_else(|| RegistryError::ServiceNotFound {
                name: name.to_string(),
            })?;

        entry.metadata.status = status.clone();
        entry.updated_at = chrono::Utc::now();

        Ok(())
    }

    fn get_dependency_graph(&self) -> HashMap<String, Vec<String>> {
        self.services
            .iter()
            .map(|(name, entry)| (name.clone(), entry.metadata.dependencies.clone()))
            .collect()
    }
}

/// 注册表配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    /// 功能标志配置
    pub feature_flags: HashMap<String, bool>,
    /// 服务发现配置
    pub service_discovery: ServiceDiscoveryConfig,
}

/// 服务发现配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryConfig {
    /// 自动发现服务
    pub auto_discover: bool,
    /// 扫描路径
    pub scan_paths: Vec<String>,
    /// 服务过滤器
    /// 包含模式列表
    pub include_patterns: Vec<String>,
    /// 排除模式列表
    ///
    /// 用于过滤不需要的服务
    pub exclude_patterns: Vec<String>,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            feature_flags: HashMap::new(),
            service_discovery: ServiceDiscoveryConfig {
                auto_discover: true,
                scan_paths: vec!["services".to_string()],
                include_patterns: vec!["*".to_string()],
                exclude_patterns: vec![],
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_registration() {
        let mut registry = DefaultServiceRegistry::new();

        let metadata = ServiceMetadata {
            name: "test-service".to_string(),
            version: "1.0.0".to_string(),
            description: Some("测试服务".to_string()),
            dependencies: vec![],
            provides: vec!["test-feature".to_string()],
            status: ServiceStatus::Uninitialized,
            priority: 1,
        };

        assert!(registry.register_service(metadata).is_ok());
        assert!(registry.has_service("test-service"));
    }

    #[test]
    fn test_duplicate_registration() {
        let mut registry = DefaultServiceRegistry::new();

        let metadata = ServiceMetadata {
            name: "test-service".to_string(),
            version: "1.0.0".to_string(),
            description: None,
            dependencies: vec![],
            provides: vec![],
            status: ServiceStatus::Uninitialized,
            priority: 1,
        };

        registry.register_service(metadata.clone()).unwrap();

        let result = registry.register_service(metadata);
        assert!(matches!(
            result,
            Err(RegistryError::ServiceAlreadyExists { .. })
        ));
    }
}
