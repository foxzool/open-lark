//! ccm_drive_permission（旧版）API 模块
//!
//! 对应 `api_list_export.csv` 中：
//! - bizTag = ccm
//! - meta.Project = ccm_drive_permission
//! - meta.Version = old

use openlark_core::config::Config;

/// 旧版云文档权限服务（old）
#[derive(Debug, Clone)]
pub struct CcmDrivePermissionOldService {
    config: Config,
}

impl CcmDrivePermissionOldService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

pub mod default;
pub use default::*;
