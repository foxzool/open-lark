//! CCM Drive Permission 模块
//!
//! 文档权限管理相关API实现，包含3个API：
//! - member_permitted: 判断协作者是否有某权限
//! - member_transfer: 转移拥有者
//! - public: 获取云文档权限设置V2

use openlark_core::config::Config;

/// CCM Drive Permission 服务
#[derive(Debug, Clone)]
pub struct CcmDrivePermissionService {
    config: Config,
}

impl CcmDrivePermissionService {
    /// 创建新的CCM Drive Permission服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取V2版本API
    pub fn v2(&self) -> CcmDrivePermissionV2 {
        CcmDrivePermissionV2::new(self.config.clone())
    }

    /// 获取旧版版本API（兼容性保留）
    pub fn old(&self) -> crate::ccm::ccm_drive_permission::old::v2::CcmDrivePermissionOldV2 {
        crate::ccm::ccm_drive_permission::old::v2::CcmDrivePermissionOldV2::new(self.config.clone())
    }
}

/// CCM Drive Permission V2 API访问器
#[derive(Debug, Clone)]
pub struct CcmDrivePermissionV2 {
    config: Config,
}

impl CcmDrivePermissionV2 {
    /// 创建新的V2 API访问器实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

// 导出版本模块
pub mod old;
pub mod v2;