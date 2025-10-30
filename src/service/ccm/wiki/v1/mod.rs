// wiki v1 - 知识库v1版本API
//,
// 包含知识库的基础功能
use crate::prelude::*;
/// 知识库v1版本服务
#[derive(Debug, Clone)]
pub struct WikiV1Service {
    client: std::sync::Arc<LarkClient>,
}
impl WikiV1Service {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}