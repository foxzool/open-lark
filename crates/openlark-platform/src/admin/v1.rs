//! 系统管理 V1 API
//!
//! 提供系统管理 V1 版本的 API 访问

use crate::PlatformConfig;
use std::sync::Arc;

/// 系统管理 V1 API
#[derive(Debug, Clone)]
pub struct AdminV1 {
    /// 客户端配置
    config: Arc<PlatformConfig>,
}

impl AdminV1 {
    /// 创建新的系统管理 V1 实例
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }

    /// 系统设置
    pub fn settings(&self) -> super::v1::settings::SettingsApi {
        super::v1::settings::SettingsApi::new(self.config.clone(), self.client.clone())
    }

    /// 用户管理
    pub fn users(&self) -> super::v1::users::UsersApi {
        super::v1::users::UsersApi::new(self.config.clone(), self.client.clone())
    }

    /// 审计日志
    pub fn audit(&self) -> super::v1::audit::AuditApi {
        super::v1::audit::AuditApi::new(self.config.clone(), self.client.clone())
    }
}

pub mod audit;
pub mod settings;
pub mod users;
