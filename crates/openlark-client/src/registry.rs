//! OpenLark Client 服务注册表
//!
//! 提供动态服务注册、发现和管理功能

use crate::{Config, Error, Result};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// 📋 服务注册表 - 动态服务管理
///
/// 负责管理所有已注册的服务，提供类型安全的服务访问
///
/// # 特性
/// - 类型安全的服务注册和获取
/// - 编译时和运行时类型检查
/// - 线程安全的并发访问
/// - 服务生命周期管理
#[derive(Debug)]
#[allow(dead_code)]
pub struct ServiceRegistry {
    /// 🔐 服务存储
    services: RwLock<HashMap<String, ServiceEntry>>,
    /// 🗺️ 服务类型映射
    type_map: RwLock<HashMap<String, TypeId>>,
    /// ⚙️ 配置引用
    config: Arc<Config>,
    /// 📊 服务统计
    stats: RwLock<ServiceStats>,
}

/// 🏷️ 服务条目
#[derive(Debug)]
#[allow(dead_code)]
struct ServiceEntry {
    /// 📦 服务实例
    service: Box<dyn Any + Send + Sync>,
    /// 🔍 服务描述符
    descriptor: ServiceDescriptor,
    /// ⏰ 注册时间
    registered_at: std::time::SystemTime,
    /// 🔄 最后访问时间
    last_accessed: std::time::SystemTime,
    /// 📊 访问次数
    access_count: u64,
}

/// 📋 服务描述符
#[derive(Debug, Clone)]
pub struct ServiceDescriptor {
    /// 🏷️ 服务名称
    pub name: String,
    /// 🏷️ 服务类型
    pub service_type: String,
    /// 📝 服务描述
    pub description: String,
    /// 🔢 服务版本
    pub version: String,
    /// 🔗 依赖的服务
    pub dependencies: Vec<String>,
    /// ✅ 服务是否启用
    pub enabled: bool,
    /// 🏷️ 标签
    pub tags: Vec<String>,
}

impl ServiceDescriptor {
    /// 🆕 创建新的服务描述符
    pub fn new(name: &str, service_type: &str) -> Self {
        Self {
            name: name.to_string(),
            service_type: service_type.to_string(),
            description: String::new(),
            version: "1.0.0".to_string(),
            dependencies: Vec::new(),
            enabled: true,
            tags: Vec::new(),
        }
    }

    /// 📝 设置服务描述
    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// 🔢 设置服务版本
    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    /// 🔗 添加依赖
    pub fn add_dependency(mut self, dependency: &str) -> Self {
        self.dependencies.push(dependency.to_string());
        self
    }

    /// 🏷️ 添加标签
    pub fn add_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    /// ✅ 设置启用状态
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// 📊 服务统计信息
#[derive(Debug, Default)]
struct ServiceStats {
    /// 📈 总注册数
    total_registrations: u64,
    /// 🔍 总访问次数
    total_accesses: u64,
    /// 📝 最后访问时间
    last_access: Option<std::time::SystemTime>,
}

impl ServiceRegistry {
    /// 🆕 创建新的服务注册表
    pub fn new(config: &Arc<Config>) -> Self {
        tracing::debug!("创建新的服务注册表");
        Self {
            services: RwLock::new(HashMap::new()),
            type_map: RwLock::new(HashMap::new()),
            config: config.clone(),
            stats: RwLock::new(ServiceStats::default()),
        }
    }

