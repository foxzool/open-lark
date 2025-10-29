// core v1 persons - 人员管理API,
//,
// 实现人员管理的核心功能,
use crate::prelude::*;
/// 人员管理服务
#[derive(Debug, Clone)],
pub struct PersonsService {
    client: std::sync::Arc<LarkClient>,
}
impl PersonsService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client },
}
}