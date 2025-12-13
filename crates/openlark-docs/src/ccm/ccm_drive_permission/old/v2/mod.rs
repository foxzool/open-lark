/// CCM Drive Permission API Old V2 模块
///
/// 包含所有文档权限管理相关的API实现

use openlark_core::config::Config;

/// 文档权限管理服务
#[derive(Debug, Clone)]
pub struct CcmDrivePermissionOldV2 {
    config: Config,
}

impl CcmDrivePermissionOldV2 {
    /// 创建新的文档权限管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 判断协作者是否有某权限
    pub fn member_permitted(&self) -> member_permitted::CheckMemberPermittedRequest {
        member_permitted::CheckMemberPermittedRequest::new(self.config.clone())
    }

    /// 转移拥有者
    pub fn member_transfer(&self) -> member_transfer::TransferOwnershipRequest {
        member_transfer::TransferOwnershipRequest::new(self.config.clone())
    }

    /// 获取云文档权限设置V2
    pub fn public(&self) -> public::GetPublicPermissionRequest {
        public::GetPublicPermissionRequest::new(self.config.clone())
    }
}

// 导出所有API模块
pub mod member_permitted;
pub mod member_transfer;
pub mod public;