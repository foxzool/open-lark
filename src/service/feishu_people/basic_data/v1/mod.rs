// basic_data v1 - 基础数据管理v1版本API
//,
// 包含基础数据管理的完整功能
use crate::prelude::*;
/// 基础数据管理v1版本服务
#[derive(Debug, Clone)]
pub struct BasicDataV1Service {
    client: std::sync::Arc<LarkClient>,
}
impl BasicDataV1Service {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}