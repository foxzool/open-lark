// rooms - 会议室管理服务
//
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
    /// 创建新的会议室管理服务实例
    pub fn new(config: Config) -> Self {
        Self {
            default: RoomsDefaultService::new(config.clone()),
        }
    }
}

/// default版本API
pub mod default;