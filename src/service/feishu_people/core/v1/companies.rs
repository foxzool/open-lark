// core v1 companies - 公司管理API,
//,
// 实现公司管理的功能,
use crate::prelude::*;
/// 公司管理服务
#[derive(Debug, Clone)],
pub struct CompaniesService {
    client: std::sync::Arc<LarkClient>,
}
impl CompaniesService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client },
}
}