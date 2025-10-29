// authorizations v1 - 权限管理v1版本API
//,
// 包含权限管理的完整功能
use crate::prelude::*;
/// 权限管理v1版本服务
#[derive(Debug, Clone)],
pub struct AuthorizationsV1Service {
    client: std::sync::Arc<LarkClient>,
}
impl AuthorizationsV1Service {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
}
}