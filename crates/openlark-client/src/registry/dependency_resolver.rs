//! 依赖解析器
//!
//! 处理服务间的依赖关系，解决循环依赖和依赖排序问题

use std::collections::{HashMap, HashSet, VecDeque};
use thiserror::Error;

/// 依赖解析错误
#[derive(Error, Debug, Clone)]
pub enum DependencyError {
    /// 循环依赖错误
    ///
    /// 当检测到服务间存在循环依赖关系时触发
    #[error("检测到循环依赖: {chain}")]
    CircularDependency {
        /// 循环依赖链
        chain: String,
    },

    /// 缺少依赖错误
    ///
    /// 当服务依赖的其他服务不存在时触发
    #[error("缺少依赖: {missing_dependencies:?}")]
    MissingDependencies {
        /// 缺失的依赖服务列表
        missing_dependencies: Vec<String>,
    },

    /// 服务不存在错误
    ///
    /// 当尝试访问不存在的服务时触发
    #[error("服务 '{service}' 不存在")]
    ServiceNotFound {
        /// 不存在的服务名称
        service: String,
    },
}

/// 依赖解析结果类型
pub type DependencyResult<T> = Result<T, DependencyError>;

/// 依赖解析器
#[derive(Debug)]
pub struct DependencyResolver {
    /// 缓存已解析的依赖顺序
    #[allow(dead_code)]
    resolved_orders: HashMap<String, Vec<String>>,
}

impl DependencyResolver {
    /// 创建新的依赖解析器
    pub fn new() -> Self {
        Self {
            resolved_orders: HashMap::new(),
        }
    }

    /// 解析依赖关系，返回正确的启动顺序
    pub fn resolve_dependencies(
        &self,
        dependency_graph: HashMap<String, Vec<String>>,
    ) -> DependencyResult<Vec<String>> {
        // 检查循环依赖
        self.detect_circular_dependencies(&dependency_graph)?;

        // 拓扑排序
        self.topological_sort(dependency_graph)
    }

    /// 检测循环依赖
    fn detect_circular_dependencies(
        &self,
        dependency_graph: &HashMap<String, Vec<String>>,
    ) -> DependencyResult<()> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        for service in dependency_graph.keys() {
            if !visited.contains(service) {
                if let Err(chain) = self.dfs_detect_cycle(
                    service,
                    dependency_graph,
                    &mut visited,
                    &mut rec_stack,
                    &mut path,
                ) {
                    return Err(DependencyError::CircularDependency { chain });
                }
            }
        }

