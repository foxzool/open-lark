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

    /// 获取客户端配置。
    pub fn config(&self) -> Arc<UserConfig> {
        self.config.clone()
    }

    /// 获取 V1 版本 API 入口。
    #[cfg(feature = "v1")]
    pub fn v1(&self) -> crate::settings::v1::SettingsV1 {
        crate::settings::v1::SettingsV1::new(self.config.clone())
    }
}

#[cfg(feature = "v1")]
/// 个人设置 v1 API。
pub mod v1;

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let config = UserConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = SettingsService::new(std::sync::Arc::new(config));
        // UserConfig 实现了 Deref，可以直接访问 app_id
        assert_eq!(service.config().app_id(), "test_app_id");
    }
}
