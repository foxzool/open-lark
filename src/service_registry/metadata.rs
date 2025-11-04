//! 服务元数据管理

use std::collections::HashMap;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

use super::service::{ServiceInfo, ServiceStatus};

/// 服务元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetadata {
    /// 服务信息映射
    services: HashMap<String, ServiceInfo>,
    /// 全局配置
    config: MetadataConfig,
    /// 创建时间
    created_at: SystemTime,
    /// 最后更新时间
    updated_at: SystemTime,
}

/// 元数据配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataConfig {
    /// 是否自动清理过期服务
    pub auto_cleanup: bool,
    /// 服务过期时间（秒）
    pub service_ttl: u64,
    /// 最大历史记录数
    pub max_history: usize,
    /// 是否启用统计信息
    pub enable_stats: bool,
}

impl Default for MetadataConfig {
    fn default() -> Self {
        Self {
            auto_cleanup: true,
            service_ttl: 3600, // 1小时
            max_history: 100,
            enable_stats: true,
        }
    }
}

impl ServiceMetadata {
    /// 创建新的服务元数据实例
    pub fn new() -> Self {
        let now = SystemTime::now();
        Self {
            services: HashMap::new(),
            config: MetadataConfig::default(),
            created_at: now,
            updated_at: now,
        }
    }

    /// 使用自定义配置创建元数据实例
    pub fn with_config(config: MetadataConfig) -> Self {
        let now = SystemTime::now();
        Self {
            services: HashMap::new(),
            config,
            created_at: now,
            updated_at: now,
        }
    }

    /// 注册服务信息
    pub fn register_service(&mut self, info: ServiceInfo) {
        let name = info.name.clone();
        self.services.insert(name, info);
        self.updated_at = SystemTime::now();
    }

    /// 更新服务信息
    pub fn update_service(&mut self, info: ServiceInfo) -> bool {
        let name = info.name.clone();
        if self.services.contains_key(&name) {
            self.services.insert(name, info);
            self.updated_at = SystemTime::now();
            true
        } else {
            false
        }
    }

    /// 获取服务信息
    pub fn get_service(&self, name: &str) -> Option<&ServiceInfo> {
        self.services.get(name)
    }

    /// 移除服务信息
    pub fn remove_service(&mut self, name: &str) -> Option<ServiceInfo> {
        let result = self.services.remove(name);
        if result.is_some() {
            self.updated_at = SystemTime::now();
        }
        result
    }

    /// 获取所有服务信息
    pub fn get_all_services(&self) -> &HashMap<String, ServiceInfo> {
        &self.services
    }

    /// 获取服务名称列表
    pub fn get_service_names(&self) -> Vec<String> {
        self.services.keys().cloned().collect()
    }

    /// 按状态筛选服务
    pub fn get_services_by_status(&self, status: ServiceStatus) -> Vec<&ServiceInfo> {
        self.services
            .values()
            .filter(|info| info.status == status)
            .collect()
    }

    /// 获取健康服务数量
    pub fn get_healthy_count(&self) -> usize {
        self.get_services_by_status(ServiceStatus::Healthy).len()
    }

    /// 获取总服务数量
    pub fn get_total_count(&self) -> usize {
        self.services.len()
    }

    /// 清理过期服务
    pub fn cleanup_expired_services(&mut self) -> usize {
        if !self.config.auto_cleanup {
            return 0;
        }

        let now = SystemTime::now();
        let ttl_duration = std::time::Duration::from_secs(self.config.service_ttl);
        let mut expired_keys = Vec::new();

        for (name, info) in &self.services {
            if let Ok(elapsed) = now.duration_since(info.registered_at) {
                if elapsed > ttl_duration {
                    expired_keys.push(name.clone());
                }
            }
        }

        let count = expired_keys.len();
        for key in expired_keys {
            self.services.remove(&key);
        }

        if count > 0 {
            self.updated_at = SystemTime::now();
        }

        count
    }

