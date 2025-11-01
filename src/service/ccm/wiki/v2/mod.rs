// wiki v2 - 知识库v2版本API
//,
// 包含知识库的核心功能
use crate::prelude::*;
/// 知识库v2版本服务
#[derive(Debug, Clone)]
pub struct WikiV2Service {
    client: std::sync::Arc<LarkClient>,
}
impl WikiV2Service {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}