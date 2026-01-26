/// 多维表格服务模块
///
/// 提供多维表格应用、数据表、视图管理等功能。
use openlark_core::config::Config;

pub mod v1;

// 使用通配符导出所有子模块,避免维护大量重复的导出列表
pub use v1::*;

/// Bitable 服务
#[derive(Debug, Clone)]
pub struct BitableService {
    config: Config,
}

impl BitableService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