    /// 检查服务是否存在
    pub fn has_service(&self, name: &str) -> bool {
        self.services.contains_key(name)
    }

    /// 获取统计信息
    pub fn get_stats(&self) -> MetadataStats {
        let mut status_counts = HashMap::new();
        for info in self.services.values() {
            *status_counts.entry(info.status).or_insert(0) += 1;
        }

        MetadataStats {
            total_services: self.services.len(),
            status_counts,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    /// 更新配置
    pub fn update_config(&mut self, config: MetadataConfig) {
        self.config = config;
        self.updated_at = SystemTime::now();
    }

    /// 获取配置
    pub fn get_config(&self) -> &MetadataConfig {
        &self.config
    }
}

/// 元数据统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataStats {
    /// 总服务数量
    pub total_services: usize,
    /// 按状态统计的服务数量
    pub status_counts: HashMap<ServiceStatus, usize>,
    /// 创建时间
    pub created_at: SystemTime,
    /// 最后更新时间
    pub updated_at: SystemTime,
}

impl MetadataStats {
    /// 获取指定状态的服务数量
    pub fn get_count(&self, status: ServiceStatus) -> usize {
        self.status_counts.get(&status).copied().unwrap_or(0)
    }

    /// 检查是否有服务
    pub fn has_services(&self) -> bool {
        self.total_services > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_metadata() {
        let mut metadata = ServiceMetadata::new();

        // 创建测试服务信息
        let info = ServiceInfo {
            name: "test-service".to_string(),
            version: "1.0.0".to_string(),
            status: ServiceStatus::Healthy,
            description: "Test service".to_string(),
            registered_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        };

        // 注册服务
        metadata.register_service(info);

        assert_eq!(metadata.get_total_count(), 1);
        assert!(metadata.has_service("test-service"));
        assert!(metadata.get_service("test-service").is_some());

        // 测试按状态筛选
        let healthy_services = metadata.get_services_by_status(ServiceStatus::Healthy);
        assert_eq!(healthy_services.len(), 1);

        let unhealthy_services = metadata.get_services_by_status(ServiceStatus::Unhealthy);
        assert_eq!(unhealthy_services.len(), 0);
    }

    #[test]
    fn test_metadata_stats() {
        let mut metadata = ServiceMetadata::new();

        // 添加多个不同状态的服务
        let info1 = ServiceInfo {
            name: "service1".to_string(),
            version: "1.0.0".to_string(),
            status: ServiceStatus::Healthy,
            description: "Service 1".to_string(),
            registered_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        };

        let info2 = ServiceInfo {
            name: "service2".to_string(),
            version: "1.0.0".to_string(),
            status: ServiceStatus::Unhealthy,
            description: "Service 2".to_string(),
            registered_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        };

        metadata.register_service(info1);
        metadata.register_service(info2);

        let stats = metadata.get_stats();
        assert_eq!(stats.total_services, 2);
        assert_eq!(stats.get_count(ServiceStatus::Healthy), 1);
        assert_eq!(stats.get_count(ServiceStatus::Unhealthy), 1);
    }

    #[test]
    fn test_service_cleanup() {
        let config = MetadataConfig {
            auto_cleanup: true,
            service_ttl: 0, // 立即过期
            max_history: 100,
            enable_stats: true,
        };

        let mut metadata = ServiceMetadata::with_config(config);

        // 注册服务
        let info = ServiceInfo {
            name: "test-service".to_string(),
            version: "1.0.0".to_string(),
            status: ServiceStatus::Healthy,
            description: "Test service".to_string(),
            registered_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        };

        metadata.register_service(info);
        assert_eq!(metadata.get_total_count(), 1);

        // 清理过期服务
        let cleaned = metadata.cleanup_expired_services();
        assert_eq!(cleaned, 1);
        assert_eq!(metadata.get_total_count(), 0);
    }
}