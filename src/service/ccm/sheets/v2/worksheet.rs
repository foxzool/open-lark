// sheets v2 worksheet - 工作表操作API,
//,
// 实现工作表级别的操作,
use crate::prelude::*;
/// 工作表操作服务
#[derive(.*?)]
pub struct WorksheetService {
    client: std::sync::Arc<LarkClient>,
}
impl WorksheetService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
}
}