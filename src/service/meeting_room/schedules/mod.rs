// schedules - 会议室日程管理服务
//,
// 提供会议室日程管理相关的功能
use crate::core::config::Config;
use crate::service::meeting_room::schedules::default::SchedulesDefaultService;
/// 会议室日程管理服务
#[derive(Debug, Clone)]
pub struct SchedulesService {
    /// default版本API服务
pub default: SchedulesDefaultService,
}
impl SchedulesService {
pub fn new(config: Config) -> Self {
        Self {
            default: SchedulesDefaultService::new(config),
}
/// default版本API
pub mod default;
}
}