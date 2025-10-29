// sheets v2 range - 范围操作API,
//,
// 实现单元格范围的操作,
use crate::prelude::*;
/// 范围操作服务
#[derive(.*?)]
pub struct RangeService {
    client: std::sync::Arc<LarkClient>,
}
impl RangeService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client },
}
}