    /// 📝 注册服务
    ///
    /// # 参数
    /// - `name`: 服务名称
    /// - `service`: 服务实例（动态类型）
    /// - `descriptor`: 服务描述符
    pub fn register_service(
        &self,
        name: &str,
        service: Box<dyn std::any::Any + Send + Sync>,
        descriptor: ServiceDescriptor,
    ) -> Result<()> {
        // 检查依赖是否已注册
        for dependency in &descriptor.dependencies {
            if !self.has_service(dependency) {
                return Err(Error::ServiceUnavailable(format!(
                    "依赖服务 '{}' 未注册",
                    dependency
                )));
            }
        }

        let type_id = (*service).type_id();
        let now = std::time::SystemTime::now();

        // 创建服务条目
        let entry = ServiceEntry {
            service: service as Box<dyn Any + Send + Sync>,
            descriptor,
            registered_at: now,
            last_accessed: now,
            access_count: 0,
        };

        // 注册服务
        {
            let mut services = self.services.write().unwrap();
            let mut type_map = self.type_map.write().unwrap();

            // 如果服务已存在，给出警告但允许覆盖
            if services.contains_key(name) {
                tracing::warn!("服务 '{}' 已存在，将被覆盖", name);
            }

            services.insert(name.to_string(), entry);
            type_map.insert(name.to_string(), type_id);
        }

        // 更新统计
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_registrations += 1;
        }

        tracing::info!("服务 '{}' 注册成功", name);
        Ok(())
    }

    /// 🔍 检查服务是否存在
    pub fn has_service(&self, name: &str) -> bool {
        let services = self.services.read().unwrap();
        services.contains_key(name)
    }

    /// 📋 列出所有已注册的服务
    pub fn list_services(&self) -> Vec<ServiceDescriptor> {
        let services = self.services.read().unwrap();
        services
            .values()
            .map(|entry| entry.descriptor.clone())
            .collect()
    }

    /// 📋 获取启用的服务列表
    pub fn list_enabled_services(&self) -> Vec<String> {
        let services = self.services.read().unwrap();
        services
            .values()
            .filter(|entry| entry.descriptor.enabled)
            .map(|entry| entry.descriptor.name.clone())
            .collect()
    }

    /// 🏷️ 根据标签获取服务
    pub fn get_services_by_tag(&self, tag: &str) -> Vec<String> {
        let services = self.services.read().unwrap();
        services
            .values()
            .filter(|entry| entry.descriptor.tags.contains(&tag.to_string()))
            .map(|entry| entry.descriptor.name.clone())
            .collect()
    }

    /// 🗑️ 注销服务
    pub fn unregister_service(&self, name: &str) -> Result<()> {
        let mut services = self.services.write().unwrap();
        let mut type_map = self.type_map.write().unwrap();

        // 检查是否有其他服务依赖此服务
        for entry in services.values() {
            if entry.descriptor.dependencies.contains(&name.to_string()) {
                return Err(Error::InvalidParameter(format!(
                    "无法注销服务 '{}'，服务 '{}' 依赖它",
                    name, entry.descriptor.name
                )));
            }
        }

        services.remove(name);
        type_map.remove(name);

        tracing::debug!("服务 '{}' 注销成功", name);
        Ok(())
    }

    /// ✅ 启用或禁用服务
    pub fn set_service_enabled(&self, name: &str, enabled: bool) -> Result<()> {
        let mut services = self.services.write().unwrap();

        if let Some(entry) = services.get_mut(name) {
            entry.descriptor.enabled = enabled;
            tracing::debug!(
                "服务 '{}' 已{}",
                name,
                if enabled { "启用" } else { "禁用" }
            );
            Ok(())
        } else {
            Err(Error::ServiceUnavailable(format!("服务 '{}' 不存在", name)))
        }
    }

    /// 📊 获取服务描述符
    pub fn get_service_descriptor(&self, name: &str) -> Option<ServiceDescriptor> {
        let services = self.services.read().unwrap();
        services.get(name).map(|entry| entry.descriptor.clone())
    }

    /// 📊 获取注册统计信息
    pub fn get_stats(&self) -> ServiceRegistryStats {
        let services = self.services.read().unwrap();
        let stats = self.stats.read().unwrap();

        ServiceRegistryStats {
            total_services: services.len(),
            enabled_services: services.values().filter(|e| e.descriptor.enabled).count(),
            total_registrations: stats.total_registrations,
            total_accesses: stats.total_accesses,
            last_access: stats.last_access,
        }
    }