        Ok(())
    }

    /// 深度优先搜索检测循环依赖
    #[allow(clippy::only_used_in_recursion)]
    fn dfs_detect_cycle(
        &self,
        service: &str,
        dependency_graph: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Result<(), String> {
        visited.insert(service.to_string());
        rec_stack.insert(service.to_string());
        path.push(service.to_string());

        if let Some(dependencies) = dependency_graph.get(service) {
            for dep in dependencies {
                if !visited.contains(dep) {
                    self.dfs_detect_cycle(dep, dependency_graph, visited, rec_stack, path)?;
                } else if rec_stack.contains(dep) {
                    // 找到循环依赖
                    let cycle_start = path.iter().position(|s| s == dep).unwrap();
                    let cycle_path = path[cycle_start..]
                        .iter()
                        .chain(std::iter::once(dep))
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(" -> ");
                    return Err(cycle_path);
                }
            }
        }

        rec_stack.remove(service);
        path.pop();
        Ok(())
    }

    /// 拓扑排序
    fn topological_sort(
        &self,
        dependency_graph: HashMap<String, Vec<String>>,
    ) -> DependencyResult<Vec<String>> {
        let mut in_degree = HashMap::new();
        let mut graph = HashMap::new();

        // 初始化入度表和邻接表
        for service in dependency_graph.keys() {
            in_degree.insert(service.clone(), 0);
            graph.insert(service.clone(), Vec::new());
        }

        // 构建图和计算入度
        for (service, dependencies) in &dependency_graph {
            for dep in dependencies {
                // 检查依赖是否存在
                if !dependency_graph.contains_key(dep) {
                    return Err(DependencyError::MissingDependencies {
                        missing_dependencies: vec![dep.clone()],
                    });
                }

                graph.get_mut(dep).unwrap().push(service.clone());
                *in_degree.get_mut(service).unwrap() += 1;
            }
        }

        // 使用队列进行拓扑排序
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        // 找到所有入度为0的节点
        for (service, degree) in &in_degree {
            if *degree == 0 {
                queue.push_back(service.clone());
            }
        }

        while let Some(current) = queue.pop_front() {
            result.push(current.clone());

            // 更新相邻节点的入度
            if let Some(neighbors) = graph.get(&current) {
                for neighbor in neighbors {
                    let degree = in_degree.get_mut(neighbor).unwrap();
                    *degree -= 1;

                    if *degree == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        // 检查是否所有节点都被处理（确保没有循环依赖）
        if result.len() != dependency_graph.len() {
            return Err(DependencyError::CircularDependency {
                chain: "未知循环依赖".to_string(),
            });
        }

        Ok(result)
    }

    /// 计算服务的启动优先级
    pub fn calculate_priorities(
        &self,
        dependency_graph: &HashMap<String, Vec<String>>,
    ) -> DependencyResult<HashMap<String, u32>> {
        let sorted_services = self.resolve_dependencies(dependency_graph.clone())?;
        let mut priorities = HashMap::new();

        for (index, service) in sorted_services.iter().enumerate() {
            priorities.insert(service.clone(), index as u32);
        }

        Ok(priorities)
    }

    /// 获取服务的直接依赖
    pub fn get_direct_dependencies(
        &self,
        service: &str,
        dependency_graph: &HashMap<String, Vec<String>>,
    ) -> DependencyResult<Vec<String>> {
        dependency_graph
            .get(service)
            .cloned()
            .ok_or_else(|| DependencyError::ServiceNotFound {
                service: service.to_string(),
            })
    }

    /// 获取服务的所有依赖（包括间接依赖）
    pub fn get_all_dependencies(
        &self,
        service: &str,
        dependency_graph: &HashMap<String, Vec<String>>,
    ) -> DependencyResult<HashSet<String>> {
        let mut all_deps = HashSet::new();
        let mut to_visit = vec![service.to_string()];

        while let Some(current) = to_visit.pop() {
            if let Some(deps) = dependency_graph.get(&current) {
                for dep in deps {
                    if !all_deps.contains(dep) {
                        all_deps.insert(dep.clone());
                        to_visit.push(dep.clone());
                    }
                }
            }
        }

        // 移除自身
        all_deps.remove(service);
        Ok(all_deps)
    }

    /// 检查服务是否可以启动（所有依赖都已就绪）
    pub fn can_start(
        &self,
        service: &str,
        dependency_graph: &HashMap<String, Vec<String>>,
        running_services: &HashSet<String>,
    ) -> DependencyResult<bool> {
        let deps = self.get_all_dependencies(service, dependency_graph)?;
        Ok(deps.is_subset(running_services))
    }

    /// 获取下一个可以启动的服务
    pub fn get_next_startable_service(
        &self,
        dependency_graph: &HashMap<String, Vec<String>>,
        running_services: &HashSet<String>,
        pending_services: &HashSet<String>,
    ) -> DependencyResult<Option<String>> {
        for service in pending_services {
            if self.can_start(service, dependency_graph, running_services)? {
                return Ok(Some(service.clone()));
            }
        }

        Ok(None)
    }

    /// 生成依赖报告
    pub fn generate_dependency_report(
        &self,
        dependency_graph: &HashMap<String, Vec<String>>,
    ) -> DependencyResult<DependencyReport> {
        let sorted_services = self.resolve_dependencies(dependency_graph.clone())?;
        let priorities = self.calculate_priorities(dependency_graph)?;

        let mut service_details = HashMap::new();

        for service in dependency_graph.keys() {
            let direct_deps = self.get_direct_dependencies(service, dependency_graph)?;
            let all_deps = self.get_all_dependencies(service, dependency_graph)?;

            service_details.insert(
                service.clone(),
                ServiceDependencyDetail {
                    name: service.clone(),
                    direct_dependencies: direct_deps,
                    all_dependencies: all_deps.into_iter().collect(),
                    priority: priorities.get(service).copied().unwrap_or(0),
                },
            );
        }

        Ok(DependencyReport {
            total_services: dependency_graph.len(),
            sorted_services,
            service_details,
            has_circular_dependencies: false,
        })
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// 服务依赖详情
#[derive(Debug, Clone)]
pub struct ServiceDependencyDetail {
    /// 服务名称
    pub name: String,
    /// 直接依赖
    pub direct_dependencies: Vec<String>,
    /// 所有依赖（包括间接依赖）
    pub all_dependencies: Vec<String>,
    /// 启动优先级
    pub priority: u32,
}

/// 依赖关系报告
#[derive(Debug, Clone)]
pub struct DependencyReport {
    /// 总服务数
    pub total_services: usize,
    /// 排序后的服务列表
    pub sorted_services: Vec<String>,
    /// 服务依赖详情
    pub service_details: HashMap<String, ServiceDependencyDetail>,
    /// 是否存在循环依赖
    pub has_circular_dependencies: bool,
}

impl DependencyReport {
    /// 生成文本报告
    pub fn to_text(&self) -> String {
        let mut report = String::new();

        report.push_str("# 依赖关系分析报告\n\n");
        report.push_str(&format!("📊 **总服务数**: {}\n", self.total_services));
        report.push_str(&format!(
            "🔄 **循环依赖**: {}\n\n",
            if self.has_circular_dependencies {
                "是"
            } else {
                "否"
            }
        ));

        report.push_str("## 📋 服务启动顺序\n\n");
        for (index, service) in self.sorted_services.iter().enumerate() {
            if let Some(detail) = self.service_details.get(service) {
                report.push_str(&format!(
                    "{}. **{}** (优先级: {})\n",
                    index + 1,
                    detail.name,
                    detail.priority
                ));

                if !detail.direct_dependencies.is_empty() {
                    report.push_str(&format!(
                        "   - 依赖: {}\n",
                        detail.direct_dependencies.join(", ")
                    ));
                }
                report.push('\n');
            }
        }

        report.push_str("## 🔍 详细依赖关系\n\n");
        for detail in self.service_details.values() {
            report.push_str(&format!("### {}\n", detail.name));
            report.push_str(&format!(
                "- 直接依赖: {}\n",
                if detail.direct_dependencies.is_empty() {
                    "无".to_string()
                } else {
                    detail.direct_dependencies.join(", ")
                }
            ));
            report.push_str(&format!(
                "- 全部依赖: {}\n",
                if detail.all_dependencies.is_empty() {
                    "无".to_string()
                } else {
                    detail.all_dependencies.join(", ")
                }
            ));
            report.push_str(&format!("- 启动优先级: {}\n\n", detail.priority));
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_dependencies() {
        let resolver = DependencyResolver::new();

        let mut graph = HashMap::new();
        graph.insert("service-a".to_string(), vec![]);
        graph.insert("service-b".to_string(), vec!["service-a".to_string()]);
        graph.insert("service-c".to_string(), vec!["service-b".to_string()]);

        let result = resolver.resolve_dependencies(graph);
        assert!(result.is_ok());

        let sorted = result.unwrap();
        assert_eq!(sorted[0], "service-a");
        assert_eq!(sorted[1], "service-b");
        assert_eq!(sorted[2], "service-c");
    }

    #[test]
    fn test_circular_dependency_detection() {
        let resolver = DependencyResolver::new();

        let mut graph = HashMap::new();
        graph.insert("service-a".to_string(), vec!["service-b".to_string()]);
        graph.insert("service-b".to_string(), vec!["service-c".to_string()]);
        graph.insert("service-c".to_string(), vec!["service-a".to_string()]);

        let result = resolver.resolve_dependencies(graph);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(DependencyError::CircularDependency { .. })
        ));
    }

    #[test]
    fn test_dependency_report() {
        let resolver = DependencyResolver::new();

        let mut graph = HashMap::new();
        graph.insert("database".to_string(), vec![]);
        graph.insert("cache".to_string(), vec![]);
        graph.insert("auth".to_string(), vec!["database".to_string()]);
        graph.insert(
            "api".to_string(),
            vec!["auth".to_string(), "cache".to_string()],
        );

        let report = resolver.generate_dependency_report(&graph);
        assert!(report.is_ok());

        let report = report.unwrap();
        assert_eq!(report.total_services, 4);
        assert!(!report.has_circular_dependencies);

        // 验证启动顺序
        let first_services: HashSet<_> = report.sorted_services.iter().take(2).collect();
        assert!(first_services.contains(&"database".to_string()));
        assert!(first_services.contains(&"cache".to_string()));
    }
}
