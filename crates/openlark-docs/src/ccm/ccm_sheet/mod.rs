/// CCM Sheet API 模块
///
/// 电子表格操作API实现，包含全面的表格功能：
///
/// ## 数据读写API (8个)
use openlark_core::config::Config;

pub mod models;
pub mod v2;

// 使用通配符导出所有子模块
pub use models::*;
pub use v2::*;

/// CCM Sheet 服务
#[derive(Debug, Clone)]
pub struct CcmSheetService {
    config: Config,
}

impl CcmSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
