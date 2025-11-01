// sheets v3 - 电子表格v3版本API
//,
// 包含电子表格的扩展功能
use crate::prelude::*;
/// 电子表格v3版本服务
#[derive(Debug, Clone)]
pub struct SheetsV3Service {
    client: std::sync::Arc<LarkClient>,
}
impl SheetsV3Service {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}