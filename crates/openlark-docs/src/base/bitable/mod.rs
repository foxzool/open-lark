//! 多维表格服务
//!
//! 基础服务架构，具体功能在后续版本中实现。

use openlark_core::{
    config::Config,
    trait_system::Service,
};

/// 多维表格服务
///
/// 基础服务架构，具体功能在后续版本中实现。
#[derive(Clone)]
pub struct BitableService {
    config: Config,
}

impl BitableService {
    /// 创建多维表格服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Service for BitableService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "BitableService"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::Service;

    #[test]
    fn test_bitable_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = BitableService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_service_trait_implementation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = BitableService::new(config);

        let config_ref = service.config();
        assert_eq!(config_ref.app_id, "test_app_id");
    }
}