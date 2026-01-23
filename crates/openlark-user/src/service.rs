//! 用户设置服务
//!
//! 提供用户设置相关的服务入口

use crate::UserConfig;
use openlark_core::SDKResult;
use std::sync::Arc;

/// 用户设置服务
///
/// 提供个人设置、用户偏好等功能的统一入口。
#[derive(Debug, Clone)]
pub struct UserService {
    /// 客户端配置
    config: Arc<UserConfig>,
}

impl UserService {
    /// 创建新的用户设置服务实例
    ///
    /// # 参数
    ///
    /// * `config` - 用户设置服务配置
    ///
    /// # 返回
    ///
    /// 返回用户设置服务实例或错误
    pub fn new(config: UserConfig) -> SDKResult<Self> {
        Ok(Self {
            config: Arc::new(config),
        })
    }

    /// 获取客户端配置
    pub fn config(&self) -> Arc<UserConfig> {
        self.config.clone()
    }

    /// 个人设置
    ///
    /// 提供通知设置、隐私设置、界面设置等功能。
    #[cfg(feature = "settings")]
    pub fn settings(&self) -> crate::settings::SettingsService {
        crate::settings::SettingsService::new(self.config.clone())
    }

    /// 用户偏好
    ///
    /// 提供个人偏好、自定义选项等功能。
    #[cfg(feature = "preferences")]
    pub fn preferences(&self) -> crate::preferences::PreferencesService {
        crate::preferences::PreferencesService::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let config = UserConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = UserService::new(config);
        assert!(service.is_ok());
    }
}
