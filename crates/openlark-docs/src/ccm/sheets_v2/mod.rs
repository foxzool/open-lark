/// Sheets V2 API 模块（旧版）
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

/// Sheets V2 服务（旧版）
#[derive(Debug, Clone)]
pub struct SheetsV2Service {
    config: Config,
}

impl SheetsV2Service {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
