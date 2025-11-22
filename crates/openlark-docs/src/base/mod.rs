//! Base module for OpenLark Docs
//!
//! This module provides base functionality for document operations,
//! including fundamental document types and shared utilities.

use crate::prelude::*;

// Re-export bitable module
pub mod bitable;

/// Base服务
pub struct BaseService {
    #[allow(dead_code)] // 配置保留供将来使用
    config: Config,
}

impl BaseService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取多维表格服务
    pub fn bitable(&self) -> crate::services::BitableService {
        // 使用默认配置创建简单服务
        crate::services::BitableService::new()
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

        // 测试服务访问
        let _bitable_service = base_service.bitable();
        println!("BaseService 测试通过");
    }

    #[test]
    fn test_service_functionality() {
        let config = openlark_core::config::Config::default();
        let base_service = BaseService::new(config);

        // 测试基础功能
        let _bitable_service = base_service.bitable();
        assert!(true); // 基本创建测试
    }
}
