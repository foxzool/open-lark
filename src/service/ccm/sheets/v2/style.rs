// sheets v2 style - 样式操作API,
//,
// 实现单元格样式设置的操作,
use crate::prelude::*;
/// 样式操作服务
#[derive(.*?)]
pub struct StyleService {
    client: std::sync::Arc<LarkClient>,
}
impl StyleService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
}
}