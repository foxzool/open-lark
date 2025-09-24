use crate::{core::config::Config, impl_full_service};

pub mod create;
pub mod delete;
pub mod get;
pub mod instance_view;
pub mod instances;
pub mod list;
pub mod patch;
pub mod reply;
pub mod search;
pub mod subscription;
pub mod unsubscription;

// 重新导出所有请求和响应类型

/// 日程管理服务
///
/// 提供日程的创建、删除、查询、更新、搜索等功能
pub struct CalendarEventService {
    pub config: Config,
}

// Service 抽象接入：Calendar v4 CalendarEventService
impl_full_service!(CalendarEventService, "calendar.event", "v4");

impl CalendarEventService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
