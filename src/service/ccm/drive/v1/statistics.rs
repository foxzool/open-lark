// drive v1 statistics - 统计服务,
//,
// 提供文件统计相关的功能,
use crate::prelude::*;
/// 统计服务
#[derive(Debug, Clone)]
pub struct StatisticsService {
    client: std::sync::Arc<LarkClient>,
}
impl StatisticsService {
    pub fn new(config: Config) -> Self {
        Self { config }
}