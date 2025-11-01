// rooms - 会议室管理服务
//,
// 提供会议室管理相关的功能
use crate::core::config::Config;
use crate::service::meeting_room::rooms::default::RoomsDefaultService;
/// 会议室管理服务
#[derive(Debug, Clone)]
pub struct RoomsService {
    /// default版本API服务
pub default: RoomsDefaultService,
}
impl RoomsService {
pub fn new(config: Config) -> Self {
        Self {
            default: RoomsDefaultService::new(config),
}
/// default版本API
pub mod default;
}
}