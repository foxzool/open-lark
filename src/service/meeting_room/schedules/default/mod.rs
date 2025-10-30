// schedules default - 会议室日程管理default版本API
//,
// 包含会议室日程管理的完整功能
use crate::core::config::Config;
/// 会议室日程管理default版本服务
#[derive(Debug, Clone)]
pub struct SchedulesDefaultService {
    config: Config,
}
impl SchedulesDefaultService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}