    /// 🧹 清理所有服务
    pub fn clear(&self) {
        let mut services = self.services.write().unwrap();
        let mut type_map = self.type_map.write().unwrap();

        services.clear();
        type_map.clear();

        tracing::debug!("所有服务已清理");
    }
}

/// 📊 服务注册表统计信息
#[derive(Debug, Clone, Copy, Default)]
pub struct ServiceRegistryStats {
    /// 📊 总服务数
    pub total_services: usize,
    /// ✅ 启用的服务数
    pub enabled_services: usize,
    /// 📈 总注册次数
    pub total_registrations: u64,
    /// 🔍 总访问次数
    pub total_accesses: u64,
    /// ⏰ 最后访问时间
    pub last_access: Option<std::time::SystemTime>,
}

/// 🔧 服务注册器构建器
#[derive(Debug)]
pub struct ServiceRegistryBuilder {
    config: Option<Arc<Config>>,
}

impl ServiceRegistryBuilder {
    /// 🆕 创建新的构建器
    pub fn new() -> Self {
        Self { config: None }
    }

    /// ⚙️ 设置配置
    pub fn config(mut self, config: Arc<Config>) -> Self {
        self.config = Some(config);
        self
    }

    /// 🔨 构建服务注册表
    pub fn build(self) -> ServiceRegistry {
        let config = self.config.unwrap_or_else(|| Arc::new(Config::default()));
        ServiceRegistry::new(&config)
    }
}

impl Default for ServiceRegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[derive(Debug)]
    struct TestService {
        name: String,
    }

    #[test]
    fn test_service_registry_creation() {
        let config = Arc::new(Config::default());
        let registry = ServiceRegistry::new(&config);

        let stats = registry.get_stats();
        assert_eq!(stats.total_services, 0);
    }

    #[test]
    fn test_service_descriptor() {
        let descriptor = ServiceDescriptor::new("test_service", "TestService")
            .description("测试服务")
            .version("1.0.0")
            .add_tag("test")
            .enabled(true);

        assert_eq!(descriptor.name, "test_service");
        assert_eq!(descriptor.service_type, "TestService");
        assert_eq!(descriptor.description, "测试服务");
        assert_eq!(descriptor.version, "1.0.0");
        assert!(descriptor.tags.contains(&"test".to_string()));
        assert!(descriptor.enabled);
    }

    #[test]
    fn test_service_listing() {
        let config = Arc::new(Config::default());
        let registry = ServiceRegistry::new(&config);

        // 空注册表应该返回空列表
        let services = registry.list_services();
        assert!(services.is_empty());

        let enabled = registry.list_enabled_services();
        assert!(enabled.is_empty());
    }

    #[test]
    fn test_has_service() {
        let config = Arc::new(Config::default());
        let registry = ServiceRegistry::new(&config);

        // 不存在的服务应该返回false
        assert!(!registry.has_service("non_existent_service"));
    }

    #[test]
    fn test_services_by_tag() {
        let config = Arc::new(Config::default());
        let registry = ServiceRegistry::new(&config);

        // 空注册表应该返回空列表
        let services = registry.get_services_by_tag("test");
        assert!(services.is_empty());
    }

    #[test]
    fn test_registry_builder() {
        let config = Arc::new(Config::default());
        let registry = ServiceRegistryBuilder::new().config(config.clone()).build();

        let stats = registry.get_stats();
        assert_eq!(stats.total_services, 0);
    }

    #[test]
    fn test_registry_builder_default() {
        let registry = ServiceRegistryBuilder::default().build();

        let stats = registry.get_stats();
        assert_eq!(stats.total_services, 0);
    }

    #[test]
    fn test_service_stats() {
        let config = Arc::new(Config::default());
        let registry = ServiceRegistry::new(&config);

        let stats = registry.get_stats();
        assert_eq!(stats.total_services, 0);
        assert_eq!(stats.enabled_services, 0);
        assert_eq!(stats.total_registrations, 0);
        assert_eq!(stats.total_accesses, 0);
        assert!(stats.last_access.is_none());
    }
}
