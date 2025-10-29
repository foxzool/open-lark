// permissions - 权限管理服务
//,
// 提供权限管理相关的功能
use crate::prelude::*;
use crate::service::app_engine::permissions::v1::PermissionsV1Service;
/// 权限管理服务
#[derive(Debug, Clone)],
pub struct PermissionsService {
    /// v1版本API服务
    pub v1: PermissionsV1Service,
}
impl PermissionsService {
    /// 创建新的权限管理服务实例
pub fn new() -> Self {
        Self {
            v1: PermissionsV1Service::new(client.clone()),
        }
}
}
/// v1版本API
pub mod v1;