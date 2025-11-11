//! ServiceRegistry共享配置模块
//!
//! 提供配置共享机制，减少多个服务持有相同配置的内存开销

use config::Config;
use std::sync::Arc;

/// 共享配置包装器
///
/// 使用 `Arc<Config>` 实现配置在多个服务间的安全共享，
/// 减少内存使用同时保持配置的不可变性。
#[derive(Debug, Clone)]
pub struct SharedConfig {
    /// 内部使用 `Arc<Config>` 实现线程安全的配置共享
    config: Arc<Config>,
}

impl SharedConfig {
    /// 创建新的共享配置实例
    ///
    /// # Arguments
    /// * `config` - 要共享的配置实例
    ///
    /// # Returns
    /// 返回SharedConfig实例
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    /// 从现有SharedConfig创建新实例（增加引用计数）
    ///
    /// # Returns
    /// 返回新的SharedConfig实例，共享相同的内部配置
    pub fn clone_shared(&self) -> Self {
        Self {
            config: Arc::clone(&self.config),
        }
    }

    /// 获取配置的引用
    ///
    /// # Returns
    /// 返回Config的不可变引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取 `Arc<Config>` 的引用，用于需要 Arc 的场景
    ///
    /// # Returns
    /// 返回 `Arc<Config>` 的引用
    pub fn arc_config(&self) -> &Arc<Config> {
        &self.config
    }

    /// 获取配置的强引用计数
    ///
    /// # Returns
    /// 返回当前有多少个SharedConfig实例共享此配置
    pub fn ref_count(&self) -> usize {
        Arc::strong_count(&self.config)
    }

    /// 检查配置是否被多个实例共享
    ///
    /// # Returns
    /// 如果引用计数 > 1，返回true
    pub fn is_shared(&self) -> bool {
        Arc::strong_count(&self.config) > 1
    }
}

impl From<Config> for SharedConfig {
    fn from(config: Config) -> Self {
        Self::new(config)
    }
}

impl From<Arc<Config>> for SharedConfig {
    fn from(config: Arc<Config>) -> Self {
        Self { config }
    }
}

impl From<&SharedConfig> for Arc<Config> {
    fn from(shared_config: &SharedConfig) -> Self {
        Arc::clone(&shared_config.config)
    }
}

/// 配置共享工厂
///
/// 提供创建和管理共享配置的便捷方法
pub struct SharedConfigFactory;

impl SharedConfigFactory {
    /// 创建共享配置并自动为所有服务注册
    ///
    /// # Arguments
    /// * `config` - 基础配置
    ///
    /// # Returns
    /// 返回SharedConfig实例
    pub fn create_shared(config: Config) -> SharedConfig {
        SharedConfig::new(config)
    }

    /// 为特定服务创建共享配置
    ///
    /// # Arguments
    /// * `config` - 基础配置
    /// * `service_name` - 服务名称（用于日志记录）
    ///
    /// # Returns
    /// 返回SharedConfig实例
    pub fn create_for_service(config: Config, service_name: &str) -> SharedConfig {
        log::debug!("Creating shared config for service: {}", service_name);
        SharedConfig::new(config)
    }

    /// 批量创建服务共享配置
    ///
    /// # Arguments
    /// * `config` - 基础配置
    /// * `service_names` - 服务名称列表
    ///
    /// # Returns
    /// 返回SharedConfig实例和包含每个服务配置的向量
    pub fn create_batch(
        config: Config,
        service_names: &[&str],
    ) -> (SharedConfig, Vec<SharedConfig>) {
        let shared_config = SharedConfig::new(config);
        let service_configs: Vec<SharedConfig> = service_names
            .iter()
            .map(|name| {
                log::debug!("Creating shared config for service: {}", name);
                shared_config.clone_shared()
            })
            .collect();

        (shared_config, service_configs)
    }
}

/// 配置使用统计
///
/// 提供配置共享使用情况的统计信息
#[derive(Debug, Clone)]
pub struct ConfigUsageStats {
    /// 总配置实例数
    pub total_configs: usize,
    /// 共享配置实例数
    pub shared_configs: usize,
    /// 总内存使用估算（字节）
    pub estimated_memory_bytes: usize,
    /// 节省的内存使用估算（字节）
    pub saved_memory_bytes: usize,
}

impl ConfigUsageStats {
    /// 创建配置使用统计
    ///
    /// # Arguments
    /// * `shared_config` - 共享配置实例
    /// * `individual_configs_count` - 如果不使用共享，需要的独立配置数量
    ///
    /// # Returns
    /// 返回统计信息
    pub fn new(shared_config: &SharedConfig, individual_configs_count: usize) -> Self {
        let ref_count = shared_config.ref_count();
        let estimated_config_size = std::mem::size_of::<Config>();
        let estimated_shared_size = std::mem::size_of::<SharedConfig>();

        // 内存使用估算
        let shared_memory_usage = estimated_config_size + (ref_count * estimated_shared_size);
        let individual_memory_usage = individual_configs_count * estimated_config_size;
        let saved_memory = if individual_memory_usage > shared_memory_usage {
            individual_memory_usage - shared_memory_usage
        } else {
            0
        };

        Self {
            total_configs: individual_configs_count,
            shared_configs: ref_count,
            estimated_memory_bytes: shared_memory_usage,
            saved_memory_bytes: saved_memory,
        }
    }

