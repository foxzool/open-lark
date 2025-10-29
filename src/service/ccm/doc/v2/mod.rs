// doc v2 - 文档v2版本API
//,
// 提供文档v2版本的功能
use crate::prelude::*;
/// 文档v2版本服务
#[derive(.*?)]
pub struct DocV2Service {
    client: std::sync::Arc<LarkClient>,
}
impl DocV2Service {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
}
}