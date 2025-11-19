//! Base module for OpenLark Docs
//!
//! This module provides base functionality for document operations,
//! including fundamental document types and shared utilities.
//!
//! Base模块包含：
//! - bitable：多维表格核心功能（49个API）
//! - 基础文档类型和共享工具

use crate::prelude::*;

pub mod bitable;

/// Base服务
pub struct BaseService {
    client: std::sync::Arc<LarkClient>,
}

impl BaseService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
    }

    /// 获取多维表格服务
    pub fn bitable(&self) -> bitable::BitableService {
        bitable::BitableService::new(openlark_core::config::Config::default())
    }
}

impl std::ops::Deref for BaseService {
    type Target = LarkClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_base_service_creation() {
        // This is a placeholder test
        // In a real implementation, you would create a mock client
        // and test the BaseService functionality
    }

    #[test]
    fn test_bitable_service_access() {
        use openlark_core::config::Config;
        let config = Config::default();
        let client = std::sync::Arc::new(LarkClient::new(config));
        let base_service = BaseService::new(client);

        // 测试可以访问bitable服务
        let _bitable_service = base_service.bitable();
    }
}
