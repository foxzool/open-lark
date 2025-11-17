//! OpenLark 服务注册系统
//!
//! 提供动态服务发现、注册和管理功能。

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::any::{Any, TypeId};

use openlark_core::SDKResult;

use super::{
    traits::{UnifiedService, ServiceDescriptor, ServiceStatus},
    error::{UnifiedError, UnifiedResult},
};

/// 服务信息
#[derive(Debug, Clone)]
pub struct ServiceInfo {
    /// 服务描述
    pub descriptor: ServiceDescriptor,
    /// 服务实例ID
    pub instance_id: String,
    /// 注册时间
    pub registered_at: chrono::DateTime<chrono::Utc>,
    /// 最后更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// 服务是否启用
    pub enabled: bool,
}

impl ServiceInfo {
    /// 创建新的服务信息
    pub fn new(descriptor: ServiceDescriptor) -> Self {
        let now = chrono::Utc::now();
        Self {
            instance_id: uuid::Uuid::new_v4().to_string(),
            descriptor,
            registered_at: now,
            updated_at: now,
            enabled: true,
        }
    }

    /// 更新状态
    pub fn update_status(&mut self, status: ServiceStatus) {
        self.descriptor.status = status;
        self.updated_at = chrono::Utc::now();
    }

    /// 启用/禁用服务
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.updated_at = chrono::Utc::now();
    }
}

/// 服务存储槽
struct ServiceSlot {
    service: Box<dyn UnifiedService>,
    info: ServiceInfo,
}

impl std::fmt::Debug for ServiceSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServiceSlot")
            .field("info", &self.info)
            .field("service_name", &self.service.name())
            .field("service_version", &self.service.version())
            .finish()
    }
}

/// 服务注册表
///
/// 管理所有服务的注册、发现和生命周期。
#[derive(Debug)]
pub struct ServiceRegistry {
    /// 服务存储（按名称索引）
    services: RwLock<HashMap<String, ServiceSlot>>,
    /// 服务类型映射（用于类型安全的查找）
    type_map: RwLock<HashMap<TypeId, String>>,
    /// 依赖关系图
    dependency_graph: RwLock<HashMap<String, Vec<String>>>,
}

impl ServiceRegistry {
    /// 创建新的服务注册表
    pub fn new() -> Self {
        Self {
            services: RwLock::new(HashMap::new()),
            type_map: RwLock::new(HashMap::new()),
            dependency_graph: RwLock::new(HashMap::new()),
        }
    }

    /// 注册服务
    ///
    /// # 参数
    /// - `service_name`: 服务名称
    /// - `service`: 服务实例
    pub fn register<S>(&self, service_name: &str, service: S) -> UnifiedResult<()>
    where
        S: UnifiedService + 'static,
    {
        // 检查服务是否已存在
        {
            let services = self.services.read().unwrap();
            if services.contains_key(service_name) {
                return Err(UnifiedError::ServiceAlreadyExists(service_name.to_string()));
            }
        }

        // 创建服务信息
        let descriptor = service.descriptor();
        let info = ServiceInfo::new(descriptor);

        // 检查依赖关系
        self.check_dependencies(&info.dependencies)?;

        // 创建服务槽
        let slot = ServiceSlot {
            service: Box::new(service),
            info,
        };

        // 注册服务
        {
            let mut services = self.services.write().unwrap();
            let mut type_map = self.type_map.write().unwrap();
            let mut dependency_graph = self.dependency_graph.write().unwrap();

            services.insert(service_name.to_string(), slot);
            type_map.insert(TypeId::of::<S>(), service_name.to_string());

            // 更新依赖图
            if !dependency_graph.contains_key(service_name) {
                dependency_graph.insert(service_name.to_string(), Vec::new());
            }

            for dependency in &services[service_name].info.dependencies {
                dependency_graph
                    .entry(dependency.clone())
                    .or_insert_with(Vec::new)
                    .push(service_name.to_string());
            }
        }

        tracing::info!("服务 {} 注册成功", service_name);
        Ok(())
    }

