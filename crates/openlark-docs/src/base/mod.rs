//! Base module for OpenLark Docs
//!
//! This module provides base functionality for document operations,
//! including fundamental document types and shared utilities.

use crate::prelude::*;

/// Base服务
pub struct BaseService {
    #[allow(dead_code)] // 配置保留供将来使用
    config: Config,
}

impl BaseService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取基础配置
    pub fn config(&self) -> &Config {
        &self.config
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
    fn test_service_functionality() {
        let config = openlark_core::config::Config::default();
        let base_service = BaseService::new(config);

        // 测试基础功能
        assert!(!base_service.config().app_id.is_empty());
        println!("BaseService 功能测试通过");
    }
}
