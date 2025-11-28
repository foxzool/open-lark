//! # Baike 知识库服务
//!
//! 提供企业知识库、Wiki管理功能。

use openlark_core::config::Config;

/// 知识库服务
#[derive(Debug, Clone)]
pub struct BaikeService {
    config: Config,
}

impl BaikeService {
    /// 创建新的知识库服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置信息
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取v1版本的API服务
    pub fn v1(&self) -> v1::BaikeV1Service {
        v1::BaikeV1Service::new(self.config.clone())
    }
}

// 版本模块
pub mod v1;

// 重新导出常用类型
pub use v1::BaikeV1Service;

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::{Config, ConfigInner};

    #[test]
    fn test_baike_service_creation() {
        let config = Config::new(ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });
        let service = BaikeService::new(config.clone());

        assert_eq!(service.config().app_id, "test_app_id");
        assert_eq!(service.config().app_secret, "test_app_secret");
    }
}