    /// 注销服务
    ///
    /// # 参数
    /// - `service_name`: 服务名称
    pub fn unregister(&self, service_name: &str) -> UnifiedResult<()> {
        let mut services = self.services.write().unwrap();
        let mut type_map = self.type_map.write().unwrap();
        let mut dependency_graph = self.dependency_graph.write().unwrap();

        // 检查是否有其他服务依赖此服务
        if let Some(dependents) = dependency_graph.get(service_name) {
            if !dependents.is_empty() {
                return Err(UnifiedError::ServiceHasDependents {
                    service: service_name.to_string(),
                    dependents: dependents.clone(),
                });
            }
        }

        // 移除服务
        if let Some(slot) = services.remove(service_name) {
            // 从类型映射中移除
            type_map.retain(|_, name| name != service_name);

            // 从依赖图中移除
            dependency_graph.remove(service_name);

            tracing::info!("服务 {} 注销成功", service_name);
            Ok(())
        } else {
            Err(UnifiedError::ServiceNotFound(service_name.to_string()))
        }
    }

    /// 获取服务（类型安全）
    ///
    /// # 类型参数
    /// - `T`: 服务类型
    ///
    /// # 返回值
    /// 返回服务引用，如果服务不存在或类型不匹配则返回None
    pub fn get_service<T>(&self, service_name: &str) -> Option<&T>
    where
        T: UnifiedService + 'static,
    {
        let services = self.services.read().unwrap();
        services
            .get(service_name)
            .and_then(|slot| slot.service.as_any().downcast_ref::<T>())
    }

    /// 获取可变服务（类型安全）
    pub fn get_service_mut<T>(&self, service_name: &str) -> Option<&mut T>
    where
        T: UnifiedService + 'static,
    {
        let mut services = self.services.write().unwrap();
        services
            .get_mut(service_name)
            .and_then(|slot| slot.service.as_any_mut().downcast_mut::<T>())
    }

    /// 获取服务信息
    pub fn get_service_info(&self, service_name: &str) -> Option<ServiceInfo> {
        let services = self.services.read().unwrap();
        services.get(service_name).map(|slot| slot.info.clone())
    }

    /// 获取所有服务名称
    pub fn list_services(&self) -> Vec<String> {
        let services = self.services.read().unwrap();
        services.keys().cloned().collect()
    }

    /// 获取所有服务描述
    pub fn list_descriptors(&self) -> Vec<ServiceDescriptor> {
        let services = self.services.read().unwrap();
        services
            .values()
            .map(|slot| slot.descriptor.clone())
            .collect()
    }

    /// 按状态筛选服务
    pub fn filter_by_status(&self, status: ServiceStatus) -> Vec<String> {
        let services = self.services.read().unwrap();
        services
            .values()
            .filter(|slot| slot.info.descriptor.status == status)
            .map(|slot| slot.info.descriptor.name.clone())
            .collect()
    }

    /// 按标签筛选服务
    pub fn filter_by_tag(&self, tag: &str) -> Vec<String> {
        let services = self.services.read().unwrap();
        services
            .values()
            .filter(|slot| slot.info.descriptor.tags.contains(&tag.to_string()))
            .map(|slot| slot.info.descriptor.name.clone())
            .collect()
    }

    /// 检查依赖关系
    fn check_dependencies(&self, dependencies: &[String]) -> UnifiedResult<()> {
        let services = self.services.read().unwrap();

        for dependency in dependencies {
            if !services.contains_key(dependency) {
                return Err(UnifiedError::DependencyNotFound(dependency.clone()));
            }
        }

        Ok(())
    }

    /// 检查循环依赖
    pub fn check_circular_dependencies(&self) -> UnifiedResult<Vec<String>> {
        let dependency_graph = self.dependency_graph.read().unwrap();

        let mut visited = HashMap::new();
        let mut recursion_stack = Vec::new();
        let mut cycles = Vec::new();

        for service_name in dependency_graph.keys() {
            if visited.get(service_name).is_none() {
                if let Some(cycle) = self.dfs_cycle_check(
                    service_name,
                    &dependency_graph,
                    &mut visited,
                    &mut recursion_stack,
                ) {
                    cycles.push(cycle);
                }
            }
        }

        Ok(cycles)
    }

    /// 深度优先搜索检查循环依赖
    fn dfs_cycle_check(
        &self,
        node: &str,
        graph: &HashMap<String, Vec<String>>,
        visited: &mut HashMap<String, bool>,
        recursion_stack: &mut Vec<String>,
    ) -> Option<String> {
        visited.insert(node.to_string(), true);
        recursion_stack.push(node.to_string());

        if let Some(dependencies) = graph.get(node) {
            for dependency in dependencies {
                if visited.get(dependency).is_none() {
                    if let Some(cycle) = self.dfs_cycle_check(dependency, graph, visited, recursion_stack) {
                        return Some(cycle);
                    }
                } else if recursion_stack.contains(dependency) {
                    // 找到循环依赖
                    let cycle_start = recursion_stack.iter().position(|n| n == dependency).unwrap();
                    let cycle = recursion_stack[cycle_start..].join(" -> ");
                    return Some(format!("{} -> {}", cycle, dependency));
                }
            }
        }

        recursion_stack.pop();
        None
    }

