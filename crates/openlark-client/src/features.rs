//! 🔥 OpenLark Client Feature Loader
//!
//! 根据feature标志动态加载和配置服务

use crate::{Config, DefaultServiceRegistry, Result};

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
        crate::registry::bootstrap::register_compiled_services(registry)?;

        tracing::info!("所有启用的服务加载完成");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_loader_debug() {
        let loader = FeatureLoader;
        let debug_str = format!("{:?}", loader);
        assert!(debug_str.contains("FeatureLoader"));
    }
}
