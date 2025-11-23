//! 会议纪要服务模块
//!
//! 提供会议记录管理功能。

use crate::error::{DocsError, DocsResult};
use openlark_core::{Config, HttpClient};
use std::sync::Arc;

/// 会议纪要服务
#[derive(Debug, Clone)]
pub struct MinutesService {
    /// 配置信息
    config: Arc<Config>,
    /// HTTP客户端
    http_client: Arc<HttpClient>,
}

impl MinutesService {
    /// 创建新的会议纪要服务实例
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            http_client: Arc::new(HttpClient::new(&config)),
        }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取HTTP客户端引用
    pub fn http_client(&self) -> &HttpClient {
        &self.http_client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minutes_service_creation() {
        let config = Arc::new(Config::new("test_app_id", "test_app_secret"));
        let service = MinutesService::new(config.clone());

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }
}