    /// 获取服务的直接依赖
    pub fn get_dependencies(&self, service_name: &str) -> Vec<String> {
        let services = self.services.read().unwrap();
        services
            .get(service_name)
            .map(|slot| slot.info.dependencies.clone())
            .unwrap_or_default()
    }

    /// 获取依赖此服务的其他服务
    pub fn get_dependents(&self, service_name: &str) -> Vec<String> {
        let dependency_graph = self.dependency_graph.read().unwrap();
        dependency_graph
            .get(service_name)
            .cloned()
            .unwrap_or_default()
    }

    /// 获取服务的启动顺序
    pub fn get_startup_order(&self) -> UnifiedResult<Vec<String>> {
        let dependency_graph = self.dependency_graph.read().unwrap();

        // 拓扑排序
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut result = Vec::new();

        // 计算入度
        for (service, dependencies) in dependency_graph.iter() {
            in_degree.entry(service.clone()).or_insert(0);
            for dependency in dependencies {
                *in_degree.entry(dependency.clone()).or_insert(0) += 1;
            }
        }

        // 找到入度为0的节点
        let mut queue: Vec<String> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(name, _)| name.clone())
            .collect();

        while let Some(current) = queue.pop() {
            result.push(current.clone());

            // 更新相邻节点的入度
            if let Some(dependents) = dependency_graph.get(&current) {
                for dependent in dependents {
                    if let Some(degree) = in_degree.get_mut(dependent) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push(dependent.clone());
                        }
                    }
                }
            }
        }

        // 检查是否存在循环依赖
        if result.len() != in_degree.len() {
            return Err(UnifiedError::CircularDependency(
                "服务依赖图中存在循环依赖".to_string(),
            ));
        }

        Ok(result)
    }

    /// 统计服务信息
    pub fn statistics(&self) -> RegistryStatistics {
        let services = self.services.read().unwrap();
        let total = services.len();
        let mut running = 0;
        let mut stopped = 0;
        let mut error = 0;

        for slot in services.values() {
            match slot.info.descriptor.status {
                ServiceStatus::Running => running += 1,
                ServiceStatus::Stopped | ServiceStatus::Uninitialized => stopped += 1,
                ServiceStatus::Error => error += 1,
                _ => {}
            }
        }

        RegistryStatistics {
            total,
            running,
            stopped,
            error,
        }
    }
}

/// 注册表统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryStatistics {
    /// 总服务数
    pub total: usize,
    /// 运行中的服务数
    pub running: usize,
    /// 已停止的服务数
    pub stopped: usize,
    /// 错误状态的服务数
    pub error: usize,
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unified::traits::ServiceStatus;

    // 创建一个测试服务
    #[derive(Debug)]
    struct TestService {
        name: String,
        status: ServiceStatus,
    }

    impl UnifiedService for TestService {
        type Config = ();
        type Error = UnifiedError;

        fn name(&self) -> &'static str {
            "test_service"
        }

        fn version(&self) -> &'static str {
            "1.0.0"
        }

        async fn configure(&mut self, _config: Self::Config) -> SDKResult<()> {
            Ok(())
        }

        fn is_available(&self) -> bool {
            true
        }

        fn status(&self) -> ServiceStatus {
            self.status
        }

        fn descriptor(&self) -> ServiceDescriptor {
            ServiceDescriptor::new("test_service", "1.0.0", "测试服务")
        }
    }

    #[tokio::test]
    async fn test_service_registration() {
        let registry = ServiceRegistry::new();
        let service = TestService {
            name: "test".to_string(),
            status: ServiceStatus::Running,
        };

        // 注册服务
        assert!(registry.register("test_service", service).is_ok());

        // 检查服务列表
        let services = registry.list_services();
        assert!(services.contains(&"test_service"));
    }

    #[test]
    fn test_circular_dependency_check() {
        let registry = ServiceRegistry::new();

        // 检查循环依赖（无服务）
        let cycles = registry.check_circular_dependencies().unwrap();
        assert!(cycles.is_empty());
    }

    #[test]
    fn test_statistics() {
        let registry = ServiceRegistry::new();
        let stats = registry.statistics();

        assert_eq!(stats.total, 0);
        assert_eq!(stats.running, 0);
        assert_eq!(stats.stopped, 0);
        assert_eq!(stats.error, 0);
    }
}