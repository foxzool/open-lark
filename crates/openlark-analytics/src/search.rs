//! 搜索服务模块
//!
//! 提供全文搜索、智能搜索等功能 (14 APIs)

use crate::AnalyticsConfig;
use std::sync::Arc;

/// 搜索服务
///
/// 提供搜索服务相关 API 的访问入口。
#[derive(Debug, Clone)]
pub struct SearchService {
    /// 客户端配置
    config: Arc<AnalyticsConfig>,
}

impl SearchService {
    /// 创建新的搜索服务实例
    pub fn new(config: Arc<AnalyticsConfig>) -> Self {
        Self { config }
    }

    /// 获取客户端配置
    pub fn config(&self) -> Arc<AnalyticsConfig> {
        self.config.clone()
    }

    /// V1 版本 API
    #[cfg(feature = "v1")]
    pub fn v1(&self) -> crate::search::v1::SearchV1 {
        crate::search::v1::SearchV1::new(self.config.clone(), self.client.clone())
    }
}

#[cfg(feature = "v1")]
pub mod v1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let config = AnalyticsConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let client = LarkClient::new(config.clone()).unwrap();
        let service = SearchService::new(config, client);
        assert_eq!(service.config().app_id, "test_app_id");
    }
}
