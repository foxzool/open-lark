//! 🔥 OpenLark Client Feature Loader
//!
//! 根据feature标志动态加载和配置服务

use crate::{Config, DefaultServiceRegistry, Result}; // 移除未使用的导入

/// 🔥 功能加载器 - 编译时feature驱动加载
///
/// 根据feature标志动态加载crates，提供类型安全的服务发现
#[derive(Debug, Clone, Copy)]
pub struct FeatureLoader;

impl FeatureLoader {
    /// 🚀 加载所有启用的服务
    pub async fn load_services(
        &self,
        _config: &Config,
        registry: &mut DefaultServiceRegistry,
    ) -> Result<()> {
        tracing::debug!("开始加载启用的服务");

        // 注册核心层服务
        self.register_core_services(registry)?;

        // 注册专业层服务
        self.register_professional_services(registry)?;

        // 注册企业层服务
        self.register_enterprise_services(registry)?;

        tracing::info!("所有启用的服务加载完成");
        Ok(())
    }

    /// 注册核心层服务
    fn register_core_services(&self, _registry: &mut DefaultServiceRegistry) -> Result<()> {
        // #[cfg(feature = "auth")]  // auth 功能暂未启用
        // {
        //     tracing::debug!("注册认证服务");
        //     let metadata = ServiceMetadata {
        //         name: "auth".to_string(),
        //         version: "1.0.0".to_string(),
        //         description: Some("飞书认证服务，提供令牌管理、身份验证等功能".to_string()),
        //         dependencies: vec![],
        //         provides: vec!["token-management".to_string()],
        //         status: ServiceStatus::Uninitialized,
        //         priority: 1,
        //     };
        //     registry.register_service(metadata)?;
        // }

        Ok(())
    }

    /// 注册专业层服务
    fn register_professional_services(&self, _registry: &mut DefaultServiceRegistry) -> Result<()> {
        // 在这里添加专业层服务的注册逻辑
        // 由于与 client.rs 中的逻辑重复，这里暂时简化
        Ok(())
    }

    /// 注册企业层服务
    fn register_enterprise_services(&self, _registry: &mut DefaultServiceRegistry) -> Result<()> {
        // 在这里添加企业层服务的注册逻辑
        // 由于与 client.rs 中的逻辑重复，这里暂时简化
        Ok(())
    }
}

/// 功能集统计信息
#[derive(Debug, Clone, Copy)]
pub struct FeatureStats {
    /// 启用的功能数量
    pub enabled_features: usize,
    /// 可用的功能数量
    pub available_features: usize,
    /// 已加载的服务数量
    pub loaded_services: usize,
}

/// 功能集合管理器
#[derive(Debug)]
pub struct FeatureSet {
    /// 启用的功能列表
    pub enabled_features: Vec<String>,
    /// 功能统计
    pub stats: FeatureStats,
}

impl FeatureSet {
    /// 创建新的功能集合
    pub fn new() -> Self {
        Self {
            enabled_features: vec![],
            stats: FeatureStats {
                enabled_features: 0,
                available_features: 0,
                loaded_services: 0,
            },
        }
    }

    /// 获取当前启用的功能
    pub fn get_enabled_features(&self) -> &[String] {
        &self.enabled_features
    }

    /// 检查功能是否启用
    pub fn is_enabled(&self, feature: &str) -> bool {
        self.enabled_features.contains(&feature.to_string())
    }

    /// 获取功能统计信息
    pub fn get_stats(&self) -> &FeatureStats {
        &self.stats
    }
}

impl Default for FeatureSet {
    fn default() -> Self {
        Self::new()
    }
}
