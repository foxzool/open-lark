//! ccm_sheet（旧版）API 模块
//!
//! 对应 `api_list_export.csv` 中：
//! - bizTag = ccm
//! - meta.Project = ccm_sheet
//! - meta.Version = old

use openlark_core::config::Config;

/// 旧版表格服务（old）
#[derive(Debug, Clone)]
pub struct CcmSheetOldService {
    config: Config,
}

impl CcmSheetOldService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

pub mod default;
pub use default::*;

// 保留旧的 v2 命名入口（兼容性），但不再作为主入口使用
pub mod v2;
