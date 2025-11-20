//! Base module for OpenLark Docs
//!
//! This module provides base functionality for document operations,
//! including fundamental document types and shared utilities.

use crate::prelude::*;

/// Base服务
pub struct BaseService {
    client: std::sync::Arc<LarkClient>,
}

impl BaseService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
    }

    /// 获取多维表格服务
    pub fn bitable(&self) -> crate::services::BitableService {
        // 使用默认配置创建简单服务
        crate::services::BitableService::new()
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
        // 创建测试配置
        let config = openlark_core::config::Config {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        // 创建适配器客户端
        let client = std::sync::Arc::new(LarkClient::new(config));
        let base_service = BaseService::new(client);

        // 测试服务访问
        let _bitable_service = base_service.bitable();
        println!("BaseService 测试通过");
    }

    #[test]
    fn test_deref_functionality() {
        let config = openlark_core::config::Config::default();
        let client = std::sync::Arc::new(LarkClient::new(config));
        let base_service = BaseService::new(client);

        // 测试 deref 功能
        assert!(base_service.is_configured() == false); // 默认配置应该是未配置状态
    }
}