//! 用户偏好模块
//!
//! 提供个人偏好、自定义选项等功能

use crate::UserConfig;
use std::sync::Arc;

/// 用户偏好服务
///
/// 提供用户偏好相关 API 的访问入口。
#[derive(Debug, Clone)]
pub struct PreferencesService {
    /// 客户端配置
    config: Arc<UserConfig>,
}

impl PreferencesService {
    /// 创建新的用户偏好服务实例
    pub fn new(config: Arc<UserConfig>) -> Self {
        Self { config }
    }

    /// 获取客户端配置
    pub fn config(&self) -> Arc<UserConfig> {
        self.config.clone()
    }

    /// V1 版本 API
    #[cfg(feature = "v1")]
    pub fn v1(&self) -> crate::preferences::v1::PreferencesV1 {
        crate::preferences::v1::PreferencesV1::new(self.config.clone())
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
            .build();

        let service = PreferencesService::new(std::sync::Arc::new(config));
        // UserConfig 实现了 Deref，可以直接访问 app_id
        assert_eq!(service.config().app_id(), "test_app_id");
    }
}
