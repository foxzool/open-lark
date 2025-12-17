//! ccm_drive_explorer（旧版）API 模块
//!
//! 对应 `api_list_export.csv` 中：
//! - bizTag = ccm
//! - meta.Project = ccm_drive_explorer
//! - meta.Version = old

use openlark_core::config::Config;

/// 旧版云盘浏览器服务（old）
#[derive(Debug, Clone)]
pub struct CcmDriveExplorerOldService {
    config: Config,
}

impl CcmDriveExplorerOldService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

pub mod default;
pub use default::*;
