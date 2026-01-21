//! 个人设置模块
//!
//! 提供通知设置、隐私设置、界面设置等功能

use crate::UserConfig;
use std::sync::Arc;

/// 个人设置服务
///
/// 提供个人设置相关 API 的访问入口。
#[derive(Debug, Clone)]
pub struct SettingsService {
    /// 客户端配置
    config: Arc<UserConfig>,
}

impl SettingsService {
    /// 创建新的个人设置服务实例
    pub fn new(config: Arc<UserConfig>) -> Self {
        Self { config }
    }

    /// 获取客户端配置
    pub fn config(&self) -> Arc<UserConfig> {
        self.config.clone()
    }

    /// V1 版本 API
    #[cfg(feature = "v1")]
    pub fn v1(&self) -> crate::settings::v1::SettingsV1 {
        crate::settings::v1::SettingsV1::new(self.config.clone(), self.client.clone())
    }
}

#[cfg(feature = "v1")]
pub mod v1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let config = UserConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let client = LarkClient::new(config.clone()).unwrap();
        let service = SettingsService::new(config, client);
        assert_eq!(service.config().app_id, "test_app_id");
    }
}
