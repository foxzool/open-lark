/// 云空间文件管理服务
///
/// 提供完整的云空间文件管理功能，包括：
/// - 文件操作（上传、下载、删除、元数据获取）
/// - 文件夹管理（创建、移动、重命名、删除）
/// - 权限管理（设置、查询、删除权限）
/// - 文件版本管理（版本历史、恢复）
/// - 导入导出任务（批量操作、异步处理）
/// - 统计信息（文件数量、空间使用等）
/// - 事件订阅（文件变更通知）
/// - 收藏和查看记录
/// - 媒体文件处理
use openlark_core::config::Config;

// 导入子模块
pub mod permission;

// 版本化API
pub mod v1;
pub mod v2;

// 重新导出permission模块
pub use permission::PermissionService;

/// 云空间文件管理服务
#[derive(Debug, Clone)]
pub struct DriveService {
    config: Config,
}

impl DriveService {
    /// 创建新的云空间文件管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    // 获取V1版本API
    // TODO: 实现DriveV1Service
    // pub fn v1(&self) -> crate::ccm::drive::v1::DriveV1Service {
    //     crate::ccm::drive::v1::DriveV1Service::new(self.config.clone())
    // }

    // 获取V2版本API
    // TODO: 实现DriveV2Service
    // pub fn v2(&self) -> crate::ccm::drive::v2::DriveV2Service {
    //     crate::ccm::drive::v2::DriveV2Service::new(self.config.clone())
    // }

    /// 获取权限管理服务
    pub fn permission(&self) -> crate::ccm::drive::permission::PermissionService {
        crate::ccm::drive::permission::PermissionService::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::DriveService;
    use openlark_core::config::Config;

    #[test]
    fn test_drive_service_creation() {
        let service = DriveService::new(Config::default());
        assert!(!service.config().base_url.is_empty());
    }
}