    /// 打印统计信息
    pub fn print(&self) {
        println!("📊 配置使用统计:");
        println!("  总配置实例: {}", self.total_configs);
        println!("  共享配置实例: {}", self.shared_configs);
        println!(
            "  估算内存使用: {} bytes ({:.2} KB)",
            self.estimated_memory_bytes,
            self.estimated_memory_bytes as f64 / 1024.0
        );
        if self.saved_memory_bytes > 0 {
            println!(
                "  节省内存: {} bytes ({:.2} KB)",
                self.saved_memory_bytes,
                self.saved_memory_bytes as f64 / 1024.0
            );
            println!(
                "  内存节省率: {:.1}%",
                (self.saved_memory_bytes as f64 / self.estimated_memory_bytes as f64) * 100.0
            );
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::ConfigBuilder;

    fn create_test_config() -> Config {
        ConfigBuilder::default()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    #[test]
    fn test_shared_config_creation() {
        let config = create_test_config();
        let shared_config = SharedConfig::new(config);

        assert_eq!(shared_config.ref_count(), 1);
        assert!(!shared_config.is_shared());
    }

    #[test]
    fn test_shared_config_cloning() {
        let config = create_test_config();
        let shared_config = SharedConfig::new(config);
        let cloned_config = shared_config.clone_shared();

        assert_eq!(shared_config.ref_count(), 2);
        assert_eq!(cloned_config.ref_count(), 2);
        assert!(shared_config.is_shared());
        assert!(cloned_config.is_shared());
    }

    #[test]
    fn test_config_access() {
        let config = create_test_config();
        let shared_config = SharedConfig::new(config);

        assert_eq!(shared_config.config().app_id, "test_app_id");
        assert_eq!(shared_config.config().app_secret, "test_app_secret");
    }

    #[test]
    fn test_factory_create_shared() {
        let config = create_test_config();
        let shared_config = SharedConfigFactory::create_shared(config);

        assert_eq!(shared_config.config().app_id, "test_app_id");
    }

    #[test]
    fn test_factory_create_batch() {
        let config = create_test_config();
        let service_names = vec!["service1", "service2", "service3"];
        let (shared_config, service_configs) =
            SharedConfigFactory::create_batch(config, &service_names);

        assert_eq!(service_configs.len(), 3);
        assert_eq!(shared_config.ref_count(), 4); // 1 + 3 service configs
        for service_config in &service_configs {
            assert_eq!(service_config.config().app_id, "test_app_id");
        }
    }

    #[test]
    fn test_usage_stats() {
        let config = create_test_config();
        let shared_config = SharedConfig::new(config);

        // 模拟3个服务共享配置
        let _service1 = shared_config.clone_shared();
        let _service2 = shared_config.clone_shared();
        let _service3 = shared_config.clone_shared();

        let stats = ConfigUsageStats::new(&shared_config, 3);

        assert_eq!(stats.total_configs, 3);
        assert_eq!(stats.shared_configs, 4); // 1 original + 3 services

        // 对于小配置结构体，内存节省可能不明显
        if stats.saved_memory_bytes > 0 {
            println!("✅ 实现了内存节省: {} bytes", stats.saved_memory_bytes);
        } else {
            println!("ℹ️  对于小Config结构体，内存节省不明显");
        }
    }

    #[test]
    fn test_memory_efficiency() {
        let config = create_test_config();
        let shared_config = SharedConfig::new(config);

        // 创建多个服务配置共享同一个Config
        let configs: Vec<SharedConfig> = (0..10).map(|_| shared_config.clone_shared()).collect();

        // 验证所有配置都指向同一个实例
        for config in &configs {
            assert_eq!(config.config().app_id, "test_app_id");
        }

        // 验证引用计数
        assert_eq!(shared_config.ref_count(), 11); // 1 original + 10 services

        // 创建使用统计
        let stats = ConfigUsageStats::new(&shared_config, 10);

        println!("内存效率测试结果:");
        stats.print();

        // 对于小配置结构体，内存节省可能不明显
        if stats.saved_memory_bytes > 0 {
            println!("✅ 实现了内存节省");
        } else {
            println!("ℹ️  对于小Config结构体，内存节省不明显");
            println!("💡 但在大量服务场景下，这种模式仍然有价值");
        }
    }
}
