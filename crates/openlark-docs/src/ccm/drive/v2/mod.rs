/// Drive v2 API 模块
///
/// 提供云空间文件管理的增强功能
use openlark_core::config::Config;

pub mod file;
pub mod permission;

// 重新导出所有模块
pub use file::*;
pub use permission::*;

/// Drive V2 服务
#[derive(Debug, Clone)]
pub struct DriveV2Service {
    config: Config,
}

impl DriveV2Service {
    /// 创建新的 Drive V2 服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}
