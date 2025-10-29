// authorizations - 权限管理服务
//,
// 提供权限管理相关的所有功能，包括：
// - 用户授权管理
// - 角色管理
// - 权限范围管理
use crate::prelude::*;
use crate::service::feishu_people::authorizations::v1::AuthorizationsV1Service;
/// 权限管理服务
#[derive(Debug, Clone)],
pub struct AuthorizationsService {
    /// v1版本API服务
    pub v1: AuthorizationsV1Service,
}
impl AuthorizationsService {
    /// 创建新的权限管理服务实例
pub fn new() -> Self {
        Self {
            v1: AuthorizationsV1Service::new(client.clone()),
        }
}
}
/// v1版本API
pub mod v1;