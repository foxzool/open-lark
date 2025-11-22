//! Base module for OpenLark Docs
//!
//! This module provides base functionality for document operations,
//! including fundamental document types and shared utilities.

use openlark_core::config::Config;

// 导入v2模块
pub mod v2;

/// Base服务
pub struct BaseService {
    config: Config,
}

impl BaseService {
    /// 创建Base服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取v2版本服务
    pub fn v2(&self) -> v2::BaseV2Service {
        v2::BaseV2Service::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_base_service_creation() {
        // 使用默认配置创建BaseService
        let config = openlark_core::config::Config::default();

        // 创建BaseService
        let base_service = BaseService::new(config.clone());

        // 测试基础功能
        assert_eq!(base_service.config().app_id, config.app_id);
        println!("BaseService 测试通过");
    }

    #[test]
    fn test_v2_service_access() {
        let config = openlark_core::config::Config::default();
        let base_service = BaseService::new(config.clone());

        // 测试v2版本服务
        let v2_service = base_service.v2();
        assert_eq!(v2_service.config().app_id, config.app_id);
        println!("Base V2服务访问测试通过");
    }

    #[test]
    fn test_service_functionality() {
        let config = openlark_core::config::Config::default();
        let base_service = BaseService::new(config);

        // 测试基础功能
        assert!(!base_service.config().app_id.is_empty());
        println!("BaseService 功能测试通过");
    }
}
