// rooms default - 会议室管理default版本API
//,
// 包含会议室管理的完整功能
use crate::core::config::Config;
/// 会议室管理default版本服务
#[derive(.*?)]
pub struct RoomsDefaultService {
    config: Config,
}
impl RoomsDefaultService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}