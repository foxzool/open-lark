// core v1 contracts - 合同管理API,
//,
// 实现合同管理的功能,
use crate::prelude::*;
/// 合同管理服务
#[derive(.*?)]
pub struct ContractsService {
    client: std::sync::Arc<LarkClient>,
}
impl ContractsService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client },
}
}