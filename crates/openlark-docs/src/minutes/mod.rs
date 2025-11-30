//! 会议纪要服务模块
//!
//! 提供会议记录管理功能。

use openlark_core::config::Config;

/// 会议纪要服务
#[derive(Debug, Clone)]
pub struct MinutesService {
    /// 配置信息
    config: Config,
}

impl MinutesService {
    /// 创建新的会议纪要服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minutes_service_creation() {
        let config = Config::new("test_app_id", "test_app_secret");
        let service = MinutesService::new(config.clone());

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }
}
