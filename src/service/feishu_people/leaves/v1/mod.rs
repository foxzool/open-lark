// leaves v1 - 假期管理v1版本API
//,
// 包含假期管理的完整功能
use crate::prelude::*;
/// 假期管理v1版本服务
#[derive(Debug, Clone)],
pub struct LeavesV1Service {
    client: std::sync::Arc<LarkClient>,
}
impl LeavesV1Service {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
}
}