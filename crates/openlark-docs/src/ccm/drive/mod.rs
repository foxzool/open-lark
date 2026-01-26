/// 云空间文件管理服务
///
/// 提供完整的云空间文件管理功能，包括：
use openlark_core::config::Config;

pub mod permission;
pub mod v1;
pub mod v2;

// 使用通配符导出所有子模块,避免维护大量重复的导出列表
pub use permission::*;
pub use v1::*;
pub use v2::*;

/// Drive 服务
#[derive(Debug, Clone)]
pub struct DriveService {
    config: Config,
}

impl DriveService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
