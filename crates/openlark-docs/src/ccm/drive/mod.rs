//! 云空间文件管理服务
//!
//! 提供完整的云空间文件管理功能，包括：
//! - 文件操作（上传、下载、删除、元数据获取）
//! - 文件夹管理（创建、移动、重命名、删除）
//! - 权限管理（设置、查询、删除权限）
//! - 文件版本管理（版本历史、恢复）
//! - 导入导出任务（批量操作、异步处理）
//! - 统计信息（文件数量、空间使用等）
//! - 事件订阅（文件变更通知）
//! - 收藏和查看记录
//! - 媒体文件处理

use crate::prelude::*;

// 导入子模块
// pub mod explorer; // 暂时禁用，修复编译错误
pub mod permission;

// 版本化API
pub mod v1;
pub mod v2;

// 重新导出版本模块，避免名称冲突
// pub use v1::{}; // 暂时注释掉，避免未定义的导入

// pub use v2::{}; // 暂时注释掉，避免未定义的导入

// 重新导出permission模块，避免名称冲突
// pub use explorer::ExplorerService; // 暂时禁用
pub use permission::PermissionService;

/// 云空间文件管理服务
#[derive(Clone, Debug)]
pub struct DriveService {
    #[allow(dead_code)] // 配置保留供将来使用
    config: Config,
    /// V1版本API服务
    pub v1: v1::DriveV1Service,
    /// V2版本API服务
    pub v2: v2::DriveV2Service,
}

impl DriveService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v1: v1::DriveV1Service::new(config.clone()),
            v2: v2::DriveV2Service::new(config),
        }
    }
}

impl std::ops::Deref for DriveService {
    type Target = LarkClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drive_service_creation() {
        // This is a placeholder test
        // In a real implementation, you would create a mock client
        // and test the DriveService functionality
    }
}
