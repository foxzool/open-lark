/// CCM Drive Permission v1 API 模块
///
/// 提供云盘权限管理相关的API功能,包括权限查询、拥有者转移等。
use openlark_core::config::Config;

pub mod permission;
pub mod v2;

// 使用通配符导出所有子模块
// pub use permission::*; // 模块为空
pub use v2::*;

/// CCM Drive Permission 服务
#[derive(Debug, Clone)]
pub struct CcmDrivePermissionService {
    config: Config,
}

impl CcmDrivePermissionService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
