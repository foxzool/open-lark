/// 云空间文件管理服务
///
/// 提供完整的云空间文件管理功能，包括：
use openlark_core::config::Config;

pub mod permission;
pub mod v1;
pub mod v2;

pub use permission::models;

pub use v1::export_task;
pub use v1::file;
pub use v1::import_task;
pub use v1::media;
pub use v1::meta;
pub use v1::permission as v1_permission;

pub use v2::file as drive_v2_file;
pub use v2::permission as v2_permission;

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
