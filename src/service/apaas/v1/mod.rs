//! APAAS应用开放平台服务 v1
//!
//! 提供飞书应用开放平台的完整管理功能，包括：
//! - 应用信息查询和管理
//! - 应用权限和配置管理
//! - 应用数据分析和监控
//! - 多租户应用部署

pub mod apps;

// 重新导出所有服务类型
pub use apps::*;

use crate::config::Config;

/// APAAS应用开放平台服务 v1版本
///
/// 提供飞书应用开放平台的统一入口，支持应用的全生命周期管理。
/// 包括应用创建、配置、部署、监控等企业级功能。
#[derive(Debug, Clone)]
pub struct ApaasServiceV1 {
    config: Config,
    /// 应用管理服务
    pub apps: AppsService,
}

impl ApaasServiceV1 {
    /// 创建APAAS v1服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::apaas::v1::ApaasServiceV1;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = ApaasServiceV1::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            apps: AppsService::new(config),
        }
    }
}

impl crate::core::trait_system::Service for ApaasServiceV1 {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "ApaasServiceV1"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trait_system::Service;

    #[test]
    fn test_apaas_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = ApaasServiceV1::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_service_trait_implementation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = ApaasServiceV1::new(config);

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id, "test_app_id");
    }
}
