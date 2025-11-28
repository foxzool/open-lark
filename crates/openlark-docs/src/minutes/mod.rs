//! 会议纪要服务模块
//!
//! 提供会议记录管理功能。

//! # Minutes 会议纪要服务
//!
//! 提供会议纪要的创建、编辑、管理等功能。

use openlark_core::config::Config;

/// 会议纪要服务
#[derive(Debug, Clone)]
pub struct MinutesService {
    config: Config,
}

impl MinutesService {
    /// 创建新的会议纪要服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置信息
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取v1版本的API服务
    pub fn v1(&self) -> v1::MinutesV1Service {
        v1::MinutesV1Service::new(self.config.clone())
    }
}

// 版本模块
pub mod v1;

// 重新导出常用类型
pub use v1::MinutesV1Service;

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::{Config, ConfigInner};

    #[test]
    fn test_minutes_service_creation() {
        let config = Config::new(ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });
        let service = MinutesService::new(config.clone());

        assert_eq!(service.config().app_id, "test_app_id");
        assert_eq!(service.config().app_secret, "test_app_secret");
    }
}
