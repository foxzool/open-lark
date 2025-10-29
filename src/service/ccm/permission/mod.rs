// permission - 文档权限管理API
//,
// 提供文档权限相关的功能
use crate::prelude::*;
/// 文档权限管理服务
#[derive(Debug, Clone)],
pub struct PermissionService {
    client: std::sync::Arc<LarkClient>,
}
impl PermissionService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
}
}